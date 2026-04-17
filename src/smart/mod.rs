//! Smart-tier subsystems for Pairion Node.
//!
//! All Hailo integration, Whisper-small, and mini-LLM integration live here.
//! Code in this module **must check capability at entry** before doing work —
//! a Smart subsystem running on Dumb hardware is a bug (CONVENTIONS §2.7).
//!
//! **Invariant (Architecture §16.1):** This module compiles on EVERY target.
//! Which functions actually run is a runtime decision driven by the capability
//! manifest, never a compile-time decision. **No Cargo feature flag gates
//! this module.** This is the single-binary discipline from CONVENTIONS §2.2.
//!
//! In M0, these are **trait definitions and type signatures only**. Every
//! function body is `unimplemented!()`. The capability check pattern is in
//! place even though the functions panic immediately afterward.

pub mod hailo_ffi;
pub mod mini_llm;
pub mod skill_invoker;
pub mod whisper_small;
