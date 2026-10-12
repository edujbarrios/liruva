// SPDX-License-Identifier: GPL-3.0-or-later
//! A deliberately small melodic grid: one note (or a rest) per sixteenth step.
//! All indexing, note labels and pitch conversions are independent of audio hardware.

pub const MELODY_PITCH_COUNT: usize = 24;
pub const NO_NOTE: u8 = u8::MAX;
pub const MELODY_NOTE_NAMES: [&str; MELODY_PITCH_COUNT] = [
    "C4", "C#4", "D4", "D#4", "E4", "F4", "F#4", "G4", "G#4", "A4", "A#4", "B4",
    "C5", "C#5", "D5", "D#5", "E5", "F5", "F#5", "G5", "G#5", "A5", "A#5", "B5",
];
/// Equal-tempered notes from MIDI 60 (C4) to MIDI 83 (B5).
pub const FREQUENCIES: [f32; MELODY_PITCH_COUNT] = [
    261.626, 277.183, 293.665, 311.127, 329.628, 349.228, 369.994, 391.995,
    415.305, 440.000, 466.164, 493.883, 523.251, 554.365, 587.330, 622.254,
    659.255, 698.456, 739.989, 783.991, 830.609, 880.000, 932.328, 987.767,
];

pub fn pitch_name(pitch: usize) -> &'static str {
    MELODY_NOTE_NAMES.get(pitch).copied().unwrap_or("Unknown")
}

#[cfg(test)]
mod tests {
    use super::{FREQUENCIES, MELODY_PITCH_COUNT, pitch_name};

    #[test]
    fn two_octaves_have_ascending_frequencies_and_correct_labels() {
        assert_eq!(MELODY_PITCH_COUNT, 24);
        assert_eq!(pitch_name(0), "C4");
        assert_eq!(pitch_name(12), "C5");
        assert_eq!(pitch_name(23), "B5");
        assert_eq!(pitch_name(24), "Unknown");
        for notes in FREQUENCIES.windows(2) {
            assert!(notes[1] > notes[0]);
        }
    }
}
