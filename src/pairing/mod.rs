//! Pairing subsystem for Pairion Node.
//!
//! Handles first-run pairing: pair code generation, LED encoding, the
//! temporary HTTP 8080 web UI, and mDNS advertisement. In M0, these are
//! scaffolding only — no HTTP server runs, no pair code is generated.
//!
//! The bearer token used at startup is either read from the encrypted
//! local config or generated on first run (see [`crate::secrets`]).
//!
//! Real pairing (pair code, HTTP 8080 web UI, mDNS advertisement) arrives at M6.

pub mod pair_code;
pub mod web_server;
