/// Drawing Tools: Rectangle, Circle, and Text generators producing GCode directly

use egui::{Ui, RichText};
use crate::theme;
use crate::ui::layers_new::{CutLayer, CutMode};
use crate::gcode::generator::GCodeBuilder;
use std::sync::Arc;
use crate::imaging::raster::RasterParams;

#[derive(Clone)]
pub struct ImageData(pub Arc<image::DynamicImage>);

impl std::fmt::Debug for ImageData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (w, h) = (self.0.width(), self.0.height());
        write!(f, "ImageData({}x{})", w, h)
    }
}

impl PartialEq for ImageData {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeKind {
    Rectangle,
    Circle,
    TextLine,
    Path(Vec<(f32, f32)>), // Centerline or Vector path
    RasterImage { data: ImageData, params: RasterParams },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapeParams {
    pub shape: ShapeKind,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub radius: f32,
    pub layer_idx: usize,
    pub text: String,
    pub font_size_mm: f32,
    pub rotation: f32, // Degrees
}
impl ShapeParams {
    pub fn world_pos(&self, lx: f32, ly: f32) -> (f32, f32) {
        let angle = self.rotation.to_radians();
        let rx = lx * angle.cos() - ly * angle.sin();
        let ry = lx * angle.sin() + ly * angle.cos();
        (self.x + rx, self.y + ry)
    }

    pub fn local_center(&self) -> (f32, f32) {
        match &self.shape {
            ShapeKind::Rectangle => (self.width / 2.0, self.height / 2.0),
            ShapeKind::Circle => (0.0, 0.0), // Circles are anchored at center
            ShapeKind::TextLine => {
                let char_w = self.font_size_mm * 0.6;
                let w = self.text.len() as f32 * char_w;
                (w / 2.0, self.font_size_mm / 2.0)
            }
            ShapeKind::Path(pts) => {
                if pts.is_empty() { return (0.0, 0.0); }
                let mut min_x: f32 = f32::MAX; let mut max_x: f32 = f32::MIN;
                let mut min_y: f32 = f32::MAX; let mut max_y: f32 = f32::MIN;
                for p in pts {
                    min_x = min_x.min(p.0); max_x = max_x.max(p.0);
                    min_y = min_y.min(p.1); max_y = max_y.max(p.1);
                }
                ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0)
            }
            ShapeKind::RasterImage { params, .. } => (params.width_mm / 2.0, params.height_mm / 2.0),
        }
    }
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
            layer_idx: 0,
            text: "Hello".into(),
            font_size_mm: 10.0,
            rotation: 0.0,
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

pub fn show(ui: &mut Ui, state: &mut DrawingState, layers: &[CutLayer], active_layer_idx: usize) -> DrawingAction {
    let mut action = DrawingAction { generate_gcode: None };

    ui.group(|ui| {
        ui.label(RichText::new(format!("✏ {}", crate::i18n::tr("Drawing Tools"))).color(theme::LAVENDER).strong());
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

        match &state.current.shape {
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
            ShapeKind::Path(pts) => {
                ui.label(format!("Path with {} points", pts.len()));
            }
            ShapeKind::RasterImage { params, .. } => {
                ui.label(format!("Image: {}x{} mm", params.width_mm, params.height_mm));
            }
        }

        ui.add_space(4.0);

        // Layer Selector
        ui.horizontal(|ui| {
             ui.label("Layer:");
             // Simple integer drag for now, could be a combobox
             if ui.add(egui::DragValue::new(&mut state.current.layer_idx).range(0..=29)).changed() {
                 // Clamp handled by drag value range
             }
             if let Some(l) = layers.get(state.current.layer_idx) {
                 let (rect, _) = ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
                 ui.painter().rect_filled(rect, 2.0, l.color);
             }
        });

        // Auto-update layer index if no shapes present and user changes active layer elsewhere?
        // Or just let user set it manually. Manual is safer for now.
        // However, standard UX is "new objects take active layer".
        // We can do this: if state.current.layer_idx != active_layer_idx AND shapes is empty, sync?
        // Let's just provide a button "Use Active Layer"
        if ui.button("Set to Active Layer").clicked() {
            state.current.layer_idx = active_layer_idx;
        }


        ui.add_space(4.0);

        ui.horizontal(|ui| {
            if ui.button(RichText::new("➕ Add Shape").color(theme::GREEN).strong()).clicked() {
                state.shapes.push(state.current.clone());
                let lines = generate_all_gcode(state, layers);
                action.generate_gcode = Some(lines);
            }
            if ui.button("⮪ Undo").clicked() {
                state.shapes.pop();
                let lines = generate_all_gcode(state, layers);
                action.generate_gcode = Some(lines);
            }
            if ui.button("🗑 Clear").clicked() {
                state.shapes.clear();
                let lines = generate_all_gcode(state, layers);
                action.generate_gcode = Some(lines);
            }
        });
        
        if !state.shapes.is_empty() {
            ui.label(RichText::new(format!("{} shapes in drawing", state.shapes.len())).small().color(theme::SUBTEXT));
        }
    });

    action
}

pub fn generate_all_gcode(state: &DrawingState, layers: &[CutLayer]) -> Vec<String> {
    let mut builder = GCodeBuilder::new();

    builder.comment("Compiled Drawing — All4Laser");
    builder.raw("G90");
    builder.raw("G21");

    // Create a default layer fallback once, outside the loop
    let default_layer = if !layers.is_empty() {
        layers[0].clone()
    } else {
        // Fallback if empty (shouldn't happen with default_palette)
        let mut l = CutLayer::default_palette()[0].clone();
        l.color = egui::Color32::WHITE;
        l
    };

    for (idx, shape) in state.shapes.iter().enumerate() {
        // Retrieve layer settings
        let layer = layers.get(shape.layer_idx).unwrap_or(&default_layer);

        if !layer.visible {
            continue;
        }

        builder.comment(&format!("Shape {}: {:?} [Layer C{:02}]", idx + 1, shape.shape, layer.id));

        // Apply Z-offset if needed (simple implementation: move Z before start)
        if layer.z_offset != 0.0 {
            builder.raw(&format!("G0 Z{:.2}", layer.z_offset));
        }

        // Air Assist
        if layer.air_assist {
             builder.raw("M8");
        }

        for pass in 0..layer.passes {
            if layer.passes > 1 { builder.comment(&format!("Pass {}", pass + 1)); }

            // Check for Fill mode
            if layer.mode == CutMode::Fill || layer.mode == CutMode::FillAndLine {
                let mut temp_lines = Vec::new();
                crate::gcode::fill::generate_fill(&mut temp_lines, shape, layer, 0.1);
                // Ingest lines into builder (or ideally generate_fill should accept builder, but for now we mix)
                // Actually, since generate_fill uses its own builder and returns lines, let's just append.
                // But GCodeBuilder tracks state. We should probably reset state or make generate_fill use *our* builder.
                // For this refactor, let's just dump the strings and reset builder state to unknown.
                builder.lines.extend(temp_lines);
                builder.reset_state(); // Because generate_fill might have left laser on or moved without us knowing
            }

            if layer.mode == CutMode::Line || layer.mode == CutMode::FillAndLine {
                match &shape.shape {
                    ShapeKind::Rectangle => gen_rect(&mut builder, shape, layer),
                    ShapeKind::Circle => gen_circle(&mut builder, shape, layer),
                    ShapeKind::TextLine => gen_text(&mut builder, shape, layer),
                    ShapeKind::Path(pts) => gen_path(&mut builder, pts, shape, layer),
                    ShapeKind::RasterImage { data, params } => gen_raster(&mut builder, data, params, shape),
                }
            }
        }

        if layer.air_assist {
            builder.raw("M9");
        }
    }

    builder.laser_off();
    builder.rapid(0.0, 0.0);

    builder.finish()
}

fn gen_rect(builder: &mut GCodeBuilder, s: &ShapeParams, layer: &CutLayer) {
    let (x0, y0) = (0.0, 0.0);
    let (x1, y1) = (s.width, s.height);
    let pts = vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1), (x0, y0)];
    let path: Vec<(f32, f32)> = pts.into_iter().map(|p| rotate_point(p.0, p.1, s)).collect();
    crate::gcode::path_utils::apply_tabs(builder, &path, layer);
}

fn gen_circle(builder: &mut GCodeBuilder, s: &ShapeParams, layer: &CutLayer) {
    use std::f32::consts::PI;
    let r = s.radius;
    let steps = 64;

    let mut pts = Vec::with_capacity(steps + 1);
    for i in 0..=steps {
        let angle = 2.0 * PI * (i as f32) / (steps as f32);
        let px = r * angle.cos();
        let py = r * angle.sin();
        pts.push((px, py));
    }
    let path: Vec<(f32, f32)> = pts.into_iter().map(|p| rotate_point(p.0, p.1, s)).collect();
    crate::gcode::path_utils::apply_tabs(builder, &path, layer);
}

fn gen_path(builder: &mut GCodeBuilder, points: &[(f32, f32)], s: &ShapeParams, layer: &CutLayer) {
    if points.is_empty() { return; }

    let abs_path: Vec<(f32, f32)> = points.iter()
        .map(|p| rotate_point(p.0, p.1, s))
        .collect();

    crate::gcode::path_utils::apply_tabs(builder, &abs_path, layer);
}

fn gen_text(builder: &mut GCodeBuilder, s: &ShapeParams, layer: &CutLayer) {
    let char_w = s.font_size_mm * 0.6;
    let char_h = s.font_size_mm;
    let sp = layer.speed;
    let pw = layer.power;
    let mut cursor_x = 0.0; // Local X

    for ch in s.text.chars() {
        let strokes = get_char_strokes(ch, cursor_x, 0.0, char_w, char_h);
        for (lx0, ly0, lx1, ly1) in strokes {
            let (x0, y0) = rotate_point(lx0, ly0, s);
            let (x1, y1) = rotate_point(lx1, ly1, s);
            builder.laser_off();
            builder.rapid(x0, y0);
            builder.linear(x1, y1, sp, pw);
        }
        cursor_x += char_w + (s.font_size_mm * 0.1);
    }
    builder.laser_off();
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

fn gen_raster(builder: &mut GCodeBuilder, img_data: &ImageData, params: &RasterParams, s: &ShapeParams) {
    let processed = crate::imaging::raster::preprocess_image(&img_data.0, params);
    let gray = processed.to_luma8();
    
    let target_w = (params.width_mm * params.dpi / 25.4) as u32;
    let target_h = (params.height_mm * params.dpi / 25.4) as u32;

    let resized = ::image::imageops::resize(
        &gray, target_w, target_h,
        ::image::imageops::FilterType::Lanczos3,
    );

    let (rw, rh) = resized.dimensions();
    let x_scale = params.width_mm / rw as f32;
    let y_scale = params.height_mm / rh as f32;

    builder.laser_off();
    builder.raw("M4"); // Dynamic power

    for row in 0..rh {
        let ly = (rh - 1 - row) as f32 * y_scale;
        let reverse = row % 2 == 1;

        let cols: Vec<u32> = if reverse {
            (0..rw).rev().collect()
        } else {
            (0..rw).collect()
        };

        let mut first = true;
        for col in cols {
            let pixel = resized.get_pixel(col, row)[0];
            let power = ((255 - pixel) as f32 / 255.0 * params.max_power) as u32;
            let lx = col as f32 * x_scale;
            
            let (wx, wy) = rotate_point(lx, ly, s);
            
            if first {
                builder.laser_off();
                builder.rapid(wx, wy);
                builder.linear(wx, wy, params.max_speed, power as f32);
                first = false;
            } else {
                builder.linear(wx, wy, params.max_speed, power as f32);
            }
        }
    }
    builder.laser_off();
}

fn rotate_point(lx: f32, ly: f32, s: &ShapeParams) -> (f32, f32) {
    let angle = s.rotation.to_radians();
    let rx = lx * angle.cos() - ly * angle.sin();
    let ry = lx * angle.sin() + ly * angle.cos();
    (s.x + rx, s.y + ry)
}
