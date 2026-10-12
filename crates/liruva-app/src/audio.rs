// SPDX-License-Identifier: GPL-3.0-or-later
//! Four original percussion synths. The audio callback never locks or allocates.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample, Stream};

use crate::melody::{FREQUENCIES, MELODY_PITCH_COUNT, NO_NOTE};
use crate::transport::{STARTER_PATTERN, STEPS_PER_BAR, StepClock, TRACK_COUNT};

const STOPPED_STEP: u8 = u8::MAX;

pub struct AudioControls {
    playing: AtomicBool,
    bpm: AtomicU32,
    generation: AtomicU64,
    steps_triggered: AtomicU64,
    current_step: AtomicU8,
    patterns: [AtomicU16; TRACK_COUNT],
    melody: [AtomicU8; STEPS_PER_BAR],
}

impl Default for AudioControls {
    fn default() -> Self {
        Self {
            playing: AtomicBool::new(false),
            bpm: AtomicU32::new(120),
            generation: AtomicU64::new(0),
            steps_triggered: AtomicU64::new(0),
            current_step: AtomicU8::new(STOPPED_STEP),
            patterns: [
                AtomicU16::new(STARTER_PATTERN[0]),
                AtomicU16::new(STARTER_PATTERN[1]),
                AtomicU16::new(STARTER_PATTERN[2]),
                AtomicU16::new(STARTER_PATTERN[3]),
            ],
            melody: std::array::from_fn(|_| AtomicU8::new(NO_NOTE)),
        }
    }
}

impl AudioControls {
    pub fn is_playing(&self) -> bool {
        self.playing.load(Ordering::Acquire)
    }

    pub fn set_playing(&self, playing: bool) {
        if self.playing.swap(playing, Ordering::AcqRel) != playing {
            if playing {
                self.steps_triggered.store(0, Ordering::Relaxed);
            }
            self.current_step.store(STOPPED_STEP, Ordering::Release);
            self.generation.fetch_add(1, Ordering::AcqRel);
        }
    }

    pub fn bpm(&self) -> u32 {
        self.bpm.load(Ordering::Relaxed)
    }

    pub fn set_bpm(&self, bpm: u32) {
        self.bpm.store(bpm.clamp(40, 240), Ordering::Relaxed);
    }

    pub fn steps_triggered(&self) -> u64 {
        self.steps_triggered.load(Ordering::Relaxed)
    }

    pub fn current_step(&self) -> Option<usize> {
        let step = self.current_step.load(Ordering::Acquire);
        (step < STEPS_PER_BAR as u8).then_some(usize::from(step))
    }

    pub fn step_enabled(&self, track: usize, step: usize) -> bool {
        debug_assert!(track < TRACK_COUNT && step < STEPS_PER_BAR);
        self.patterns[track].load(Ordering::Acquire) & (1u16 << step) != 0
    }

    pub fn toggle_step(&self, track: usize, step: usize) {
        if track < TRACK_COUNT && step < STEPS_PER_BAR {
            self.patterns[track].fetch_xor(1u16 << step, Ordering::AcqRel);
        }
    }

    pub fn pattern_masks(&self) -> [u16; TRACK_COUNT] {
        std::array::from_fn(|track| self.patterns[track].load(Ordering::Acquire))
    }

    pub fn set_pattern_masks(&self, masks: [u16; TRACK_COUNT]) {
        for (track, mask) in self.patterns.iter().zip(masks) {
            track.store(mask, Ordering::Release);
        }
    }

    pub fn melody_note(&self, step: usize) -> Option<u8> {
        let pitch = self.melody[step].load(Ordering::Acquire);
        (usize::from(pitch) < MELODY_PITCH_COUNT).then_some(pitch)
    }

    pub fn set_melody_note(&self, step: usize, note: Option<u8>) {
        if step < STEPS_PER_BAR {
            let encoded = note
                .filter(|pitch| usize::from(*pitch) < MELODY_PITCH_COUNT)
                .unwrap_or(NO_NOTE);
            self.melody[step].store(encoded, Ordering::Release);
        }
    }

    pub fn melody_notes(&self) -> [Option<u8>; STEPS_PER_BAR] {
        std::array::from_fn(|step| self.melody_note(step))
    }

    pub fn set_melody_notes(&self, notes: [Option<u8>; STEPS_PER_BAR]) {
        for (step, pitch) in notes.into_iter().enumerate() {
            self.set_melody_note(step, pitch);
        }
    }

    pub fn clear_melody(&self) {
        self.set_melody_notes([None; STEPS_PER_BAR]);
    }

    pub fn load_demo_melody(&self) {
        let mut notes = [None; STEPS_PER_BAR];
        for (step, pitch) in [(0, 0), (4, 4), (8, 7), (12, 12)] {
            notes[step] = Some(pitch);
        }
        self.set_melody_notes(notes);
    }

    pub fn clear_pattern(&self) {
        for track in &self.patterns {
            track.store(0, Ordering::Release);
        }
    }

    pub fn load_starter_pattern(&self) {
        for (track, mask) in self.patterns.iter().zip(STARTER_PATTERN) {
            track.store(mask, Ordering::Release);
        }
    }
}

/// One independent, locally synthesized percussion voice.
struct DrumVoice {
    kind: usize,
    active: bool,
    time: f32,
    phase: f32,
    filter: f32,
    noise_state: u32,
}

impl DrumVoice {
    fn new(kind: usize) -> Self {
        Self {
            kind,
            active: false,
            time: 0.0,
            phase: 0.0,
            filter: 0.0,
            noise_state: 0xA591_6C3Du32.wrapping_add(kind as u32 * 31),
        }
    }

    fn trigger(&mut self) {
        self.active = true;
        self.time = 0.0;
        self.phase = 0.0;
        self.filter = 0.0;
    }

    fn stop(&mut self) {
        self.active = false;
    }

    fn noise(&mut self) -> f32 {
        let mut value = self.noise_state;
        value ^= value << 13;
        value ^= value >> 17;
        value ^= value << 5;
        self.noise_state = value;
        (value as f32 / u32::MAX as f32) * 2.0 - 1.0
    }

    fn next(&mut self, sample_rate: f32) -> f32 {
        if !self.active {
            return 0.0;
        }

        let time = self.time;
        self.time += 1.0 / sample_rate;
        let result = match self.kind {
            // Punchy falling sine wave: no sampled audio or external assets.
            0 => {
                let frequency = 52.0 + 135.0 * (-35.0 * time).exp();
                self.phase += std::f32::consts::TAU * frequency / sample_rate;
                self.phase.sin() * (-16.0 * time).exp() * 0.90
            }
            // Noise burst with a subtle tonal body.
            1 => {
                self.phase += std::f32::consts::TAU * 175.0 / sample_rate;
                let burst = self.noise() * 0.75 + self.phase.sin() * 0.25;
                burst * (-20.0 * time).exp() * 0.65
            }
            // High-pass filtered white noise for a closed hi-hat.
            2 => {
                let noise = self.noise();
                self.filter = self.filter * 0.90 + noise * 0.10;
                (noise - self.filter) * (-95.0 * time).exp() * 0.46
            }
            // Clustered transients followed by a soft diffuse tail.
            _ => {
                let noise = self.noise();
                self.filter = self.filter * 0.83 + noise * 0.17;
                let high = noise - self.filter;
                let burst = (time / 0.014).floor();
                let envelope = if burst < 3.0 {
                    (-135.0 * (time - burst * 0.014)).exp() * 0.42
                } else {
                    (-30.0 * (time - 0.042)).exp() * 0.25
                };
                high * envelope
            }
        };

        if self.time > 0.55 {
            self.active = false;
        }
        result
    }
}

/// Lightweight, deterministic melodic oscillator; no device calls or allocations.
struct MelodyVoice {
    active: bool,
    phase: f32,
    elapsed: f32,
    frequency: f32,
}

impl MelodyVoice {
    fn new() -> Self {
        Self {
            active: false,
            phase: 0.0,
            elapsed: 0.0,
            frequency: 0.0,
        }
    }

    fn trigger(&mut self, pitch: u8) {
        self.active = true;
        self.phase = 0.0;
        self.elapsed = 0.0;
        self.frequency = FREQUENCIES[usize::from(pitch)];
    }

    fn stop(&mut self) {
        self.active = false;
    }

    fn next(&mut self, sample_rate: f32) -> f32 {
        if !self.active {
            return 0.0;
        }
        let attack = (self.elapsed * 140.0).min(1.0);
        let envelope = attack * (-9.0 * self.elapsed).exp();
        self.phase += std::f32::consts::TAU * self.frequency / sample_rate;
        self.elapsed += 1.0 / sample_rate;
        if self.elapsed > 0.55 {
            self.active = false;
        }
        (self.phase.sin() * 0.82 + (self.phase * 2.0).sin() * 0.18) * envelope * 0.32
    }
}

struct DrumEngine {
    clock: StepClock,
    sample_rate: f32,
    voices: [DrumVoice; TRACK_COUNT],
    melody_voice: MelodyVoice,
    seen_generation: u64,
}

impl DrumEngine {
    fn new(sample_rate: u32) -> Self {
        Self {
            clock: StepClock::new(sample_rate),
            sample_rate: sample_rate as f32,
            voices: [
                DrumVoice::new(0),
                DrumVoice::new(1),
                DrumVoice::new(2),
                DrumVoice::new(3),
            ],
            melody_voice: MelodyVoice::new(),
            seen_generation: 0,
        }
    }

    fn next(&mut self, controls: &AudioControls) -> f32 {
        let generation = controls.generation.load(Ordering::Acquire);
        if generation != self.seen_generation {
            self.seen_generation = generation;
            self.clock.reset();
            for voice in &mut self.voices {
                voice.stop();
            }
            self.melody_voice.stop();
        }

        let playing = controls.is_playing();
        if !playing {
            self.clock.reset();
            for voice in &mut self.voices {
                voice.stop();
            }
            self.melody_voice.stop();
            return 0.0;
        }

        if let Some(step) = self.clock.advance(true, controls.bpm()) {
            controls.current_step.store(step as u8, Ordering::Release);
            controls.steps_triggered.fetch_add(1, Ordering::Relaxed);
            let bit = 1u16 << step;
            for (voice, mask) in self.voices.iter_mut().zip(&controls.patterns) {
                if mask.load(Ordering::Acquire) & bit != 0 {
                    voice.trigger();
                }
            }
            if let Some(pitch) = controls.melody_note(step) {
                self.melody_voice.trigger(pitch);
            }
        }

        let sum: f32 = self
            .voices
            .iter_mut()
            .map(|voice| voice.next(self.sample_rate))
            .sum();
        ((sum * 0.75 + self.melody_voice.next(self.sample_rate)) * 0.85).tanh()
    }

    fn fill<T: SizedSample + FromSample<f32>>(
        &mut self,
        data: &mut [T],
        channels: usize,
        controls: &AudioControls,
    ) {
        for frame in data.chunks_exact_mut(channels) {
            let sample = self.next(controls);
            for channel in frame {
                *channel = T::from_sample(sample);
            }
        }
    }
}

/// Keep the returned stream alive while the desktop application is running.
pub fn open_stream(controls: Arc<AudioControls>) -> Result<Stream, String> {
    let device = cpal::default_host()
        .default_output_device()
        .ok_or_else(|| "No audio output device is available.".to_string())?;
    let supported = device
        .default_output_config()
        .map_err(|error| format!("Cannot read output configuration: {error}"))?;
    let sample_format = supported.sample_format();
    let config = supported.config();
    let channels = usize::from(config.channels);
    if channels == 0 || config.sample_rate == 0 {
        return Err("Invalid output device configuration.".to_string());
    }

    let rate = config.sample_rate;
    let stream = match sample_format {
        SampleFormat::F32 => {
            let mut engine = DrumEngine::new(rate);
            let ctrl = Arc::clone(&controls);
            device.build_output_stream(
                config,
                move |data: &mut [f32], _| engine.fill(data, channels, &ctrl),
                |error| eprintln!("LIRUVA audio error: {error}"),
                None,
            )
        }
        SampleFormat::I16 => {
            let mut engine = DrumEngine::new(rate);
            let ctrl = Arc::clone(&controls);
            device.build_output_stream(
                config,
                move |data: &mut [i16], _| engine.fill(data, channels, &ctrl),
                |error| eprintln!("LIRUVA audio error: {error}"),
                None,
            )
        }
        SampleFormat::U16 => {
            let mut engine = DrumEngine::new(rate);
            let ctrl = Arc::clone(&controls);
            device.build_output_stream(
                config,
                move |data: &mut [u16], _| engine.fill(data, channels, &ctrl),
                |error| eprintln!("LIRUVA audio error: {error}"),
                None,
            )
        }
        unsupported => {
            return Err(format!(
                "Output sample format {unsupported:?} is not supported yet."
            ));
        }
    }
    .map_err(|error| format!("Cannot create audio stream: {error}"))?;

    stream
        .play()
        .map_err(|error| format!("Cannot start audio stream: {error}"))?;
    Ok(stream)
}

#[cfg(test)]
mod tests {
    use super::{AudioControls, DrumEngine, DrumVoice};
    use crate::transport::{STARTER_PATTERN, STEPS_PER_BAR, TRACK_COUNT};

    #[test]
    fn each_voice_produces_original_finite_sound() {
        for kind in 0..TRACK_COUNT {
            let mut voice = DrumVoice::new(kind);
            voice.trigger();
            let samples: Vec<f32> = (0..2_000).map(|_| voice.next(48_000.0)).collect();
            assert!(samples.iter().all(|sample| sample.is_finite()));
            assert!(samples.iter().any(|sample| sample.abs() > 0.01));
        }
    }

    #[test]
    fn patterns_toggle_without_affecting_other_tracks() {
        let controls = AudioControls::default();
        let before = controls.step_enabled(0, 1);
        controls.toggle_step(0, 1);
        assert_ne!(controls.step_enabled(0, 1), before);
        assert_eq!(controls.step_enabled(1, 1), STARTER_PATTERN[1] & 2 != 0);
        controls.toggle_step(0, 1);
        assert_eq!(controls.step_enabled(0, 1), before);
        controls.clear_pattern();
        for track in 0..TRACK_COUNT {
            for step in 0..STEPS_PER_BAR {
                assert!(!controls.step_enabled(track, step));
            }
        }
        controls.load_starter_pattern();
        assert!(controls.step_enabled(0, 0));
    }

    #[test]
    fn sequence_advances_and_stops_without_audio_device() {
        let controls = AudioControls::default();
        controls.set_playing(true);
        let mut engine = DrumEngine::new(48_000);
        let samples: Vec<f32> = (0..6_000).map(|_| engine.next(&controls)).collect();
        assert!(samples.iter().any(|sample| sample.abs() > 0.01));
        assert!(
            samples
                .iter()
                .all(|sample| sample.is_finite() && sample.abs() <= 0.85)
        );
        assert_eq!(controls.current_step(), Some(0));
        assert_eq!(controls.steps_triggered(), 1);
        engine.next(&controls);
        assert_eq!(controls.current_step(), Some(1));
        assert_eq!(controls.steps_triggered(), 2);

        controls.set_playing(false);
        assert_eq!(engine.next(&controls), 0.0);
        assert_eq!(controls.current_step(), None);
        controls.set_playing(true);
        engine.next(&controls);
        assert_eq!(controls.current_step(), Some(0));
    }

    #[test]
    fn stereo_buffer_repeats_samples_per_channel() {
        let controls = AudioControls::default();
        controls.set_playing(true);
        let mut engine = DrumEngine::new(48_000);
        let mut output = [0.0f32; 512];
        engine.fill(&mut output, 2, &controls);
        for pair in output.as_chunks::<2>().0 {
            assert_eq!(pair[0], pair[1]);
        }
    }

    #[test]
    fn melodic_notes_can_be_edited_without_changing_drums() {
        let controls = AudioControls::default();
        let drums = controls.pattern_masks();
        assert_eq!(controls.melody_note(0), None);
        controls.set_melody_note(0, Some(12));
        assert_eq!(controls.melody_note(0), Some(12));
        controls.set_melody_note(0, Some(25));
        assert_eq!(controls.melody_note(0), None);
        controls.load_demo_melody();
        assert_eq!(controls.melody_note(4), Some(4));
        controls.clear_melody();
        assert_eq!(controls.melody_notes(), [None; STEPS_PER_BAR]);
        assert_eq!(controls.pattern_masks(), drums);
    }

    #[test]
    fn synth_plays_with_drums_and_stops_without_audio_hardware() {
        let controls = AudioControls::default();
        controls.clear_pattern();
        controls.set_melody_note(0, Some(9));
        controls.set_playing(true);
        let mut engine = DrumEngine::new(48_000);
        let output: Vec<_> = (0..2_000).map(|_| engine.next(&controls)).collect();
        assert!(output.iter().any(|sample| sample.abs() > 0.02));
        assert!(
            output
                .iter()
                .all(|sample| sample.is_finite() && sample.abs() <= 1.0)
        );
        controls.set_playing(false);
        assert_eq!(engine.next(&controls), 0.0);
    }

    #[test]
    fn tempo_bounds_and_empty_pattern() {
        let controls = AudioControls::default();
        controls.set_bpm(1);
        assert_eq!(controls.bpm(), 40);
        controls.set_bpm(999);
        assert_eq!(controls.bpm(), 240);

        controls.clear_pattern();
        controls.set_playing(true);
        let mut engine = DrumEngine::new(44_100);
        for _ in 0..10_000 {
            assert_eq!(engine.next(&controls), 0.0);
        }
        assert!(controls.steps_triggered() > 0);
    }
}
