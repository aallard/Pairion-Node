//! Hardware capability detector for Pairion Node.
//!
//! At boot, the detector probes the hardware environment and produces a
//! [`NodeCapabilities`] manifest. In M0 this returns a hardcoded Dumb-tier
//! manifest. Real hardware probing (ALSA enumeration, PCI device scanning for
//! Hailo, HailoRT initialization) arrives at M6.
//!
//! The detector interface is stable — M6 replaces the body of [`detect`], not
//! its signature.
//!
//! **Invariant (Architecture §16.1):** Capabilities are detected at runtime,
//! never at compile-time. The manifest returned here is the sole source of
//! truth for tier-specific behavior.

use super::manifest::{AiAccelerator, NodeCapabilities};

/// Detect hardware capabilities and return the manifest.
///
/// In M0, this returns a hardcoded Dumb-tier manifest:
/// - `audioIn: true`, `audioOut: true`
/// - `localWakeWord: true`, `localVad: true`
/// - `localStt: false`, `localLlmSmall: false`
/// - `localTtsCache: true`
/// - `aiAccelerator: "none"`, `dedicatedNpuRamGb: 0`
///
/// At M6, this function will probe real hardware: enumerate USB audio devices,
/// scan `/sys/bus/pci/devices/` for Hailo presence, query HailoRT, and check
/// for model files on disk.
pub fn detect() -> NodeCapabilities {
    NodeCapabilities {
        audio_in: true,
        audio_out: true,
        local_wake_word: true,
        local_vad: true,
        local_stt: false,
        local_llm_small: false,
        local_tts_cache: true,
        ai_accelerator: AiAccelerator::None,
        dedicated_npu_ram_gb: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::manifest::NodeTier;

    #[test]
    fn detect_returns_dumb_tier_manifest() {
        let manifest = detect();
        assert_eq!(manifest.derived_tier(), NodeTier::Dumb);
    }

    #[test]
    fn detect_returns_correct_dumb_flags() {
        let m = detect();
        assert!(m.audio_in);
        assert!(m.audio_out);
        assert!(m.local_wake_word);
        assert!(m.local_vad);
        assert!(!m.local_stt);
        assert!(!m.local_llm_small);
        assert!(m.local_tts_cache);
        assert_eq!(m.ai_accelerator, AiAccelerator::None);
        assert_eq!(m.dedicated_npu_ram_gb, 0);
    }
}
