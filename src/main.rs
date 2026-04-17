//! Pairion Node — ambient voice-first AI endpoint.
//!
//! This is the entry point for the Pairion Node binary. It starts subsystems
//! in the order Architecture §3 implies: logs first, then secrets, then config,
//! then capability detection, then WebSocket client.
//!
//! The binary accepts a `--server-url` command-line flag and a
//! `PAIRION_SERVER_URL` environment variable for specifying the Server
//! WebSocket URL, defaulting to `ws://localhost:18789/ws/v1`.
//!
//! **Invariant (Architecture §16.1):** Single codebase, single binary.
//! Tier-specific behavior gates on the capability manifest at runtime.

use clap::Parser;
use pairion_node::{capabilities, config, logs, secrets, ws};

/// Pairion Node — ambient voice-first AI endpoint for Raspberry Pi.
#[derive(Parser, Debug)]
#[command(name = "pairion-node", version, about)]
struct Cli {
    /// WebSocket URL of the Pairion Server.
    #[arg(
        long,
        env = "PAIRION_SERVER_URL",
        default_value = "ws://localhost:18789/ws/v1"
    )]
    server_url: String,
}

#[tokio::main]
async fn main() {
    // 1. Initialize logging (first — everything else logs through this)
    let _log_guard = logs::init();
    tracing::info!("Pairion Node starting");

    // 2. Parse CLI arguments
    let cli = Cli::parse();
    tracing::info!(server_url = %cli.server_url, "Configuration loaded");

    // 3. Load or generate bearer token from encrypted secrets store
    let token = match secrets::load_token() {
        Some(t) => {
            tracing::info!("Using existing bearer token");
            t
        }
        None => {
            let t = secrets::generate_token();
            tracing::info!(token = %t, "Generated new bearer token (first run)");
            if let Err(e) = secrets::save_token(&t) {
                tracing::error!(error = %e, "Failed to save bearer token — continuing with ephemeral token");
            }
            t
        }
    };

    // 4. Load or create node config
    let node_config = config::NodeConfig::new();
    tracing::info!(
        node_id = %node_config.node_id,
        firmware_version = %node_config.firmware_version,
        "Node config initialized"
    );

    // 5. Detect hardware capabilities
    let capabilities = capabilities::detector::detect();
    let tier = capabilities.derived_tier();
    tracing::info!(
        tier = ?tier,
        audio_in = capabilities.audio_in,
        audio_out = capabilities.audio_out,
        local_stt = capabilities.local_stt,
        ai_accelerator = ?capabilities.ai_accelerator,
        "Hardware capabilities detected"
    );

    // 6. Connect to Server and run WebSocket client with reconnection
    tracing::info!("Starting WebSocket client");
    ws::client::run_with_reconnect(&cli.server_url, &node_config, &token, &capabilities).await;
}
