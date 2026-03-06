use crate::theme;
use crate::ui::drawing::DrawingState;
use egui::RichText;

/// Grid Array: Repeat selected shapes in a grid pattern

#[derive(Clone, Debug)]
pub struct GridArrayState {
    pub is_open: bool,
    pub columns: u32,
    pub rows: u32,
    pub dx: f32, // x spacing (mm)
    pub dy: f32, // y spacing (mm)
}

impl Default for GridArrayState {
    fn default() -> Self {
        Self {
            is_open: false,
            columns: 2,
            rows: 2,
            dx: 10.0,
            dy: 10.0,
        }
    }
}

pub struct GridArrayAction {
    pub apply: bool,
}

pub fn show(ctx: &egui::Context, state: &mut GridArrayState) -> GridArrayAction {
    let mut action = GridArrayAction { apply: false };

    if !state.is_open {
        return action;
    }

    egui::Window::new("🔲 Grid Array")
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            egui::Grid::new("grid_array_grid")
                .num_columns(2)
                .spacing([12.0, 6.0])
                .show(ui, |ui| {
                    ui.label("X Columns:");
                    ui.add(egui::DragValue::new(&mut state.columns).range(1..=100));
                    ui.end_row();

                    ui.label("Y Rows:");
                    ui.add(egui::DragValue::new(&mut state.rows).range(1..=100));
                    ui.end_row();

                    ui.label("X Spacing (mm):");
                    ui.add(
                        egui::DragValue::new(&mut state.dx)
                            .speed(1.0)
                            .suffix(" mm"),
                    );
                    ui.end_row();

                    ui.label("Y Spacing (mm):");
                    ui.add(
                        egui::DragValue::new(&mut state.dy)
                            .speed(1.0)
                            .suffix(" mm"),
                    );
                    ui.end_row();
                });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui
                    .button(
                        RichText::new("🔲 Create Array")
                            .color(theme::GREEN)
                            .strong(),
                    )
                    .clicked()
                {
                    action.apply = true;
                }
                if ui.button("Close").clicked() {
                    state.is_open = false;
                }
            });
        });

    action
}

pub fn apply_array(
    state: &GridArrayState,
    drawing: &mut DrawingState,
    selected_indices: &[usize],
) {
    if selected_indices.is_empty() || (state.columns == 1 && state.rows == 1) {
        return;
    }

    let mut new_shapes = Vec::new();

    for &idx in selected_indices {
        if let Some(base_shape) = drawing.shapes.get(idx) {
            for col in 0..state.columns {
                for row in 0..state.rows {
                    if col == 0 && row == 0 {
                        continue; // Skip original
                    }

                    let mut new_shape = base_shape.clone();
                    new_shape.x += (col as f32) * state.dx;
                    new_shape.y += (row as f32) * state.dy;
                    new_shapes.push(new_shape);
                }
            }
        }
    }

    drawing.shapes.extend(new_shapes);
}
