//! Pairion Node library crate.
//!
//! Exposes all Node subsystems for use by the binary entry point and
//! by integration tests. The binary (`src/main.rs`) is thin — it parses
//! CLI args and calls into these modules.

pub mod audio;
pub mod capabilities;
pub mod config;
pub mod led;
pub mod logs;
pub mod offline;
pub mod pairing;
pub mod secrets;
pub mod smart;
pub mod sound;
pub mod vad;
pub mod wake;
pub mod ws;
