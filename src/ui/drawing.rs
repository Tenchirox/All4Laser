/// Drawing Tools: Rectangle, Circle, and Text generators producing GCode directly

use egui::{Ui, RichText};
use crate::theme;

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeKind {
    Rectangle,
    Circle,
    TextLine,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapeParams {
    pub shape: ShapeKind,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub radius: f32,
    pub speed: f32,
    pub power: f32,
    pub passes: u32,
    pub text: String,
    pub font_size_mm: f32,
}

impl Default for ShapeParams {
    fn default() -> Self {
        Self {
            shape: ShapeKind::Rectangle,
            x: 0.0,
            y: 0.0,
            width: 50.0,
            height: 30.0,
            radius: 20.0,
            speed: 500.0,
            power: 800.0,
            passes: 1,
            text: "Hello".into(),
            font_size_mm: 10.0,
        }
    }
}

pub struct DrawingState {
    pub current: ShapeParams,
    pub shapes: Vec<ShapeParams>,
}

impl Default for DrawingState {
    fn default() -> Self {
        Self {
            current: ShapeParams::default(),
            shapes: Vec::new(),
        }
    }
}

pub struct DrawingAction {
    pub generate_gcode: Option<Vec<String>>,
}

pub fn show(ui: &mut Ui, state: &mut DrawingState) -> DrawingAction {
    let mut action = DrawingAction { generate_gcode: None };

    ui.group(|ui| {
        ui.label(RichText::new("✏ Drawing Tools").color(theme::LAVENDER).strong());
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            if ui.selectable_label(state.current.shape == ShapeKind::Rectangle, "▭ Rect").clicked() {
                state.current.shape = ShapeKind::Rectangle;
            }
            if ui.selectable_label(state.current.shape == ShapeKind::Circle, "○ Circle").clicked() {
                state.current.shape = ShapeKind::Circle;
            }
            if ui.selectable_label(state.current.shape == ShapeKind::TextLine, "T Text").clicked() {
                state.current.shape = ShapeKind::TextLine;
            }
        });

        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label("Origin X:");
            ui.add(egui::DragValue::new(&mut state.current.x).speed(1.0).suffix(" mm"));
            ui.label("Y:");
            ui.add(egui::DragValue::new(&mut state.current.y).speed(1.0).suffix(" mm"));
        });

        match state.current.shape {
            ShapeKind::Rectangle => {
                ui.horizontal(|ui| {
                    ui.label("W:");
                    ui.add(egui::DragValue::new(&mut state.current.width).speed(1.0).suffix(" mm"));
                    ui.label("H:");
                    ui.add(egui::DragValue::new(&mut state.current.height).speed(1.0).suffix(" mm"));
                });
            }
            ShapeKind::Circle => {
                ui.horizontal(|ui| {
                    ui.label("Radius:");
                    ui.add(egui::DragValue::new(&mut state.current.radius).speed(1.0).suffix(" mm"));
                });
            }
            ShapeKind::TextLine => {
                ui.horizontal(|ui| {
                    ui.label("Text:");
                    ui.text_edit_singleline(&mut state.current.text);
                });
                ui.horizontal(|ui| {
                    ui.label("Font size:");
                    ui.add(egui::DragValue::new(&mut state.current.font_size_mm).speed(0.5).suffix(" mm"));
                });
            }
        }

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label("Speed:");
            ui.add(egui::DragValue::new(&mut state.current.speed).speed(10.0));
            ui.label("Power:");
            ui.add(egui::DragValue::new(&mut state.current.power).speed(5.0));
            ui.label("Passes:");
            ui.add(egui::DragValue::new(&mut state.current.passes).range(1..=50));
        });

        ui.add_space(4.0);

        ui.horizontal(|ui| {
            if ui.button(RichText::new("➕ Add Shape").color(theme::GREEN).strong()).clicked() {
                state.shapes.push(state.current.clone());
                let lines = generate_all_gcode(state);
                action.generate_gcode = Some(lines);
            }
            if ui.button("⮪ Undo").clicked() {
                state.shapes.pop();
                let lines = generate_all_gcode(state);
                action.generate_gcode = Some(lines);
            }
            if ui.button("🗑 Clear").clicked() {
                state.shapes.clear();
                let lines = generate_all_gcode(state);
                action.generate_gcode = Some(lines);
            }
        });
        
        if !state.shapes.is_empty() {
            ui.label(RichText::new(format!("{} shapes in drawing", state.shapes.len())).small().color(theme::SUBTEXT));
        }
    });

    action
}

fn generate_all_gcode(state: &DrawingState) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push("; Compiled Drawing — All4Laser".into());
    lines.push("G90".into());
    lines.push("G21".into());

    for (idx, shape) in state.shapes.iter().enumerate() {
        lines.push(format!("; Shape {}: {:?}", idx + 1, shape.shape));
        for pass in 0..shape.passes {
            if shape.passes > 1 { lines.push(format!("; Pass {}", pass + 1)); }
            match shape.shape {
                ShapeKind::Rectangle => gen_rect(&mut lines, shape),
                ShapeKind::Circle => gen_circle(&mut lines, shape),
                ShapeKind::TextLine => gen_text(&mut lines, shape),
            }
        }
    }

    lines.push("M5".into());
    lines.push("G0 X0 Y0".into());
    lines
}

fn generate_gcode(_state: &DrawingState) -> Vec<String> {
    // This is no longer used, we use generate_all_gcode instead
    Vec::new()
}

fn gen_rect(lines: &mut Vec<String>, s: &ShapeParams) {
    let (x0, y0) = (s.x, s.y);
    let (x1, y1) = (s.x + s.width, s.y + s.height);
    let sp = s.speed;
    let pw = s.power;
    lines.push(format!("M5"));
    lines.push(format!("G0 X{:.3} Y{:.3}", x0, y0));
    lines.push(format!("M3 S{:.0}", pw));
    lines.push(format!("G1 X{:.3} Y{:.3} F{:.0}", x1, y0, sp));
    lines.push(format!("G1 X{:.3} Y{:.3} F{:.0}", x1, y1, sp));
    lines.push(format!("G1 X{:.3} Y{:.3} F{:.0}", x0, y1, sp));
    lines.push(format!("G1 X{:.3} Y{:.3} F{:.0}", x0, y0, sp));
    lines.push("M5".into());
}

fn gen_circle(lines: &mut Vec<String>, s: &ShapeParams) {
    use std::f32::consts::PI;
    let cx = s.x;
    let cy = s.y;
    let r = s.radius;
    let steps = 64;
    let sp = s.speed;
    let pw = s.power;

    // Start at right of circle
    let start_x = cx + r;
    let start_y = cy;

    lines.push("M5".into());
    lines.push(format!("G0 X{:.3} Y{:.3}", start_x, start_y));
    lines.push(format!("M3 S{:.0}", pw));

    for i in 1..=steps {
        let angle = 2.0 * PI * (i as f32) / (steps as f32);
        let px = cx + r * angle.cos();
        let py = cy + r * angle.sin();
        lines.push(format!("G1 X{:.3} Y{:.3} F{:.0}", px, py, sp));
    }
    lines.push("M5".into());
}

fn gen_text(lines: &mut Vec<String>, s: &ShapeParams) {
    let char_w = s.font_size_mm * 0.6;
    let char_h = s.font_size_mm;
    let sp = s.speed;
    let pw = s.power;
    let mut cursor_x = s.x;

    for ch in s.text.chars() {
        let strokes = get_char_strokes(ch, cursor_x, s.y, char_w, char_h);
        for (x0, y0, x1, y1) in strokes {
            lines.push("M5".into());
            lines.push(format!("G0 X{:.3} Y{:.3}", x0, y0));
            lines.push(format!("M3 S{:.0}", pw));
            lines.push(format!("G1 X{:.3} Y{:.3} F{:.0}", x1, y1, sp));
        }
        cursor_x += char_w + (s.font_size_mm * 0.1);
    }
    lines.push("M5".into());
}

/// Returns a list of (x0, y0, x1, y1) strokes approximating a character
fn get_char_strokes(c: char, ox: f32, oy: f32, w: f32, h: f32) -> Vec<(f32, f32, f32, f32)> {
    let t = h;       // top
    let m = h / 2.0; // middle
    let b = 0.0;     // bottom
    let l = ox;
    let r = ox + w;
    let ml = ox + w * 0.3;

    match c.to_ascii_uppercase() {
        'A' => vec![(l, oy+b, ml, oy+t), (ml, oy+t, r, oy+b), (l+w*0.2, oy+m, r-w*0.2, oy+m)],
        'B' => vec![(l, oy+b, l, oy+t), (l, oy+t, r-w*0.1, oy+t-h*0.05), (l, oy+m, r-w*0.1, oy+m), (l, oy+b, r-w*0.1, oy+b)],
        'C' => vec![(r, oy+t, l, oy+t), (l, oy+t, l, oy+b), (l, oy+b, r, oy+b)],
        'H' => vec![(l, oy+b, l, oy+t), (l, oy+m, r, oy+m), (r, oy+b, r, oy+t)],
        'I' => vec![(ml, oy+b, ml, oy+t), (l, oy+t, r, oy+t), (l, oy+b, r, oy+b)],
        'L' => vec![(l, oy+t, l, oy+b), (l, oy+b, r, oy+b)],
        'O' | '0' => vec![(l, oy+b, l, oy+t), (l, oy+t, r, oy+t), (r, oy+t, r, oy+b), (r, oy+b, l, oy+b)],
        'T' => vec![(l, oy+t, r, oy+t), (ml, oy+t, ml, oy+b)],
        'V' => vec![(l, oy+t, ml, oy+b), (ml, oy+b, r, oy+t)],
        'Z' => vec![(l, oy+t, r, oy+t), (r, oy+t, l, oy+b), (l, oy+b, r, oy+b)],
        // Space/fallback: no strokes
        _ => vec![],
    }
}
