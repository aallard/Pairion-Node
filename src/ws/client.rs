//! WebSocket client implementation.
//!
//! Manages the lifecycle of the WebSocket connection to the Pairion Server:
//! connect, identify, heartbeat, reconnect with exponential backoff.
//!
//! **Reconnection strategy:** On socket error or heartbeat timeout, the client
//! reconnects with exponential backoff: 1s, 2s, 4s, 8s, max 30s.

use crate::capabilities::manifest::NodeCapabilities;
use crate::config::NodeConfig;
use crate::ws::dispatch::{self, DispatchResult};
use crate::ws::heartbeat::HeartbeatTracker;
use crate::ws::identify;
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio_tungstenite::tungstenite::Message;
use tracing;

/// Maximum backoff duration between reconnection attempts.
const MAX_BACKOFF: Duration = Duration::from_secs(30);

/// Initial backoff duration.
const INITIAL_BACKOFF: Duration = Duration::from_secs(1);

/// Calculate the next backoff duration using exponential backoff.
pub fn next_backoff(current: Duration) -> Duration {
    let doubled = current.saturating_mul(2);
    if doubled > MAX_BACKOFF {
        MAX_BACKOFF
    } else {
        doubled
    }
}

/// Run the WebSocket client loop.
///
/// This function connects to the Server, identifies, and enters the
/// heartbeat/dispatch loop. On failure it returns, and the caller should
/// retry with backoff.
///
/// Returns `Ok(())` if the connection was cleanly closed, or `Err` with
/// a description of the failure.
pub async fn run_connection(
    server_url: &str,
    config: &NodeConfig,
    token: &str,
    capabilities: &NodeCapabilities,
) -> Result<(), String> {
    tracing::info!(url = %server_url, "Connecting to Server");

    let (ws_stream, _response) = tokio_tungstenite::connect_async(server_url)
        .await
        .map_err(|e| format!("WebSocket connect failed: {e}"))?;

    let (mut sink, mut stream) = ws_stream.split();

    // Send NodeIdentify
    let identify_payload = identify::build_identify(config, token, capabilities);
    let identify_json = serde_json::to_string(&identify_payload).map_err(|e| e.to_string())?;
    sink.send(Message::Text(identify_json))
        .await
        .map_err(|e| format!("Failed to send NodeIdentify: {e}"))?;

    tracing::info!(node_id = %config.node_id, "NodeIdentify sent");

    // Wait for IdentifyAck
    let ack_msg = tokio::time::timeout(Duration::from_secs(10), stream.next())
        .await
        .map_err(|_| "Timeout waiting for IdentifyAck".to_string())?
        .ok_or_else(|| "Connection closed before IdentifyAck".to_string())?
        .map_err(|e| format!("Error reading IdentifyAck: {e}"))?;

    if let Message::Text(text) = ack_msg {
        let json: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| format!("Invalid JSON: {e}"))?;
        match dispatch::dispatch(&json) {
            DispatchResult::IdentifyAccepted { server_version } => {
                tracing::info!(
                    server_version = %server_version,
                    "Connected and identified"
                );
            }
            DispatchResult::IdentifyRejected { reason } => {
                return Err(format!("Server rejected identify: {reason}"));
            }
            other => {
                return Err(format!("Unexpected response to identify: {other:?}"));
            }
        }
    } else {
        return Err("Expected text message for IdentifyAck".to_string());
    }

    // Enter heartbeat/dispatch loop
    let mut heartbeat = HeartbeatTracker::new();
    let mut heartbeat_interval = tokio::time::interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = heartbeat_interval.tick() => {
                if heartbeat.is_timed_out() {
                    tracing::warn!("Heartbeat timeout — connection lost");
                    return Err("Heartbeat timeout".to_string());
                }
                if heartbeat.should_ping() {
                    let ping = heartbeat.build_ping();
                    let ping_json = serde_json::to_string(&ping).map_err(|e| e.to_string())?;
                    if let Err(e) = sink.send(Message::Text(ping_json)).await {
                        tracing::error!(error = %e, "Failed to send HeartbeatPing");
                        return Err(format!("Failed to send HeartbeatPing: {e}"));
                    }
                    tracing::trace!("HeartbeatPing sent");
                }
            }
            msg = stream.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<serde_json::Value>(&text) {
                            Ok(json) => {
                                let result = dispatch::dispatch(&json);
                                if let DispatchResult::HeartbeatPong { .. } = result {
                                    heartbeat.record_pong();
                                }
                            }
                            Err(e) => {
                                tracing::warn!(error = %e, "Failed to parse inbound message");
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) => {
                        tracing::info!("Server closed connection");
                        return Ok(());
                    }
                    Some(Ok(_)) => {
                        // Binary or other message types — ignore in M0
                    }
                    Some(Err(e)) => {
                        tracing::error!(error = %e, "WebSocket error");
                        return Err(format!("WebSocket error: {e}"));
                    }
                    None => {
                        tracing::info!("WebSocket stream ended");
                        return Ok(());
                    }
                }
            }
        }
    }
}

/// Run the WebSocket client with automatic reconnection.
///
/// This is the top-level entry point for the WebSocket subsystem. It
/// connects, and on failure retries with exponential backoff.
pub async fn run_with_reconnect(
    server_url: &str,
    config: &NodeConfig,
    token: &str,
    capabilities: &NodeCapabilities,
) {
    let mut backoff = INITIAL_BACKOFF;

    loop {
        match run_connection(server_url, config, token, capabilities).await {
            Ok(()) => {
                tracing::info!("Connection closed cleanly, reconnecting...");
                backoff = INITIAL_BACKOFF;
            }
            Err(e) => {
                tracing::error!(
                    error = %e,
                    backoff_secs = backoff.as_secs(),
                    "Connection failed, retrying after backoff"
                );
            }
        }
        tokio::time::sleep(backoff).await;
        backoff = next_backoff(backoff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_doubles() {
        assert_eq!(next_backoff(Duration::from_secs(1)), Duration::from_secs(2));
        assert_eq!(next_backoff(Duration::from_secs(2)), Duration::from_secs(4));
        assert_eq!(next_backoff(Duration::from_secs(4)), Duration::from_secs(8));
    }

    #[test]
    fn backoff_caps_at_max() {
        assert_eq!(
            next_backoff(Duration::from_secs(16)),
            Duration::from_secs(30)
        );
        assert_eq!(
            next_backoff(Duration::from_secs(30)),
            Duration::from_secs(30)
        );
    }

    #[test]
    fn initial_backoff_is_one_second() {
        assert_eq!(INITIAL_BACKOFF, Duration::from_secs(1));
    }

    #[test]
    fn max_backoff_is_thirty_seconds() {
        assert_eq!(MAX_BACKOFF, Duration::from_secs(30));
    }
}
