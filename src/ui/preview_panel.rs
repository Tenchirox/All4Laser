use egui::{Ui, RichText};
use crate::preview::renderer::{PreviewRenderer, InteractiveAction};
use crate::gcode::types::PreviewSegment;
use crate::ui::drawing::ShapeParams;
use crate::theme;

pub struct PreviewAction {
    pub zoom_in: bool,
    pub zoom_out: bool,
    pub auto_fit: bool,
    pub interactive_action: InteractiveAction,
}

impl Default for PreviewAction {
    fn default() -> Self {
        Self {
            zoom_in: false,
            zoom_out: false,
            auto_fit: false,
            interactive_action: InteractiveAction::None,
        }
    }
}

pub fn show(
    ui: &mut Ui,
    renderer: &mut PreviewRenderer,
    segments: &[PreviewSegment],
    shapes: &[ShapeParams],
    is_light: bool,
    job_offset: egui::Vec2,
    job_rotation_deg: f32,
    camera_state: &mut crate::ui::camera::CameraState,
) -> PreviewAction {
    let mut action = PreviewAction::default();

    // Zoom toolbar
    ui.horizontal(|ui| {
        ui.label(RichText::new("Preview").color(theme::LAVENDER).strong().size(14.0));
        ui.checkbox(&mut renderer.show_rapids, "Show Rapids").on_hover_text("Show G0 rapid moves");
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
    action.interactive_action = renderer.show(ui, segments, shapes, is_light, job_offset, job_rotation_deg, camera_state);

    action
}
