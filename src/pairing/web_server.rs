//! Pairing web server (HTTP 8080).
//!
//! During pairing mode, a temporary HTTP server runs on port 8080 displaying
//! the pair code and Node status. The server is shut down immediately after
//! successful pairing.
//!
//! In M0, this is scaffolding only. No HTTP server runs. Real implementation
//! arrives at M6.

/// Placeholder for the pairing web server.
pub struct PairingWebServer;

impl PairingWebServer {
    /// Create a new pairing web server (stub).
    pub fn new() -> Self {
        Self
    }
}

impl Default for PairingWebServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn web_server_can_be_created() {
        let _server = PairingWebServer::new();
    }

    #[test]
    fn web_server_default() {
        let _server = PairingWebServer;
    }
}
