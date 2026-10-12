# LIRUVA design system — Dark Studio

LIRUVA has its own original dark design language, inspired by the clarity and progressive disclosure of Material Design and the efficiency of modern music-production software. It does **not** copy FL Studio's branding, graphics, or interface assets.

**Product language: English.** UI labels, warnings, tooltips, documentation, GitHub issues, and pull requests use English.

## Core principles

1. **Dark by default:** reduce visual fatigue without sacrificing contrast.
2. **One obvious next action:** beginners should see how to start.
3. **Musicians over menus:** avoid extensive settings and unexplained controls.
4. **Every control is real:** no fake mixer knobs, transport controls, or buttons.
5. **Accessible UI:** readable labels, keyboard-focusable controls, visible selection and progress.
6. **Original identity:** warm accent colors, clear surfaces and intuitive rhythm visualization.
7. **AI is optional:** full basic music creation works offline without AI.

## Palette

| Token | Hex | Role |
| --- | --- | --- |
| Background | `#11141A` | Application background |
| Surface | `#1D212A` | Main panels |
| Raised surface | `#272C37` | Project strip and secondary surfaces |
| Border | `#3E4451` | Subtle outlines |
| Primary | `#FFA751` | Main action and selected control |
| Foreground | `#EEF2F7` | Main text |
| Muted | `#A6AFBC` | Supporting text |
| Success | `#71DEB0` | Ready / playing / saved |
| Kick | `#FFA751` | Amber |
| Snare | `#79B5FF` | Blue |
| Hi-Hat | `#71DEB0` | Mint |
| Clap | `#C797FF` | Violet |

Token definitions live in `crates/liruva-app/src/theme.rs`. Main surfaces use rounded corners of about 12–18 logical pixels.

## Current interactions

- Click numbered step buttons to enable or disable each instrument.
- Play and Stop start and stop locally synthesized drum audio.
- Tempo adjusts sixteenth-note playback timing.
- The current step is outlined using the actual audio clock state.
- Save, Save As and Open use native dialogs, with plain-English error messages.
- Unsaved changes trigger a confirmation when the user opens a project, creates a new project, or closes the window.
- The user can Save and continue, Discard changes, or Cancel.
- Loading a malformed project never replaces the existing pattern.

## Piano roll\n\nA compact, vertical-scroll note editor maps one note per sixteenth step across two octaves (C4–B5). Its playhead shares the drum machine's timing. Enabled notes are mint-colored and exposed through English labels and tooltips. The default state contains no melodic notes; a demo melody button provides a quick starting point.\n\n## Not built yet

Advanced piano-roll editing, arrangement, mixer, sample browser, waveform editor, plugin hosting, MIDI input, undo/redo and advanced accessibility features are future milestones, **not interactive placeholders**.

## Real application screenshot

The public README includes a **capture from the real Rust app**, not an artist's rendering. The automated capture workflow builds the app in Linux, starts it with Xvfb, locates the visible application window, and captures its actual pixels. Physical speakers and file-dialog behavior are still separate desktop smoke tests.

Capture source: `scripts/capture-screenshot.sh` and `.github/workflows/screenshot-preview.yml`.

## Follow-up accessibility work

Test the native file dialogs and screen readers on Windows, macOS and Linux. Add a high-contrast mode and full keyboard navigation as dedicated, tested improvements. Avoid using color as the only way to indicate state.
