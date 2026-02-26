use egui::{Ui, RichText};
use crate::preview::renderer::PreviewRenderer;
use crate::gcode::types::PreviewSegment;
use crate::theme;

pub struct PreviewAction {
    pub zoom_in: bool,
    pub zoom_out: bool,
    pub auto_fit: bool,
}

impl Default for PreviewAction {
    fn default() -> Self {
        Self { zoom_in: false, zoom_out: false, auto_fit: false }
    }
}

pub fn show(
    ui: &mut Ui,
    renderer: &mut PreviewRenderer,
    segments: &[PreviewSegment],
    is_light: bool,
    job_offset: egui::Vec2,
    job_rotation_deg: f32,
    camera_state: &mut crate::ui::camera::CameraState,
) -> PreviewAction {
    let mut action = PreviewAction::default();

    // Zoom toolbar
    ui.horizontal(|ui| {
        ui.label(RichText::new("Preview").color(theme::LAVENDER).strong().size(14.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("⊞ Fit").clicked() {
                action.auto_fit = true;
            }
            if ui.button("🔍−").clicked() {
                action.zoom_out = true;
            }
            if ui.button("🔍+").clicked() {
                action.zoom_in = true;
            }
        });
    });

    // Render preview
    renderer.show(ui, segments, is_light, job_offset, job_rotation_deg, camera_state);

    action
}
