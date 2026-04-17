# CONVENTIONS.md — Pairion-Node

This document is authoritative for engineering discipline on `Pairion-Node`.
Every contributor (human and AI) reads it before touching code. Every Claude Code prompt assumes these conventions.

---

## 1. Project Standards (Pairion-Wide)

These standards apply across every Pairion repository and are reproduced verbatim in `Pairion-Server/CONVENTIONS.md` and `Pairion-Client/CONVENTIONS.md`.

### 1.1 Development philosophy

This is an AI-first codebase. AI writes 100% of production code. Development velocity is measured in AI-first benchmarks (proven track record of 200K+ lines of code per hour across prior projects). Traditional software development assumptions — sprint timelines, backlogs, phased severity-based deferrals — do not apply.

- **Never phase work by severity or priority.** Every fix, feature, or task is completed in a single pass. There is no cost/time justification for deferral. If it's worth doing, it's done now.
- **Never offer traditional time estimates** in PR descriptions, architecture discussions, or project planning. Effort is a function of prompt iteration, not calendar weeks.
- **No "next sprint" framing.** No "backlog" framing.

### 1.2 Source-of-truth hierarchy

When documents conflict, later beats earlier in this order:

1. `Pairion-Server/openapi.yaml`
2. `Pairion-Server/asyncapi.yaml`
3. `Architecture.md`
4. `Pairion-Node-Overview.md`
5. `Pairion-Charter.md`
6. Any Claude Code prompt
7. Prior conversation context

**Never assume, infer, or guess about a codebase.** Before generating any code, tests, fixes, prompts, or recommendations, verified source-of-truth documents must be consulted. Both Claude (conversational) and Claude Code (filesystem-capable) work from the same verified source — never from memory, conversation context, or inference.

### 1.3 Tests ship with code, always

- **Unit tests and integration tests ship in the same pass as the feature or fix.** No follow-up tickets for tests. Ever.
- **100% code coverage is mandatory, not aspirational.** Enforced by local verification during task execution.
- Tests validate behavior from public API, not internal implementation details.
- Test fixtures and harnesses are first-class code — same conventions, same review bar.

### 1.4 Documentation ships with code, always

- **Every class, every module, every public function has a documentation comment.** (Excluding DTOs, entities, and generated code.)
- TypeScript/JavaScript uses TSDoc/JSDoc. Java uses Javadoc. Dart uses DartDoc. C#/.NET uses XML Doc Comments. Rust uses RustDoc.
- Documentation ships in the same pass as the code. Not a follow-up.
- Auto-generated API docs publish on every release.

### 1.5 Centralized logging

- **All software projects must have centralized logging.**
- Structured, not free-text. Log level, subsystem, correlation id, session/user id where relevant.
- Client and Node logs forward to the Server's centralized log store.
- No `console.log`, no `println!` in production code paths. Use the project's logger.

### 1.6 Database migrations

- **Never use Flyway during development.** Flyway's lifecycle creates significant delays during restart-heavy development cycles.
- Use a lightweight migration runner during development.
- Flyway adoption happens only when moving to production.
- (Not directly applicable to this repo — the Node has no relational database; it stores only its own pairing token, offline policy, and model/sound assets. Included for consistency across Pairion projects.)

### 1.7 Build tooling

- **Never Gradle. Always Maven** (for Java projects — not applicable to this Rust project, but applies if this convention is adopted for any Java module).
- This project uses **Cargo** only. No wrapper build tools, no `make`, no `just` for primary flows.

### 1.8 Password policy

- **Development passwords:** minimal requirements. Short, simple, memorable.
- **Production passwords:** strong (length, special characters, numbers).
- (Not directly applicable to Node pairing — pairing uses tokens, not passwords. Included for consistency.)

### 1.9 Claude Code prompt format

All prompts to Claude Code are `.md` file artifacts. Every prompt:

- **Begins with a STOP directive** reading the required source-of-truth files:
  > STOP: Before writing ANY code, read these files completely:
  > 1. `~/Documents/Github/Pairion-Server/openapi.yaml` — The OpenAPI spec defines every field name, type, enum value, and endpoint path. Your code must match it exactly.
  > 2. `~/Documents/Github/Pairion-Server/asyncapi.yaml` — The AsyncAPI spec defines the streaming WebSocket protocol.
  > 3. `~/Documents/Github/Pairion-Node/Architecture.md` — The architecture spec defines the module layout, audio pipeline, LED driver, capability manifest, offline state machine, and invariants.
  > 4. `~/Documents/Github/Pairion-Node/Pairion-Node-Overview.md` — Hardware BOM, tier behavior, install procedure, LED reference.
  > 5. `~/Documents/Github/Pairion-Server/Pairion-Charter.md` — The project charter.
  > Do not rely on the descriptions in this prompt alone. If this prompt conflicts with the source files, the source files win.
  >
  > If any of these files are missing, STOP and ask for them before proceeding.
- **Ends with:** *"Compile, Run, Test, Commit, Push to Github."*
- **Includes a report template** that Claude Code fills in with a summary of work performed, ending with the Git commit hash.

**Prompts do not contain code.** This includes implementation code, test code, configuration snippets, YAML, shell commands, and code examples. Prompts direct Claude Code with goals, constraints, and instructions — never with code. Claude Code has direct filesystem access and must read actual source files before producing any output.

---

## 2. Pairion-Node Stack Conventions

### 2.1 Runtime and tooling

- **Rust stable** pinned via `rust-toolchain.toml`.
- **Cargo** is the only build tool. No `make`, no `just` for primary build flows.
- **Primary target:** `aarch64-unknown-linux-gnu` (Raspberry Pi 5 / Pi 4 / ARM64 SBCs).
- **Secondary targets:** `x86_64-unknown-linux-gnu` tolerated for community-supported hardware.
- **Cross-compilation** from macOS supported via `cross` or a Linux container.

### 2.2 Single binary, single codebase

**This is the defining architectural discipline of `Pairion-Node`.** Charter §6.5.1 and Node Architecture §3, §16 all require it:

- **One binary runs on both Dumb and Smart tier hardware.**
- **Capabilities are detected at runtime**, never at compile-time.
- **Cargo features are banned for tier differentiation.** No `#[cfg(feature = "smart")]` gating user-visible behavior. Features are permitted only for platform variance (e.g., a macOS target for development testing) and are enforced not to branch tier behavior.
- **CI rejects PRs** with tier-conditional file organization, tier-conditional module gates, or tier-specific build profiles.
- **All tier-specific code lives in `src/smart/`** and is gated by **runtime capability checks**, not compile-time features.

### 2.3 Async runtime

- **Tokio** (multi-threaded) for async. One runtime for the whole binary.
- **Dedicated OS threads** for real-time audio callbacks (ALSA/cpal input + output). Audio callbacks must never call into tokio.
- **Task prioritization** — audio and WS I/O over log forwarding or housekeeping.

### 2.4 Audio pipeline discipline

- **Latency is the primary quality metric.** Charter §11 mandates < 700ms voice round-trip and < 150ms barge-in.
- **Audio callbacks (cpal) are hard-real-time.** Non-blocking. No allocations, no locks, no logging, no `.await`. Ring buffers bridge to async code.
- **Opus encode/decode** on dedicated threads.
- **Raw audio is NEVER written to disk.** A Cargo clippy lint forbids file-system writes from the audio modules. Explicit disk writes outside the audio path require PR justification.

### 2.5 LED driver discipline

- **Animations are local.** The Node ships with a predefined animation library in `animations/`. Server commands reference animation IDs; the Node renders locally.
- **Network-delivered animation data is rejected.** LED payloads that carry raw frame data rather than an animation id are logged and dropped. This is a hard invariant (Node Architecture §16).
- **Offline state machine drives LEDs autonomously** when offline. Server commands are not required for offline LED behavior.

### 2.6 Capability detection discipline

- **Detected once at boot.** Written to the capability manifest. Sent with `NodeIdentify`.
- **Re-detected on SIGHUP** (for live hardware changes during development; not relied upon in production).
- **Manifest is read-only at runtime.** No code mutates capabilities post-detection.
- **Tier is derived from capabilities**, not configured.

### 2.7 Smart-tier subsystems (`src/smart/`)

- All Hailo integration, Whisper-small, and mini-LLM integration live in `src/smart/`.
- Code in `src/smart/` **must check capability at entry** before doing work. A Smart subsystem running on Dumb hardware is a bug.
- **HailoRT FFI** is wrapped in `src/smart/hailo_ffi.rs`. No other module calls HailoRT directly.
- Smart-tier models are **downloaded at pairing time** from the Server, not bundled into the binary.

### 2.8 Offline mode discipline

- **Offline Smart mode treats all speakers as Guests.** Hardwired in `src/offline/smart_mode.rs`. Cannot be toggled. Node Architecture §16 invariant.
- **No user data is stored on the Node.** Ever. Per Charter §12.4 and Node Architecture §16. The Node holds only its own pairing token, offline policy, cached models, cached sounds, and local logs.
- **Offline mode transitions are logged** and surface on reconnect via `NodeOfflineTransition`.

### 2.9 Pairing discipline

- Pairing tokens are stored in an **encrypted local config** file at `/opt/pairion-node/config.enc`.
- The encryption key is derived from the Pi's machine id + a fixed salt. Simple, but resists casual filesystem snooping.
- Pair codes expire after 10 minutes.
- The pairing HTTP server (port 8080) runs only during pairing mode and is shut down immediately after successful pair.
- mDNS advertisement is active only during pairing mode.

### 2.10 Logging

- **`tracing`** for all structured logs.
- **Local buffer:** `/var/log/pairion-node/node.log` rotated daily. Used when Server is unreachable.
- **Forwarding:** batched `LogForward` messages to Server every few seconds.
- **`println!` and `eprintln!` are banned** in production code paths. Cargo clippy lint enforces.

### 2.11 Secrets

- The pairing token is the primary secret. Encrypted at rest per §2.9.
- No other secrets on Node — adapter API keys, user credentials, and any vendor tokens live on the Server, never on the Node.

### 2.12 Testing

- **Cargo unit tests** for every module.
- **Integration tests** with a WebSocket test harness Server in `tests/harness/`.
- **Hardware-in-the-loop tests** run on real Pi hardware in a self-hosted CI runner (from M6 onward). Tests real audio I/O, LED driver, wake-word detection.
- **Offline mode tests** simulate heartbeat loss and verify transitions, offline STT (Smart), cached-error (Dumb), and online restoration.
- **Capability detection tests** mock ALSA and PCI probes for various simulated configurations.
- **Latency benchmarks** cover wake-to-audio-byte, audio-in-to-Server-delivery, Server-audio-to-playback, offline wake-to-response.
- **100% coverage** on public surfaces.

### 2.13 Documentation

- **RustDoc** on every `pub` item. No exceptions for "obvious" code.
- Module-level docs describe the subsystem's purpose and invariants.
- Complex invariants (single-binary discipline, offline-Guest hardwiring, audio callback real-time constraints) are explicitly called out in module docs, not buried in prose.
- Docs build on every release via `cargo doc`.

### 2.14 Linting and formatting

- **`rustfmt`** on every file. CI checks formatting.
- **`cargo clippy --deny warnings`** on every PR.
- **Custom clippy lints** (or CI text linters where clippy can't cover):
  - No `#[cfg(feature = "smart")]` or similar tier-differentiating cfg gates
  - No `println!` / `eprintln!` in production modules
  - No filesystem writes from `src/audio/`
  - No direct vendor HTTP calls (all external network is WS to Server, except mDNS during pairing)

### 2.15 Git and PR conventions

- **Trunk-based.** `main` is always green.
- **Conventional Commits:** `feat(offline): implement smart-tier local stt`, `fix(led): prevent flicker on arbitration loss`.
- **Every PR links its Claude Code prompt.**
- **Local verification must pass before merge.** No GitHub Actions CI is configured. Do not re-add CI workflows unless a prompt explicitly directs it.

### 2.16 Deployment

- **systemd unit** `pairion-node.service` installed to `/etc/systemd/system/`.
- **Binary location:** `/opt/pairion-node/pairion-node`.
- **Assets:** `/opt/pairion-node/models/`, `/opt/pairion-node/sounds/`, `/opt/pairion-node/animations/`.
- **Config:** `/opt/pairion-node/config.enc` (encrypted).
- **Logs:** `/var/log/pairion-node/`.
- **Install script** at `curl -fsSL install.pairion.ai | bash -s node` handles fresh install and upgrades idempotently.

---

## 3. Invariants (from `Architecture.md` §16)

1. **Single codebase, single binary.** Tier-specific behavior gates on the capability manifest, not on compile-time features. CI rejects tier-conditional `#[cfg]` or file organization.
2. **Offline Smart mode treats all speakers as Guests.** Hardwired. Cannot be toggled.
3. **Raw audio is never written to disk.** Zero exceptions.
4. **No user data is stored on the Node.** Ever. Only the Node's own config, token, and cached assets.
5. **LED state is driven from local authoritative animation library.** Network-delivered animation data is rejected; only animation ids are accepted.
6. **Wake-word runs always on Pi CPU.** Even on Smart tier. The Hailo is reserved for STT and mini-LLM.
7. **Screen capture is not supported on Nodes.** No camera is shipped and no software pretends otherwise.
8. **The Node never makes outbound network requests except to the paired Server.** Exception: mDNS advertisement during pairing only.
9. **Pairing tokens are encrypted at rest.** Never plaintext on disk.
10. **Tests ship with code.**

---

## 4. Local Verification Pipeline

**Verification runs locally during task execution** — Claude Code runs tests, lints, typechecks, and coverage as part of every task. No GitHub Actions CI is configured. Do not re-add CI workflows unless a prompt explicitly directs it.

Every task runs, in order:

1. `cargo fmt -- --check`
2. `cargo clippy --all-targets --all-features -- --deny warnings`
3. `cargo build --release --target aarch64-unknown-linux-gnu` (cross-compile)
4. `cargo build --release` (host)
5. `cargo test --workspace`
6. `cargo test --workspace -- --ignored` (integration, with test harness)
7. `cargo test -p pairion-node --test offline_mode`
8. `cargo test -p pairion-node --test capability_detection`
9. `./scripts/lint-single-binary-discipline.sh` (custom grep-based linter for tier-cfg violations)
10. `./scripts/lint-audio-no-disk-writes.sh` (custom linter)
11. `cargo tarpaulin --fail-under 100` (coverage)
12. **Hardware-in-the-loop** (from M6): runs on a self-hosted Pi 5 runner for actual audio + LED + wake-word validation against reference audio.

Failure at any step fails the task.

---

## 5. When In Doubt

Read, in order: this document, `Pairion-Charter.md`, `Architecture.md`, `Pairion-Node-Overview.md`, `Pairion-Server/openapi.yaml`, `Pairion-Server/asyncapi.yaml`. If still uncertain, the code you're about to write is probably wrong — stop and ask.