use egui::{Ui, RichText};
use crate::preview::renderer::{PreviewRenderer, InteractiveAction};
use crate::gcode::types::PreviewSegment;
use crate::ui::drawing::ShapeParams;
use crate::ui::layers_new::CutLayer;
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
    layers: &[CutLayer],
    is_light: bool,
    job_offset: egui::Vec2,
    job_rotation_deg: f32,
    camera_state: &mut crate::ui::camera::CameraState,
) -> PreviewAction {
    let mut action = PreviewAction::default();

    // Zoom toolbar
    ui.horizontal(|ui| {
        ui.label(RichText::new("Preview").color(theme::LAVENDER).strong().size(14.0));
        ui.checkbox(&mut renderer.show_rapids, "Rapids");
        ui.checkbox(&mut renderer.show_fill_preview, "Fill");
        ui.checkbox(&mut renderer.show_thermal_risk, "Risk");
        ui.checkbox(&mut renderer.realistic_preview, "Realistic");

        if !segments.is_empty() {
            ui.separator();
            let mut sim_on = renderer.simulation_progress.is_some();
            if ui.checkbox(&mut sim_on, "Simulation").changed() {
                renderer.simulation_progress = if sim_on { Some(0.0) } else { None };
            }
            if let Some(progress) = renderer.simulation_progress.as_mut() {
                ui.add_sized(
                    egui::vec2(120.0, 0.0),
                    egui::Slider::new(progress, 0.0..=1.0)
                        .show_value(false)
                        .text("Progress"),
                );
                if ui
                    .button("Reset")
                    .on_hover_text("Reset simulation progress to start")
                    .clicked()
                {
                    *progress = 0.0;
                }
            }
        }

        if renderer.show_thermal_risk {
            ui.separator();
            ui.label(RichText::new("Risk Thr").small().color(theme::SUBTEXT));
            ui.add_sized(
                egui::vec2(90.0, 0.0),
                egui::Slider::new(&mut renderer.risk_threshold, 1.0..=80.0)
                    .show_value(false)
                    .text("Risk Thr"),
            );
            ui.label(RichText::new("Cell").small().color(theme::SUBTEXT));
            ui.add(
                egui::DragValue::new(&mut renderer.risk_cell_mm)
                    .speed(0.1)
                    .range(0.5..=20.0)
                    .suffix(" mm"),
            );
            ui.label(
                RichText::new(format!("⚠ {}", renderer.last_risk_alert_cells))
                    .small()
                    .color(if renderer.last_risk_alert_cells > 0 {
                        theme::PEACH
                    } else {
                        theme::SUBTEXT
                    }),
            );
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .button("⊞ Fit")
                .on_hover_text("Fit the full job in the preview")
                .clicked()
            {
                action.auto_fit = true;
            }
            if ui
                .button("🔍−")
                .on_hover_text("Zoom out")
                .clicked()
            {
                action.zoom_out = true;
            }
            if ui
                .button("🔍+")
                .on_hover_text("Zoom in")
                .clicked()
            {
                action.zoom_in = true;
            }
        });
    });

    // Render preview
    action.interactive_action = renderer.show(ui, segments, shapes, layers, is_light, job_offset, job_rotation_deg, camera_state);

    action
}
