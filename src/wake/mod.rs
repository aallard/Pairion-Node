//! Wake-word detection for Pairion Node.
//!
//! Uses openWakeWord (via ONNX Runtime) to detect the "Hey, Pairion" wake
//! phrase. Runs continuously on the Pi CPU regardless of tier.
//!
//! **Invariant (Architecture §16.6):** Wake-word runs always on Pi CPU.
//! Even on Smart tier. The Hailo is reserved for STT and mini-LLM.
//!
//! In M0, this module is scaffolding only. Real wake-word detection arrives at M6.

/// openWakeWord integration stub.
pub mod open_wake_word {
    /// Placeholder for the openWakeWord detector.
    pub struct WakeWordDetector;

    impl WakeWordDetector {
        /// Create a new wake-word detector (stub).
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for WakeWordDetector {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn detector_can_be_created() {
            let _detector = WakeWordDetector::new();
        }

        #[test]
        fn detector_default() {
            let _detector = WakeWordDetector;
        }
    }
}
