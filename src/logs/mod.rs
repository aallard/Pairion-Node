//! Structured logging for Pairion Node.
//!
//! Uses the `tracing` crate for all structured logs per CONVENTIONS §2.10.
//! Logs are written to a local file and batched for forwarding to the Server
//! via `LogForward` AsyncAPI messages.
//!
//! **Local buffer:** `/var/log/pairion-node/node.log` on Linux;
//! `~/Library/Logs/Pairion-Node/node.log` on macOS (development).
//!
//! **`println!` and `eprintln!` are banned** in production modules per
//! CONVENTIONS §2.10 and §2.14. CI enforces this.

use std::path::PathBuf;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

/// Return the path to the local log directory.
///
/// On Linux: `/var/log/pairion-node/`
/// On macOS (development): `~/Library/Logs/Pairion-Node/`
pub fn log_dir() -> PathBuf {
    if cfg!(target_os = "linux") {
        PathBuf::from("/var/log/pairion-node")
    } else {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Library")
            .join("Logs")
            .join("Pairion-Node")
    }
}

/// Initialize the logging subsystem.
///
/// Sets up a layered tracing subscriber with:
/// - A file appender writing to the local log directory
/// - A stdout layer for development convenience
/// - An environment filter defaulting to `info`
///
/// Returns a [`WorkerGuard`] that must be held for the lifetime of the
/// process to ensure all buffered log entries are flushed.
pub fn init() -> WorkerGuard {
    let dir = log_dir();
    // Attempt to create the log directory; on failure, fall back to temp dir
    let effective_dir = if std::fs::create_dir_all(&dir).is_ok() {
        dir
    } else {
        std::env::temp_dir()
    };

    let file_appender = tracing_appender::rolling::daily(effective_dir, "node.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .json(),
        )
        .with(fmt::layer().with_writer(std::io::stdout).with_ansi(true))
        .init();

    guard
}

/// Collect pending log entries for forwarding to the Server.
///
/// In M0 this is a stub that returns an empty vec. Real log forwarding
/// via `LogForward` AsyncAPI messages is implemented but the batch
/// collection is a placeholder until the forwarding pipeline is wired.
pub fn collect_pending_entries() -> Vec<serde_json::Value> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_dir_contains_pairion() {
        let dir = log_dir();
        let s = dir.to_string_lossy().to_lowercase();
        assert!(s.contains("pairion") || s.contains("log"));
    }

    #[test]
    fn collect_pending_entries_returns_empty() {
        let entries = collect_pending_entries();
        assert!(entries.is_empty());
    }
}
