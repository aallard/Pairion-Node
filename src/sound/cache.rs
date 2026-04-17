//! Sound sample cache.
//!
//! In M0 the cache is empty. The playback interface exists but uses a mock
//! implementation for testing. Real samples and playback arrive at M1/M6.

use std::collections::HashMap;

/// A cached sound sample.
#[derive(Debug, Clone)]
pub struct SoundSample {
    /// Unique sound identifier (e.g., "wake-chime", "acknowledgment-tone").
    pub id: String,
    /// Raw audio data (empty in M0).
    pub data: Vec<u8>,
}

/// Trait for sound playback (real or mock).
pub trait SoundPlayer: Send + Sync {
    /// Play a sound sample by id.
    fn play(&self, id: &str);
}

/// Mock sound player that records which sounds were played.
#[derive(Debug, Default)]
pub struct MockSoundPlayer {
    /// Recorded sound ids in order.
    pub played: std::sync::Mutex<Vec<String>>,
}

impl MockSoundPlayer {
    /// Create a new mock player.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the list of played sound ids.
    pub fn played_sounds(&self) -> Vec<String> {
        self.played.lock().unwrap().clone()
    }
}

impl SoundPlayer for MockSoundPlayer {
    fn play(&self, id: &str) {
        self.played.lock().unwrap().push(id.to_string());
    }
}

/// The sound sample cache.
#[derive(Debug, Default)]
pub struct SoundCache {
    samples: HashMap<String, SoundSample>,
}

impl SoundCache {
    /// Create a new empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a sample to the cache.
    pub fn insert(&mut self, sample: SoundSample) {
        self.samples.insert(sample.id.clone(), sample);
    }

    /// Look up a sample by id.
    pub fn get(&self, id: &str) -> Option<&SoundSample> {
        self.samples.get(id)
    }

    /// Return the number of cached samples.
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_starts_empty() {
        let cache = SoundCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn cache_insert_and_get() {
        let mut cache = SoundCache::new();
        cache.insert(SoundSample {
            id: "wake-chime".to_string(),
            data: vec![1, 2, 3],
        });
        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());
        let sample = cache.get("wake-chime").unwrap();
        assert_eq!(sample.data, vec![1, 2, 3]);
    }

    #[test]
    fn cache_get_missing() {
        let cache = SoundCache::new();
        assert!(cache.get("nonexistent").is_none());
    }

    #[test]
    fn mock_player_records_plays() {
        let player = MockSoundPlayer::new();
        player.play("wake-chime");
        player.play("acknowledgment-tone");
        assert_eq!(
            player.played_sounds(),
            vec!["wake-chime", "acknowledgment-tone"]
        );
    }
}
