# Changelog

All notable changes to Pairion-Node will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-17

### Added

- M0 Walking Skeleton: project bootstrap with all modules scaffolded
- WebSocket client connecting to Pairion Server with `NodeIdentify` and heartbeat
- Hardcoded Dumb-tier capability manifest (real detection at M6)
- Offline state machine with four states: Online, Reconnecting, OfflineDumb, OfflineSmart
- LED animation library with all 14 animations from Architecture §7.1
- Mock LED driver for testing
- Sound cache scaffolding
- Smart-tier subsystem trait definitions (unimplemented bodies)
- Pairing module scaffolding
- Encrypted bearer token storage (AES-256-GCM)
- Structured logging via `tracing` with local file + Server forwarding stub
- Exponential backoff reconnection (1s, 2s, 4s, 8s, max 30s)
- Cross-compilation support for `aarch64-unknown-linux-gnu`
- GitHub Actions CI pipeline
- Single-binary discipline lint script
- Audio no-disk-writes lint script
- Integration test harness (mocked WebSocket Server)
- systemd unit template
- RustDoc on every public item
