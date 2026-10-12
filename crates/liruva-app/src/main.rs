// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Eduardo J. Barrios and LIRUVA contributors.

mod audio;
mod theme;
mod transport;

use std::sync::Arc;
use std::time::Duration;

use eframe::egui;

use crate::audio::AudioControls;
use crate::transport::{STEPS_PER_BAR, TRACK_COUNT, TRACK_NAMES};

const MISSION: &str = "In the AI music era, let's make music production tools accessible, intuitive, and free for everyone.";

struct LiruvaApp {
    controls: Arc<AudioControls>,
    _stream: Option<cpal::Stream>,
    audio_error: Option<String>,
}

impl LiruvaApp {
    fn new(ctx: &egui::Context) -> Self {
        theme::apply(ctx);
        let controls = Arc::new(AudioControls::default());
        let (stream, audio_error) = match audio::open_stream(Arc::clone(&controls)) {
            Ok(stream) => (Some(stream), None),
            Err(error) => (None, Some(error)),
        };

        Self {
            controls,
            _stream: stream,
            audio_error,
        }
    }

    fn show_transport(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(theme::SURFACE)
            .stroke((1.0, theme::BORDER))
            .corner_radius(16)
            .inner_margin(20)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.colored_label(theme::PRIMARY, "THE RHYTHM LAB");
                ui.add_space(5.0);
                ui.label(
                    egui::RichText::new("A beat in a few clicks.")
                        .size(23.0)
                        .strong(),
                );
                ui.add_space(8.0);
                ui.colored_label(
                    theme::MUTED,
                    "Four original drum sounds, one simple loop. Everything stays on your computer.",
                );
                ui.add_space(20.0);

                let playing = self.controls.is_playing();
                let button_color = if playing {
                    theme::SURFACE_HIGH
                } else {
                    theme::PRIMARY
                };
                let button_text = if playing { "■  Stop" } else { "▶  Play" };
                let text_color = if playing {
                    theme::FOREGROUND
                } else {
                    theme::BACKGROUND
                };

                ui.horizontal(|ui| {
                    let play_button = egui::Button::new(
                        egui::RichText::new(button_text)
                            .size(18.0)
                            .strong()
                            .color(text_color),
                    )
                    .fill(button_color)
                    .corner_radius(10)
                    .min_size(egui::vec2(130.0, 46.0));

                    if ui
                        .add_enabled(self._stream.is_some(), play_button)
                        .on_hover_text("Play or stop the drum pattern")
                        .clicked()
                    {
                        self.controls.set_playing(!playing);
                    }

                    ui.add_space(20.0);
                    ui.vertical(|ui| {
                        ui.colored_label(theme::MUTED, "TEMPO");
                        let mut bpm = self.controls.bpm();
                        if ui
                            .add(egui::Slider::new(&mut bpm, 60..=200).suffix(" BPM"))
                            .changed()
                        {
                            self.controls.set_bpm(bpm);
                        }
                    });
                });

                ui.add_space(13.0);
                ui.separator();
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if self._stream.is_some() {
                        let status = if self.controls.is_playing() {
                            "●  PLAYING"
                        } else {
                            "●  AUDIO READY"
                        };
                        ui.colored_label(theme::SUCCESS, status);
                    } else {
                        ui.colored_label(theme::PRIMARY, "●  AUDIO UNAVAILABLE");
                    }

                    ui.add_space(20.0);
                    let step = self.controls.current_step();
                    ui.colored_label(
                        theme::MUTED,
                        match step {
                            Some(value) => format!("Step {:02} / 16", value + 1),
                            None => "Step — / 16".to_owned(),
                        },
                    );

                    ui.add_space(10.0);
                    ui.colored_label(
                        theme::MUTED,
                        format!("Steps played: {}", self.controls.steps_triggered()),
                    );
                });

                if let Some(error) = &self.audio_error {
                    ui.add_space(9.0);
                    ui.colored_label(theme::MUTED, error);
                    ui.label("Connect an audio output device and restart LIRUVA.");
                }
            });
    }

    fn show_step_grid(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(theme::SURFACE)
            .stroke((1.0, theme::BORDER))
            .corner_radius(16)
            .inner_margin(20)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("STEP SEQUENCER").size(16.0).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.colored_label(theme::MUTED, "1 BAR  ·  4/4");
                    });
                });
                ui.add_space(7.0);
                ui.colored_label(
                    theme::MUTED,
                    "Click a square to turn a drum hit on or off. The loop repeats automatically.",
                );
                ui.add_space(16.0);

                let current_step = self.controls.current_step();
                let available = ui.available_width();
                let width = ((available - 98.0 - 16.0 * 4.0) / 16.0).max(20.0);

                for (track, name) in TRACK_NAMES.iter().enumerate().take(TRACK_COUNT) {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        ui.add_sized(
                            [94.0, 36.0],
                            egui::Label::new(
                                egui::RichText::new(*name).color(theme::FOREGROUND).strong(),
                            ),
                        );

                        for step in 0..STEPS_PER_BAR {
                            let enabled = self.controls.step_enabled(track, step);
                            let is_current = current_step == Some(step);
                            let fill = if enabled {
                                theme::DRUM_COLORS[track]
                            } else {
                                theme::SURFACE_HIGH
                            };
                            let foreground = if enabled {
                                theme::BACKGROUND
                            } else {
                                theme::MUTED
                            };
                            let stroke = if is_current {
                                (2.0, theme::FOREGROUND)
                            } else if step % 4 == 0 {
                                (1.0, theme::BORDER)
                            } else {
                                (0.0, theme::SURFACE)
                            };
                            let button = egui::Button::new(
                                egui::RichText::new(format!("{:02}", step + 1))
                                    .size(11.0)
                                    .strong()
                                    .color(foreground),
                            )
                            .fill(fill)
                            .stroke(stroke)
                            .corner_radius(6);

                            if ui
                                .add_sized([width, 36.0], button)
                                .on_hover_text(format!(
                                    "{name} — step {} — {}",
                                    step + 1,
                                    if enabled { "on" } else { "off" }
                                ))
                                .clicked()
                            {
                                self.controls.toggle_step(track, step);
                            }
                        }
                    });
                    ui.add_space(7.0);
                }

                ui.add_space(11.0);
                ui.horizontal(|ui| {
                    if ui.button("Load starter beat").clicked() {
                        self.controls.load_starter_pattern();
                    }
                    if ui.button("Clear all steps").clicked() {
                        self.controls.clear_pattern();
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.colored_label(theme::MUTED, "Outlined column = current playhead");
                    });
                });
            });
    }
}

impl eframe::App for LiruvaApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(theme::BACKGROUND).inner_margin(22))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("LIRUVA")
                                .size(32.0)
                                .strong()
                                .color(theme::PRIMARY),
                        );
                        ui.colored_label(theme::MUTED, "Make music. Freely.");
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.colored_label(theme::MUTED, "OPEN SOURCE / EARLY ALPHA");
                    });
                });

                ui.add_space(18.0);
                self.show_transport(ui);
                ui.add_space(14.0);
                self.show_step_grid(ui);
                ui.add_space(17.0);
                ui.colored_label(
                    theme::MUTED,
                    "Next milestone: save and reopen your patterns. No account or AI required.",
                );
                ui.add_space(8.0);
                ui.colored_label(theme::MUTED, MISSION);
                ui.hyperlink_to(
                    "View the source code on GitHub",
                    "https://github.com/edujbarrios/liruva",
                );

                if self.controls.is_playing() {
                    ui.ctx().request_repaint_after(Duration::from_millis(40));
                }
            });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("LIRUVA — Make music. Freely.")
            .with_inner_size([1040.0, 760.0])
            .with_min_inner_size([780.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "liruva",
        options,
        Box::new(|cc| Ok(Box::new(LiruvaApp::new(&cc.egui_ctx)))),
    )
}
