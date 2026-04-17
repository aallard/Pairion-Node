//! NodeIdentify message construction.
//!
//! Builds the `NodeIdentify` payload sent as the first message after
//! opening the WebSocket connection to the Server.

use crate::capabilities::manifest::NodeCapabilities;
use crate::config::NodeConfig;
use crate::ws::messages::NodeIdentifyPayload;

/// Build a `NodeIdentify` payload from the current config and capabilities.
pub fn build_identify(
    config: &NodeConfig,
    token: &str,
    capabilities: &NodeCapabilities,
) -> NodeIdentifyPayload {
    NodeIdentifyPayload {
        msg_type: "NodeIdentify".to_string(),
        node_id: config.node_id.clone(),
        token: token.to_string(),
        firmware_version: config.firmware_version.clone(),
        capabilities: capabilities.clone(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::manifest::AiAccelerator;

    #[test]
    fn build_identify_sets_correct_type() {
        let config = NodeConfig::new();
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
        let payload = build_identify(&config, "tok", &caps);
        assert_eq!(payload.msg_type, "NodeIdentify");
        assert_eq!(payload.node_id, config.node_id);
        assert_eq!(payload.token, "tok");
        assert_eq!(payload.firmware_version, config.firmware_version);
    }

    #[test]
    fn build_identify_includes_capabilities() {
        let config = NodeConfig::new();
        let caps = crate::capabilities::detector::detect();
        let payload = build_identify(&config, "token", &caps);
        assert!(payload.capabilities.audio_in);
        assert!(!payload.capabilities.local_stt);
    }

    #[test]
    fn build_identify_timestamp_is_valid() {
        let config = NodeConfig::new();
        let caps = crate::capabilities::detector::detect();
        let payload = build_identify(&config, "t", &caps);
        assert!(chrono::DateTime::parse_from_rfc3339(&payload.timestamp).is_ok());
    }
}
