use egui::{RichText, Ui};
use qrcode::QrCode;

pub struct GeneratorState {
    pub qr_text: String,
    pub box_w: f32,
    pub box_h: f32,
    pub box_d: f32,
    pub box_thickness: f32,
    pub box_tab_size: f32,
    pub fiducial_width: f32,
    pub fiducial_height: f32,
    pub fiducial_margin: f32,
    pub fiducial_mark_size: f32,
    pub fiducial_feed: f32,
    pub fiducial_power: f32,
    pub fiducial_draw_frame: bool,
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
            fiducial_width: 200.0,
            fiducial_height: 150.0,
            fiducial_margin: 10.0,
            fiducial_mark_size: 6.0,
            fiducial_feed: 1200.0,
            fiducial_power: 350.0,
            fiducial_draw_frame: true,
        }
    }
}

pub struct GeneratorAction {
    pub generate_gcode: Option<Vec<String>>,
}

pub fn show(ui: &mut Ui, state: &mut GeneratorState) -> GeneratorAction {
    let mut action = GeneratorAction {
        generate_gcode: None,
    };

    ui.group(|ui| {
        ui.label(
            RichText::new("📦 Object Generators")
                .color(crate::theme::LAVENDER)
                .strong(),
        );
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
                let w = state.box_w;
                let h = state.box_h;
                let d = state.box_d;
                let t = state.box_thickness;
                let ts = state.box_tab_size;

                // Edge types: true = Male (starts with tab at boundary), false = Female (starts with recess)
                // Bottom/Top Faces (W x H)
                add_tabbed_face(&mut gcode, w, h, t, ts, 0.0, 0.0, [true, true, true, true]); // Bottom face
                add_tabbed_face(
                    &mut gcode,
                    w,
                    h,
                    t,
                    ts,
                    w + 10.0,
                    0.0,
                    [true, true, true, true],
                ); // Top face

                // Front/Back Faces (W x D)
                add_tabbed_face(
                    &mut gcode,
                    w,
                    d,
                    t,
                    ts,
                    0.0,
                    h + 10.0,
                    [false, true, false, true],
                ); // Front
                add_tabbed_face(
                    &mut gcode,
                    w,
                    d,
                    t,
                    ts,
                    w + 10.0,
                    h + 10.0,
                    [false, true, false, true],
                ); // Back

                // Left/Right Faces (H x D)
                add_tabbed_face(
                    &mut gcode,
                    h,
                    d,
                    t,
                    ts,
                    0.0,
                    h + d + 20.0,
                    [false, false, false, false],
                ); // Left
                add_tabbed_face(
                    &mut gcode,
                    h,
                    d,
                    t,
                    ts,
                    h + 10.0,
                    h + d + 20.0,
                    [false, false, false, false],
                ); // Right

                action.generate_gcode = Some(gcode);
            }
        });

        ui.collapsing("🎯 Print & Cut Fiducials", |ui| {
            ui.label("Generate 4 registration marks + optional outer frame.");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Width:");
                ui.add(
                    egui::DragValue::new(&mut state.fiducial_width)
                        .range(20.0..=2000.0)
                        .suffix(" mm"),
                );
                ui.label("Height:");
                ui.add(
                    egui::DragValue::new(&mut state.fiducial_height)
                        .range(20.0..=2000.0)
                        .suffix(" mm"),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Margin:");
                ui.add(
                    egui::DragValue::new(&mut state.fiducial_margin)
                        .range(1.0..=200.0)
                        .suffix(" mm"),
                );
                ui.label("Mark size:");
                ui.add(
                    egui::DragValue::new(&mut state.fiducial_mark_size)
                        .range(1.0..=100.0)
                        .suffix(" mm"),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Feed:");
                ui.add(egui::DragValue::new(&mut state.fiducial_feed).range(50.0..=10000.0));
                ui.label("Power (S):");
                ui.add(egui::DragValue::new(&mut state.fiducial_power).range(1.0..=1000.0));
            });
            ui.checkbox(&mut state.fiducial_draw_frame, "Include outer frame");

            if ui.button("🚀 Generate Fiducials").clicked() {
                action.generate_gcode = Some(generate_fiducials_gcode(state));
            }
        });
    });

    action
}

fn generate_fiducials_gcode(state: &GeneratorState) -> Vec<String> {
    let mut gcode = Vec::new();
    let w = state.fiducial_width.max(20.0);
    let h = state.fiducial_height.max(20.0);
    let margin = state.fiducial_margin.clamp(1.0, (w.min(h) * 0.45).max(1.0));
    let mark = state.fiducial_mark_size.clamp(1.0, 100.0);
    let half = mark * 0.5;
    let feed = state.fiducial_feed.max(50.0);
    let power = state.fiducial_power.clamp(1.0, 1000.0);

    let corners = [
        (margin, margin),
        (w - margin, margin),
        (w - margin, h - margin),
        (margin, h - margin),
    ];

    gcode.push("; Print & Cut Fiducials — All4Laser".to_string());
    gcode.push("G90".to_string());
    gcode.push("G21".to_string());
    gcode.push("M5".to_string());

    if state.fiducial_draw_frame {
        gcode.push("; Outer frame".to_string());
        gcode.push("M5".to_string());
        gcode.push(format!("G0 X{:.3} Y{:.3}", 0.0, 0.0));
        gcode.push(format!("M3 S{:.0}", power));
        gcode.push(format!("G1 X{:.3} Y{:.3} F{:.0}", w, 0.0, feed));
        gcode.push(format!("G1 X{:.3} Y{:.3} F{:.0}", w, h, feed));
        gcode.push(format!("G1 X{:.3} Y{:.3} F{:.0}", 0.0, h, feed));
        gcode.push(format!("G1 X{:.3} Y{:.3} F{:.0}", 0.0, 0.0, feed));
        gcode.push("M5".to_string());
    }

    for (idx, (cx, cy)) in corners.into_iter().enumerate() {
        gcode.push(format!("; Fiducial {}", idx + 1));

        gcode.push(format!("G0 X{:.3} Y{:.3}", cx - half, cy));
        gcode.push(format!("M3 S{:.0}", power));
        gcode.push(format!("G1 X{:.3} Y{:.3} F{:.0}", cx + half, cy, feed));
        gcode.push("M5".to_string());

        gcode.push(format!("G0 X{:.3} Y{:.3}", cx, cy - half));
        gcode.push(format!("M3 S{:.0}", power));
        gcode.push(format!("G1 X{:.3} Y{:.3} F{:.0}", cx, cy + half, feed));
        gcode.push("M5".to_string());
    }

    gcode.push("M5".to_string());
    gcode.push("G0 X0 Y0 F3000".to_string());
    gcode
}

fn add_tabbed_face(
    gcode: &mut Vec<String>,
    w: f32,
    h: f32,
    t: f32,
    ts: f32,
    ox: f32,
    oy: f32,
    edges: [bool; 4],
) {
    gcode.push(format!("; Face {}x{}", w, h));
    gcode.push(format!("G0 X{} Y{}", ox, oy));
    gcode.push("M3 S1000".into());

    // Edges: 0: Bottom (+X), 1: Right (+Y), 2: Top (-X), 3: Left (-Y)
    draw_edge(gcode, ox, oy, w, 0.0, t, ts, edges[0]);
    draw_edge(gcode, ox + w, oy, 0.0, h, t, ts, edges[1]);
    draw_edge(gcode, ox + w, oy + h, -w, 0.0, t, ts, edges[2]);
    draw_edge(gcode, ox, oy + h, 0.0, -h, t, ts, edges[3]);

    gcode.push("M5".into());
}

fn draw_edge(
    gcode: &mut Vec<String>,
    sx: f32,
    sy: f32,
    dx: f32,
    dy: f32,
    t: f32,
    ts: f32,
    is_male: bool,
) {
    let length = (dx * dx + dy * dy).sqrt();
    let num_tabs = (length / ts).floor() as i32;
    if num_tabs < 1 {
        gcode.push(format!("G1 X{} Y{} F800", sx + dx, sy + dy));
        return;
    }

    let (nx, ny) = if dy == 0.0 {
        if dx > 0.0 { (0.0, -1.0) } else { (0.0, 1.0) }
    } else {
        if dy > 0.0 { (1.0, 0.0) } else { (-1.0, 0.0) }
    };

    for i in 0..num_tabs {
        let is_tab = (i % 2 == 0) == is_male;
        let offset = if is_tab { 0.0 } else { -t };

        let p_start_x = sx + (dx * (i as f32 / num_tabs as f32));
        let p_start_y = sy + (dy * (i as f32 / num_tabs as f32));

        let p_end_x = sx + (dx * ((i + 1) as f32 / num_tabs as f32));
        let p_end_y = sy + (dy * ((i + 1) as f32 / num_tabs as f32));

        // Move to offset starting point
        gcode.push(format!(
            "G1 X{} Y{}",
            p_start_x + nx * offset,
            p_start_y + ny * offset
        ));
        // Cut the segment
        gcode.push(format!(
            "G1 X{} Y{}",
            p_end_x + nx * offset,
            p_end_y + ny * offset
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::{GeneratorState, generate_fiducials_gcode};

    #[test]
    fn fiducials_generator_emits_four_marks() {
        let mut state = GeneratorState::default();
        state.fiducial_draw_frame = false;

        let lines = generate_fiducials_gcode(&state);
        let marks = lines
            .iter()
            .filter(|line| line.starts_with("; Fiducial "))
            .count();
        assert_eq!(marks, 4);
    }

    #[test]
    fn fiducials_generator_can_emit_outer_frame() {
        let mut state = GeneratorState::default();
        state.fiducial_draw_frame = true;

        let lines = generate_fiducials_gcode(&state);
        assert!(lines.iter().any(|line| line == "; Outer frame"));
    }
}
