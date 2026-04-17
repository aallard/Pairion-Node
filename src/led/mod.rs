//! LED driver subsystem for Pairion Node.
//!
//! Drives the ReSpeaker USB mic array's 12 on-board APA102 LEDs via USB HID.
//! Animations are defined locally in [`animations`]; the Server commands LED
//! state by referencing animation ids via `NodeLedCommand`.
//!
//! **Invariant (Architecture §16.5):** LED state is driven from the local
//! authoritative animation library. Network-delivered raw animation data is
//! rejected; only animation ids are accepted.
//!
//! In M0, the USB HID driver is stubbed. A [`controller::MockLedDriver`] is
//! provided for testing. The real driver arrives at M6.

pub mod animations;
pub mod controller;
