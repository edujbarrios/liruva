# Architecture

Status: **Milestone 0 proposal**. Only the desktop shell is implemented today.

## Philosophy

The smallest usable musical workflow wins. The interface should reveal complexity only when the user needs it. The application must remain functional without AI services, accounts, or network access.

## Current implementation

```text
Cargo workspace
└── crates/liruva-app/      Native desktop shell using Rust + eframe/egui
```

The UI currently displays static, truthful project status. It does not generate audio.

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

Initial product flow: **Create → Arrange → Export**.

Keep the step sequencer and piano roll discoverable. Avoid mimicking complex menus from existing products. Accessibility (labels, keyboard navigation, readable contrast) is a design requirement.
