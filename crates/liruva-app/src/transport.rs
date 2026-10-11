// SPDX-License-Identifier: GPL-3.0-or-later
//! Sample-accurate sixteenth-note timing, independent of any audio device.

pub const STEPS_PER_BAR: usize = 16;
pub const TRACK_COUNT: usize = 4;
pub const TRACK_NAMES: [&str; TRACK_COUNT] = ["Kick", "Snare", "Hi-Hat", "Clap"];

/// One 4/4 bar: kick on quarters, snare on beats 2 and 4,
/// closed hi-hat on eighth notes, clap layered on beat 4.
pub const STARTER_PATTERN: [u16; TRACK_COUNT] = [0x1111, 0x1010, 0x5555, 0x1000];

pub struct StepClock {
    sample_rate: f64,
    samples_since_step: f64,
    next_step: usize,
    was_playing: bool,
}

impl StepClock {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate: f64::from(sample_rate),
            samples_since_step: 0.0,
            next_step: 0,
            was_playing: false,
        }
    }

    pub fn reset(&mut self) {
        self.samples_since_step = 0.0;
        self.next_step = 0;
        self.was_playing = false;
    }

    /// Returns the step to trigger at this exact sample, or None.
    /// BPM changes take effect during playback without restarting the bar.
    pub fn advance(&mut self, playing: bool, bpm: u32) -> Option<usize> {
        if !playing {
            self.reset();
            return None;
        }

        if !self.was_playing {
            self.was_playing = true;
            self.next_step = 1;
            return Some(0);
        }

        let interval = self.sample_rate * 60.0 / (f64::from(bpm.clamp(40, 240)) * 4.0);
        self.samples_since_step += 1.0;

        if self.samples_since_step >= interval {
            self.samples_since_step -= interval;
            let step = self.next_step;
            self.next_step = (self.next_step + 1) % STEPS_PER_BAR;
            Some(step)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{STEPS_PER_BAR, StepClock};

    #[test]
    fn sixteenth_notes_are_sample_accurate_at_120_bpm() {
        let mut clock = StepClock::new(48_000);
        let positions: Vec<_> = (0..96_000)
            .filter_map(|sample| clock.advance(true, 120).map(|step| (sample, step)))
            .collect();
        assert_eq!(positions.len(), STEPS_PER_BAR);
        for (index, (sample, step)) in positions.into_iter().enumerate() {
            assert_eq!(sample, index * 6_000);
            assert_eq!(step, index);
        }
    }

    #[test]
    fn patterns_loop_back_to_zero_after_step_sixteen() {
        let mut clock = StepClock::new(48_000);
        let steps: Vec<_> = (0..102_000).filter_map(|_| clock.advance(true, 120)).collect();
        assert_eq!(steps.len(), 17);
        assert_eq!(steps[0], 0);
        assert_eq!(steps[15], 15);
        assert_eq!(steps[16], 0);
    }

    #[test]
    fn stop_and_restart_begin_on_first_step() {
        let mut clock = StepClock::new(44_100);
        assert_eq!(clock.advance(true, 120), Some(0));
        for _ in 0..1_000 {
            assert_eq!(clock.advance(true, 120), None);
        }
        assert_eq!(clock.advance(false, 120), None);
        assert_eq!(clock.advance(true, 120), Some(0));
    }

    #[test]
    fn fractional_sample_intervals_do_not_drift() {
        let mut clock = StepClock::new(44_100);
        let positions: Vec<_> = (0..441_000)
            .filter(|_| clock.advance(true, 137).is_some())
            .collect();
        // 10 seconds at 137 BPM = 91.333 sixteenth-note intervals.
        assert_eq!(positions.len(), 92);
    }

    #[test]
    fn tempo_changes_keep_step_order() {
        let mut clock = StepClock::new(48_000);
        let mut events = Vec::new();
        for sample in 0..48_000 {
            if let Some(step) = clock.advance(true, if sample < 24_000 { 100 } else { 160 }) {
                events.push(step);
            }
        }
        for pair in events.windows(2) {
            assert_eq!(pair[1], (pair[0] + 1) % STEPS_PER_BAR);
        }
        assert!(events.len() >= 6);
    }
}
