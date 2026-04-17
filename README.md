# Pairion-Node

Ambient voice-first AI endpoint for Raspberry Pi. Part of the [Pairion](https://pairion.ai) household AI system.

## What This Is

A Pairion Node is a Pi-based device with a microphone array, speaker, and LED ring that provides ambient voice interaction in a room. Nodes detect wake words locally, stream audio to the Pairion Server for processing, and play back responses. Two hardware tiers (Dumb and Smart) run the same single binary — capabilities are detected at runtime.

See `Pairion-Node-Overview.md` for hardware BOM, tier details, and installation.

## Development Setup

### Prerequisites

- Rust stable (automatically managed via `rust-toolchain.toml`)
- For cross-compilation to Pi: `aarch64-linux-gnu-gcc` or the `cross` tool

### Build

```bash
# Host build (macOS / Linux x86)
cargo build --release

# Cross-compile for Raspberry Pi 5
cargo build --release --target aarch64-unknown-linux-gnu

# Alternatively, using cross:
cross build --release --target aarch64-unknown-linux-gnu
```

### Run (Development)

```bash
# Connect to a local Pairion Server
cargo run -- --server-url ws://localhost:18789/ws/v1

# Or via environment variable
PAIRION_SERVER_URL=ws://192.168.1.100:18789/ws/v1 cargo run
```

### Test

```bash
# Unit tests
cargo test --workspace

# Integration tests (mocked Server harness)
cargo test --workspace -- --ignored

# Lint checks
cargo fmt -- --check
cargo clippy --all-targets --all-features -- --deny warnings
bash scripts/lint-single-binary-discipline.sh
bash scripts/lint-audio-no-disk-writes.sh
```

## Project Structure

```
src/
├── main.rs              # Entry point
├── lib.rs               # Library crate
├── audio/               # Audio pipeline (M6)
├── wake/                # Wake-word detection (M6)
├── vad/                 # Voice activity detection (M6)
├── ws/                  # WebSocket client (working)
├── led/                 # LED driver + animation library
├── sound/               # Sound-design cache
├── capabilities/        # Hardware capability detection
├── offline/             # Offline state machine
├── smart/               # Smart-tier subsystems (M7)
├── pairing/             # Pairing flow (M6)
├── logs/                # Structured logging
├── secrets/             # Encrypted config storage
└── config/              # Persistent node configuration
```

## Current Status: M0 Walking Skeleton

The Node connects to the Server, sends `NodeIdentify` with a hardcoded Dumb-tier capability manifest, and maintains a heartbeat. All subsystem modules are scaffolded; real audio, LED, and hardware integration arrive at M6/M7.

## Charter

See `Pairion-Charter.md` in the `Pairion-Server` repository for the full project vision.

## License

Source-available, non-commercial. See `LICENSE`.
