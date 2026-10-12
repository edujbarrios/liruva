# Architecture

Status: **Milestone 3.1**. The drum sequencer now also supports validated, versioned local project files. The full DAW architecture remains a proposal.

## Philosophy

The smallest usable musical workflow wins. The interface should reveal complexity only when the user needs it. The application must remain functional without AI services, accounts, or network access.

## Current implementation

```text
Cargo workspace
└── crates/liruva-app/      Native dark desktop UI (egui/eframe)
    └── src/audio.rs       CPAL stream, four drum synthesizers, atomic patterns
    └── src/transport.rs   Sample-accurate 16-step clock (no audio hardware)
    └── src/project.rs     Versioned JSON serialization, validation, safe file I/O
    └── src/melody.rs      24 chromatic note names and frequencies\n    └── src/theme.rs       LIRUVA design tokens
```

The UI has a working transport and 16-step grid connected to a CPAL output callback. Four local percussion sounds are generated in the callback, and per-track 16-bit pattern masks are read via atomics. Editing any step never locks the audio thread. No audio is produced until Play is pressed.

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

## Project files

The project model in `src/project.rs` stores four drum masks, sixteen optional melodic notes, tempo and a display name in a documented, versioned `.liruva` JSON format. Deserialization and validation complete before applying data to the audio controls. The audio thread never performs file I/O. Saving first writes and synchronizes a temporary sibling file, then renames it to the chosen path, with errors displayed in the GUI.

Native file dialogs are intentionally modal and run from the desktop UI event loop; they are not part of the audio callback. Version 2 adds melody with an explicit migration for version-1 drum-only files. The format contains no samples or plugin data.

See [PROJECT_FORMAT.md](PROJECT_FORMAT.md).

## Unsaved project handling

The UI tracks unsaved edits and guards the Open, New, and window-close actions with a confirmation. A failed or canceled save must not automatically continue the destructive action. The existing sample-accurate audio callback remains separate from this UI-only workflow.

## Command model

Once editing exists, both manual UI and optional AI operations must call the same validated command system, with undo/redo and explicit user control.

## Simplicity constraints

Initial product flow: **Create → Arrange → Export**. See [the design system](DESIGN.md).

Keep the step sequencer and piano roll discoverable. Avoid mimicking complex menus from existing products. Accessibility (labels, keyboard navigation, readable contrast) is a design requirement.
