# Roadmap

LIRUVA is developed incrementally; no roadmap item is considered delivered before it can be tested.

| Milestone | Deliverable | Status |
| --- | --- | --- |
| 0 — Foundation | Native Rust shell, complete public source, licensing, docs, CI | PR #1 checks passed; pending merge |
| 1.1 — First sound | One local kick, BPM slider, Play/Stop, dark Material-inspired UI | In review; device smoke test pending |
| 1.2 — First beat | 16-step sequencer, 4 synthesized percussion voices | Planned |
| 2 — Save projects | Open documented project format, create/save/load, round-trip tests | Planned |
| 3 — Notes | Simplified piano roll, note editing, pattern duplication | Planned |
| 4 — Arrange | Minimal timeline with repeatable patterns | Planned |
| 5 — Export | Deterministic offline WAV export | Planned |
| 6 — Mix | Volume, panning, metering, basic effects | Planned |
| 7 — Extend | MIDI and evaluation of compatible open plugin formats | Planned |
| 8 — Optional AI | Structured validated commands, optional local models and MCP | Planned |

## Milestone 1.2 acceptance criteria

- The desktop app opens into a clear, usable 16-step grid.
- Four percussion sounds are generated locally with no proprietary samples.
- Play/stop actually emits audio through a device.
- BPM adjusts musical timing consistently.
- Sequencer state remains stable under common UI interactions.
- Audio callback contains no blocking operations.
- Tests cover timing independently of an audio device.
- No AI key, account, network, or plugin is required.

## Not our goal

A feature-by-feature FL Studio clone, a purely visual DAW simulation, or an AI-only music generator.

Contributors: please open one focused issue per verifiable feature and avoid mixing future milestones into present-day PRs.
