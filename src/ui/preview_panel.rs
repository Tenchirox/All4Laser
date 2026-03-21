use crate::gcode::types::PreviewSegment;
use crate::i18n::tr;
use crate::preview::renderer::{InteractiveAction, PreviewRenderer};
use crate::theme;
use crate::ui::drawing::ShapeParams;
use crate::ui::layers_new::CutLayer;
use egui::{RichText, Ui};

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

    // Zoom toolbar — adaptive sizing
    let avail = ui.available_width();
    let compact = avail < 600.0;

    ui.horizontal_wrapped(|ui| {
        if !compact {
            ui.label(
                RichText::new(tr("Preview"))
                    .color(theme::LAVENDER)
                    .strong()
                    .size(14.0),
            );
        }
        let l_rapids = tr("Rapids");
        let l_fill = tr("Fill");
        let l_risk = tr("Risk");
        let l_realistic = tr("Realistic");
        let l_energy = tr("Energy");
        let r1 = ui.checkbox(&mut renderer.show_rapids, if compact { "R" } else { &l_rapids });
        if compact { r1.on_hover_text(&l_rapids); }
        let r2 = ui.checkbox(&mut renderer.show_fill_preview, if compact { "F" } else { &l_fill });
        if compact { r2.on_hover_text(&l_fill); }
        let r3 = ui.checkbox(&mut renderer.show_thermal_risk, if compact { "!" } else { &l_risk });
        if compact { r3.on_hover_text(&l_risk); }
        let r4 = ui.checkbox(&mut renderer.realistic_preview, if compact { "3D" } else { &l_realistic });
        if compact { r4.on_hover_text(&l_realistic); }
        let r5 = ui.checkbox(&mut renderer.power_speed_opacity, if compact { "E" } else { &l_energy });
        if compact { r5.on_hover_text(&l_energy); }

        if !segments.is_empty() {
            ui.separator();
            let mut sim_on = renderer.simulation_progress.is_some();
            let l_sim = tr("Simulation");
            let sim_resp = ui.checkbox(&mut sim_on, if compact { "Sim" } else { &l_sim });
            if compact { sim_resp.clone().on_hover_text(&l_sim); }
            if sim_resp.changed() {
                renderer.simulation_progress = if sim_on { Some(0.0) } else { None };
            }
            if let Some(progress) = renderer.simulation_progress.as_mut() {
                let slider_w = if compact { 60.0 } else { 120.0 };
                ui.add_sized(
                    egui::vec2(slider_w, 0.0),
                    egui::Slider::new(progress, 0.0..=1.0)
                        .show_value(false)
                        .text(""),
                );
                if ui
                    .button("↺")
                    .on_hover_text(tr("Reset simulation"))
                    .clicked()
                {
                    *progress = 0.0;
                }
            }
        }

        if renderer.show_thermal_risk {
            ui.separator();
            let slider_w = if compact { 50.0 } else { 90.0 };
            ui.add_sized(
                egui::vec2(slider_w, 0.0),
                egui::Slider::new(&mut renderer.risk_threshold, 1.0..=80.0)
                    .show_value(false)
                    .text(""),
            );
            ui.add(
                egui::DragValue::new(&mut renderer.risk_cell_mm)
                    .speed(0.1)
                    .range(0.5..=20.0)
                    .suffix("mm"),
            );
            ui.label(
                RichText::new(format!("⚠{}", renderer.last_risk_alert_cells))
                    .small()
                    .color(if renderer.last_risk_alert_cells > 0 {
                        theme::PEACH
                    } else {
                        theme::SUBTEXT
                    }),
            );
        }

        ui.separator();
        if ui.button("🔍+").on_hover_text(tr("Zoom in")).clicked() {
            action.zoom_in = true;
        }
        if ui.button("🔍−").on_hover_text(tr("Zoom out")).clicked() {
            action.zoom_out = true;
        }
        if ui
            .button("⊞")
            .on_hover_text(tr("Fit"))
            .clicked()
        {
            action.auto_fit = true;
        }
    });

    // Render preview
    action.interactive_action = renderer.show(
        ui,
        segments,
        shapes,
        layers,
        is_light,
        job_offset,
        job_rotation_deg,
        camera_state,
    );

    action
}
