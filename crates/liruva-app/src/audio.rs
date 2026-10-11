// SPDX-License-Identifier: GPL-3.0-or-later
//! A single fully synthesized kick, played by a lock-free audio callback.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample, Stream};

use crate::transport::BeatClock;

pub struct AudioControls {
    playing: AtomicBool,
    bpm: AtomicU32,
    beats: AtomicU64,
}

impl Default for AudioControls {
    fn default() -> Self {
        Self {
            playing: AtomicBool::new(false),
            bpm: AtomicU32::new(120),
            beats: AtomicU64::new(0),
        }
    }
}

impl AudioControls {
    pub fn is_playing(&self) -> bool {
        self.playing.load(Ordering::Acquire)
    }

    pub fn set_playing(&self, playing: bool) {
        if playing {
            self.beats.store(0, Ordering::Relaxed);
        }
        self.playing.store(playing, Ordering::Release);
    }

    pub fn bpm(&self) -> u32 {
        self.bpm.load(Ordering::Relaxed)
    }

    pub fn set_bpm(&self, bpm: u32) {
        self.bpm.store(bpm.clamp(40, 240), Ordering::Relaxed);
    }

    pub fn beats(&self) -> u64 {
        self.beats.load(Ordering::Relaxed)
    }
}

struct Kick {
    time: f32,
    phase: f32,
    active: bool,
}

impl Kick {
    fn new() -> Self {
        Self {
            time: 0.0,
            phase: 0.0,
            active: false,
        }
    }

    fn trigger(&mut self) {
        self.time = 0.0;
        self.phase = 0.0;
        self.active = true;
    }

    fn stop(&mut self) {
        self.active = false;
    }

    fn next(&mut self, sample_rate: f32) -> f32 {
        if !self.active {
            return 0.0;
        }

        let frequency = 52.0 + 110.0 * (-28.0 * self.time).exp();
        let envelope = (-16.0 * self.time).exp();
        self.phase += std::f32::consts::TAU * frequency / sample_rate;
        self.time += 1.0 / sample_rate;

        if envelope < 0.00005 {
            self.active = false;
        }

        (self.phase.sin() * envelope * 0.65).tanh()
    }
}

struct BeatEngine {
    sample_rate: f32,
    clock: BeatClock,
    kick: Kick,
}

impl BeatEngine {
    fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate: sample_rate as f32,
            clock: BeatClock::new(sample_rate),
            kick: Kick::new(),
        }
    }

    fn next(&mut self, controls: &AudioControls) -> f32 {
        let playing = controls.is_playing();
        if self.clock.next_sample_is_beat(playing, controls.bpm()) {
            self.kick.trigger();
            controls.beats.fetch_add(1, Ordering::Relaxed);
        }
        if !playing {
            self.kick.stop();
            return 0.0;
        }
        self.kick.next(self.sample_rate)
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

/// Open and start the default output device. Keep the returned stream alive
/// for as long as playback is needed. Unsupported devices return a UI-visible error.
pub fn open_stream(controls: Arc<AudioControls>) -> Result<Stream, String> {
    let device = cpal::default_host()
        .default_output_device()
        .ok_or_else(|| "No audio output device is available.".to_string())?;

    let supported = device
        .default_output_config()
        .map_err(|err| format!("Cannot read output configuration: {err}"))?;

    let sample_format = supported.sample_format();
    let config = supported.config();
    let channels = usize::from(config.channels);
    if channels == 0 || config.sample_rate == 0 {
        return Err("Invalid output device configuration.".to_string());
    }

    let rate = config.sample_rate;
    let stream = match sample_format {
        SampleFormat::F32 => {
            let mut engine = BeatEngine::new(rate);
            let ctrl = Arc::clone(&controls);
            device.build_output_stream(
                config,
                move |data: &mut [f32], _| engine.fill(data, channels, &ctrl),
                |err| eprintln!("LIRUVA audio error: {err}"),
                None,
            )
        }
        SampleFormat::I16 => {
            let mut engine = BeatEngine::new(rate);
            let ctrl = Arc::clone(&controls);
            device.build_output_stream(
                config,
                move |data: &mut [i16], _| engine.fill(data, channels, &ctrl),
                |err| eprintln!("LIRUVA audio error: {err}"),
                None,
            )
        }
        SampleFormat::U16 => {
            let mut engine = BeatEngine::new(rate);
            let ctrl = Arc::clone(&controls);
            device.build_output_stream(
                config,
                move |data: &mut [u16], _| engine.fill(data, channels, &ctrl),
                |err| eprintln!("LIRUVA audio error: {err}"),
                None,
            )
        }
        unsupported => {
            return Err(format!(
                "Output sample format {unsupported:?} is not supported yet."
            ));
        }
    }
    .map_err(|err| format!("Cannot create audio stream: {err}"))?;

    stream
        .play()
        .map_err(|err| format!("Cannot start audio stream: {err}"))?;

    Ok(stream)
}

#[cfg(test)]
mod tests {
    use super::{AudioControls, BeatEngine};

    #[test]
    fn playback_generates_audible_samples_and_stops_cleanly() {
        let controls = AudioControls::default();
        let mut engine = BeatEngine::new(48_000);
        assert_eq!(engine.next(&controls), 0.0);

        controls.set_playing(true);
        let samples: Vec<f32> = (0..2_048).map(|_| engine.next(&controls)).collect();
        assert!(samples.iter().all(|sample| sample.is_finite()));
        assert!(samples.iter().any(|sample| sample.abs() > 0.01));
        assert_eq!(controls.beats(), 1);

        controls.set_playing(false);
        assert_eq!(engine.next(&controls), 0.0);
        assert_eq!(engine.next(&controls), 0.0);
    }

    #[test]
    fn stereo_output_duplicates_each_generated_frame() {
        let controls = AudioControls::default();
        controls.set_playing(true);
        let mut engine = BeatEngine::new(44_100);
        let mut output = [0.0f32; 512];
        engine.fill(&mut output, 2, &controls);
        for pair in output.as_chunks::<2>().0 {
            assert_eq!(pair[0], pair[1]);
        }
    }

    #[test]
    fn tempo_control_clamps_to_supported_range() {
        let controls = AudioControls::default();
        controls.set_bpm(0);
        assert_eq!(controls.bpm(), 40);
        controls.set_bpm(10_000);
        assert_eq!(controls.bpm(), 240);
    }
}
