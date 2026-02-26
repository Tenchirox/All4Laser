use egui::{Ui, RichText, TextureHandle};
use serde::{Deserialize, Serialize};

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
    pub opacity: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            enabled: false,
            calibration: CameraCalibration::default(),
            texture: None,
            opacity: 0.5,
        }
    }
}

pub fn show(ui: &mut Ui, state: &mut CameraState) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("📷 Camera Overlay").color(crate::theme::LAVENDER).strong());
            ui.checkbox(&mut state.enabled, "Enabled");
        });

        if state.enabled {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("Opacity:");
                ui.add(egui::Slider::new(&mut state.opacity, 0.0..=1.0));
            });
            
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
                if ui.button("Reset Calibration").clicked() {
                    state.calibration = CameraCalibration::default();
                }
            });
        }
    });
}
