# Contributing to LIRUVA

Thanks for helping make music production accessible to everyone.

## Before opening a pull request

- Read the [roadmap](docs/ROADMAP.md) and [architecture](docs/ARCHITECTURE.md).
- Propose one small feature or fix per issue/PR.
- Favor a simpler user workflow over more controls and preferences.
- Use original code and assets; do not copy proprietary DAW UI assets or code.
- Check dependency and sample licensing; see [dependencies policy](docs/DEPENDENCIES.md).
- Write clear English documentation and describe testing performed.
- Keep core functionality offline and accessible without AI.

## Development

Install a stable Rust toolchain (Rust 1.95 or newer) and the platform-specific dependencies listed in README.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

For each PR, explain **why** the change improves music production, what is actually working, and what is deferred.

## Licensing

By contributing, you agree to license your contributions to this project under GPL-3.0-or-later. Include an SPDX identifier in new source files. Only submit code/assets you have rights to distribute.

## Community

Be constructive and welcome both first-time contributors and experienced audio developers. Follow our [Code of Conduct](CODE_OF_CONDUCT.md).
