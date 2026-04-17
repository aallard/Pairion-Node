//! Offline mode management for Pairion Node.
//!
//! This module implements the offline state machine (Architecture §9) that
//! governs Node behavior when the Server is unreachable. The state machine
//! has four states: `Online`, `Reconnecting`, `OfflineDumb`, `OfflineSmart`.
//!
//! **Invariant (Architecture §16.2):** Offline Smart mode treats all speakers
//! as Guests. Hardwired. Cannot be toggled.
//!
//! **Invariant (Architecture §16.4):** No user data is stored on the Node.

pub mod state_machine;
