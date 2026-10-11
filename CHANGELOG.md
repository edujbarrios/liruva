# Changelog

All notable project milestones will be documented here.

## Unreleased — Milestone 2.1

- Added a documented, versioned, human-readable `.liruva` project format.
- Added native Open, Save and Save As actions plus editable project names.
- Added a confirmation before opening another project with unsaved edits.
- Added validation and 64 KiB load limit; rejected projects leave active session unchanged.
- Added temporary-file-based writes and file round-trip/error tests.
- Cleaned English documentation and removed accidental literal newline escapes.

## Unreleased — Milestone 1.2

- Added a real, looping 16-step sequencer in 4/4 with four synthesized drum voices.
- Added an interactive dark step grid, playhead, starter pattern and clear action.
- Added lock-free step editing via atomic pattern masks, with no real-time callback locks.
- Added sample-accurate timing and deterministic tests for clock, percussion, and transport.
- Committed to English-only product text, documentation and contributor communications.

### Previous: Milestone 1.1

- Added a custom dark Material-inspired interface with LIRUVA's own design tokens.
- Added an optional-device-safe CPAL audio stream, real synthesized kick sound, Play/Stop and adjustable BPM.
- Added a deterministic beat clock and independent unit tests.
- Added design guidelines and audio-development notes.

### Previous: Milestone 0

- GPL-3.0-or-later project foundation.
- Minimal Rust workspace and native welcome screen.
- CI and initial contributor documentation.

Milestone 0 did not include playable audio; Milestone 1.1 introduces the first sound.
