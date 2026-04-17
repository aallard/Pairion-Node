//! Audio pipeline for Pairion Node.
//!
//! Handles mic capture via ALSA/cpal, ring buffering, Opus encoding/decoding,
//! jitter buffering, and speaker playback. This is the Node's most
//! performance-critical subsystem.
//!
//! **Invariant (Architecture §16.3):** Raw audio is NEVER written to disk.
//! Zero exceptions. CI static-analysis flags filesystem writes in this module.
//!
//! **CONVENTIONS §2.4:** Audio callbacks are hard-real-time. No allocations,
//! no locks, no logging, no `.await` in the callback path.
//!
//! In M0, this module is scaffolding only. Real audio capture and playback
//! arrive at M1 and M6.

/// Audio capture stub (ALSA/cpal input). Real implementation at M6.
pub mod capture {
    /// Placeholder for the audio capture subsystem.
    pub struct AudioCapture;

    impl AudioCapture {
        /// Create a new audio capture instance (stub).
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for AudioCapture {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn capture_can_be_created() {
            let _capture = AudioCapture::new();
        }

        #[test]
        fn capture_default() {
            let _capture = AudioCapture;
        }
    }
}

/// Audio playback stub (ALSA/cpal output). Real implementation at M6.
pub mod playback {
    /// Placeholder for the audio playback subsystem.
    pub struct AudioPlayback;

    impl AudioPlayback {
        /// Create a new audio playback instance (stub).
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for AudioPlayback {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn playback_can_be_created() {
            let _playback = AudioPlayback::new();
        }

        #[test]
        fn playback_default() {
            let _playback = AudioPlayback;
        }
    }
}

/// Opus codec stub. Real implementation at M1.
pub mod opus_codec {
    /// Placeholder for the Opus encoder/decoder.
    pub struct OpusCodec;

    impl OpusCodec {
        /// Create a new Opus codec instance (stub).
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for OpusCodec {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn codec_can_be_created() {
            let _codec = OpusCodec::new();
        }
    }
}

/// Ring buffer stub for bridging real-time audio callbacks to async code.
pub mod ring_buffer {
    /// Placeholder for the lock-free ring buffer.
    pub struct RingBuffer;

    impl RingBuffer {
        /// Create a new ring buffer (stub).
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for RingBuffer {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn ring_buffer_can_be_created() {
            let _buf = RingBuffer::new();
        }
    }
}
