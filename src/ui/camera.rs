use egui::{Ui, RichText, TextureHandle};
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
        }
    }
}

pub fn show(ui: &mut Ui, state: &mut CameraState) -> CameraAction {
    let mut action = CameraAction::default();

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("📷 Camera Overlay").color(crate::theme::LAVENDER).strong());
            ui.checkbox(&mut state.enabled, "Enabled");
        });

        ui.horizontal(|ui| {
            if ui.button("📸 Open Image").clicked() {
                action.load_snapshot = true;
            }
            let live_label = if state.live_streaming { "⏹ Stop Live" } else { "▶ Live Camera" };
            if ui.button(live_label).clicked() {
                if state.live_streaming {
                    action.stop_live_stream = true;
                } else {
                    action.start_live_stream = true;
                }
            }
            if ui
                .add_enabled(state.texture.is_some(), egui::Button::new("🗑 Clear"))
                .clicked()
            {
                action.clear_snapshot = true;
            }
        });

        let devices = crate::camera_stream::list_camera_devices();
        ui.horizontal(|ui| {
            ui.label("Device:");
            if devices.is_empty() {
                ui.add(egui::DragValue::new(&mut state.device_index).range(0..=32).speed(1));
            } else {
                let selected_label = devices
                    .iter()
                    .find(|d| d.index as i32 == state.device_index)
                    .map(|d| d.label.clone())
                    .unwrap_or_else(|| format!("/dev/video{}", state.device_index));
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
                ui.label(RichText::new("Live stream active").small().color(crate::theme::GREEN));
            }
        });

        if let Some(texture) = &state.texture {
            let [w, h] = texture.size();
            ui.label(
                RichText::new(format!("Camera frame: {w}×{h}px"))
                    .small()
                    .color(crate::theme::SUBTEXT),
            );
        } else {
            ui.label(
                RichText::new("No camera image yet (start live camera or open image)")
                    .small()
                    .color(crate::theme::SUBTEXT),
            );
        }

        if state.enabled {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("Opacity:");
                ui.add(egui::Slider::new(&mut state.opacity, 0.0..=1.0));
            });

            ui.horizontal(|ui| {
                if ui.button("🎯 Align Job to Camera").clicked() {
                    action.align_job_to_camera = true;
                }
            });

            ui.horizontal(|ui| {
                if !state.calibration_wizard_active {
                    if ui.button("🧭 Calibration Wizard").clicked() {
                        action.start_calibration_wizard = true;
                    }
                } else if ui.button("🛑 Stop Calibration").clicked() {
                    action.stop_calibration_wizard = true;
                }

                if !state.point_align_active {
                    if ui.button("📍 2-Point Align").clicked() {
                        action.start_point_align = true;
                    }
                } else if ui.button("🛑 Stop Point Align").clicked() {
                    action.stop_point_align = true;
                }
            });

            if state.calibration_wizard_active {
                let step_hint = match state.calibration_pick_count {
                    0 => "Calibration step 1/3: click workspace origin (0,0) on camera overlay.",
                    1 => "Calibration step 2/3: click point matching workspace +X edge.",
                    _ => "Calibration step 3/3: click point matching workspace +Y edge.",
                };
                ui.label(RichText::new(step_hint).small().color(crate::theme::PEACH));
            }

            if state.point_align_active {
                let step_hint = if state.point_align_pick_count == 0 {
                    "Point align 1/2: click where job bottom-left should be."
                } else {
                    "Point align 2/2: click where job bottom-right should be."
                };
                ui.label(RichText::new(step_hint).small().color(crate::theme::PEACH));
            }
            
            ui.collapsing("Calibration", |ui| {
                egui::Grid::new("cam_calib").num_columns(2).show(ui, |ui| {
                    ui.label("Offset X:"); ui.add(egui::DragValue::new(&mut state.calibration.offset_x).speed(1.0));
                    ui.end_row();
                    ui.label("Offset Y:"); ui.add(egui::DragValue::new(&mut state.calibration.offset_y).speed(1.0));
                    ui.end_row();
                    ui.label("Scale:"); ui.add(egui::DragValue::new(&mut state.calibration.scale).speed(0.01));
                    ui.end_row();
                    ui.label("Rotation:"); ui.add(egui::Slider::new(&mut state.calibration.rotation, -180.0..=180.0));
                    ui.end_row();
                });

                ui.horizontal(|ui| {
                    ui.label("Nudge:");
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

                if ui.button("Reset Calibration").clicked() {
                    state.calibration = CameraCalibration::default();
                }
            });
        }
    });

    action
}
