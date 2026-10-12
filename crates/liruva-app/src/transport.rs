// SPDX-License-Identifier: GPL-3.0-or-later
//! Musical timing that can be tested without audio hardware.

pub struct BeatClock {
    sample_rate: f64,
    samples_to_next_beat: f64,
    was_playing: bool,
}

impl BeatClock {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate: f64::from(sample_rate),
            samples_to_next_beat: 0.0,
            was_playing: false,
        }
    }

    pub fn next_sample_is_beat(&mut self, playing: bool, bpm: u32) -> bool {
        if !playing {
            self.was_playing = false;
            self.samples_to_next_beat = 0.0;
            return false;
        }

        let interval = self.sample_rate * 60.0 / f64::from(bpm.clamp(40, 240));

        if !self.was_playing {
            self.was_playing = true;
            self.samples_to_next_beat = interval;
            return true;
        }

        self.samples_to_next_beat -= 1.0;

        if self.samples_to_next_beat <= 0.0 {
            self.samples_to_next_beat += interval;
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::BeatClock;

    #[test]
    fn emits_beats_at_120_bpm_and_48khz() {
        let mut clock = BeatClock::new(48_000);
        let indices: Vec<_> = (0..96_000)
            .filter(|_| clock.next_sample_is_beat(true, 120))
            .collect();
        assert_eq!(indices, [0, 24_000, 48_000, 72_000]);
    }

    #[test]
    fn handles_fractional_beat_intervals() {
        let mut clock = BeatClock::new(44_100);
        let beats = (0..44_100)
            .filter(|_| clock.next_sample_is_beat(true, 137))
            .count();
        assert_eq!(beats, 3);
    }

    #[test]
    fn silence_and_restarting_resets_beat_phase() {
        let mut clock = BeatClock::new(48_000);
        assert!(clock.next_sample_is_beat(true, 120));
        for _ in 0..200 {
            assert!(!clock.next_sample_is_beat(true, 120));
        }
        assert!(!clock.next_sample_is_beat(false, 120));
        assert!(clock.next_sample_is_beat(true, 120));
    }

    #[test]
    fn clamps_tempo_to_safe_boundaries() {
        let mut lower = BeatClock::new(48_000);
        let mut bounded = BeatClock::new(48_000);
        for _ in 0..50_000 {
            assert_eq!(
                lower.next_sample_is_beat(true, 0),
                bounded.next_sample_is_beat(true, 40)
            );
        }
    }
}
