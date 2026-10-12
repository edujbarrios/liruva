// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Eduardo J. Barrios and LIRUVA contributors.

mod audio;
mod theme;
mod transport;

use std::sync::Arc;
use std::time::Duration;

use eframe::egui;

use crate::audio::AudioControls;

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
            .corner_radius(18)
            .inner_margin(24)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.colored_label(theme::PRIMARY, "01 / FIRST SOUND");
                ui.add_space(12.0);
                ui.label(
                    egui::RichText::new("Your first beat starts here.")
                        .size(26.0)
                        .strong(),
                );
                ui.add_space(7.0);
                ui.colored_label(
                    theme::MUTED,
                    "One locally synthesized kick. No samples, account, or AI required.",
                );
                ui.add_space(26.0);

                let playing = self.controls.is_playing();
                let button_text = if playing { "■  Stop" } else { "▶  Play" };
                let button_color = if playing {
                    theme::SURFACE_HIGH
                } else {
                    theme::PRIMARY
                };
                let text_color = if playing {
                    theme::FOREGROUND
                } else {
                    theme::BACKGROUND
                };

                ui.horizontal(|ui| {
                    let button = egui::Button::new(
                        egui::RichText::new(button_text)
                            .size(18.0)
                            .strong()
                            .color(text_color),
                    )
                    .fill(button_color)
                    .corner_radius(12)
                    .min_size(egui::vec2(150.0, 48.0));

                    if ui
                        .add_enabled(self._stream.is_some(), button)
                        .on_hover_text("Space: Play / Stop")
                        .clicked()
                    {
                        self.controls.set_playing(!playing);
                    }

                    ui.add_space(24.0);
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

                ui.add_space(21.0);
                ui.separator();
                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    if self._stream.is_some() {
                        let playing = self.controls.is_playing();
                        ui.colored_label(
                            theme::SUCCESS,
                            if playing {
                                "●  PLAYING"
                            } else {
                                "●  AUDIO READY"
                            },
                        );
                        ui.add_space(20.0);
                        ui.colored_label(
                            theme::MUTED,
                            format!("Beats triggered: {}", self.controls.beats()),
                        );
                    } else {
                        ui.colored_label(theme::PRIMARY, "●  AUDIO UNAVAILABLE");
                    }
                });

                if let Some(error) = &self.audio_error {
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new(error).color(theme::MUTED).small());
                    ui.label("Connect an audio output device and restart LIRUVA.");
                }
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
                        ui.colored_label(theme::MUTED, "OPEN SOURCE  /  EARLY ALPHA");
                    });
                });

                ui.add_space(24.0);
                self.show_transport(ui);
                ui.add_space(22.0);

                egui::Frame::new()
                    .fill(theme::SURFACE_HIGH)
                    .corner_radius(14)
                    .inner_margin(18)
                    .show(ui, |ui| {
                        ui.colored_label(theme::MUTED, "COMING NEXT");
                        ui.add_space(6.0);
                        ui.label("A simple 16-step rhythm grid with four drum sounds.");
                        ui.add_space(4.0);
                        ui.colored_label(
                            theme::MUTED,
                            "We build one small, working feature at a time.",
                        );
                    });

                ui.add_space(25.0);
                ui.colored_label(theme::MUTED, MISSION);
                ui.add_space(6.0);
                ui.hyperlink_to(
                    "Explore the source on GitHub",
                    "https://github.com/edujbarrios/liruva",
                );

                if self._stream.is_some() && ui.input(|input| input.key_pressed(egui::Key::Space)) {
                    self.controls.set_playing(!self.controls.is_playing());
                }

                if self.controls.is_playing() {
                    ui.ctx().request_repaint_after(Duration::from_millis(120));
                }
            });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("LIRUVA — Make music. Freely.")
            .with_inner_size([950.0, 650.0])
            .with_min_inner_size([740.0, 550.0]),
        ..Default::default()
    };

    eframe::run_native(
        "liruva",
        options,
        Box::new(|cc| Ok(Box::new(LiruvaApp::new(&cc.egui_ctx)))),
    )
}
