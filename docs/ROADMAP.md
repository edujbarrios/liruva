# Development roadmap

LIRUVA is built incrementally. Features are not considered shipped until tested and reviewed. All source and user-facing content are in English.

| Milestone | Deliverable | Status |
| --- | --- | --- |
| 0 — Foundation | Native Rust shell, GPL license, docs, CI | PR #1 — checks passed, pending merge |
| 1.1 — First sound | Locally synthesized kick, Play/Stop, tempo, dark interface | PR #3 — checks passed, pending merge |
| 1.2 — First beat | Four voices, 16-step looping drum sequencer | PR #5 — tests passed, desktop smoke test pending |
| 2.1 — Project files | Versioned JSON, Save/Open, validation and tests | In development, separate PR |
| 2.2 — Project polish | Improved unsaved-change handling, project naming and file recovery | Planned |
| 3 — Notes | Simplified piano roll, note editing, pattern duplication | Planned |
| 4 — Arrange | Timeline with repeatable patterns | Planned |
| 5 — Export | Deterministic offline WAV export | Planned |
| 6 — Mix | Volume, panning, metering and essential effects | Planned |
| 7 — Extend | MIDI and compatible open plugin formats | Planned |
| 8 — Optional AI | Validated commands, optional local models and MCP | Planned |

## Milestone 2.1 acceptance criteria

- Save a named pattern in a small, open, documented `.liruva` file.
- Restore the exact 16-step masks and BPM after closing and reopening the desktop app.
- An invalid file must leave the current project untouched and display an understandable error.
- Unsaved edits must not be silently lost when opening another project.
- Disk operations must remain outside the real-time audio callback.
- Automated serialization, validation and file I/O tests must pass.
- File dialogs and audio must be tested on a real desktop before a stable release.

## Non-goals

We are not recreating FL Studio feature-for-feature, distributing proprietary samples, introducing accounts, or building an AI-only music generator.

Contributors should keep each Pull Request small, verifiable, and focused on one user-facing improvement.
