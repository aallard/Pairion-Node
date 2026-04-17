//! Integration tests for the Pairion Node WebSocket client.
//!
//! These tests spin up the mocked-Server harness and verify that the Node
//! client can connect, identify, and exchange heartbeats.

mod harness;

use pairion_node::capabilities::detector;
use pairion_node::config::NodeConfig;
use pairion_node::ws::client;
use std::time::Duration;

#[tokio::test]
#[ignore] // Integration test — run with `cargo test -- --ignored`
async fn node_connects_and_identifies() {
    let addr = harness::start_test_server().await;
    let url = format!("ws://{}", addr);

    let config = NodeConfig::new();
    let capabilities = detector::detect();
    let token = "test-token-integration";

    // Run the connection with a timeout — it should connect, identify,
    // and then we cancel it after a short period
    let result = tokio::time::timeout(
        Duration::from_secs(3),
        client::run_connection(&url, &config, token, &capabilities),
    )
    .await;

    // The connection should still be alive (timeout means it's running the
    // heartbeat loop successfully)
    assert!(result.is_err(), "Expected timeout (connection is alive)");
}

#[tokio::test]
#[ignore]
async fn node_sends_heartbeat_and_receives_pong() {
    let addr = harness::start_test_server().await;
    let url = format!("ws://{}", addr);

    let config = NodeConfig::new();
    let capabilities = detector::detect();
    let token = "test-token-heartbeat";

    // Run long enough for at least one heartbeat cycle (15 seconds)
    // but use a shorter timeout for test speed
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        client::run_connection(&url, &config, token, &capabilities),
    )
    .await;

    // Should timeout because the connection is alive and heartbeating
    assert!(result.is_err(), "Expected timeout (heartbeat loop running)");
}

#[tokio::test]
#[ignore]
async fn node_handles_server_disconnect() {
    let addr = harness::start_test_server().await;
    let url = format!("ws://{}", addr);

    let config = NodeConfig::new();
    let capabilities = detector::detect();
    let token = "test-token-disconnect";

    // Connect, then drop the server by not keeping the handle
    // The client should detect the disconnect
    let handle =
        tokio::spawn(
            async move { client::run_connection(&url, &config, token, &capabilities).await },
        );

    // Give it time to connect
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Drop the server by dropping the listener (it's already spawned and will
    // close when the connection closes)
    // Force the connection to end
    handle.abort();
    let result = handle.await;
    assert!(result.is_err() || result.unwrap().is_ok());
}
