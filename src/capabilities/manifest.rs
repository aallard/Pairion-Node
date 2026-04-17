//! Capability manifest types for Pairion Node.
//!
//! The manifest describes what a Node can do, derived from hardware detection
//! at boot. It is read-only once constructed (CONVENTIONS §2.6) and sent to the
//! Server with `NodeIdentify`.
//!
//! **Invariant (Architecture §16.1):** Tier is derived from capabilities at
//! runtime, never from compile-time features. The manifest is the sole
//! branching mechanism for tier-specific behavior.

use serde::{Deserialize, Serialize};

/// The set of AI accelerator types a Node may report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiAccelerator {
    /// No AI accelerator present (Dumb tier).
    None,
    /// Hailo-10H NPU (AI HAT+ 2).
    Hailo10h,
    /// Hailo-8 NPU (AI HAT+ 26 TOPS).
    Hailo8,
    /// Hailo-8L NPU (AI HAT+ 13 TOPS).
    Hailo8l,
    /// An unrecognized accelerator.
    Other,
}

/// Hardware capability manifest reported by a Node.
///
/// Matches the `NodeCapabilitiesPayload` schema in `asyncapi.yaml`.
/// Once constructed, this struct is immutable for the lifetime of the process
/// (re-detected only on SIGHUP per CONVENTIONS §2.6).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeCapabilities {
    /// Whether the Node has audio input hardware.
    pub audio_in: bool,
    /// Whether the Node has audio output hardware.
    pub audio_out: bool,
    /// Whether local wake-word detection is available (always true).
    pub local_wake_word: bool,
    /// Whether local VAD is available (always true).
    pub local_vad: bool,
    /// Whether local STT is available (Smart tier only).
    pub local_stt: bool,
    /// Whether a local small LLM is available (Smart tier only).
    pub local_llm_small: bool,
    /// Whether a local TTS cache is available (always true).
    pub local_tts_cache: bool,
    /// The type of AI accelerator present.
    pub ai_accelerator: AiAccelerator,
    /// Dedicated NPU RAM in gigabytes.
    pub dedicated_npu_ram_gb: u32,
}

/// The derived tier of a Node based on its capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeTier {
    /// Dumb tier — no AI accelerator, no local STT/LLM.
    Dumb,
    /// Smart tier — AI accelerator present with local STT and/or LLM.
    Smart,
}

impl NodeCapabilities {
    /// Derive the tier from the capability manifest.
    ///
    /// A Node is Smart if it has an AI accelerator other than `None` and
    /// has local STT capability. Otherwise it is Dumb.
    pub fn derived_tier(&self) -> NodeTier {
        if self.ai_accelerator != AiAccelerator::None && self.local_stt {
            NodeTier::Smart
        } else {
            NodeTier::Dumb
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dumb_manifest_derives_dumb_tier() {
        let manifest = NodeCapabilities {
            audio_in: true,
            audio_out: true,
            local_wake_word: true,
            local_vad: true,
            local_stt: false,
            local_llm_small: false,
            local_tts_cache: true,
            ai_accelerator: AiAccelerator::None,
            dedicated_npu_ram_gb: 0,
        };
        assert_eq!(manifest.derived_tier(), NodeTier::Dumb);
    }

    #[test]
    fn smart_manifest_derives_smart_tier() {
        let manifest = NodeCapabilities {
            audio_in: true,
            audio_out: true,
            local_wake_word: true,
            local_vad: true,
            local_stt: true,
            local_llm_small: true,
            local_tts_cache: true,
            ai_accelerator: AiAccelerator::Hailo10h,
            dedicated_npu_ram_gb: 8,
        };
        assert_eq!(manifest.derived_tier(), NodeTier::Smart);
    }

    #[test]
    fn accelerator_without_stt_is_dumb() {
        let manifest = NodeCapabilities {
            audio_in: true,
            audio_out: true,
            local_wake_word: true,
            local_vad: true,
            local_stt: false,
            local_llm_small: false,
            local_tts_cache: true,
            ai_accelerator: AiAccelerator::Hailo10h,
            dedicated_npu_ram_gb: 8,
        };
        assert_eq!(manifest.derived_tier(), NodeTier::Dumb);
    }

    #[test]
    fn serialization_round_trip() {
        let manifest = NodeCapabilities {
            audio_in: true,
            audio_out: true,
            local_wake_word: true,
            local_vad: true,
            local_stt: false,
            local_llm_small: false,
            local_tts_cache: true,
            ai_accelerator: AiAccelerator::None,
            dedicated_npu_ram_gb: 0,
        };
        let json = serde_json::to_string(&manifest).unwrap();
        let deserialized: NodeCapabilities = serde_json::from_str(&json).unwrap();
        assert_eq!(manifest, deserialized);
    }

    #[test]
    fn json_field_names_are_camel_case() {
        let manifest = NodeCapabilities {
            audio_in: true,
            audio_out: true,
            local_wake_word: true,
            local_vad: true,
            local_stt: false,
            local_llm_small: false,
            local_tts_cache: true,
            ai_accelerator: AiAccelerator::None,
            dedicated_npu_ram_gb: 0,
        };
        let json = serde_json::to_string(&manifest).unwrap();
        assert!(json.contains("audioIn"));
        assert!(json.contains("localWakeWord"));
        assert!(json.contains("aiAccelerator"));
        assert!(json.contains("dedicatedNpuRamGb"));
    }
}
