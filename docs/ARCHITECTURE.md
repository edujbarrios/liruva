# Architecture

Status: **Milestone 1.1**. A first audio slice exists alongside the dark desktop UI; the full DAW architecture is still a proposal.

## Philosophy

The smallest usable musical workflow wins. The interface should reveal complexity only when the user needs it. The application must remain functional without AI services, accounts, or network access.

## Current implementation

```text
Cargo workspace
└── crates/liruva-app/      Native dark desktop UI (egui/eframe)
    └── src/audio.rs       CPAL stream, real kick synthesis, atomics
    └── src/transport.rs   Deterministic musical clock (no audio hardware)
    └── src/theme.rs       LIRUVA design tokens
```

The UI has a working transport connected to a CPAL output callback. A locally synthesized kick is generated in the callback, and playback is controlled with atomics. No audio is produced until Play is pressed.

## Planned separation (not implemented)

```text
                 ┌──────────────────────────┐
                 │  UI / CLI / future agent │
                 └────────────┬─────────────┘
                              │ validated commands
                 ┌────────────▼─────────────┐
                 │  Project + command model │
                 └──────┬───────────┬───────┘
                        │           │
              ┌─────────▼────┐  ┌───▼───────────┐
              │  Sequencer   │  │ Persistence   │
              └────────┬─────┘  └───────────────┘
                       │ time-stamped events
              ┌────────▼─────────────────────┐
              │  Real-time audio engine      │
              └──────────────────────────────┘
```

Possible future workspace crates: `liruva-core`, `liruva-audio`, and `liruva-project`. Do not create them until the implementation has genuinely distinct responsibilities.

## Audio safety principles

- Keep the audio callback independent of GUI rendering.
- No file I/O, network requests, blocking locks, or unbounded allocations in real-time processing.
- Deterministic transport/sequencing where possible.
- Offline unit tests for timing and DSP; device tests are separate.
- Prioritize audible correctness before graphical polish.

## Command model

Once editing exists, both manual UI and optional AI operations must call the same validated command system, with undo/redo and explicit user control.

## Simplicity constraints

Initial product flow: **Create → Arrange → Export**. See [the design system](DESIGN.md).

Keep the step sequencer and piano roll discoverable. Avoid mimicking complex menus from existing products. Accessibility (labels, keyboard navigation, readable contrast) is a design requirement.
