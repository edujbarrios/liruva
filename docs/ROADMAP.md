# LIRUVA development roadmap

Every feature must work offline without AI. The entire interface, code documentation and GitHub collaboration are in English.

| Milestone | Deliverable | Status |
| --- | --- | --- |
| 0 — Foundation | Native Rust shell, GPL, docs and CI | Ready for main |
| 1.1 — First sound | Synthesized kick, Play/Stop, BPM and dark UI | Ready for main |
| 1.2 — Drum machine | Four voices and a looping 16-step sequencer | Ready for main |
| 2.1 — Project files | Versioned JSON, Save/Open and validation | Ready for main |
| 2.2 — Safe editing | New Project, unsaved-change guards, real GUI screenshot | Ready for main |
| 3.1 — First melody | C4–B5 piano roll, local synth, saved notes and v1 migration | In review |
| 3.2 — Notes refinement | Note lengths, multiple patterns and keyboard editing | Planned |
| 4 — Arrangement | Repeatable patterns on an intuitive timeline | Planned |
| 5 — Export | Offline WAV rendering | Planned |
| 6 — Mix | Volume, panning, meters and basic effects | Planned |
| 7 — Extend | MIDI and open plugin formats | Planned |
| 8 — Optional AI | Local and optional validated AI commands | Planned |

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
