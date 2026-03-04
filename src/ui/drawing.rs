/// Drawing Tools: Rectangle, Circle, and Text generators producing GCode directly

use egui::{Ui, RichText};
use crate::theme;
use crate::ui::layers_new::{CutLayer, CutMode};
use crate::gcode::generator::GCodeBuilder;
use std::collections::HashSet;
use std::sync::Arc;
use crate::imaging::raster::RasterParams;
use geo::Buffer;
use geo::algorithm::buffer::{BufferStyle, LineJoin};
use geo::LineString;

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
    pub group_id: Option<u32>, // Group ID for grouping (F51)
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
            group_id: None,
        }
    }
}

/// Bounding box of a shape in world coordinates (F39 helper)
fn shape_world_bounds(s: &ShapeParams) -> (f32, f32, f32, f32) {
    match &s.shape {
        ShapeKind::Rectangle => {
            let corners = [(0.0, 0.0), (s.width, 0.0), (s.width, s.height), (0.0, s.height)];
            let world: Vec<(f32, f32)> = corners.iter().map(|&(lx, ly)| s.world_pos(lx, ly)).collect();
            let min_x = world.iter().map(|p| p.0).fold(f32::MAX, f32::min);
            let max_x = world.iter().map(|p| p.0).fold(f32::MIN, f32::max);
            let min_y = world.iter().map(|p| p.1).fold(f32::MAX, f32::min);
            let max_y = world.iter().map(|p| p.1).fold(f32::MIN, f32::max);
            (min_x, min_y, max_x, max_y)
        }
        ShapeKind::Circle => (s.x - s.radius, s.y - s.radius, s.x + s.radius, s.y + s.radius),
        ShapeKind::Path(pts) => {
            let mut min_x = f32::MAX; let mut max_x = f32::MIN;
            let mut min_y = f32::MAX; let mut max_y = f32::MIN;
            for p in pts {
                let (wx, wy) = s.world_pos(p.0, p.1);
                min_x = min_x.min(wx); max_x = max_x.max(wx);
                min_y = min_y.min(wy); max_y = max_y.max(wy);
            }
            if min_x > max_x { return (s.x, s.y, s.x, s.y); }
            (min_x, min_y, max_x, max_y)
        }
        _ => {
            let (cx, cy) = s.local_center();
            let (wx, wy) = s.world_pos(cx, cy);
            (wx - 5.0, wy - 5.0, wx + 5.0, wy + 5.0)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlignOp {
    Left, Right, Top, Bottom, CenterH, CenterV, DistributeH, DistributeV,
}

/// Align/distribute selected shapes (F39)
pub fn align_shapes(shapes: &mut [ShapeParams], selection: &[usize], op: AlignOp) {
    if selection.len() < 2 && !matches!(op, AlignOp::Left | AlignOp::Right | AlignOp::Top | AlignOp::Bottom | AlignOp::CenterH | AlignOp::CenterV) {
        return;
    }
    if selection.is_empty() { return; }

    let bounds: Vec<(usize, f32, f32, f32, f32)> = selection.iter()
        .filter_map(|&i| shapes.get(i).map(|s| { let b = shape_world_bounds(s); (i, b.0, b.1, b.2, b.3) }))
        .collect();
    if bounds.is_empty() { return; }

    match op {
        AlignOp::Left => {
            let target = bounds.iter().map(|b| b.1).fold(f32::MAX, f32::min);
            for &(i, min_x, _, _, _) in &bounds {
                shapes[i].x += target - min_x;
            }
        }
        AlignOp::Right => {
            let target = bounds.iter().map(|b| b.3).fold(f32::MIN, f32::max);
            for &(i, _, _, max_x, _) in &bounds {
                shapes[i].x += target - max_x;
            }
        }
        AlignOp::Top => {
            let target = bounds.iter().map(|b| b.1 + 0.0).fold(f32::MAX, f32::min);
            let target_y = bounds.iter().map(|b| b.2).fold(f32::MAX, f32::min);
            for &(i, _, min_y, _, _) in &bounds {
                shapes[i].y += target_y - min_y;
            }
        }
        AlignOp::Bottom => {
            let target = bounds.iter().map(|b| b.4).fold(f32::MIN, f32::max);
            for &(i, _, _, _, max_y) in &bounds {
                shapes[i].y += target - max_y;
            }
        }
        AlignOp::CenterH => {
            let sum: f32 = bounds.iter().map(|b| (b.1 + b.3) / 2.0).sum();
            let center = sum / bounds.len() as f32;
            for &(i, min_x, _, max_x, _) in &bounds {
                let cx = (min_x + max_x) / 2.0;
                shapes[i].x += center - cx;
            }
        }
        AlignOp::CenterV => {
            let sum: f32 = bounds.iter().map(|b| (b.2 + b.4) / 2.0).sum();
            let center = sum / bounds.len() as f32;
            for &(i, _, min_y, _, max_y) in &bounds {
                let cy = (min_y + max_y) / 2.0;
                shapes[i].y += center - cy;
            }
        }
        AlignOp::DistributeH => {
            if bounds.len() < 3 { return; }
            let mut sorted = bounds.clone();
            sorted.sort_by(|a, b| ((a.1 + a.3) / 2.0).partial_cmp(&((b.1 + b.3) / 2.0)).unwrap());
            let first_cx = (sorted[0].1 + sorted[0].3) / 2.0;
            let last_cx = (sorted.last().unwrap().1 + sorted.last().unwrap().3) / 2.0;
            let step = (last_cx - first_cx) / (sorted.len() - 1) as f32;
            for (j, &(i, min_x, _, max_x, _)) in sorted.iter().enumerate() {
                let cx = (min_x + max_x) / 2.0;
                let target = first_cx + step * j as f32;
                shapes[i].x += target - cx;
            }
        }
        AlignOp::DistributeV => {
            if bounds.len() < 3 { return; }
            let mut sorted = bounds.clone();
            sorted.sort_by(|a, b| ((a.2 + a.4) / 2.0).partial_cmp(&((b.2 + b.4) / 2.0)).unwrap());
            let first_cy = (sorted[0].2 + sorted[0].4) / 2.0;
            let last_cy = (sorted.last().unwrap().2 + sorted.last().unwrap().4) / 2.0;
            let step = (last_cy - first_cy) / (sorted.len() - 1) as f32;
            for (j, &(i, _, min_y, _, max_y)) in sorted.iter().enumerate() {
                let cy = (min_y + max_y) / 2.0;
                let target = first_cy + step * j as f32;
                shapes[i].y += target - cy;
            }
        }
    }
}

/// Public wrapper for shape_world_bounds (F59)
pub fn shape_world_bounds_pub(s: &ShapeParams) -> (f32, f32, f32, f32) {
    shape_world_bounds(s)
}

/// Group selected shapes under a new group ID (F51)
pub fn group_shapes(shapes: &mut [ShapeParams], selection: &[usize]) -> Option<u32> {
    if selection.len() < 2 { return None; }
    // Generate a unique group ID based on current max
    let max_id = shapes.iter()
        .filter_map(|s| s.group_id)
        .max()
        .unwrap_or(0);
    let new_id = max_id + 1;
    for &idx in selection {
        if let Some(s) = shapes.get_mut(idx) {
            s.group_id = Some(new_id);
        }
    }
    Some(new_id)
}

/// Ungroup shapes that share the same group ID (F51)
pub fn ungroup_shapes(shapes: &mut [ShapeParams], selection: &[usize]) -> usize {
    let mut ungrouped = 0;
    // Collect group IDs from selection
    let group_ids: Vec<u32> = selection.iter()
        .filter_map(|&idx| shapes.get(idx).and_then(|s| s.group_id))
        .collect();
    for s in shapes.iter_mut() {
        if let Some(gid) = s.group_id {
            if group_ids.contains(&gid) {
                s.group_id = None;
                ungrouped += 1;
            }
        }
    }
    ungrouped
}

/// Get all shape indices that share a group with the given shape (F51)
pub fn expand_group_selection(shapes: &[ShapeParams], idx: usize) -> Vec<usize> {
    let Some(gid) = shapes.get(idx).and_then(|s| s.group_id) else {
        return vec![idx];
    };
    shapes.iter().enumerate()
        .filter(|(_, s)| s.group_id == Some(gid))
        .map(|(i, _)| i)
        .collect()
}

/// Export shapes to SVG string (F54)
pub fn export_shapes_to_svg(shapes: &[ShapeParams], layers: &[CutLayer]) -> String {
    if shapes.is_empty() {
        return String::from(r#"<?xml version="1.0" encoding="UTF-8"?><svg xmlns="http://www.w3.org/2000/svg"/>"#);
    }
    // Compute global bounds
    let mut gmin_x = f32::MAX; let mut gmin_y = f32::MAX;
    let mut gmax_x = f32::MIN; let mut gmax_y = f32::MIN;
    for s in shapes {
        let (a, b, c, d) = shape_world_bounds(s);
        gmin_x = gmin_x.min(a); gmin_y = gmin_y.min(b);
        gmax_x = gmax_x.max(c); gmax_y = gmax_y.max(d);
    }
    let w = (gmax_x - gmin_x).max(1.0);
    let h = (gmax_y - gmin_y).max(1.0);

    let mut svg = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{w:.3}mm" height="{h:.3}mm" viewBox="{} {} {w:.3} {h:.3}">
"#, gmin_x, gmin_y
    );

    for s in shapes {
        let color = layers.get(s.layer_idx).map(|l| l.color).unwrap_or(egui::Color32::BLACK);
        let hex = format!("#{:02x}{:02x}{:02x}", color.r(), color.g(), color.b());
        match &s.shape {
            ShapeKind::Rectangle => {
                let pts = [(0.0,0.0),(s.width,0.0),(s.width,s.height),(0.0,s.height),(0.0,0.0)];
                let d: Vec<String> = pts.iter().enumerate().map(|(i,(lx,ly))| {
                    let (wx,wy) = s.world_pos(*lx,*ly);
                    if i==0 { format!("M{wx:.3},{wy:.3}") } else { format!("L{wx:.3},{wy:.3}") }
                }).collect();
                svg += &format!(r#"  <path d="{}" fill="none" stroke="{hex}" stroke-width="0.1"/>"#, d.join(" "));
                svg += "\n";
            }
            ShapeKind::Circle => {
                svg += &format!(r#"  <circle cx="{:.3}" cy="{:.3}" r="{:.3}" fill="none" stroke="{hex}" stroke-width="0.1"/>"#, s.x, s.y, s.radius);
                svg += "\n";
            }
            ShapeKind::Path(pts) if pts.len() >= 2 => {
                let d: Vec<String> = pts.iter().enumerate().map(|(i,p)| {
                    let (wx,wy) = s.world_pos(p.0,p.1);
                    if i==0 { format!("M{wx:.3},{wy:.3}") } else { format!("L{wx:.3},{wy:.3}") }
                }).collect();
                svg += &format!(r#"  <path d="{}" fill="none" stroke="{hex}" stroke-width="0.1"/>"#, d.join(" "));
                svg += "\n";
            }
            _ => {}
        }
    }
    svg += "</svg>\n";
    svg
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

    if state.current.shape == ShapeKind::TextLine {
        state.current.shape = ShapeKind::Rectangle;
    }

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
                ui.label("Use the Text Tool panel below to create text paths.");
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
    builder.comment("");

    // Create a default layer fallback once, outside the loop
    let default_layer = if !layers.is_empty() {
        layers[0].clone()
    } else {
        // Fallback if empty (shouldn't happen with default_palette)
        let mut l = CutLayer::default_palette()[0].clone();
        l.color = egui::Color32::WHITE;
        l
    };

    // LightBurn-like ordering: lower layer output_order first, then layer id, then insertion order
    let mut ordered_indices: Vec<usize> = (0..state.shapes.len()).collect();
    ordered_indices.sort_by_key(|&shape_idx| {
        let layer_idx = state.shapes[shape_idx].layer_idx;
        let layer = layers.get(layer_idx).unwrap_or(&default_layer);
        (layer.output_order, layer.id as i32, shape_idx as i32)
    });

    let mut seen_layers: HashSet<usize> = HashSet::new();
    let mut ordered_layers: Vec<usize> = Vec::new();
    for &shape_idx in &ordered_indices {
        let layer_idx = state.shapes[shape_idx].layer_idx;
        if seen_layers.insert(layer_idx) {
            ordered_layers.push(layer_idx);
        }
    }

    for layer_idx in ordered_layers {
        let layer = layers.get(layer_idx).unwrap_or(&default_layer);
        if !layer.visible || layer.is_construction {
            continue;
        }

        let layer_shape_indices: Vec<usize> = ordered_indices
            .iter()
            .copied()
            .filter(|&idx| state.shapes[idx].layer_idx == layer_idx)
            .collect();
        if layer_shape_indices.is_empty() {
            continue;
        }

        builder.comment(&format!("Layer C{:02} ({}) — Speed:{:.0} Power:{:.0} Passes:{} Mode:{:?}{}",
            layer.id, layer.name, layer.speed, layer.power, layer.passes, layer.mode,
            if layer.air_assist { " AirAssist:ON" } else { "" }
        ));

        // Apply Z-offset if needed (simple implementation: move Z before layer start)
        if layer.z_offset != 0.0 {
            builder.raw(&format!("G0 Z{:.2}", layer.z_offset));
        }

        if layer.air_assist {
            builder.raw("M8");
        }
        if layer.exhaust_enabled {
            builder.raw("M7"); // Exhaust fan on (F77)
        }

        for pass in 0..layer.passes {
            if layer.passes > 1 {
                builder.comment(&format!("Pass {}", pass + 1));
            }

            if matches!(layer.mode, CutMode::Fill | CutMode::FillAndLine | CutMode::Offset) {
                let layer_shapes: Vec<&ShapeParams> = layer_shape_indices
                    .iter()
                    .map(|&idx| &state.shapes[idx])
                    .collect();

                let mut temp_lines = Vec::new();
                crate::gcode::fill::generate_fill_group(&mut temp_lines, &layer_shapes, layer);
                builder.lines.extend(temp_lines);
                // `generate_fill_group` uses its own builder; reset our tracking state after merging lines.
                builder.reset_state();
            }

            if matches!(layer.mode, CutMode::Line | CutMode::FillAndLine) {
                for &shape_idx in &layer_shape_indices {
                    let shape = &state.shapes[shape_idx];
                    builder.comment(&format!("Shape {}: {:?} [Layer C{:02}]", shape_idx + 1, shape.shape, layer.id));

                    match &shape.shape {
                        ShapeKind::Rectangle => gen_rect(&mut builder, shape, layer),
                        ShapeKind::Circle => gen_circle(&mut builder, shape, layer),
                        ShapeKind::TextLine => gen_text(&mut builder, shape, layer),
                        ShapeKind::Path(pts) => gen_path(&mut builder, pts, shape, layer),
                        ShapeKind::RasterImage { data, params } => gen_raster(&mut builder, data, params, shape),
                    }
                }
            }
        }

        if layer.air_assist {
            builder.raw("M9");
        }
        if layer.exhaust_enabled && layer.exhaust_post_delay_s > 0.0 {
            builder.comment(&format!("Exhaust post-delay {:.1}s", layer.exhaust_post_delay_s));
            builder.raw(&format!("G4 P{:.1}", layer.exhaust_post_delay_s));
            builder.raw("M9"); // Exhaust off after delay (F77)
        } else if layer.exhaust_enabled {
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
    gen_layer_path(builder, &path, layer);
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
    gen_layer_path(builder, &path, layer);
}

fn gen_path(builder: &mut GCodeBuilder, points: &[(f32, f32)], s: &ShapeParams, layer: &CutLayer) {
    if points.is_empty() { return; }

    let abs_path: Vec<(f32, f32)> = points.iter()
        .map(|p| rotate_point(p.0, p.1, s))
        .collect();

    gen_layer_path(builder, &abs_path, layer);
}

fn gen_layer_path(builder: &mut GCodeBuilder, path: &[(f32, f32)], layer: &CutLayer) {
    if path.is_empty() {
        return;
    }

    if layer.kerf_mm.abs() > 0.000_1 && path_is_closed(path) {
        if let Some(offset_paths) = kerf_offset_closed_path(path, layer.kerf_mm) {
            for p in offset_paths {
                crate::gcode::path_utils::apply_tabs(builder, &p, layer);
            }
            return;
        }
    }

    crate::gcode::path_utils::apply_tabs(builder, path, layer);
}

fn path_is_closed(path: &[(f32, f32)]) -> bool {
    if path.len() < 3 {
        return false;
    }

    let first = path[0];
    let last = path[path.len() - 1];
    (first.0 - last.0).abs() < 0.01 && (first.1 - last.1).abs() < 0.01
}

fn ensure_closed_path(mut path: Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    if let (Some(first), Some(last)) = (path.first().copied(), path.last().copied()) {
        if (first.0 - last.0).abs() > 0.000_1 || (first.1 - last.1).abs() > 0.000_1 {
            path.push(first);
        }
    }
    path
}

fn kerf_offset_closed_path(path: &[(f32, f32)], kerf_mm: f32) -> Option<Vec<Vec<(f32, f32)>>> {
    if path.len() < 3 {
        return None;
    }

    let closed = ensure_closed_path(path.to_vec());
    if closed.len() < 4 {
        return None;
    }

    let line: LineString<f64> = closed
        .iter()
        .map(|(x, y)| (*x as f64, *y as f64))
        .collect();

    let poly = geo::Polygon::new(line, vec![]);
    let style = BufferStyle::new(kerf_mm as f64).line_join(LineJoin::Round(0.1));
    let buffered = poly.buffer_with_style(style);

    let mut out = Vec::new();

    for p in buffered.0 {
        let mut exterior: Vec<(f32, f32)> = p
            .exterior()
            .coords()
            .map(|c| (c.x as f32, c.y as f32))
            .collect();
        exterior = ensure_closed_path(exterior);
        if exterior.len() >= 4 {
            out.push(exterior);
        }

        for hole in p.interiors() {
            let mut interior: Vec<(f32, f32)> = hole
                .coords()
                .map(|c| (c.x as f32, c.y as f32))
                .collect();
            interior = ensure_closed_path(interior);
            if interior.len() >= 4 {
                out.push(interior);
            }
        }
    }

    if out.is_empty() { None } else { Some(out) }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_axis(line: &str, axis: char) -> Option<f32> {
        line.split_whitespace()
            .find_map(|tok| tok.strip_prefix(axis).and_then(|v| v.parse::<f32>().ok()))
    }

    fn gcode_bounds(lines: &[String]) -> Option<(f32, f32, f32, f32)> {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        let mut any = false;

        for line in lines {
            if !(line.starts_with("G0") || line.starts_with("G1")) {
                continue;
            }
            let Some(x) = parse_axis(line, 'X') else { continue; };
            let Some(y) = parse_axis(line, 'Y') else { continue; };
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            any = true;
        }

        if any {
            Some((min_x, min_y, max_x, max_y))
        } else {
            None
        }
    }

    fn rectangle_state() -> DrawingState {
        let shape = ShapeParams {
            shape: ShapeKind::Rectangle,
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
            layer_idx: 0,
            ..Default::default()
        };

        DrawingState {
            current: ShapeParams::default(),
            shapes: vec![shape],
        }
    }

    #[test]
    fn kerf_offset_expands_closed_cut_geometry() {
        let state = rectangle_state();
        let mut layers = CutLayer::default_palette();
        layers[0].mode = CutMode::Line;
        layers[0].kerf_mm = 1.0;

        let lines = generate_all_gcode(&state, &layers);
        let (min_x, min_y, max_x, max_y) = gcode_bounds(&lines).expect("expected G0/G1 bounds");

        assert!(min_x < -0.5, "expected min_x < -0.5, got {min_x}");
        assert!(min_y < -0.5, "expected min_y < -0.5, got {min_y}");
        assert!(max_x > 10.5, "expected max_x > 10.5, got {max_x}");
        assert!(max_y > 10.5, "expected max_y > 10.5, got {max_y}");
    }

    #[test]
    fn zero_kerf_keeps_original_rectangle_bounds() {
        let state = rectangle_state();
        let mut layers = CutLayer::default_palette();
        layers[0].mode = CutMode::Line;
        layers[0].kerf_mm = 0.0;

        let lines = generate_all_gcode(&state, &layers);
        let (min_x, min_y, max_x, max_y) = gcode_bounds(&lines).expect("expected G0/G1 bounds");

        assert!(min_x >= -0.001, "expected non-negative min_x, got {min_x}");
        assert!(min_y >= -0.001, "expected non-negative min_y, got {min_y}");
        assert!(max_x <= 10.001, "expected max_x near 10, got {max_x}");
        assert!(max_y <= 10.001, "expected max_y near 10, got {max_y}");
    }

    #[test]
    fn fill_and_line_runs_single_fill_before_line_shapes() {
        let shape_a = ShapeParams {
            shape: ShapeKind::Circle,
            x: 0.0,
            y: 0.0,
            radius: 10.0,
            layer_idx: 0,
            ..Default::default()
        };
        let shape_b = ShapeParams {
            shape: ShapeKind::Circle,
            x: 0.0,
            y: 0.0,
            radius: 4.0,
            layer_idx: 0,
            ..Default::default()
        };
        let state = DrawingState {
            current: ShapeParams::default(),
            shapes: vec![shape_a, shape_b],
        };

        let mut layers = CutLayer::default_palette();
        layers[0].mode = CutMode::FillAndLine;
        layers[0].fill_interval_mm = 2.0;

        let lines = generate_all_gcode(&state, &layers);

        let fill_comment_indices: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter_map(|(i, l)| l.starts_with("; Fill Scan").then_some(i))
            .collect();
        assert_eq!(fill_comment_indices.len(), 1, "expected one grouped fill per layer/pass");

        let first_shape_comment = lines
            .iter()
            .position(|l| l.starts_with("; Shape "))
            .expect("expected shape comment");

        assert!(
            fill_comment_indices[0] < first_shape_comment,
            "expected fill scan to run before contour cuts in FillAndLine mode"
        );
    }
}
