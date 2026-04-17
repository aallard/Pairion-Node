//! Test harness — mocked Pairion Server.
//!
//! An in-process WebSocket server implementing just enough of the AsyncAPI
//! protocol to test the Node client:
//! - Accept `NodeIdentify`, respond with `IdentifyAck`
//! - Handle `HeartbeatPing` / `HeartbeatPong`
//! - Respond to unknown messages with `Error`

use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;

/// Start a test WebSocket server and return the address it's listening on.
///
/// The server handles one connection, processes messages according to the
/// AsyncAPI protocol subset, and shuts down when the connection closes.
pub async fn start_test_server() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        if let Ok((stream, _)) = listener.accept().await {
            let ws_stream = tokio_tungstenite::accept_async(stream).await.unwrap();
            let (mut sink, mut source) = ws_stream.split();

            while let Some(Ok(msg)) = source.next().await {
                if let Message::Text(text) = msg {
                    let json: serde_json::Value = match serde_json::from_str(&text) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };

                    let msg_type = json
                        .get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");

                    let response = match msg_type {
                        "NodeIdentify" => serde_json::json!({
                            "type": "IdentifyAck",
                            "accepted": true,
                            "serverVersion": "1.0.0-test",
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        }),
                        "HeartbeatPing" => serde_json::json!({
                            "type": "HeartbeatPong",
                            "timestamp": chrono::Utc::now().to_rfc3339(),
                            "latencyMs": 1.0
                        }),
                        _ => serde_json::json!({
                            "type": "Error",
                            "code": "not_implemented",
                            "message": format!("Message type '{}' not implemented in test harness", msg_type)
                        }),
                    };

                    let response_text = serde_json::to_string(&response).unwrap();
                    if sink.send(Message::Text(response_text)).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    addr
}
