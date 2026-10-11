# LIRUVA Design System — Dark Studio

LIRUVA has an original dark design language inspired by the clarity of Material Design and the efficiency of modern music-production interfaces. It does **not** use FL Studio assets, branding, icons, or copied visual layouts.

## Design principles\n\n**Product language: English.** All UI labels, accessibility text, tooltips, errors, documentation, issues, and pull requests are written in English. No translation system is included yet.

1. **Dark by default**: reduce visual fatigue during long music sessions without obscuring interactive controls.
2. **Make the first action obvious**: one primary action per work area.
3. **Musicians over menus**: progressively disclose advanced features instead of showing all controls.
4. **Every control works**: do not display nonfunctional mixer knobs, audio waveforms, or play icons.
5. **Accessible contrast**: labels remain legible, primary and warning states are distinguishable, keyboard actions are discoverable.
6. **Original product identity**: LIRUVA uses its own typography treatment, warm accent and rounded surface structure.
7. **No mandatory AI**: music must always be possible without AI tools.

## Current palette

| Token | Hex | Role |
| --- | --- | --- |
| Background | `#11141A` | Window |
| Surface | `#1D212A` | Cards and transport |
| Raised surface | `#272C37` | Secondary card |
| Border | `#3E4451` | Subtle outlines |
| Primary | `#FFA751` | Main action, selected control |
| Foreground | `#EEF2F7` | Main text |
| Muted | `#A6AFBC` | Supporting text |
| Success | `#71DEB0` | Ready/playing indicator |\n| Kick | `#FFA751` | Warm amber |\n| Snare | `#79B5FF` | Soft blue |\n| Hi-Hat | `#71DEB0` | Mint |\n| Clap | `#C797FF` | Violet |

Use corners of 12–18 logical pixels, with generous spacing. Token values live in `crates/liruva-app/src/theme.rs`.

## Current real behavior

- Play and Stop control four locally synthesized drum sounds (Kick, Snare, Hi-Hat, Clap).
- The BPM slider affects the 16-step looping rhythm.
- Playback state, current step and triggered step count are read from the audio callback.
- Each numbered step is a clickable, keyboard-focusable button. Toggle sounds without stopping playback.
- A clear error explains when no compatible audio device is available.

## Not built

The 16-step grid, four drum sounds, instrument browsers, piano roll, playlist, mixer, and visual peak meter do not exist yet. Mockups are not features. Do not render interactive-looking placeholders for these areas.

## Accessibility to improve later

Keyboard focus indication and screen-reader labels need end-to-end testing on Windows, macOS and Linux. High-contrast color testing and non-color status cues must continue as the UI expands.
