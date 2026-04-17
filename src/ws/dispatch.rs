//! Message dispatch layer for inbound WebSocket messages.
//!
//! Routes messages by their `type` discriminator field. In M0, the only
//! handled types are `IdentifyAck`, `HeartbeatPong`, and `Error`. Any other
//! type is logged at debug level and dropped. Later milestones populate
//! additional handlers.

use crate::ws::messages::{self, ErrorPayload, HeartbeatPongPayload, IdentifyAckPayload};
use tracing;

/// The result of dispatching a single inbound message.
#[derive(Debug, PartialEq)]
pub enum DispatchResult {
    /// Server accepted our identify.
    IdentifyAccepted {
        /// Server version string.
        server_version: String,
    },
    /// Server rejected our identify.
    IdentifyRejected {
        /// Reason for rejection.
        reason: String,
    },
    /// Heartbeat pong received.
    HeartbeatPong {
        /// Round-trip latency in milliseconds.
        latency_ms: f64,
    },
    /// Server sent an error.
    Error {
        /// Machine-readable error code.
        code: String,
        /// Human-readable message.
        message: String,
    },
    /// Message type not handled in M0; logged and dropped.
    Unhandled {
        /// The unhandled message type.
        msg_type: String,
    },
    /// Message had no type field or was unparseable.
    Invalid,
}

/// Dispatch a raw JSON message to the appropriate handler.
///
/// Returns a [`DispatchResult`] indicating what was received. The caller
/// (the WebSocket client loop) uses this to update its state.
pub fn dispatch(json: &serde_json::Value) -> DispatchResult {
    let msg_type = match messages::extract_message_type(json) {
        Some(t) => t,
        None => {
            tracing::warn!("Received message without type field");
            return DispatchResult::Invalid;
        }
    };

    match msg_type {
        "IdentifyAck" => match serde_json::from_value::<IdentifyAckPayload>(json.clone()) {
            Ok(ack) => {
                if ack.accepted {
                    tracing::info!(
                        server_version = %ack.server_version,
                        "Server accepted NodeIdentify"
                    );
                    DispatchResult::IdentifyAccepted {
                        server_version: ack.server_version,
                    }
                } else {
                    let reason = ack.reason.unwrap_or_else(|| "unknown".to_string());
                    tracing::error!(reason = %reason, "Server rejected NodeIdentify");
                    DispatchResult::IdentifyRejected { reason }
                }
            }
            Err(e) => {
                tracing::warn!(error = %e, "Failed to parse IdentifyAck");
                DispatchResult::Invalid
            }
        },
        "HeartbeatPong" => match serde_json::from_value::<HeartbeatPongPayload>(json.clone()) {
            Ok(pong) => {
                tracing::trace!(latency_ms = pong.latency_ms, "HeartbeatPong received");
                DispatchResult::HeartbeatPong {
                    latency_ms: pong.latency_ms,
                }
            }
            Err(e) => {
                tracing::warn!(error = %e, "Failed to parse HeartbeatPong");
                DispatchResult::Invalid
            }
        },
        "Error" => match serde_json::from_value::<ErrorPayload>(json.clone()) {
            Ok(err) => {
                tracing::error!(
                    code = %err.code,
                    message = %err.message,
                    "Server sent error"
                );
                DispatchResult::Error {
                    code: err.code,
                    message: err.message,
                }
            }
            Err(e) => {
                tracing::warn!(error = %e, "Failed to parse Error");
                DispatchResult::Invalid
            }
        },
        other => {
            tracing::debug!(msg_type = %other, "Unhandled message type (dropped)");
            DispatchResult::Unhandled {
                msg_type: other.to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatch_identify_ack_accepted() {
        let json = serde_json::json!({
            "type": "IdentifyAck",
            "accepted": true,
            "serverVersion": "1.0.0",
            "timestamp": "2026-01-01T00:00:00Z"
        });
        let result = dispatch(&json);
        assert_eq!(
            result,
            DispatchResult::IdentifyAccepted {
                server_version: "1.0.0".to_string()
            }
        );
    }

    #[test]
    fn dispatch_identify_ack_rejected() {
        let json = serde_json::json!({
            "type": "IdentifyAck",
            "accepted": false,
            "serverVersion": "1.0.0",
            "reason": "invalid token",
            "timestamp": "2026-01-01T00:00:00Z"
        });
        let result = dispatch(&json);
        assert_eq!(
            result,
            DispatchResult::IdentifyRejected {
                reason: "invalid token".to_string()
            }
        );
    }

    #[test]
    fn dispatch_heartbeat_pong() {
        let json = serde_json::json!({
            "type": "HeartbeatPong",
            "timestamp": "2026-01-01T00:00:00Z",
            "latencyMs": 12.5
        });
        let result = dispatch(&json);
        assert_eq!(result, DispatchResult::HeartbeatPong { latency_ms: 12.5 });
    }

    #[test]
    fn dispatch_error() {
        let json = serde_json::json!({
            "type": "Error",
            "code": "auth.expired",
            "message": "Token expired"
        });
        let result = dispatch(&json);
        assert_eq!(
            result,
            DispatchResult::Error {
                code: "auth.expired".to_string(),
                message: "Token expired".to_string()
            }
        );
    }

    #[test]
    fn dispatch_unhandled_type() {
        let json = serde_json::json!({
            "type": "NodeLedCommand",
            "nodeId": "n1",
            "animationId": "idle-breathe"
        });
        let result = dispatch(&json);
        assert_eq!(
            result,
            DispatchResult::Unhandled {
                msg_type: "NodeLedCommand".to_string()
            }
        );
    }

    #[test]
    fn dispatch_missing_type() {
        let json = serde_json::json!({"data": "no type"});
        assert_eq!(dispatch(&json), DispatchResult::Invalid);
    }

    #[test]
    fn dispatch_malformed_identify_ack() {
        let json = serde_json::json!({
            "type": "IdentifyAck",
            "wrongField": true
        });
        assert_eq!(dispatch(&json), DispatchResult::Invalid);
    }
}
