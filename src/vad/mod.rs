//! Voice Activity Detection (VAD) for Pairion Node.
//!
//! Uses Silero VAD via ONNX Runtime to detect when the user has finished
//! speaking. Runs only after the wake word fires; terminates capture when
//! 800ms of silence is detected.
//!
//! In M0, this module is scaffolding only. Real VAD execution arrives at M6.

/// Silero VAD integration stub.
pub mod silero {
    /// Placeholder for the Silero VAD engine.
    pub struct SileroVad;

    impl SileroVad {
        /// Create a new VAD instance (stub).
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for SileroVad {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn vad_can_be_created() {
            let _vad = SileroVad::new();
        }

        #[test]
        fn vad_default() {
            let _vad = SileroVad;
        }
    }
}
