//! WebSocket message types matching the AsyncAPI schema.
//!
//! Every JSON payload carries a `type` discriminator string. These structs
//! map directly to the `asyncapi.yaml` component schemas.

use crate::capabilities::manifest::NodeCapabilities;
use serde::{Deserialize, Serialize};

/// `NodeIdentify` — first message sent by a Node after opening the WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeIdentifyPayload {
    /// Message type discriminator.
    #[serde(rename = "type")]
    pub msg_type: String,
    /// Unique Node identifier.
    pub node_id: String,
    /// Bearer token for authentication.
    pub token: String,
    /// Firmware version string.
    pub firmware_version: String,
    /// Hardware capability manifest.
    pub capabilities: NodeCapabilities,
    /// ISO-8601 timestamp.
    pub timestamp: String,
}

/// `IdentifyAck` — Server response to a successful identify.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentifyAckPayload {
    /// Message type discriminator.
    #[serde(rename = "type")]
    pub msg_type: String,
    /// Whether the identification was accepted.
    pub accepted: bool,
    /// Server version string.
    pub server_version: String,
    /// Reason for rejection (present only when `accepted: false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// ISO-8601 timestamp.
    pub timestamp: String,
}

/// `HeartbeatPing` — sent by the Node to the Server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatPingPayload {
    /// Message type discriminator.
    #[serde(rename = "type")]
    pub msg_type: String,
    /// ISO-8601 timestamp.
    pub timestamp: String,
}

/// `HeartbeatPong` — Server response to a heartbeat ping.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatPongPayload {
    /// Message type discriminator.
    #[serde(rename = "type")]
    pub msg_type: String,
    /// ISO-8601 timestamp.
    pub timestamp: String,
    /// Round-trip latency in milliseconds.
    pub latency_ms: f64,
}

/// `Error` — Server-emitted error, in-band.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorPayload {
    /// Message type discriminator.
    #[serde(rename = "type")]
    pub msg_type: String,
    /// Dotted machine-readable code.
    pub code: String,
    /// Human-readable error message.
    pub message: String,
}

/// `LogForward` — batched log entries sent to the Server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogForwardPayload {
    /// Message type discriminator.
    #[serde(rename = "type")]
    pub msg_type: String,
    /// The Node sending logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_node_id: Option<String>,
    /// Batched log entries.
    pub entries: Vec<serde_json::Value>,
    /// ISO-8601 timestamp.
    pub timestamp: String,
}

/// Extract the `type` field from a raw JSON message.
pub fn extract_message_type(json: &serde_json::Value) -> Option<&str> {
    json.get("type").and_then(|v| v.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::manifest::AiAccelerator;

    #[test]
    fn node_identify_serializes_correctly() {
        let payload = NodeIdentifyPayload {
            msg_type: "NodeIdentify".to_string(),
            node_id: "test-node-1".to_string(),
            token: "test-token".to_string(),
            firmware_version: "0.1.0".to_string(),
            capabilities: NodeCapabilities {
                audio_in: true,
                audio_out: true,
                local_wake_word: true,
                local_vad: true,
                local_stt: false,
                local_llm_small: false,
                local_tts_cache: true,
                ai_accelerator: AiAccelerator::None,
                dedicated_npu_ram_gb: 0,
            },
            timestamp: "2026-01-01T00:00:00Z".to_string(),
        };
        let json = serde_json::to_value(&payload).unwrap();
        assert_eq!(json["type"], "NodeIdentify");
        assert_eq!(json["nodeId"], "test-node-1");
        assert_eq!(json["capabilities"]["audioIn"], true);
    }

    #[test]
    fn heartbeat_ping_serializes_correctly() {
        let payload = HeartbeatPingPayload {
            msg_type: "HeartbeatPing".to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
        };
        let json = serde_json::to_value(&payload).unwrap();
        assert_eq!(json["type"], "HeartbeatPing");
    }

    #[test]
    fn extract_type_from_json() {
        let json: serde_json::Value = serde_json::json!({"type": "IdentifyAck", "accepted": true});
        assert_eq!(extract_message_type(&json), Some("IdentifyAck"));
    }

    #[test]
    fn extract_type_returns_none_for_missing() {
        let json: serde_json::Value = serde_json::json!({"data": "no type"});
        assert_eq!(extract_message_type(&json), None);
    }

    #[test]
    fn identify_ack_deserializes() {
        let json = serde_json::json!({
            "type": "IdentifyAck",
            "accepted": true,
            "serverVersion": "1.0.0",
            "timestamp": "2026-01-01T00:00:00Z"
        });
        let ack: IdentifyAckPayload = serde_json::from_value(json).unwrap();
        assert!(ack.accepted);
        assert_eq!(ack.server_version, "1.0.0");
    }

    #[test]
    fn error_payload_deserializes() {
        let json = serde_json::json!({
            "type": "Error",
            "code": "auth.invalid_token",
            "message": "Token rejected"
        });
        let err: ErrorPayload = serde_json::from_value(json).unwrap();
        assert_eq!(err.code, "auth.invalid_token");
    }
}
