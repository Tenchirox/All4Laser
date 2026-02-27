use egui::{Ui, RichText};
use crate::theme;
use crate::ui::drawing::{ShapeParams, ShapeKind, DrawingState};
use geo::{Polygon, MultiPolygon, BooleanOps};

#[derive(Clone, Debug)]
pub struct BooleanOpsState {
    pub is_open: bool,
    pub op: BooleanOp,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BooleanOp {
    Union,
    Subtract,
    Intersect,
    Xor,
}

impl Default for BooleanOpsState {
    fn default() -> Self {
        Self {
            is_open: false,
            op: BooleanOp::Union,
        }
    }
}

pub struct BooleanAction {
    pub apply: bool,
}

pub fn show(ctx: &egui::Context, state: &mut BooleanOpsState) -> BooleanAction {
    let mut action = BooleanAction { apply: false };

    if !state.is_open {
        return action;
    }

    egui::Window::new("🧩 Boolean Operations")
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut state.op, BooleanOp::Union, "Union (Combine)");
                ui.selectable_value(&mut state.op, BooleanOp::Subtract, "Subtract (A - B)");
                ui.selectable_value(&mut state.op, BooleanOp::Intersect, "Intersection");
                ui.selectable_value(&mut state.op, BooleanOp::Xor, "XOR (Symmetric Difference)");
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button(RichText::new("🧩 Apply").color(theme::GREEN).strong()).clicked() {
                    action.apply = true;
                }
                if ui.button("Close").clicked() {
                    state.is_open = false;
                }
            });
        });

    action
}

pub fn apply_boolean(state: &BooleanOpsState, drawing: &mut DrawingState, selected_indices: &[usize]) {
    if selected_indices.len() < 2 {
        return;
    }

    // Convert selected shapes to polygons
    let mut polys: Vec<MultiPolygon<f64>> = Vec::new();
    let mut indices_to_remove = Vec::new();

    for &idx in selected_indices {
        if let Some(shape) = drawing.shapes.get(idx) {
            if let Some(p) = super::offset::shape_to_polygon(shape) {
                polys.push(MultiPolygon::new(vec![p]));
                indices_to_remove.push(idx);
            }
        }
    }

    if polys.len() < 2 {
        return;
    }

    let result: MultiPolygon<f64> = match state.op {
        BooleanOp::Union => {
            let mut res = polys[0].clone();
            for p in &polys[1..] {
                res = res.union(p);
            }
            res
        }
        BooleanOp::Subtract => {
            let mut res = polys[0].clone();
            for p in &polys[1..] {
                res = res.difference(p);
            }
            res
        }
        BooleanOp::Intersect => {
            let mut res = polys[0].clone();
            for p in &polys[1..] {
                res = res.intersection(p);
            }
            res
        }
        BooleanOp::Xor => {
            let mut res = polys[0].clone();
            for p in &polys[1..] {
                res = res.xor(p);
            }
            res
        }
    };

    // Remove old shapes (reverse order to avoid index issues)
    let mut sorted_indices = indices_to_remove;
    sorted_indices.sort_unstable_by(|a, b| b.cmp(a));
    for idx in sorted_indices {
        drawing.shapes.remove(idx);
    }

    // Add new shapes from the result
    for poly in result.0 {
        let exterior = poly.exterior();
        let points: Vec<(f32, f32)> = exterior.coords().map(|c| (c.x as f32, c.y as f32)).collect();
        if !points.is_empty() {
            let mut new_shape = ShapeParams::default();
            new_shape.shape = ShapeKind::Path(points);
            new_shape.x = 0.0;
            new_shape.y = 0.0;
            drawing.shapes.push(new_shape);
        }
        
        // Handle holes (interiors)
        for interior in poly.interiors() {
            let points: Vec<(f32, f32)> = interior.coords().map(|c| (c.x as f32, c.y as f32)).collect();
            if !points.is_empty() {
                let mut hole_shape = ShapeParams::default();
                hole_shape.shape = ShapeKind::Path(points);
                hole_shape.x = 0.0;
                hole_shape.y = 0.0;
                // Currently our drawing state doesn't have a "hole" concept explicitly in G-code gen,
                // but a Path will just be cut. If it's inside, it's a hole.
                drawing.shapes.push(hole_shape);
            }
        }
    }
}
