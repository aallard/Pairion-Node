//! Whisper-small local STT integration.
//!
//! Runs Whisper-small on the Hailo NPU for offline speech-to-text.
//! Only available on Smart-tier Nodes with `localStt: true`.
//!
//! In M0, this is a type signature only. Real integration arrives at M7.

use crate::capabilities::manifest::NodeCapabilities;

/// Result of a local STT transcription.
pub struct TranscriptionResult {
    /// The transcribed text.
    pub text: String,
    /// Confidence score (0.0–1.0).
    pub confidence: f32,
}

/// Transcribe audio locally using Whisper-small on Hailo.
///
/// # Panics
///
/// Panics with `unimplemented!()` in M0. Checks capability at entry per
/// CONVENTIONS §2.7 — a Smart subsystem running on Dumb hardware is a bug.
pub fn transcribe(capabilities: &NodeCapabilities, _audio: &[u8]) -> TranscriptionResult {
    assert!(
        capabilities.local_stt,
        "Whisper-small transcribe called on Node without localStt capability"
    );
    unimplemented!("Whisper-small integration not implemented until M7")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::manifest::AiAccelerator;

    #[test]
    #[should_panic(expected = "Whisper-small transcribe called on Node without localStt")]
    fn transcribe_rejects_dumb_tier() {
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
        let _result = transcribe(&caps, &[]);
    }

    #[test]
    #[should_panic(expected = "Whisper-small integration not implemented until M7")]
    fn transcribe_unimplemented_for_smart() {
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
        let _result = transcribe(&caps, &[]);
    }
}
