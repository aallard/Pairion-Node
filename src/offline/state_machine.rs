//! Offline state machine for Pairion Node.
//!
//! Defines the four states from Architecture §9: `Online`, `Reconnecting`,
//! `OfflineDumb`, `OfflineSmart`. Transitions are implemented with unit tests
//! for correctness. In M0 the state machine compiles and tests pass but is
//! not yet wired to real heartbeat loss detection.
//!
//! **Invariant (Architecture §16.2):** Offline Smart mode treats all speakers
//! as Guests. Hardwired. Cannot be toggled.

use crate::capabilities::manifest::NodeTier;
use serde::{Deserialize, Serialize};

/// The offline state of a Pairion Node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfflineState {
    /// Server heartbeat responsive; normal operation.
    Online,
    /// Heartbeat timeout hit; attempting to reconnect.
    Reconnecting,
    /// Dumb tier; Server unreachable after retry window.
    OfflineDumb,
    /// Smart tier; Server unreachable; local STT + skill invocation.
    OfflineSmart,
}

/// Events that drive state transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineEvent {
    /// Heartbeat was missed beyond the detection timeout.
    HeartbeatLost,
    /// Heartbeat was successfully received.
    HeartbeatReceived,
    /// Extended retry window expired without reconnection.
    RetryWindowExpired {
        /// The tier of the Node, determining which offline state to enter.
        tier: NodeTier,
    },
}

/// The offline state machine.
#[derive(Debug)]
pub struct OfflineStateMachine {
    /// Current state.
    state: OfflineState,
}

impl OfflineStateMachine {
    /// Create a new state machine in the `Online` state.
    pub fn new() -> Self {
        Self {
            state: OfflineState::Online,
        }
    }

    /// Get the current state.
    pub fn state(&self) -> OfflineState {
        self.state
    }

    /// Process an event and transition to the next state.
    ///
    /// Returns the new state after the transition.
    pub fn transition(&mut self, event: OfflineEvent) -> OfflineState {
        self.state = match (self.state, event) {
            // Online → Reconnecting on heartbeat loss
            (OfflineState::Online, OfflineEvent::HeartbeatLost) => OfflineState::Reconnecting,

            // Reconnecting → Online on heartbeat received
            (OfflineState::Reconnecting, OfflineEvent::HeartbeatReceived) => OfflineState::Online,

            // Reconnecting → Offline (tier-dependent) on retry window expired
            (OfflineState::Reconnecting, OfflineEvent::RetryWindowExpired { tier }) => match tier {
                NodeTier::Dumb => OfflineState::OfflineDumb,
                NodeTier::Smart => OfflineState::OfflineSmart,
            },

            // OfflineDumb → Online on heartbeat received
            (OfflineState::OfflineDumb, OfflineEvent::HeartbeatReceived) => OfflineState::Online,

            // OfflineSmart → Online on heartbeat received
            (OfflineState::OfflineSmart, OfflineEvent::HeartbeatReceived) => OfflineState::Online,

            // All other transitions are no-ops (state unchanged)
            (current, _) => current,
        };
        self.state
    }
}

impl Default for OfflineStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_online() {
        let sm = OfflineStateMachine::new();
        assert_eq!(sm.state(), OfflineState::Online);
    }

    #[test]
    fn online_to_reconnecting_on_heartbeat_loss() {
        let mut sm = OfflineStateMachine::new();
        let new_state = sm.transition(OfflineEvent::HeartbeatLost);
        assert_eq!(new_state, OfflineState::Reconnecting);
    }

    #[test]
    fn reconnecting_to_online_on_heartbeat_received() {
        let mut sm = OfflineStateMachine::new();
        sm.transition(OfflineEvent::HeartbeatLost);
        let new_state = sm.transition(OfflineEvent::HeartbeatReceived);
        assert_eq!(new_state, OfflineState::Online);
    }

    #[test]
    fn reconnecting_to_offline_dumb_on_retry_expired() {
        let mut sm = OfflineStateMachine::new();
        sm.transition(OfflineEvent::HeartbeatLost);
        let new_state = sm.transition(OfflineEvent::RetryWindowExpired {
            tier: NodeTier::Dumb,
        });
        assert_eq!(new_state, OfflineState::OfflineDumb);
    }

    #[test]
    fn reconnecting_to_offline_smart_on_retry_expired() {
        let mut sm = OfflineStateMachine::new();
        sm.transition(OfflineEvent::HeartbeatLost);
        let new_state = sm.transition(OfflineEvent::RetryWindowExpired {
            tier: NodeTier::Smart,
        });
        assert_eq!(new_state, OfflineState::OfflineSmart);
    }

    #[test]
    fn offline_dumb_to_online_on_heartbeat_received() {
        let mut sm = OfflineStateMachine::new();
        sm.transition(OfflineEvent::HeartbeatLost);
        sm.transition(OfflineEvent::RetryWindowExpired {
            tier: NodeTier::Dumb,
        });
        let new_state = sm.transition(OfflineEvent::HeartbeatReceived);
        assert_eq!(new_state, OfflineState::Online);
    }

    #[test]
    fn offline_smart_to_online_on_heartbeat_received() {
        let mut sm = OfflineStateMachine::new();
        sm.transition(OfflineEvent::HeartbeatLost);
        sm.transition(OfflineEvent::RetryWindowExpired {
            tier: NodeTier::Smart,
        });
        let new_state = sm.transition(OfflineEvent::HeartbeatReceived);
        assert_eq!(new_state, OfflineState::Online);
    }

    #[test]
    fn online_ignores_heartbeat_received() {
        let mut sm = OfflineStateMachine::new();
        let new_state = sm.transition(OfflineEvent::HeartbeatReceived);
        assert_eq!(new_state, OfflineState::Online);
    }

    #[test]
    fn online_ignores_retry_expired() {
        let mut sm = OfflineStateMachine::new();
        let new_state = sm.transition(OfflineEvent::RetryWindowExpired {
            tier: NodeTier::Dumb,
        });
        assert_eq!(new_state, OfflineState::Online);
    }

    #[test]
    fn default_is_online() {
        let sm = OfflineStateMachine::default();
        assert_eq!(sm.state(), OfflineState::Online);
    }
}
