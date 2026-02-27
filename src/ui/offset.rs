use egui::{Ui, RichText};
use crate::theme;
use crate::ui::drawing::{ShapeParams, ShapeKind, DrawingState};
use geo::{LineString, Polygon};

#[derive(Clone, Debug)]
pub struct OffsetState {
    pub is_open: bool,
    pub distance: f32,
    pub join_style: JoinStyle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JoinStyle {
    Round,
    Miter,
    Bevel,
}

impl Default for OffsetState {
    fn default() -> Self {
        Self {
            is_open: false,
            distance: 2.0,
            join_style: JoinStyle::Round,
        }
    }
}

pub struct OffsetAction {
    pub apply: bool,
}

pub fn show(ctx: &egui::Context, state: &mut OffsetState) -> OffsetAction {
    let mut action = OffsetAction { apply: false };

    if !state.is_open {
        return action;
    }

    egui::Window::new("📐 Offset Path")
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Distance:");
                ui.add(egui::DragValue::new(&mut state.distance).speed(0.1).suffix(" mm"));
            });

            ui.horizontal(|ui| {
                ui.label("Style:");
                ui.selectable_value(&mut state.join_style, JoinStyle::Round, "Round");
                ui.selectable_value(&mut state.join_style, JoinStyle::Miter, "Miter");
                ui.selectable_value(&mut state.join_style, JoinStyle::Bevel, "Bevel");
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button(RichText::new("📐 Create Offset").color(theme::GREEN).strong()).clicked() {
                    action.apply = true;
                }
                if ui.button("Close").clicked() {
                    state.is_open = false;
                }
            });
        });

    action
}

pub fn apply_offset(state: &OffsetState, drawing: &mut DrawingState, selected_indices: &[usize]) {
    if selected_indices.is_empty() {
        return;
    }

    let mut new_shapes = Vec::new();

    for &idx in selected_indices {
        if let Some(shape) = drawing.shapes.get(idx) {
            if let Some(poly) = shape_to_polygon(shape) {
                use geo::Buffer;
                use geo::algorithm::buffer::{BufferStyle, LineJoin};

                let dist = state.distance as f64;
                let join = match state.join_style {
                    JoinStyle::Round => LineJoin::Round(0.1), // approximation angle
                    JoinStyle::Miter => LineJoin::Miter(1.0), // miter limit
                    JoinStyle::Bevel => LineJoin::Bevel,
                };

                let style = BufferStyle::new(dist).line_join(join);
                let offset_multi_poly = poly.buffer_with_style(style);
                
                // Convert back to ShapeParams (Path)
                for p in offset_multi_poly.0 {
                    let exterior = p.exterior();
                    let points: Vec<(f32, f32)> = exterior.coords().map(|c| (c.x as f32, c.y as f32)).collect();
                    if !points.is_empty() {
                        let mut new_shape = shape.clone();
                        new_shape.shape = ShapeKind::Path(points);
                        new_shape.x = 0.0;
                        new_shape.y = 0.0;
                        new_shapes.push(new_shape);
                    }
                }
            }
        }
    }

    drawing.shapes.extend(new_shapes);
}

pub fn shape_to_polygon(s: &ShapeParams) -> Option<Polygon<f64>> {
    let angle = (s.rotation as f64).to_radians();
    let rotate = |lx: f64, ly: f64| -> (f64, f64) {
        let rx = lx * angle.cos() - ly * angle.sin();
        let ry = lx * angle.sin() + ly * angle.cos();
        (s.x as f64 + rx, s.y as f64 + ry)
    };

    match &s.shape {
        ShapeKind::Rectangle => {
            let pts = vec![(0.0, 0.0), (s.width as f64, 0.0), (s.width as f64, s.height as f64), (0.0, s.height as f64), (0.0, 0.0)];
            let points: Vec<(f64, f64)> = pts.into_iter().map(|(lx, ly)| rotate(lx, ly)).collect();
            Some(Polygon::new(LineString::from(points), vec![]))
        }
        ShapeKind::Circle => {
            use std::f64::consts::PI;
            let r = s.radius as f64;
            let steps = 64;
            let mut points = Vec::with_capacity(steps + 1);
            for i in 0..=steps {
                let angle = 2.0 * PI * (i as f64) / (steps as f64);
                points.push(rotate(r * angle.cos(), r * angle.sin()));
            }
            Some(Polygon::new(LineString::from(points), vec![]))
        }
        ShapeKind::Path(pts) => {
            let points: Vec<(f64, f64)> = pts.iter().map(|p| rotate(p.0 as f64, p.1 as f64)).collect();
            if points.len() < 3 { return None; }
            Some(Polygon::new(LineString::from(points), vec![]))
        }
        _ => None,
    }
}
