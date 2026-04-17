//! Heartbeat management for the WebSocket connection.
//!
//! Sends `HeartbeatPing` every 15 seconds and expects `HeartbeatPong` within
//! 30 seconds. On heartbeat timeout, the connection is considered lost.

use crate::ws::messages::HeartbeatPingPayload;
use std::time::{Duration, Instant};

/// Interval between heartbeat pings.
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);

/// Maximum time to wait for a pong before considering the connection dead.
pub const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(30);

/// Tracks heartbeat state for a WebSocket connection.
#[derive(Debug)]
pub struct HeartbeatTracker {
    /// When the last ping was sent.
    last_ping_sent: Option<Instant>,
    /// When the last pong was received.
    last_pong_received: Option<Instant>,
}

impl HeartbeatTracker {
    /// Create a new heartbeat tracker.
    pub fn new() -> Self {
        Self {
            last_ping_sent: None,
            last_pong_received: Some(Instant::now()),
        }
    }

    /// Check whether it is time to send a heartbeat ping.
    pub fn should_ping(&self) -> bool {
        match self.last_ping_sent {
            None => true,
            Some(last) => last.elapsed() >= HEARTBEAT_INTERVAL,
        }
    }

    /// Build a `HeartbeatPing` payload and record the send time.
    pub fn build_ping(&mut self) -> HeartbeatPingPayload {
        self.last_ping_sent = Some(Instant::now());
        HeartbeatPingPayload {
            msg_type: "HeartbeatPing".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Record that a pong was received.
    pub fn record_pong(&mut self) {
        self.last_pong_received = Some(Instant::now());
    }

    /// Check whether the heartbeat has timed out (no pong within the timeout window).
    pub fn is_timed_out(&self) -> bool {
        match self.last_pong_received {
            None => true,
            Some(last) => last.elapsed() >= HEARTBEAT_TIMEOUT,
        }
    }
}

impl Default for HeartbeatTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tracker_should_ping_immediately() {
        let tracker = HeartbeatTracker::new();
        assert!(tracker.should_ping());
    }

    #[test]
    fn tracker_should_not_ping_after_recent_ping() {
        let mut tracker = HeartbeatTracker::new();
        let _ping = tracker.build_ping();
        assert!(!tracker.should_ping());
    }

    #[test]
    fn tracker_not_timed_out_initially() {
        let tracker = HeartbeatTracker::new();
        assert!(!tracker.is_timed_out());
    }

    #[test]
    fn build_ping_sets_correct_type() {
        let mut tracker = HeartbeatTracker::new();
        let ping = tracker.build_ping();
        assert_eq!(ping.msg_type, "HeartbeatPing");
    }

    #[test]
    fn build_ping_timestamp_is_valid() {
        let mut tracker = HeartbeatTracker::new();
        let ping = tracker.build_ping();
        assert!(chrono::DateTime::parse_from_rfc3339(&ping.timestamp).is_ok());
    }

    #[test]
    fn record_pong_resets_timeout() {
        let mut tracker = HeartbeatTracker::new();
        tracker.last_pong_received = None;
        assert!(tracker.is_timed_out());
        tracker.record_pong();
        assert!(!tracker.is_timed_out());
    }

    #[test]
    fn default_is_same_as_new() {
        let a = HeartbeatTracker::new();
        let b = HeartbeatTracker::default();
        assert_eq!(a.should_ping(), b.should_ping());
        assert_eq!(a.is_timed_out(), b.is_timed_out());
    }
}
