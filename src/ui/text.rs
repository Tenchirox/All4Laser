use egui::{Ui, RichText};
use rusttype::{Font, Scale, point, OutlineBuilder};

pub struct TextToolState {
    pub is_open: bool,
    pub text: String,
    pub font_size: f32,
    pub font_path: String,
    pub font_data: Option<Vec<u8>>,
}

impl Default for TextToolState {
    fn default() -> Self {
        Self {
            is_open: false,
            text: "All4Laser".to_string(),
            font_size: 40.0,
            font_path: "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf".to_string(),
            font_data: None,
        }
    }
}

pub struct GCodeBuilder {
    pub paths: Vec<Vec<(f32, f32)>>,
    current_path: Vec<(f32, f32)>,
    pub scale: f32,
}

impl GCodeBuilder {
    fn new(scale: f32) -> Self {
        Self {
            paths: Vec::new(),
            current_path: Vec::new(),
            scale,
        }
    }
}

impl OutlineBuilder for GCodeBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        if !self.current_path.is_empty() {
            self.paths.push(std::mem::take(&mut self.current_path));
        }
        self.current_path.push((x * self.scale, -y * self.scale));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.current_path.push((x * self.scale, -y * self.scale));
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        // Simple linear approximation of quadratic Bezier
        self.current_path.push((x1 * self.scale, -y1 * self.scale));
        self.current_path.push((x * self.scale, -y * self.scale));
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        // Simple linear approximation of cubic Bezier
        self.current_path.push((x1 * self.scale, -y1 * self.scale));
        self.current_path.push((x2 * self.scale, -y2 * self.scale));
        self.current_path.push((x * self.scale, -y * self.scale));
    }
    fn close(&mut self) {
        if let Some(&first) = self.current_path.first() {
            self.current_path.push(first);
        }
        if !self.current_path.is_empty() {
            self.paths.push(std::mem::take(&mut self.current_path));
        }
    }
}

pub struct TextAction {
    pub generate_gcode: Option<Vec<String>>,
}

pub fn show(ui: &mut Ui, state: &mut TextToolState) -> TextAction {
    let mut action = TextAction { generate_gcode: None };

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("✍ Native Text Tool").color(crate::theme::LAVENDER).strong());
        });
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label("Text:");
            ui.text_edit_singleline(&mut state.text);
        });

        ui.horizontal(|ui| {
            ui.label("Size:");
            ui.add(egui::DragValue::new(&mut state.font_size).range(1.0..=300.0).suffix(" pt"));
        });

        ui.horizontal(|ui| {
            if ui.button("📁 Load Font").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("TrueType Font", &["ttf", "otf"])
                    .pick_file() 
                {
                    state.font_path = path.to_string_lossy().to_string();
                    state.font_data = std::fs::read(&state.font_path).ok();
                }
            }
            ui.label(format!("Font: {}", std::path::Path::new(&state.font_path).file_name().unwrap_or_default().to_string_lossy()));
        });

        if ui.button("🚀 Generate Text Paths").clicked() {
            if state.font_data.is_none() {
                state.font_data = std::fs::read(&state.font_path).ok();
            }

            if let Some(data) = &state.font_data {
                if let Some(font) = Font::try_from_vec(data.clone()) {
                    let scale = Scale::uniform(state.font_size);
                    let v_metrics = font.v_metrics(scale);
                    let mut builder = GCodeBuilder::new(1.0); // rusttype coordinates are fine

                    let glyphs: Vec<_> = font.layout(&state.text, scale, point(0.0, v_metrics.ascent)).collect();
                    for glyph in glyphs {
                        glyph.unpositioned().build_outline(&mut builder);
                    }

                    // Redo with positioning
                    let mut final_paths = Vec::new();
                    for glyph in font.layout(&state.text, scale, point(0.0, v_metrics.ascent)) {
                        let pos = glyph.position();
                        let mut g_builder = GCodeBuilder::new(1.0);
                        glyph.unpositioned().build_outline(&mut g_builder);
                        for mut path in g_builder.paths {
                            for p in &mut path {
                                p.0 += pos.x;
                                p.1 -= pos.y - v_metrics.ascent; // Flip Y properly
                            }
                            final_paths.push(path);
                        }
                    }

                    let mut gcode = Vec::new();
                    gcode.push("; Text: ".to_string() + &state.text);
                    gcode.push("G90".to_string());
                    gcode.push("G21".to_string());
                    gcode.push("M5".to_string());
                    
                    for path in final_paths {
                        if path.is_empty() { continue; }
                        gcode.push(format!("G0 X{:.3} Y{:.3}", path[0].0, path[0].1));
                        gcode.push("M3 S500".to_string());
                        for p in &path[1..] {
                            gcode.push(format!("G1 X{:.3} Y{:.3} F1000", p.0, p.1));
                        }
                        gcode.push("M5".to_string());
                    }
                    action.generate_gcode = Some(gcode);
                }
            }
        }
    });

    action
}
