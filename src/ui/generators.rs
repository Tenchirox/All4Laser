use egui::{Ui, RichText};
use qrcode::QrCode;

pub struct GeneratorState {
    pub qr_text: String,
    pub box_w: f32,
    pub box_h: f32,
    pub box_d: f32,
    pub box_thickness: f32,
    pub box_tab_size: f32,
}

impl Default for GeneratorState {
    fn default() -> Self {
        Self {
            qr_text: "https://github.com/arkypita/LaserGRBL".to_string(),
            box_w: 100.0,
            box_h: 80.0,
            box_d: 50.0,
            box_thickness: 3.0,
            box_tab_size: 10.0,
        }
    }
}

pub struct GeneratorAction {
    pub generate_gcode: Option<Vec<String>>,
}

pub fn show(ui: &mut Ui, state: &mut GeneratorState) -> GeneratorAction {
    let mut action = GeneratorAction { generate_gcode: None };

    ui.group(|ui| {
        ui.label(RichText::new("📦 Object Generators").color(crate::theme::LAVENDER).strong());
        ui.add_space(4.0);

        ui.collapsing("🔗 QR Code Generator", |ui| {
            ui.horizontal(|ui| {
                ui.label("Data/URL:");
                ui.text_edit_singleline(&mut state.qr_text);
            });
            if ui.button("🚀 Generate QR GCode").clicked() {
                if let Ok(code) = QrCode::new(&state.qr_text) {
                    let mut gcode = Vec::new();
                    gcode.push(format!("; QR: {}", state.qr_text));
                    let size = 1.0; // 1mm per module
                    let pixels = code.to_colors();
                    let width = code.width();
                    
                    gcode.push("G90".to_string());
                    gcode.push("M5".to_string());
                    for (i, color) in pixels.into_iter().enumerate() {
                        let x = (i % width) as f32 * size;
                        let y = (width - 1 - (i / width)) as f32 * size; // Flip Y
                        if color == qrcode::Color::Dark {
                            // Rectangle module
                            gcode.push(format!("G0 X{} Y{}", x, y));
                            gcode.push("M3 S800".to_string());
                            gcode.push(format!("G1 X{} Y{} F2000", x + size, y));
                            gcode.push(format!("G1 X{} Y{} ", x + size, y + size));
                            gcode.push(format!("G1 X{} Y{} ", x, y + size));
                            gcode.push(format!("G1 X{} Y{} ", x, y));
                            gcode.push("M5".to_string());
                        }
                    }
                    action.generate_gcode = Some(gcode);
                }
            }
        });

        ui.collapsing("📥 Box Maker (Finger Joints)", |ui| {
            ui.add(egui::Slider::new(&mut state.box_w, 20.0..=500.0).text("Width (X)"));
            ui.add(egui::Slider::new(&mut state.box_h, 20.0..=500.0).text("Height (Y)"));
            ui.add(egui::Slider::new(&mut state.box_d, 20.0..=500.0).text("Depth (Z)"));
            ui.add(egui::Slider::new(&mut state.box_thickness, 1.0..=20.0).text("Thickness"));
            ui.add(egui::Slider::new(&mut state.box_tab_size, 5.0..=50.0).text("Tab Size"));

            if ui.button("🚀 Generate Box Components").clicked() {
                let mut gcode = Vec::new();
                gcode.push("; Parametric Box Components".into());
                gcode.push("G90".into());
                
                // Front Face (W x D)
                add_face_gcode(&mut gcode, state.box_w, state.box_d, state.box_thickness, state.box_tab_size, 0.0, 0.0);
                // Back Face
                add_face_gcode(&mut gcode, state.box_w, state.box_d, state.box_thickness, state.box_tab_size, state.box_w + 10.0, 0.0);
                // Left
                add_face_gcode(&mut gcode, state.box_h, state.box_d, state.box_thickness, state.box_tab_size, 0.0, state.box_d + 10.0);
                // Right
                add_face_gcode(&mut gcode, state.box_h, state.box_d, state.box_thickness, state.box_tab_size, state.box_h + 10.0, state.box_d + 10.0);
                // Top
                add_face_gcode(&mut gcode, state.box_w, state.box_h, state.box_thickness, state.box_tab_size, 0.0, (state.box_d + 10.0) * 2.0);
                // Bottom
                add_face_gcode(&mut gcode, state.box_w, state.box_h, state.box_thickness, state.box_tab_size, state.box_w + 10.0, (state.box_d + 10.0) * 2.0);

                action.generate_gcode = Some(gcode);
            }
        });
    });

    action
}

fn add_face_gcode(gcode: &mut Vec<String>, w: f32, h: f32, t: f32, tab: f32, ox: f32, oy: f32) {
    gcode.push(format!("; Face {}x{}", w, h));
    gcode.push(format!("G0 X{} Y{}", ox, oy));
    gcode.push("M3 S1000".into());
    
    // Simple rectangular outline with tabs placeholder (simulated)
    // For a real finger joint box, we need to stagger the tabs.
    // This is a simplified version for now.
    gcode.push(format!("G1 X{} Y{} F800", ox + w, oy));
    gcode.push(format!("G1 X{} Y{}", ox + w, oy + h));
    gcode.push(format!("G1 X{} Y{}", ox, oy + h));
    gcode.push(format!("G1 X{} Y{}", ox, oy));
    gcode.push("M5".into());
}
