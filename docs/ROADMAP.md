# LIRUVA development roadmap

Every feature must work offline without AI. The entire interface, code documentation and GitHub collaboration are in English.

| Milestone | Deliverable | Status |
| --- | --- | --- |
| 0 — Foundation | Native Rust shell, GPL, docs and CI | Merged into main |
| 1.1 — First sound | Synthesized kick, Play/Stop, BPM and dark UI | Merged into main |
| 1.2 — Drum machine | Four voices and a looping 16-step sequencer | Merged into main |
| 2.1 — Project files | Versioned JSON, Save/Open and validation | Merged into main |
| 2.2 — Safe editing | New Project, unsaved-change guards, real GUI screenshot | Merged into main |
| 3.1 — First melody | C4–B5 piano roll, local synth, saved notes and v1 migration | Merged into main |
| 3.2 — Safe editing | Undo/Redo for beat, melody, BPM, and project name | In review |
| 3.3 — Notes refinement | Note lengths, multiple patterns and keyboard editing | Planned |
| 4 — Arrangement | Repeatable patterns on an intuitive timeline | Planned |
| 5 — Export | Offline WAV rendering | Planned |
| 6 — Mix | Volume, panning, meters and basic effects | Planned |
| 7 — Extend | MIDI and open plugin formats | Planned |
| 8 — Optional AI | Local and optional validated AI commands | Planned |

## Milestone 3.2 acceptance criteria

- Undo and Redo reverse actual drum hits, melody notes, tempo changes, and project-name edits.
- The editor offers visible buttons and Ctrl/Cmd+Z, Ctrl/Cmd+Shift+Z or Ctrl/Cmd+Y.
- Undoing an edit back to the saved state clears the unsaved-change marker.
- Editing after an undo discards the redo branch; history stays bounded to 64 states.
- New/Open reset history, while Save establishes the current saved-state baseline.
- Audio callback remains lock-free and does not perform history processing.
- Automated tests and genuine app screenshot capture pass before merging to main.

## Milestone 3.1 acceptance criteria

- Click a note in a simple two-octave piano roll, press Play and hear real synthesized pitched sound.
- Drum and melody tracks must share sample-accurate step timing and tempo.
- A new project clears the melody; Save/Open preserves the exact musical notes.
- All legacy `.liruva` format-v1 drum projects must open with an empty melody.
- Malformed projects or notes outside the 24-pitch range must be rejected.
- CI passes Rust formatting, strict Clippy and hardware-independent unit tests.
- The README displays a genuine capture of the app, not an illustration.
- Native file dialogs and speaker output remain subject to manual desktop smoke tests.

## Non-goals

A commercial DAW clone, proprietary dependencies, required AI services, complex plugin hosting, cloud accounts or subscriptions. Build the smallest real workflow first.
