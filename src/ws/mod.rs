//! WebSocket client for communicating with the Pairion Server.
//!
//! This module implements the Node side of the AsyncAPI streaming protocol.
//! On startup the client connects to the Server, sends `NodeIdentify` with
//! the bearer token and capability manifest, and enters a heartbeat loop.
//!
//! **Reconnection:** On socket error or heartbeat timeout, the client
//! reconnects with exponential backoff (1s, 2s, 4s, 8s, max 30s).
//!
//! **Invariant (Architecture §16.8):** The Node never makes outbound network
//! requests except to the paired Server.

pub mod client;
pub mod dispatch;
pub mod heartbeat;
pub mod identify;
pub mod messages;
