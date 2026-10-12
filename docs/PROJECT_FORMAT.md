# LIRUVA project format — version 1

LIRUVA saves patterns locally as UTF-8 JSON with the **`.liruva`** extension. The format is open, human-readable and covered by the same open-source project documentation.

## Example

```json
{
  "format": "liruva-project",
  "format_version": 1,
  "name": "My First Beat",
  "bpm": 120,
  "tracks": [4369, 4112, 21845, 4096]
}
```

## Schema

| Field | Type | Meaning |
| --- | --- | --- |
| `format` | string | Must be `liruva-project` |
| `format_version` | integer | Must be `1` |
| `name` | string | Nonblank printable title, 1–80 characters |
| `bpm` | integer | Tempo in beats per minute, range 40–240 |
| `tracks` | array of four unsigned 16-bit integers | Kick, Snare, Hi-Hat, Clap in this fixed order |

Each track's integer is a 16-bit mask. Bit 0 represents step 1, bit 15 represents step 16. A set bit means that instrument plays at the corresponding sixteenth-note subdivision. Patterns loop through one 4/4 bar.

For example, a kick playing steps 1, 5, 9 and 13 uses mask `0x1111` (decimal `4369`).

## Compatibility and safety

- Files with unknown format versions, invalid BPM values, malformed JSON, invalid arrays or unexpected keys are rejected with a user-facing error.
- Loading is validated **before** replacing the pattern. A rejected file leaves the current session unchanged.
- Opening a valid file stops transport, sets the saved BPM and updates all four tracks.
- Save first writes a temporary sibling file, flushes it, then renames to the final path. An error does not intentionally overwrite the original file with incomplete data.
- The loader reads no more than 64 KiB.
- No samples, external URLs, secrets, plugins, account identifiers or AI settings are stored.
- Cross-version migrations are **not** implemented yet. Unknown versions are rejected instead of guessed at.
- No project import or export to proprietary `.flp` format is implemented.

The project format is a deliberately small first step. Future versions may add arrangement, instruments and compatibility migrations without requiring proprietary services.
