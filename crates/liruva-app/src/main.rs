// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Eduardo J. Barrios and LIRUVA contributors.

mod audio;
mod melody;
mod project;
mod theme;
mod transport;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use eframe::egui;

use crate::audio::AudioControls;
use crate::melody::{MELODY_PITCH_COUNT, pitch_name};
use crate::project::{Project, load_from_path, normalize_save_path, save_to_path};
use crate::transport::{STEPS_PER_BAR, TRACK_COUNT, TRACK_NAMES};

const MISSION: &str = "In the AI music era, let's make music production tools accessible, intuitive, and free for everyone.";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PendingAction {
    Open,
    New,
    Quit,
}

struct LiruvaApp {
    controls: Arc<AudioControls>,
    _stream: Option<cpal::Stream>,
    audio_error: Option<String>,
    project_name: String,
    project_path: Option<PathBuf>,
    dirty: bool,
    pending_action: Option<PendingAction>,
    allow_close: bool,
    project_notice: Option<(bool, String)>,
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
            project_name: "Untitled Beat".to_owned(),
            project_path: None,
            dirty: false,
            pending_action: None,
            allow_close: false,
            project_notice: None,
        }
    }

    fn file_dialog(&self, title: &str) -> rfd::FileDialog {
        let dialog = rfd::FileDialog::new()
            .add_filter("LIRUVA project", &[project::FILE_EXTENSION])
            .set_title(title);
        if let Some(parent) = self.project_path.as_ref().and_then(|path| path.parent()) {
            dialog.set_directory(parent)
        } else {
            dialog
        }
    }

    fn save_project(&mut self, save_as: bool) -> bool {
        let path = if !save_as {
            self.project_path.clone().or_else(|| {
                self.file_dialog("Save LIRUVA project")
                    .set_file_name("my-beat.liruva")
                    .save_file()
            })
        } else {
            let suggested = self
                .project_path
                .as_ref()
                .and_then(|path| path.file_name())
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| "my-beat.liruva".to_owned());
            self.file_dialog("Save LIRUVA project as")
                .set_file_name(suggested)
                .save_file()
        };

        let Some(path) = path else {
            return false;
        };

        let path = normalize_save_path(&path);
        let file = Project::from_controls(&self.project_name, &self.controls);
        match save_to_path(&path, &file) {
            Ok(()) => {
                self.project_path = Some(path);
                self.dirty = false;
                self.project_notice = Some((true, "Project saved successfully.".to_owned()));
                true
            }
            Err(error) => {
                self.project_notice = Some((false, error));
                false
            }
        }
    }

    fn open_project(&mut self) {
        let Some(path) = self.file_dialog("Open LIRUVA project").pick_file() else {
            return;
        };

        match load_from_path(&path) {
            Ok(project) => {
                project.apply(&self.controls);
                self.project_name = project.name;
                self.project_path = Some(path);
                self.dirty = false;
                self.project_notice = Some((true, "Project opened successfully.".to_owned()));
            }
            Err(error) => {
                self.project_notice = Some((false, error));
            }
        }
    }

    fn new_project(&mut self) {
        self.controls.set_playing(false);
        self.controls.set_bpm(120);
        self.controls.load_starter_pattern();
        self.controls.clear_melody();
        self.project_name = "Untitled Beat".to_owned();
        self.project_path = None;
        self.dirty = false;
        self.project_notice = Some((true, "New project created.".to_owned()));
    }

    fn request_action(&mut self, action: PendingAction) {
        if self.dirty {
            self.pending_action = Some(action);
        } else {
            self.perform_action(action);
        }
    }

    fn perform_action(&mut self, action: PendingAction) {
        self.pending_action = None;
        match action {
            PendingAction::Open => self.open_project(),
            PendingAction::New => self.new_project(),
            PendingAction::Quit => {
                self.allow_close = true;
            }
        }
    }

    fn show_project_controls(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(theme::SURFACE_HIGH)
            .corner_radius(12)
            .inner_margin(14)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.colored_label(theme::MUTED, "PROJECT");
                    if ui
                        .add(
                            egui::TextEdit::singleline(&mut self.project_name)
                                .desired_width(180.0)
                                .hint_text("Project name"),
                        )
                        .changed()
                    {
                        self.dirty = true;
                    }

                    if ui.button("New").clicked() {
                        self.request_action(PendingAction::New);
                    }
                    if ui.button("Save").clicked() {
                        let _ = self.save_project(false);
                    }
                    if ui.button("Save As…").clicked() {
                        let _ = self.save_project(true);
                    }
                    if ui.button("Open…").clicked() {
                        self.request_action(PendingAction::Open);
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let label = if self.dirty {
                            "● Unsaved changes"
                        } else {
                            "✓ Up to date"
                        };
                        ui.colored_label(theme::MUTED, label);
                    });
                });

                if let Some((success, message)) = &self.project_notice {
                    ui.add_space(5.0);
                    ui.colored_label(
                        if *success {
                            theme::SUCCESS
                        } else {
                            theme::PRIMARY
                        },
                        message,
                    );
                }
            });
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
                    "Four drums and one melodic instrument. Everything stays on your computer.",
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
                            self.dirty = true;
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
                                self.dirty = true;
                            }
                        }
                    });
                    ui.add_space(7.0);
                }

                ui.add_space(11.0);
                ui.horizontal(|ui| {
                    if ui.button("Load starter beat").clicked() {
                        self.controls.load_starter_pattern();
                        self.dirty = true;
                    }
                    if ui.button("Clear all steps").clicked() {
                        self.controls.clear_pattern();
                        self.dirty = true;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.colored_label(theme::MUTED, "Outlined column = current playhead");
                    });
                });
            });
    }

    fn show_piano_roll(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(theme::SURFACE)
            .stroke((1.0, theme::BORDER))
            .corner_radius(16)
            .inner_margin(20)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("PIANO ROLL").size(16.0).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.colored_label(theme::MUTED, "C4–B5 · ONE NOTE PER STEP");
                    });
                });
                ui.add_space(7.0);
                ui.colored_label(
                    theme::MUTED,
                    "Click a pitch and step to compose. Click it again to remove the note.",
                );
                ui.add_space(12.0);
                let current_step = self.controls.current_step();
                let width = ((ui.available_width() - 94.0 - 16.0 * 4.0) / 16.0).max(20.0);

                egui::ScrollArea::vertical().max_height(242.0).show(ui, |ui| {
                    for pitch in (0..MELODY_PITCH_COUNT).rev() {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 4.0;
                            ui.add_sized(
                                [90.0, 25.0],
                                egui::Label::new(
                                    egui::RichText::new(pitch_name(pitch))
                                        .color(theme::FOREGROUND),
                                ),
                            );
                            for step in 0..STEPS_PER_BAR {
                                let active = self.controls.melody_note(step) == Some(pitch as u8);
                                let background = if active {
                                    theme::SUCCESS
                                } else {
                                    theme::SURFACE_HIGH
                                };
                                let outline = if current_step == Some(step) {
                                    (2.0, theme::FOREGROUND)
                                } else if step % 4 == 0 {
                                    (1.0, theme::BORDER)
                                } else {
                                    (0.0, theme::SURFACE)
                                };
                                let label = if active { "●" } else { "" };
                                let button = egui::Button::new(
                                    egui::RichText::new(label).color(theme::BACKGROUND),
                                )
                                .fill(background)
                                .stroke(outline)
                                .corner_radius(4);

                                if ui
                                    .add_sized([width, 25.0], button)
                                    .on_hover_text(format!(
                                        "{} — step {} — {}",
                                        pitch_name(pitch),
                                        step + 1,
                                        if active { "remove note" } else { "set note" }
                                    ))
                                    .clicked()
                                {
                                    self.controls.set_melody_note(
                                        step,
                                        if active { None } else { Some(pitch as u8) },
                                    );
                                    self.dirty = true;
                                }
                            }
                        });
                    }
                });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("Load demo melody").clicked() {
                        self.controls.load_demo_melody();
                        self.dirty = true;
                    }
                    if ui.button("Clear melody").clicked() {
                        self.controls.clear_melody();
                        self.dirty = true;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.colored_label(theme::MUTED, "Scroll vertically to explore two octaves");
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
                egui::ScrollArea::vertical().show(ui, |ui| {
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

                    ui.add_space(12.0);
                    self.show_project_controls(ui);
                    ui.add_space(15.0);
                    self.show_transport(ui);
                    ui.add_space(14.0);
                    self.show_step_grid(ui);
                    ui.add_space(14.0);
                    self.show_piano_roll(ui);
                    ui.add_space(17.0);
                    ui.colored_label(
                        theme::MUTED,
                        "Save drums and melody together as open .liruva projects. No account or AI required.",
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
            });

        if ui.input(|input| input.viewport().close_requested()) && !self.allow_close && self.dirty {
            self.pending_action = Some(PendingAction::Quit);
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::CancelClose);
        }

        if let Some(action) = self.pending_action {
            egui::Window::new("Unsaved changes")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ui.ctx(), |ui| {
                    ui.label("This project contains changes that have not been saved.");
                    ui.label("Save your beat before continuing, or discard your edits.");
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.pending_action = None;
                        }
                        if ui.button("Discard changes").clicked() {
                            self.perform_action(action);
                            if action == PendingAction::Quit {
                                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                        if ui.button("Save and continue").clicked() && self.save_project(false) {
                            self.perform_action(action);
                            if action == PendingAction::Quit {
                                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                    });
                });
        }

        if self._stream.is_some()
            && !ui.ctx().egui_wants_keyboard_input()
            && ui.input(|input| input.key_pressed(egui::Key::Space))
        {
            self.controls.set_playing(!self.controls.is_playing());
        }
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("LIRUVA — Make music. Freely.")
            .with_inner_size([1060.0, 1060.0])
            .with_min_inner_size([780.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "liruva",
        options,
        Box::new(|cc| Ok(Box::new(LiruvaApp::new(&cc.egui_ctx)))),
    )
}
