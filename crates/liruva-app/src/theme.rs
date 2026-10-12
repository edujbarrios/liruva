// SPDX-License-Identifier: GPL-3.0-or-later
//! LIRUVA's own dark, Material-inspired design tokens. No copied DAW assets.

use eframe::egui::{self, Color32};

pub const BACKGROUND: Color32 = Color32::from_rgb(17, 20, 26);
pub const SURFACE: Color32 = Color32::from_rgb(29, 33, 42);
pub const SURFACE_HIGH: Color32 = Color32::from_rgb(39, 44, 55);
pub const BORDER: Color32 = Color32::from_rgb(62, 68, 81);
pub const PRIMARY: Color32 = Color32::from_rgb(255, 167, 81);
pub const FOREGROUND: Color32 = Color32::from_rgb(238, 242, 247);
pub const MUTED: Color32 = Color32::from_rgb(166, 175, 188);
pub const SUCCESS: Color32 = Color32::from_rgb(113, 222, 176);

pub fn apply(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = BACKGROUND;
    visuals.window_fill = SURFACE;
    visuals.extreme_bg_color = SURFACE;
    visuals.faint_bg_color = SURFACE_HIGH;
    visuals.override_text_color = Some(FOREGROUND);
    visuals.selection.bg_fill = PRIMARY.gamma_multiply(0.35);
    visuals.selection.stroke.color = PRIMARY;
    visuals.widgets.noninteractive.bg_fill = SURFACE;
    visuals.widgets.inactive.bg_fill = SURFACE_HIGH;
    visuals.widgets.hovered.bg_fill = BORDER;
    visuals.widgets.active.bg_fill = PRIMARY.gamma_multiply(0.70);
    visuals.hyperlink_color = PRIMARY;
    ctx.set_visuals(visuals);
}
