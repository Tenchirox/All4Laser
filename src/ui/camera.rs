use crate::i18n::tr;
use egui::{Pos2, RichText, TextureHandle, Ui};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct CameraAction {
    pub load_snapshot: bool,
    pub clear_snapshot: bool,
    pub start_live_stream: bool,
    pub stop_live_stream: bool,
    pub start_calibration_wizard: bool,
    pub stop_calibration_wizard: bool,
    pub start_point_align: bool,
    pub stop_point_align: bool,
    pub align_job_to_camera: bool,
    pub auto_detect_mark: bool,
    pub auto_detect_markers: bool,
    pub apply_detected_align: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CameraCalibration {
    pub offset_x: f32,
    pub offset_y: f32,
    pub scale: f32,
    pub rotation: f32,
}

impl Default for CameraCalibration {
    fn default() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            scale: 1.0,
            rotation: 0.0,
        }
    }
}

pub struct CameraState {
    pub enabled: bool,
    pub calibration: CameraCalibration,
    pub texture: Option<TextureHandle>,
    pub snapshot_path: Option<String>,
    pub device_index: i32,
    pub live_streaming: bool,
    pub calibration_wizard_active: bool,
    pub calibration_pick_count: usize,
    pub point_align_active: bool,
    pub point_align_pick_count: usize,
    pub opacity: f32,
    pub latest_rgba: Option<(usize, usize, Vec<u8>)>,
    pub detected_cross_world: Option<Pos2>,
    pub detected_circle_world: Option<Pos2>,
    pub detection_status: String,
    pub auto_detection_success_count: u32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            enabled: false,
            calibration: CameraCalibration::default(),
            texture: None,
            snapshot_path: None,
            device_index: 0,
            live_streaming: false,
            calibration_wizard_active: false,
            calibration_pick_count: 0,
            point_align_active: false,
            point_align_pick_count: 0,
            opacity: 0.5,
            latest_rgba: None,
            detected_cross_world: None,
            detected_circle_world: None,
            detection_status: "No marker detection run yet.".to_string(),
            auto_detection_success_count: 0,
        }
    }
}

pub fn show(ui: &mut Ui, state: &mut CameraState) -> CameraAction {
    let mut action = CameraAction::default();

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("📷 {}", tr("Camera"))).color(crate::theme::LAVENDER).strong());
            ui.checkbox(&mut state.enabled, tr("Enabled"));
        });

        if !state.enabled {
            return action;
        }

        // Device & Stream Section
        ui.collapsing(format!("📸 {}", tr("Device & Stream")), |ui| {
            ui.horizontal(|ui| {
                if ui.button(format!("📸 {}", tr("Open Image"))).clicked() {
                    action.load_snapshot = true;
                }
                let live_label = if state.live_streaming { 
                    format!("⏹ {}", tr("Stop Live")) 
                } else { 
                    format!("▶ {}", tr("Live Camera")) 
                };
                if ui.button(live_label).clicked() {
                    if state.live_streaming {
                        action.stop_live_stream = true;
                    } else {
                        action.start_live_stream = true;
                    }
                }
                if ui
                    .add_enabled(state.texture.is_some(), egui::Button::new(format!("🗑 {}", tr("Clear"))))
                    .clicked()
                {
                    action.clear_snapshot = true;
                }
            });

            let devices = crate::camera_stream::list_camera_devices();
            ui.horizontal(|ui| {
                ui.label(format!("{}:", tr("Device")));
                if devices.is_empty() {
                    ui.add(egui::DragValue::new(&mut state.device_index).range(0..=32).speed(1));
                    #[cfg(target_os = "windows")]
                    ui.label(
                        RichText::new(tr("No camera detected. Check Windows camera privacy settings and close other apps using the webcam."))
                            .small()
                            .color(crate::theme::PEACH),
                    );
                } else {
                    let selected_label = devices
                        .iter()
                        .find(|d| d.index as i32 == state.device_index)
                        .map(|d| d.label.clone())
                        .unwrap_or_else(|| format!("{} {}", tr("Camera"), state.device_index));
                    egui::ComboBox::from_id_salt("camera_device_combo")
                        .selected_text(selected_label)
                        .show_ui(ui, |ui| {
                            for dev in &devices {
                                ui.selectable_value(
                                    &mut state.device_index,
                                    dev.index as i32,
                                    format!("{} ({})", dev.label, dev.index),
                                );
                            }
                        });
                }

                if state.live_streaming {
                    ui.label(RichText::new(tr("Live stream active")).small().color(crate::theme::GREEN));
                }
            });

            if let Some(texture) = &state.texture {
                let [w, h] = texture.size();
                ui.label(
                    RichText::new(format!("{}: {w}×{h}px", tr("Camera frame")))
                        .small()
                        .color(crate::theme::SUBTEXT),
                );
            } else {
                ui.label(
                    RichText::new(tr("No camera image yet (start live camera or open image)"))
                        .small()
                        .color(crate::theme::SUBTEXT),
                );
            }

            ui.horizontal(|ui| {
                ui.label(format!("{}:", tr("Opacity")));
                ui.add(egui::Slider::new(&mut state.opacity, 0.0..=1.0));
            });
        });

        // Alignment & Detection Section
        ui.collapsing(format!("🎯 {}", tr("Alignment & Detection")), |ui| {
            ui.horizontal(|ui| {
                if ui.button(format!("🎯 {}", tr("Align Job to Camera"))).clicked() {
                    action.align_job_to_camera = true;
                }
                if ui
                    .add_enabled(state.texture.is_some(), egui::Button::new(format!("🔎 {}", tr("Auto Detect Markers"))))
                    .clicked()
                {
                    action.auto_detect_markers = true;
                }
            });

            ui.label(
                RichText::new(state.detection_status.as_str())
                    .small()
                    .color(crate::theme::SUBTEXT),
            );

            if let (Some(cross), Some(circle)) = (state.detected_cross_world, state.detected_circle_world) {
                ui.label(
                    RichText::new(format!(
                        "{}: {} ({:.1}, {:.1}) mm, {} ({:.1}, {:.1}) mm",
                        tr("Detected markers"),
                        tr("Cross"), cross.x, cross.y,
                        tr("Circle"), circle.x, circle.y
                    ))
                    .small()
                    .color(crate::theme::GREEN),
                );
                if ui.button(format!("✅ {}", tr("Apply Detected Markers (2-point align)"))).clicked() {
                    action.apply_detected_align = true;
                }
            } else {
                ui.label(
                    RichText::new(tr("If detection fails, use Calibration Wizard / 2-Point Align for manual correction."))
                        .small()
                        .color(crate::theme::PEACH),
                );
            }
            ui.label(
                RichText::new(format!("{}: {}", tr("Auto-detect validated captures"), state.auto_detection_success_count))
                    .small()
                    .color(crate::theme::SUBTEXT),
            );
        });

        // Calibration Tools Section
        ui.collapsing(format!("🧭 {}", tr("Calibration Tools")), |ui| {
            ui.horizontal(|ui| {
                if !state.calibration_wizard_active {
                    if ui.button(format!("🧭 {}", tr("Calibration Wizard"))).clicked() {
                        action.start_calibration_wizard = true;
                    }
                } else if ui.button(format!("🛑 {}", tr("Stop Calibration"))).clicked() {
                    action.stop_calibration_wizard = true;
                }

                if !state.point_align_active {
                    if ui.button(format!("📍 {}", tr("2-Point Align"))).clicked() {
                        action.start_point_align = true;
                    }
                } else if ui.button(format!("🛑 {}", tr("Stop Point Align"))).clicked() {
                    action.stop_point_align = true;
                }
            });

            if state.calibration_wizard_active {
                let step_hint = match state.calibration_pick_count {
                    0 => tr("Calibration step 1/3: click workspace origin (0,0) on camera overlay."),
                    1 => tr("Calibration step 2/3: click point matching workspace +X edge."),
                    _ => tr("Calibration step 3/3: click point matching workspace +Y edge."),
                };
                ui.label(RichText::new(step_hint).small().color(crate::theme::PEACH));
            }

            if state.point_align_active {
                let step_hint = if state.point_align_pick_count == 0 {
                    tr("Point align 1/2: click where job bottom-left should be.")
                } else {
                    tr("Point align 2/2: click where job bottom-right should be.")
                };
                ui.label(RichText::new(step_hint).small().color(crate::theme::PEACH));
            }

            if state.calibration_wizard_active || state.point_align_active {
                ui.horizontal(|ui| {
                    if ui.button(format!("✨ {}", tr("Auto-Detect Mark"))).clicked() {
                        action.auto_detect_mark = true;
                    }
                });
            }
        });

        // Manual Calibration Section
        ui.collapsing(format!("⚙ {}", tr("Manual Calibration")), |ui| {
            egui::Grid::new("cam_calib").num_columns(2).show(ui, |ui| {
                ui.label(format!("{} X:", tr("Offset"))); ui.add(egui::DragValue::new(&mut state.calibration.offset_x).speed(1.0));
                ui.end_row();
                ui.label(format!("{} Y:", tr("Offset"))); ui.add(egui::DragValue::new(&mut state.calibration.offset_y).speed(1.0));
                ui.end_row();
                ui.label(format!("{}:", tr("Scale"))); ui.add(egui::DragValue::new(&mut state.calibration.scale).speed(0.01));
                ui.end_row();
                ui.label(format!("{}:", tr("Rotation"))); ui.add(egui::Slider::new(&mut state.calibration.rotation, -180.0..=180.0));
                ui.end_row();
            });

            ui.horizontal(|ui| {
                ui.label(format!("{}:", tr("Nudge")));
                if ui.button("←").clicked() {
                    state.calibration.offset_x -= 1.0;
                }
                if ui.button("→").clicked() {
                    state.calibration.offset_x += 1.0;
                }
                if ui.button("↓").clicked() {
                    state.calibration.offset_y -= 1.0;
                }
                if ui.button("↑").clicked() {
                    state.calibration.offset_y += 1.0;
                }
            });

            if ui.button(tr("Reset Calibration")).clicked() {
                state.calibration = CameraCalibration::default();
            }
        });
    });

    action
}
