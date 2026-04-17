//! Persistent Node configuration.
//!
//! Stores the Node's runtime configuration: node id, server URL, offline
//! policy, and other settings that persist across reboots. The bearer token
//! itself is stored separately in the encrypted secrets store (see
//! [`crate::secrets`]).
//!
//! In M0, configuration is minimal — just enough to identify the Node and
//! connect to the Server. Fuller configuration (wake-word sensitivity, LED
//! brightness, mic gain, etc.) arrives at M6.

use serde::{Deserialize, Serialize};

/// Persistent Node configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Unique identifier for this Node, generated on first run.
    pub node_id: String,
    /// The firmware version string reported to the Server.
    pub firmware_version: String,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            node_id: uuid::Uuid::new_v4().to_string(),
            firmware_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl NodeConfig {
    /// Create a new default configuration with a fresh node id.
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_valid_uuid() {
        let config = NodeConfig::default();
        assert!(uuid::Uuid::parse_str(&config.node_id).is_ok());
    }

    #[test]
    fn default_config_has_version() {
        let config = NodeConfig::default();
        assert!(!config.firmware_version.is_empty());
    }

    #[test]
    fn new_generates_unique_ids() {
        let a = NodeConfig::new();
        let b = NodeConfig::new();
        assert_ne!(a.node_id, b.node_id);
    }

    #[test]
    fn serialization_round_trip() {
        let config = NodeConfig::new();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: NodeConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.node_id, deserialized.node_id);
        assert_eq!(config.firmware_version, deserialized.firmware_version);
    }
}
