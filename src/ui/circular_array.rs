/// Circular Array: Repeat selected shapes in a circular pattern
use egui::{Ui, RichText};
use crate::theme;
use crate::ui::drawing::{ShapeParams, DrawingState};

#[derive(Clone, Debug)]
pub struct CircularArrayState {
    pub is_open: bool,
    pub count: u32,
    pub center_x: f32,
    pub center_y: f32,
    pub total_angle: f32, // degrees
    pub rotate_copies: bool,
}

impl Default for CircularArrayState {
    fn default() -> Self {
        Self {
            is_open: false,
            count: 4,
            center_x: 100.0,
            center_y: 100.0,
            total_angle: 360.0,
            rotate_copies: true,
        }
    }
}

pub struct CircularArrayAction {
    pub apply: bool,
}

pub fn show(ctx: &egui::Context, state: &mut CircularArrayState) -> CircularArrayAction {
    let mut action = CircularArrayAction { apply: false };

    if !state.is_open {
        return action;
    }

    egui::Window::new("🌀 Circular Array")
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            egui::Grid::new("circular_array_grid").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
                ui.label("Count:");
                ui.add(egui::DragValue::new(&mut state.count).range(2..=100));
                ui.end_row();

                ui.label("Center X (mm):");
                ui.add(egui::DragValue::new(&mut state.center_x).speed(1.0).suffix(" mm"));
                ui.end_row();

                ui.label("Center Y (mm):");
                ui.add(egui::DragValue::new(&mut state.center_y).speed(1.0).suffix(" mm"));
                ui.end_row();

                ui.label("Total Angle (°):");
                ui.add(egui::DragValue::new(&mut state.total_angle).speed(1.0).range(-360.0..=360.0).suffix("°"));
                ui.end_row();

                ui.label("Rotate copies:");
                ui.checkbox(&mut state.rotate_copies, "");
                ui.end_row();
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button(RichText::new("🌀 Create Array").color(theme::GREEN).strong()).clicked() {
                    action.apply = true;
                }
                if ui.button("Close").clicked() {
                    state.is_open = false;
                }
            });
        });

    action
}

pub fn apply_array(state: &CircularArrayState, drawing: &mut DrawingState, selected_indices: &[usize]) {
    if selected_indices.is_empty() || state.count < 2 {
        return;
    }

    let mut new_shapes = Vec::new();
    let step_angle = state.total_angle.to_radians() / (state.count as f32);

    for &idx in selected_indices {
        if let Some(base_shape) = drawing.shapes.get(idx) {
            // We skip the first copy (the original itself is at step 0 usually, 
            // but LightBurn adds N *total* copies including the original).
            // Let's create N-1 copies.
            for i in 1..state.count {
                let angle = (i as f32) * step_angle;
                let mut new_shape = base_shape.clone();

                // Rotate position around (center_x, center_y)
                let dx = new_shape.x - state.center_x;
                let dy = new_shape.y - state.center_y;

                let cos_a = angle.cos();
                let sin_a = angle.sin();

                new_shape.x = state.center_x + dx * cos_a - dy * sin_a;
                new_shape.y = state.center_y + dx * sin_a + dy * cos_a;

                // Also rotate the shape itself if requested?
                // Currently our shapes are Axis-Aligned (Rectangle, Circle).
                // If we want rotation, we need to upgrade ShapeParams to support rotation.
                // For now, let's just move the origin.
                
                new_shapes.push(new_shape);
            }
        }
    }

    drawing.shapes.extend(new_shapes);
}
