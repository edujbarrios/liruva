# Open-source dependency and asset policy

LIRUVA's original source is GPL-3.0-or-later. Every third-party crate, bundled sound, font, and graphic must be usable and redistributable consistently with that license.

## Initial direct dependency

| Component | Pinned version | Upstream license | Purpose |
| --- | --- | --- | --- |
| [eframe/egui](https://github.com/emilk/egui) | 0.36.2 | MIT OR Apache-2.0 | Native desktop UI |
| [CPAL](https://github.com/RustAudio/cpal) | 0.18.2 | Apache-2.0 | Native audio output |

The direct versions and licenses above have been checked against upstream crate metadata. This is **not** a complete audit of all transitive dependencies; maintainers must review the resolved Cargo dependency tree before distributing binaries.

## Rules

- No proprietary or source-available-only components in the core.
- No paid API required for installation or essential music-making.
- Check each new dependency's upstream license and transitive dependency licenses.
- When shipping binaries, retain required third-party copyright/license notices.
- Avoid copying existing DAW logos, screenshots, UI assets, sounds, sample packs, or plugin code.
- Favor generated percussion sounds or CC0-licensed recordings over unlicensed samples.
- Future plugin hosting must avoid bundling proprietary plugins.
- Do not publish binaries until licensing and platform smoke testing have been completed.
