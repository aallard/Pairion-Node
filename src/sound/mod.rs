//! Sound-design cache for Pairion Node.
//!
//! Stores and plays prerendered sound-design samples (wake chime, thinking
//! thrum, acknowledgment tone, error cues, etc.). Samples are triggered by
//! the Server via `NodeSoundCommand` or locally during offline mode.
//!
//! In M0, the cache is empty and the playback interface is a mock. Real sound
//! samples ship at later milestones.

pub mod cache;
