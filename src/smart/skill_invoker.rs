//! Offline skill invocation runtime.
//!
//! Executes offline-capable skills locally on Smart-tier Nodes when the
//! Server is unreachable. Skills must be tagged `offlineCapable: true` to
//! be eligible.
//!
//! In M0, this is a type signature only. Real skill invocation arrives at M7.

use crate::capabilities::manifest::NodeCapabilities;

/// Result of an offline skill invocation.
pub struct SkillResult {
    /// The skill's text response.
    pub response: String,
    /// Whether the skill executed successfully.
    pub success: bool,
}

/// Invoke an offline-capable skill locally.
///
/// # Panics
///
/// Panics with `unimplemented!()` in M0. Checks capability at entry per
/// CONVENTIONS §2.7.
pub fn invoke(capabilities: &NodeCapabilities, _skill_id: &str, _input: &str) -> SkillResult {
    assert!(
        capabilities.local_stt,
        "Skill invoker called on Node without localStt capability"
    );
    unimplemented!("Offline skill invocation not implemented until M7")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::manifest::AiAccelerator;

    #[test]
    #[should_panic(expected = "Skill invoker called on Node without localStt")]
    fn invoke_rejects_dumb_tier() {
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
        let _result = invoke(&caps, "time", "what time is it");
    }

    #[test]
    #[should_panic(expected = "Offline skill invocation not implemented until M7")]
    fn invoke_unimplemented_for_smart() {
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
        let _result = invoke(&caps, "time", "what time is it");
    }
}
