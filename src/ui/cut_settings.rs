use egui::{Ui, RichText};
use crate::ui::layers_new::{CutLayer, CutMode};
use crate::theme;

#[derive(Default)]
pub struct CutSettingsState {
    pub is_open: bool,
    pub editing_layer_idx: Option<usize>,
    pub temp_layer: Option<CutLayer>,
}

pub struct CutSettingsAction {
    pub apply: Option<(usize, CutLayer)>,
    pub close: bool,
}

pub fn show(ctx: &egui::Context, state: &mut CutSettingsState, layers: &[CutLayer]) -> CutSettingsAction {
    let mut action = CutSettingsAction { apply: None, close: false };

    if !state.is_open {
        return action;
    }

    // Ensure we have a temporary copy to edit
    if state.temp_layer.is_none() {
        if let Some(idx) = state.editing_layer_idx {
            if let Some(layer) = layers.get(idx) {
                state.temp_layer = Some(layer.clone());
            }
        }
    }

    let mut open = true;
    egui::Window::new("⚙ Cut Settings")
        .open(&mut open)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            if let Some(layer) = &mut state.temp_layer {
                ui.horizontal(|ui| {
                    // Color swatch
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 2.0, layer.color);
                    ui.label(RichText::new(format!("Layer {}", layer.name)).strong().size(18.0));
                });
                ui.separator();

                egui::Grid::new("cut_settings_grid").num_columns(2).spacing([12.0, 8.0]).show(ui, |ui| {
                    ui.label("Speed (mm/min):");
                    ui.add(egui::DragValue::new(&mut layer.speed).speed(10.0).range(1.0..=20000.0));
                    ui.end_row();

                    ui.label("Max Power (S):");
                    ui.add(egui::DragValue::new(&mut layer.power).speed(1.0).range(0.0..=1000.0));
                    ui.end_row();

                    ui.label("Output Mode:");
                    egui::ComboBox::from_id_source("mode_combo")
                        .selected_text(match layer.mode {
                            CutMode::Line => "Line (Cut)",
                            CutMode::Fill => "Fill (Scan)",
                            CutMode::FillAndLine => "Fill + Line",
                            CutMode::Offset => "Offset Fill",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut layer.mode, CutMode::Line, "Line (Cut)");
                            ui.selectable_value(&mut layer.mode, CutMode::Fill, "Fill (Scan)");
                            ui.selectable_value(&mut layer.mode, CutMode::FillAndLine, "Fill + Line");
                            ui.selectable_value(&mut layer.mode, CutMode::Offset, "Offset Fill");
                        });
                    ui.end_row();

                    ui.label("Passes:");
                    ui.add(egui::DragValue::new(&mut layer.passes).range(1..=100));
                    ui.end_row();

                    ui.label("Z Offset (mm):");
                    ui.add(egui::DragValue::new(&mut layer.z_offset).speed(0.1));
                    ui.end_row();

                    ui.label(RichText::new("🏗 Tabs / Bridges:").strong());
                    ui.checkbox(&mut layer.tab_enabled, "Enabled");
                    ui.end_row();

                    if layer.tab_enabled {
                        ui.label("Tab Spacing:");
                        ui.add(egui::DragValue::new(&mut layer.tab_spacing).speed(1.0).range(1.0..=500.0).suffix(" mm"));
                        ui.end_row();

                        ui.label("Tab Size (Gap):");
                        ui.add(egui::DragValue::new(&mut layer.tab_size).speed(0.1).range(0.1..=10.0).suffix(" mm"));
                        ui.end_row();
                    }
                });

                ui.add_space(8.0);
                ui.checkbox(&mut layer.air_assist, "Air Assist (M8)");
                ui.checkbox(&mut layer.visible, "Output Enabled");

                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    if ui.button(RichText::new("OK").color(theme::GREEN)).clicked() {
                        if let Some(idx) = state.editing_layer_idx {
                            action.apply = Some((idx, layer.clone()));
                        }
                        action.close = true;
                    }
                    if ui.button("Cancel").clicked() {
                        action.close = true;
                    }
                });
            } else {
                ui.label("No layer selected.");
                if ui.button("Close").clicked() {
                    action.close = true;
                }
            }
        });

    if !open || action.close {
        state.is_open = false;
        state.temp_layer = None;
        state.editing_layer_idx = None;
    }

    action
}
