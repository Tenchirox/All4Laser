#![allow(dead_code)]

use crate::theme;
use crate::ui::drawing::{ShapeKind, ShapeParams};
use crate::ui::layers_new::{CutLayer, CutMode};
use egui::RichText;

pub struct CutSettingsState {
    pub is_open: bool,
    pub editing_layer_idx: Option<usize>,
    pub temp_layer: Option<CutLayer>,
    pub kerf_test_nominal_mm: f32,
    pub kerf_test_measured_mm: f32,
    // Parameter comparison snapshot (F91)
    pub snapshot_layer: Option<CutLayer>,
    pub show_comparison: bool,
}

impl Default for CutSettingsState {
    fn default() -> Self {
        Self {
            is_open: false,
            editing_layer_idx: None,
            temp_layer: None,
            kerf_test_nominal_mm: 20.0,
            kerf_test_measured_mm: 19.8,
            snapshot_layer: None,
            show_comparison: false,
        }
    }
}

pub struct CutSettingsAction {
    pub apply: Option<(usize, CutLayer)>,
    pub close: bool,
}

fn path_is_closed_for_fill(points: &[(f32, f32)]) -> bool {
    if points.len() < 3 {
        return false;
    }

    let (Some(first), Some(last)) = (points.first(), points.last()) else {
        return false;
    };

    let dx = first.0 - last.0;
    let dy = first.1 - last.1;
    let dist = (dx * dx + dy * dy).sqrt();
    dist <= 0.05
}

fn layer_non_fillable_path_count(shapes: &[ShapeParams], layer_idx: usize) -> usize {
    shapes
        .iter()
        .filter(|shape| shape.layer_idx == layer_idx)
        .filter(|shape| {
            matches!(
                &shape.shape,
                ShapeKind::Path(points) if points.len() < 3 || !path_is_closed_for_fill(points)
            )
        })
        .count()
}

fn kerf_from_test_measurement(nominal_mm: f32, measured_mm: f32) -> f32 {
    if nominal_mm <= 0.0 || measured_mm <= 0.0 {
        return 0.0;
    }
    ((nominal_mm - measured_mm) * 0.5).clamp(-5.0, 5.0)
}

pub fn show(
    ctx: &egui::Context,
    state: &mut CutSettingsState,
    layers: &[CutLayer],
    shapes: &[ShapeParams],
    speed_unit: crate::config::settings::SpeedUnit,
) -> CutSettingsAction {
    let mut action = CutSettingsAction {
        apply: None,
        close: false,
    };

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
    let mut kerf_nominal = state.kerf_test_nominal_mm;
    let mut kerf_measured = state.kerf_test_measured_mm;

    egui::Window::new("⚙ Cut Settings")
        .open(&mut open)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            if let Some(layer) = &mut state.temp_layer {
                ui.horizontal(|ui| {
                    // Color picker (F55)
                    let rgba = layer.color.to_array();
                    let mut color_f = [rgba[0] as f32 / 255.0, rgba[1] as f32 / 255.0, rgba[2] as f32 / 255.0];
                    if ui.color_edit_button_rgb(&mut color_f).changed() {
                        layer.color = egui::Color32::from_rgb(
                            (color_f[0] * 255.0) as u8,
                            (color_f[1] * 255.0) as u8,
                            (color_f[2] * 255.0) as u8,
                        );
                    }
                    let _ = rgba;
                    ui.label(RichText::new(format!("Layer {}", layer.name)).strong().size(18.0));
                });
                ui.separator();

                egui::Grid::new("cut_settings_grid").num_columns(2).spacing([12.0, 8.0]).show(ui, |ui| {
                    ui.label(format!("Speed ({}):", speed_unit.label())).on_hover_text("Travel speed of the laser head. Lower = deeper cut/darker engrave. Typical: 200-3000 for cut, 1000-10000 for engrave.");
                    let mut display_speed = speed_unit.from_mmpm(layer.speed);
                    let (spd_drag, spd_max) = match speed_unit {
                        crate::config::settings::SpeedUnit::MmPerMin => (10.0, 20000.0),
                        crate::config::settings::SpeedUnit::MmPerSec => (1.0, 334.0),
                    };
                    if ui.add(egui::DragValue::new(&mut display_speed).speed(spd_drag).range(0.1..=spd_max)).changed() {
                        layer.speed = speed_unit.to_mmpm(display_speed);
                    }
                    ui.end_row();

                    ui.label("Max Power (S):").on_hover_text("Laser power (0-1000 S-value). Higher = more energy. Start low and increase. S1000 = 100% power.");
                    ui.add(egui::DragValue::new(&mut layer.power).speed(1.0).range(0.0..=1000.0));
                    ui.end_row();

                    ui.label("Output Mode:").on_hover_text("Line = vector cut along paths. Fill = raster scan inside closed shapes. Fill+Line = both. Offset = concentric fill.");
                    egui::ComboBox::from_id_salt("mode_combo")
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

                    if matches!(layer.mode, CutMode::Fill | CutMode::FillAndLine | CutMode::Offset) {
                        ui.label("Fill Interval (mm):").on_hover_text("Distance between scan lines. Smaller = denser fill, slower job. 0.1mm typical for engraving.");
                        ui.add(
                            egui::DragValue::new(&mut layer.fill_interval_mm)
                                .speed(0.01)
                                .range(0.01..=2.0)
                                .suffix(" mm"),
                        );
                        ui.end_row();

                        ui.label("Min Power (S):").on_hover_text("Power used during acceleration/deceleration at line ends. Set to 0 for clean edges.");
                        ui.add(egui::DragValue::new(&mut layer.min_power).speed(1.0).range(0.0..=1000.0));
                        ui.end_row();

                        ui.label("Bidirectional Scan:").on_hover_text("Scan in both directions (faster) vs one direction only (more consistent).");
                        ui.checkbox(&mut layer.fill_bidirectional, "");
                        ui.end_row();

                        ui.label("Overscan (mm):").on_hover_text("Extra travel beyond the shape edges to allow deceleration. Prevents edge burn. 1-5mm typical.");
                        ui.add(
                            egui::DragValue::new(&mut layer.fill_overscan_mm)
                                .speed(0.1)
                                .range(0.0..=20.0)
                                .suffix(" mm"),
                        );
                        ui.end_row();

                        ui.label("Fill Angle (°):");
                        ui.add(
                            egui::DragValue::new(&mut layer.fill_angle_deg)
                                .speed(1.0)
                                .range(-180.0..=180.0)
                                .suffix("°"),
                        );
                        ui.end_row();
                    }

                    ui.label("Output Order:");
                    ui.add(egui::DragValue::new(&mut layer.output_order).speed(1.0));
                    ui.end_row();

                    ui.label("Lead-In (mm):").on_hover_text("Extra approach distance before cutting starts. Prevents burn marks at the cut entry point.");
                    ui.add(
                        egui::DragValue::new(&mut layer.lead_in_mm)
                            .speed(0.1)
                            .range(0.0..=50.0)
                            .suffix(" mm"),
                    );
                    ui.end_row();

                    ui.label("Lead-Out (mm):").on_hover_text("Extra exit distance after cutting ends. Ensures clean exit from the material.");
                    ui.add(
                        egui::DragValue::new(&mut layer.lead_out_mm)
                            .speed(0.1)
                            .range(0.0..=50.0)
                            .suffix(" mm"),
                    );
                    ui.end_row();

                    ui.label("Kerf Offset (mm):").on_hover_text("Compensates for material removed by the laser beam width. Positive = outward offset.");
                    ui.add(
                        egui::DragValue::new(&mut layer.kerf_mm)
                            .speed(0.01)
                            .range(-5.0..=5.0)
                            .suffix(" mm"),
                    );
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

                // Multi-pass offset (F24)
                if layer.passes > 1 {
                    ui.add_space(4.0);
                    egui::Grid::new("pass_offset_grid").num_columns(2).spacing([12.0, 4.0]).show(ui, |ui| {
                        ui.label("Pass offset (mm):");
                        ui.add(egui::DragValue::new(&mut layer.pass_offset_mm).speed(0.01).range(0.0..=2.0).suffix(" mm"));
                        ui.end_row();
                    });
                }

                // Power Ramping (F12)
                ui.add_space(4.0);
                ui.checkbox(&mut layer.ramp_enabled, "⚡ Power Ramping");
                if layer.ramp_enabled {
                    egui::Grid::new("ramp_grid").num_columns(2).spacing([12.0, 4.0]).show(ui, |ui| {
                        ui.label("Ramp length:");
                        ui.add(egui::DragValue::new(&mut layer.ramp_length_mm).speed(0.5).range(0.5..=50.0).suffix(" mm"));
                        ui.end_row();
                        ui.label("Start/end power %:");
                        ui.add(egui::DragValue::new(&mut layer.ramp_start_pct).speed(1.0).range(0.0..=99.0).suffix(" %"));
                        ui.end_row();
                    });
                }

                // Perforation (F33)
                ui.add_space(4.0);
                ui.checkbox(&mut layer.perforation_enabled, "✂ Perforation / Dashed Mode");
                if layer.perforation_enabled {
                    egui::Grid::new("perf_grid").num_columns(2).spacing([12.0, 4.0]).show(ui, |ui| {
                        ui.label("Cut length:");
                        ui.add(egui::DragValue::new(&mut layer.perforation_cut_mm).speed(0.5).range(0.1..=100.0).suffix(" mm"));
                        ui.end_row();
                        ui.label("Gap length:");
                        ui.add(egui::DragValue::new(&mut layer.perforation_gap_mm).speed(0.5).range(0.1..=100.0).suffix(" mm"));
                        ui.end_row();
                    });
                }

                // Corner power (F40)
                ui.add_space(4.0);
                ui.checkbox(&mut layer.corner_power_enabled, "🔥 Corner Power Reduction");
                if layer.corner_power_enabled {
                    egui::Grid::new("corner_grid").num_columns(2).spacing([12.0, 4.0]).show(ui, |ui| {
                        ui.label("Corner power %:");
                        ui.add(egui::DragValue::new(&mut layer.corner_power_pct).speed(1.0).range(1.0..=100.0).suffix(" %"));
                        ui.end_row();
                        ui.label("Angle threshold:");
                        ui.add(egui::DragValue::new(&mut layer.corner_angle_threshold).speed(1.0).range(5.0..=175.0).suffix("°"));
                        ui.end_row();
                    });
                }

                ui.add_space(8.0);
                ui.checkbox(&mut layer.air_assist, "Air Assist (M8)");
                ui.checkbox(&mut layer.exhaust_enabled, "🌬 Exhaust Fan (M7)");
                if layer.exhaust_enabled {
                    ui.horizontal(|ui| {
                        ui.label("Post-delay:");
                        ui.add(egui::DragValue::new(&mut layer.exhaust_post_delay_s).speed(0.5).range(0.0..=60.0).suffix(" s"));
                    });
                }
                ui.checkbox(&mut layer.visible, "Output Enabled");
                ui.checkbox(&mut layer.is_construction, "🔧 Construction Layer (no output)").on_hover_text("Construction layers are visible on canvas but excluded from GCode output.");

                ui.add_space(8.0);
                ui.group(|ui| {
                    ui.label(RichText::new("📐 Kerf Calibration Assistant").strong());
                    ui.label(
                        RichText::new(
                            "Cut a square with known nominal size, then enter measured result.",
                        )
                        .small()
                        .color(theme::SUBTEXT),
                    );
                    ui.add_space(4.0);

                    ui.horizontal(|ui| {
                        ui.label("Nominal (mm):");
                        ui.add(
                            egui::DragValue::new(&mut kerf_nominal)
                                .speed(0.1)
                                .range(1.0..=500.0)
                                .suffix(" mm"),
                        );
                        ui.label("Measured (mm):");
                        ui.add(
                            egui::DragValue::new(&mut kerf_measured)
                                .speed(0.1)
                                .range(0.1..=500.0)
                                .suffix(" mm"),
                        );
                    });

                    let kerf_reco = kerf_from_test_measurement(kerf_nominal, kerf_measured);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("Recommended kerf: {kerf_reco:.3} mm"))
                                .color(theme::GREEN)
                                .strong(),
                        );
                        if ui.button("Apply to Kerf Offset").clicked() {
                            layer.kerf_mm = kerf_reco;
                        }
                    });
                });

                if matches!(layer.mode, CutMode::Fill | CutMode::FillAndLine | CutMode::Offset) {
                    if let Some(layer_idx) = state.editing_layer_idx {
                        let non_fillable = layer_non_fillable_path_count(shapes, layer_idx);
                        if non_fillable > 0 {
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new(format!(
                                    "⚠ {non_fillable} path(s) on this layer are open/invalid and will be ignored by Fill."
                                ))
                                .color(theme::PEACH),
                            );
                        }
                    }
                }

                // Parameter comparison snapshot (F91)
                ui.add_space(8.0);
                ui.group(|ui| {
                    ui.label(RichText::new("📸 Parameter Snapshot").strong());
                    ui.horizontal(|ui| {
                        if ui.button("Take Snapshot").on_hover_text("Save current parameters for comparison").clicked() {
                            state.snapshot_layer = Some(layer.clone());
                        }
                        let has_snapshot = state.snapshot_layer.is_some();
                        if ui.add_enabled(has_snapshot, egui::Button::new(if state.show_comparison { "Hide Compare" } else { "Show Compare" })).clicked() {
                            state.show_comparison = !state.show_comparison;
                        }
                        if ui.add_enabled(has_snapshot, egui::Button::new("Clear")).clicked() {
                            state.snapshot_layer = None;
                            state.show_comparison = false;
                        }
                    });
                    if state.show_comparison {
                        if let Some(snap) = &state.snapshot_layer {
                            ui.add_space(4.0);
                            ui.label(RichText::new("Current → Snapshot").small().color(theme::SUBTEXT));
                            egui::Grid::new("snapshot_compare_grid").num_columns(3).spacing([8.0, 2.0]).show(ui, |ui| {
                                let comparisons: Vec<(&str, f32, f32)> = vec![
                                    ("Speed", layer.speed, snap.speed),
                                    ("Power", layer.power, snap.power),
                                    ("Passes", layer.passes as f32, snap.passes as f32),
                                    ("Fill Interval", layer.fill_interval_mm, snap.fill_interval_mm),
                                    ("Kerf", layer.kerf_mm, snap.kerf_mm),
                                ];
                                for (name, current, previous) in &comparisons {
                                    let diff = current - previous;
                                    let color = if diff.abs() < 1e-4 { theme::SUBTEXT } else if diff > 0.0 { theme::GREEN } else { theme::RED };
                                    ui.label(*name);
                                    ui.label(format!("{current:.1}"));
                                    ui.label(RichText::new(if diff.abs() < 1e-4 { "=".into() } else { format!("{diff:+.1}") }).color(color));
                                    ui.end_row();
                                }
                            });
                        }
                    }
                });

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

    state.kerf_test_nominal_mm = kerf_nominal;
    state.kerf_test_measured_mm = kerf_measured;

    if !open || action.close {
        state.is_open = false;
        state.temp_layer = None;
        state.editing_layer_idx = None;
    }

    action
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_non_fillable_path_count_detects_open_or_too_short_paths() {
        let shapes = vec![
            ShapeParams {
                shape: ShapeKind::Path(crate::ui::drawing::PathData::from_points(vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0)])),
                layer_idx: 0,
                ..ShapeParams::default()
            },
            ShapeParams {
                shape: ShapeKind::Path(crate::ui::drawing::PathData::from_points(vec![(1.0, 1.0), (2.0, 2.0)])),
                layer_idx: 0,
                ..ShapeParams::default()
            },
            ShapeParams {
                shape: ShapeKind::Path(crate::ui::drawing::PathData::from_points(vec![
                    (0.0, 0.0),
                    (10.0, 0.0),
                    (10.0, 10.0),
                    (0.0, 10.0),
                    (0.0, 0.0),
                ])),
                layer_idx: 0,
                ..ShapeParams::default()
            },
        ];

        assert_eq!(layer_non_fillable_path_count(&shapes, 0), 2);
    }

    #[test]
    fn kerf_from_measurement_returns_positive_for_shrunk_cut() {
        let kerf = kerf_from_test_measurement(20.0, 19.6);
        assert!((kerf - 0.2).abs() < 1e-6);
    }

    #[test]
    fn kerf_from_measurement_returns_negative_for_oversized_cut() {
        let kerf = kerf_from_test_measurement(20.0, 20.4);
        assert!((kerf + 0.2).abs() < 1e-6);
    }
}
