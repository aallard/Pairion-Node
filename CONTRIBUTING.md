# Contributing to Pairion-Node

Thank you for your interest in contributing to Pairion-Node.

## Development Setup

### Prerequisites

- Rust stable (pinned via `rust-toolchain.toml`)
- For cross-compilation: `cross` tool or `aarch64-linux-gnu-gcc`

### Building

```bash
# Host build
cargo build --release

# Cross-compile for Raspberry Pi
cargo build --release --target aarch64-unknown-linux-gnu
```

### Testing

```bash
# Unit tests
cargo test --workspace

# Integration tests (requires no running server)
cargo test --workspace -- --ignored
```

### Code Quality

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- --deny warnings
bash scripts/lint-single-binary-discipline.sh
bash scripts/lint-audio-no-disk-writes.sh
```

## Conventions

Read `CONVENTIONS.md` before writing code. Key rules:

- **Single binary, single codebase** — no Cargo feature flags for tier differentiation
- **100% test coverage** on public surfaces
- **RustDoc** on every `pub` item
- **No `println!` or `eprintln!`** in production modules — use `tracing`
- **Conventional Commits** for git messages

## What We Welcome

- 3D-printed enclosure designs for Pi + ReSpeaker assemblies
- Hardware variant documentation (alternative mic arrays, speakers)
- Wake-word models for additional languages
- Offline-capable skill contributions
- Bug reports with exact hardware BOM and `pairion-node doctor` output

## Pull Requests

- Every PR links its Claude Code prompt (if AI-generated)
- CI must be green to merge
- Tests and docs ship in the same PR as code
