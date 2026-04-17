//! Mini-LLM integration for offline response generation.
//!
//! A small instruct model running on the Hailo NPU for Smart-tier Nodes
//! in offline mode. Used sparingly — most offline responses come from the
//! skill layer, not the LLM.
//!
//! In M0, this is a type signature only. Real integration arrives at M7.

use crate::capabilities::manifest::NodeCapabilities;

/// Result of a local LLM generation.
pub struct GenerationResult {
    /// The generated text response.
    pub text: String,
}

/// Generate a response using the local mini-LLM on Hailo.
///
/// # Panics
///
/// Panics with `unimplemented!()` in M0. Checks capability at entry per
/// CONVENTIONS §2.7.
pub fn generate(capabilities: &NodeCapabilities, _prompt: &str) -> GenerationResult {
    assert!(
        capabilities.local_llm_small,
        "Mini-LLM generate called on Node without localLlmSmall capability"
    );
    unimplemented!("Mini-LLM integration not implemented until M7")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::manifest::AiAccelerator;

    #[test]
    #[should_panic(expected = "Mini-LLM generate called on Node without localLlmSmall")]
    fn generate_rejects_dumb_tier() {
        let caps = NodeCapabilities {
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
        let _result = generate(&caps, "test");
    }

    #[test]
    #[should_panic(expected = "Mini-LLM integration not implemented until M7")]
    fn generate_unimplemented_for_smart() {
        let caps = NodeCapabilities {
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
        let _result = generate(&caps, "test");
    }
}
