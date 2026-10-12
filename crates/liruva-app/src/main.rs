// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Eduardo J. Barrios and LIRUVA contributors.

use eframe::egui;

const MISSION: &str = "In the AI music era, let's make music production tools accessible, intuitive, and free for everyone.";

struct LiruvaApp;

impl eframe::App for LiruvaApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.add_space(48.0);
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("LIRUVA").size(52.0).strong());
                ui.add_space(8.0);
                ui.label(egui::RichText::new("Make music. Freely.").size(23.0));
                ui.add_space(16.0);
                ui.label(MISSION);
                ui.add_space(28.0);
                ui.label(
                    egui::RichText::new("MILESTONE 0  •  FOUNDATION")
                        .small()
                        .weak(),
                );
                ui.add_space(8.0);
                ui.label("This is an early development preview, not a playable DAW.");
                ui.add_space(28.0);
                ui.separator();
                ui.add_space(18.0);
                ui.label("Next: a simple step sequencer with real drum sounds.");
                ui.add_space(18.0);
                ui.hyperlink_to(
                    "Explore the public source code",
                    "https://github.com/edujbarrios/liruva",
                );
                ui.add_space(12.0);
                ui.label(
                    egui::RichText::new("Free forever  •  Open source  •  Offline-first")
                        .small()
                        .weak(),
                );
            });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("LIRUVA — Make music. Freely.")
            .with_inner_size([900.0, 570.0])
            .with_min_inner_size([600.0, 420.0]),
        ..Default::default()
    };

    eframe::run_native(
        "liruva",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(LiruvaApp))
        }),
    )
}
