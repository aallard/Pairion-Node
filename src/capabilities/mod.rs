//! Capability detection and manifest for Pairion Node.
//!
//! This module probes the Node's hardware at boot and produces a read-only
//! [`manifest::NodeCapabilities`] struct that describes what the Node can do.
//! The manifest is sent to the Server with `NodeIdentify` and determines the
//! Node's tier (Dumb or Smart).
//!
//! **Invariant (Architecture §16.1):** Tier-specific behavior gates on the
//! capability manifest at runtime, never on compile-time features. CI rejects
//! PRs with `#[cfg(feature = "smart")]` or tier-conditional file organization.
//!
//! **Invariant (CONVENTIONS §2.6):** The manifest is detected once at boot,
//! is read-only at runtime, and is re-detected only on SIGHUP.

pub mod detector;
pub mod manifest;
