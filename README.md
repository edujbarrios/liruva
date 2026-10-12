# LIRUVA

**Make music. Freely.**

> In the AI music era, let's make music production tools accessible, intuitive, and free for everyone.

[![License: GPL-3.0-or-later](https://img.shields.io/badge/license-GPL--3.0--or--later-blue)](LICENSE)
[![CI](https://github.com/edujbarrios/liruva/actions/workflows/ci.yml/badge.svg)](https://github.com/edujbarrios/liruva/actions/workflows/ci.yml)

LIRUVA is an independent, community-driven, **free and open-source** digital audio workstation (DAW) in its earliest stage of development.

The goal is simple: make producing music approachable. We are inspired by the familiar creative workflows of existing DAWs, but LIRUVA has its own identity, implementation, and interface. **It is not affiliated with Image-Line or FL Studio.**

## Current status: Milestone 1.1 — first sound (development branch)

**Implemented in this branch:**
- Rust workspace and original dark Material-inspired interface.
- Actual synthesized kick drum on each beat, with Play/Stop and adjustable BPM.
- Thread-safe transport controls and a separate, testable beat clock.
- Visible error status when no audio output device is available.
- Project mission, GPL license, contribution guidelines, architecture notes and CI.

**Not implemented yet:** 16-step sequencer, multiple drum channels, piano roll, arrangement, samples, plugins, project saving, WAV export and AI tools.

This is an early playable prototype, not yet a full DAW. Audio device playback must still be checked manually on supported desktop platforms. See [design guide](docs/DESIGN.md).

## Build and run

Install a current stable [Rust toolchain](https://rustup.rs/) (minimum Rust 1.95) and system dependencies for the native GUI.

On Ubuntu/Debian:

```bash
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libgtk-3-dev libxkbcommon-dev libwayland-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libssl-dev libasound2-dev
```

On Windows and macOS, install Rust and your platform's native development tooling.

```bash
git clone https://github.com/edujbarrios/liruva.git
cd liruva
cargo run -p liruva
```

These changes are under review in stacked Pull Requests. For this feature, check out `feat/milestone-1-audio-dark-ui` after cloning. To run the original non-audio foundation instead, check out `feat/milestone-0-foundation`.

Check code locally:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The GUI and audio device need a real desktop environment; CI can run Rust checks but cannot verify whether a physical speaker produces sound. Windows and macOS runtime builds have not yet been verified.

## Development principles

1. **Free forever, genuinely open source.** Every core feature must be open and redistributable under compatible licenses. No subscriptions or unlocks.
2. **Simplicity first.** A beginner should be able to make a beat without learning an entire professional DAW.
3. **Offline by default.** No accounts, telemetry, cloud requirement, or API keys.
4. **AI is optional.** Basic music creation must work without any language model or internet service.
5. **Real functionality.** No fake transports, silent sequencers, or nonworking buttons advertised as features.
6. **Small, reviewable iterations.** One verifiable improvement per pull request.

## Roadmap

See [docs/ROADMAP.md](docs/ROADMAP.md) for the staged development plan. The **next milestone** is a minimal 16-step drum sequencer with real sound.

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Issues and pull requests are welcome. We especially value feedback from beginner music producers.

## License and credits

Copyright (C) 2026 Eduardo J. Barrios and LIRUVA contributors.

LIRUVA source code is released under the [GNU General Public License v3.0 or later](LICENSE) (`GPL-3.0-or-later`). Please review [docs/DEPENDENCIES.md](docs/DEPENDENCIES.md) before adding dependencies or assets.

The eframe/egui framework is a third-party dependency under MIT OR Apache-2.0. It is not part of the original LIRUVA codebase.

**LIRUVA — Make music. Freely.**
