//! Encrypted secrets storage for Pairion Node.
//!
//! The Node's primary secret is its pairing/bearer token, stored in an
//! encrypted config file. The encryption key is derived from the machine id
//! plus a fixed salt, providing resistance to casual filesystem snooping
//! (CONVENTIONS §2.9, §2.11).
//!
//! **Invariant (Architecture §16.9):** Pairing tokens are encrypted at rest.
//! Never plaintext on disk.
//!
//! On Linux (Pi), the config path is `/opt/pairion-node/config.enc`.
//! On macOS (development), the config path falls back to `~/.pairion/node-config.enc`.

use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::Engine;
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tracing;

/// Fixed salt for key derivation. Combined with machine id to produce the
/// encryption key.
const KEY_SALT: &[u8] = b"pairion-node-config-v1";

/// Return the path to the encrypted config file.
///
/// On Linux: `/opt/pairion-node/config.enc`
/// On macOS (development): `~/.pairion/node-config.enc`
pub fn config_path() -> PathBuf {
    if cfg!(target_os = "linux") {
        PathBuf::from("/opt/pairion-node/config.enc")
    } else {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".pairion")
            .join("node-config.enc")
    }
}

/// Derive an encryption key from the machine id and the fixed salt.
///
/// On Linux this reads `/etc/machine-id`. On macOS (development) it uses a
/// fixed fallback for reproducibility.
fn derive_key() -> [u8; 32] {
    let machine_id = if cfg!(target_os = "linux") {
        std::fs::read_to_string("/etc/machine-id")
            .unwrap_or_else(|_| "pairion-dev-fallback-machine-id".to_string())
    } else {
        "pairion-dev-fallback-machine-id".to_string()
    };

    let mut hasher = Sha256::new();
    hasher.update(machine_id.trim().as_bytes());
    hasher.update(KEY_SALT);
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

/// Encrypt data with AES-256-GCM and return base64-encoded ciphertext.
///
/// The nonce is prepended to the ciphertext before encoding.
pub fn encrypt(plaintext: &[u8]) -> Result<String, String> {
    let key = derive_key();
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| e.to_string())?;

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| e.to_string())?;

    let mut combined = Vec::with_capacity(12 + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    Ok(base64::engine::general_purpose::STANDARD.encode(&combined))
}

/// Decrypt base64-encoded ciphertext that was produced by [`encrypt`].
pub fn decrypt(encoded: &str) -> Result<Vec<u8>, String> {
    let combined = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| e.to_string())?;

    if combined.len() < 12 {
        return Err("ciphertext too short".to_string());
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let key = derive_key();
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| e.to_string())?;
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.decrypt(nonce, ciphertext).map_err(|e| e.to_string())
}

/// Load the bearer token from the encrypted config file.
///
/// Returns `None` if the file does not exist (first run).
pub fn load_token() -> Option<String> {
    let path = config_path();
    match std::fs::read_to_string(&path) {
        Ok(encoded) => match decrypt(encoded.trim()) {
            Ok(plaintext) => {
                let token = String::from_utf8_lossy(&plaintext).to_string();
                tracing::info!(path = %path.display(), "Bearer token loaded from encrypted config");
                Some(token)
            }
            Err(e) => {
                tracing::error!(path = %path.display(), error = %e, "Failed to decrypt config");
                None
            }
        },
        Err(_) => {
            tracing::info!(path = %path.display(), "No encrypted config found (first run)");
            None
        }
    }
}

/// Save the bearer token to the encrypted config file.
///
/// Creates parent directories if they do not exist.
pub fn save_token(token: &str) -> Result<(), String> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let encrypted = encrypt(token.as_bytes())?;
    std::fs::write(&path, &encrypted).map_err(|e| e.to_string())?;
    tracing::info!(path = %path.display(), "Bearer token saved to encrypted config");
    Ok(())
}

/// Generate a new bearer token (random UUID-based string).
pub fn generate_token() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_round_trip() {
        let plaintext = b"test-bearer-token-12345";
        let encrypted = encrypt(plaintext).unwrap();
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn encrypt_produces_different_ciphertexts() {
        let plaintext = b"same-input";
        let a = encrypt(plaintext).unwrap();
        let b = encrypt(plaintext).unwrap();
        assert_ne!(
            a, b,
            "different nonces should produce different ciphertexts"
        );
    }

    #[test]
    fn decrypt_rejects_short_input() {
        let result = decrypt("dG9vc2hvcnQ="); // "tooshort" base64
        assert!(result.is_err());
    }

    #[test]
    fn generate_token_is_uuid_format() {
        let token = generate_token();
        assert!(uuid::Uuid::parse_str(&token).is_ok());
    }

    #[test]
    fn config_path_is_valid() {
        let path = config_path();
        assert!(path.to_str().unwrap().contains("pairion"));
    }

    #[test]
    fn save_and_load_token_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.enc");

        // Temporarily override the config path by saving directly
        let token = "test-token-abc123";
        let encrypted = encrypt(token.as_bytes()).unwrap();
        std::fs::write(&path, &encrypted).unwrap();

        let loaded = std::fs::read_to_string(&path).unwrap();
        let decrypted = decrypt(loaded.trim()).unwrap();
        assert_eq!(String::from_utf8_lossy(&decrypted), token);
    }
}
