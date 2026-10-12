# LIRUVA project format — version 2

LIRUVA saves patterns locally as UTF-8 JSON with the **`.liruva`** extension. The format is open, human-readable and designed for offline use. Both drum patterns and melodic notes survive an app restart.

## Example (version 2)

```json
{
  "format": "liruva-project",
  "format_version": 2,
  "name": "My First Song",
  "bpm": 120,
  "tracks": [4369, 4112, 21845, 4096],
  "melody": [0, null, null, null, 4, null, null, null, 7, null, null, null, 12, null, null, null]
}
```

## Schema

| Field | Type | Meaning |
| --- | --- | --- |
| `format` | string | Must be `liruva-project` |
| `format_version` | integer | Version 2 is written by this release |
| `name` | string | Nonblank printable title, 1–80 characters |
| `bpm` | integer | 40–240 beats per minute |
| `tracks` | array of 4 unsigned 16-bit integers | Kick, Snare, Hi-Hat, Clap |
| `melody` | array of 16 integers or null | One melodic pitch per sixteenth step, or a rest |

Drum integers are bitmasks, with bit 0 = step 1 and bit 15 = step 16. For instance, four quarter-note kick hits use `0x1111` (decimal `4369`).

Melody uses pitch indices `0`–`23` corresponding to chromatic semitones from **C4** (MIDI note 60) through **B5** (MIDI note 83); `null` means no new note is triggered at that step. Only one note per step is supported in the initial piano roll. Melodic audio is synthesized locally with no external samples.

## Backward compatibility

- **Version 1** drum-only files can be opened. They are migrated into an in-memory version-2 project with an empty melody.
- Saving a migrated project writes version 2. The original file is not changed merely by opening it.
- Unknown future versions are refused rather than guessed.

## Security and resilience

- Validate the entire project before modifying tempo, drum patterns, or melody.
- Reject invalid format identifiers, array sizes, note indices, unknown fields and malformed JSON.
- Read at most 64 KiB.
- Use a temporary sibling file, synchronize it and rename it over the chosen destination when saving.
- Loading a valid project stops the transport; a failed load leaves the current session untouched.
- No plugin state, accounts, third-party samples, online URLs, secrets or AI data are included.
- No proprietary FL Studio project import/export is implemented.

The schema will evolve with explicit version increments and migrations when genuinely new musical features require it.
