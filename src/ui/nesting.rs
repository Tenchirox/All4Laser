#![allow(dead_code)]

use std::cmp::Ordering;

use egui::RichText;

use crate::i18n::tr;
use crate::theme;
use crate::ui::drawing::{DrawingState, ShapeKind, ShapeParams};

#[derive(Clone, Debug)]
pub struct NestingState {
    pub is_open: bool,
    pub spacing_mm: f32,
    pub margin_mm: f32,
    pub allow_rotation: bool,
    pub selection_only: bool,
}

impl Default for NestingState {
    fn default() -> Self {
        Self {
            is_open: false,
            spacing_mm: 2.0,
            margin_mm: 5.0,
            allow_rotation: true,
            selection_only: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NestingOptions {
    pub spacing_mm: f32,
    pub margin_mm: f32,
    pub allow_rotation: bool,
    pub selection_only: bool,
}

impl NestingState {
    pub fn options(&self) -> NestingOptions {
        NestingOptions {
            spacing_mm: self.spacing_mm,
            margin_mm: self.margin_mm,
            allow_rotation: self.allow_rotation,
            selection_only: self.selection_only,
        }
    }
}

#[derive(Default)]
pub struct NestingAction {
    pub apply: bool,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct NestingResult {
    pub considered: usize,
    pub placed: usize,
    pub skipped: usize,
    pub used_rotation: usize,
}

#[derive(Clone)]
struct Candidate {
    shape: ShapeParams,
    width: f32,
    height: f32,
    rotated: bool,
}

pub fn show(
    ctx: &egui::Context,
    state: &mut NestingState,
    selected_count: usize,
    total_shapes: usize,
    workspace: egui::Vec2,
) -> NestingAction {
    let mut action = NestingAction::default();

    if !state.is_open {
        return action;
    }

    let mut close_clicked = false;

    egui::Window::new("🧩 Auto Nesting")
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.label(
                RichText::new(format!(
                    "Workspace {:.1} x {:.1} mm",
                    workspace.x.max(0.0),
                    workspace.y.max(0.0)
                ))
                .small()
                .color(theme::SUBTEXT),
            );

            ui.add_space(6.0);
            egui::Grid::new("nesting_grid")
                .num_columns(2)
                .spacing([12.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Spacing (mm):");
                    ui.add(
                        egui::DragValue::new(&mut state.spacing_mm)
                            .speed(0.2)
                            .range(0.0..=50.0)
                            .suffix(" mm"),
                    );
                    ui.end_row();

                    ui.label("Margins (mm):");
                    ui.add(
                        egui::DragValue::new(&mut state.margin_mm)
                            .speed(0.2)
                            .range(0.0..=100.0)
                            .suffix(" mm"),
                    );
                    ui.end_row();

                    ui.label("Allow 90° rotation:");
                    ui.checkbox(&mut state.allow_rotation, "");
                    ui.end_row();

                    ui.label("Selection only:");
                    ui.checkbox(&mut state.selection_only, "");
                    ui.end_row();
                });

            ui.add_space(6.0);
            ui.label(
                RichText::new(format!(
                    "{} shape(s) selected, {} total",
                    selected_count, total_shapes
                ))
                .small()
                .color(theme::SUBTEXT),
            );
            if state.selection_only && selected_count == 0 {
                ui.label(
                    RichText::new(tr("No selection: fallback to all shapes."))
                        .small()
                        .color(theme::PEACH),
                );
            }

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(
                        total_shapes > 0,
                        egui::Button::new(
                            RichText::new("🧩 Apply Nesting")
                                .color(theme::GREEN)
                                .strong(),
                        ),
                    )
                    .clicked()
                {
                    action.apply = true;
                }
                if ui.button("Close").clicked() {
                    close_clicked = true;
                }
            });
        });

    if close_clicked {
        state.is_open = false;
    }

    action
}

pub fn apply_nesting(
    drawing: &mut DrawingState,
    selection: &[usize],
    workspace: egui::Vec2,
    options: NestingOptions,
) -> NestingResult {
    let margin = options.margin_mm.max(0.0);
    let spacing = options.spacing_mm.max(0.0);
    let min_x = margin;
    let min_y = margin;
    let max_x = (workspace.x.max(0.0) - margin).max(0.0);
    let max_y = (workspace.y.max(0.0) - margin).max(0.0);

    if max_x <= min_x || max_y <= min_y || drawing.shapes.is_empty() {
        return NestingResult::default();
    }

    let mut target_indices = drawing.get_target_indices(selection, options.selection_only);
    if target_indices.is_empty() {
        return NestingResult::default();
    }

    target_indices.sort_by(|a, b| {
        let a_area = drawing.shapes[*a].world_bounds()
            .map(|(x0, y0, x1, y1)| (x1 - x0).abs() * (y1 - y0).abs())
            .unwrap_or(0.0);
        let b_area = drawing.shapes[*b].world_bounds()
            .map(|(x0, y0, x1, y1)| (x1 - x0).abs() * (y1 - y0).abs())
            .unwrap_or(0.0);
        b_area.partial_cmp(&a_area).unwrap_or(Ordering::Equal)
    });

    let mut result = NestingResult {
        considered: target_indices.len(),
        ..NestingResult::default()
    };

    let mut cursor_x = min_x;
    let mut cursor_y = min_y;
    let mut row_height = 0.0f32;

    for idx in target_indices {
        let original = drawing.shapes[idx].clone();
        let base = candidate_for_shape(&original, false);
        let rotated = if options.allow_rotation && !matches!(original.shape, ShapeKind::Circle) {
            let mut s = original.clone();
            s.rotation += 90.0;
            candidate_for_shape(&s, true)
        } else {
            None
        };

        let mut chosen = choose_candidate_for_row(base.as_ref(), rotated.as_ref(), cursor_x, max_x);

        if chosen.is_none() {
            cursor_x = min_x;
            cursor_y += row_height + spacing;
            row_height = 0.0;
            chosen = choose_candidate_for_row(base.as_ref(), rotated.as_ref(), cursor_x, max_x);
        }

        let Some(mut candidate) = chosen.cloned() else {
            result.skipped += 1;
            continue;
        };

        if cursor_y + candidate.height > max_y + 1e-3 {
            result.skipped += 1;
            continue;
        }

        let Some((shape_min_x, shape_min_y, _, _)) = candidate.shape.world_bounds() else {
            result.skipped += 1;
            continue;
        };

        let dx = cursor_x - shape_min_x;
        let dy = cursor_y - shape_min_y;
        candidate.shape.x += dx;
        candidate.shape.y += dy;

        drawing.shapes[idx] = candidate.shape;

        cursor_x += candidate.width + spacing;
        row_height = row_height.max(candidate.height);
        result.placed += 1;
        if candidate.rotated {
            result.used_rotation += 1;
        }
    }

    result
}

fn choose_candidate_for_row<'a>(
    base: Option<&'a Candidate>,
    rotated: Option<&'a Candidate>,
    cursor_x: f32,
    max_x: f32,
) -> Option<&'a Candidate> {
    let fits_base = base
        .map(|c| cursor_x + c.width <= max_x + 1e-3)
        .unwrap_or(false);
    let fits_rot = rotated
        .map(|c| cursor_x + c.width <= max_x + 1e-3)
        .unwrap_or(false);

    match (fits_base, fits_rot) {
        (true, false) => base,
        (false, true) => rotated,
        (true, true) => {
            let b = base?;
            let r = rotated?;
            if b.height < r.height - 1e-3 {
                Some(b)
            } else if r.height < b.height - 1e-3 {
                Some(r)
            } else if b.width <= r.width {
                Some(b)
            } else {
                Some(r)
            }
        }
        (false, false) => None,
    }
}

fn candidate_for_shape(shape: &ShapeParams, rotated: bool) -> Option<Candidate> {
    let (min_x, min_y, max_x, max_y) = shape.world_bounds()?;
    let width = (max_x - min_x).abs();
    let height = (max_y - min_y).abs();
    if width <= 1e-6 || height <= 1e-6 {
        return None;
    }

    Some(Candidate {
        shape: shape.clone(),
        width,
        height,
        rotated,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::drawing::{ShapeKind, ShapeParams};

    fn rect(w: f32, h: f32, x: f32, y: f32) -> ShapeParams {
        ShapeParams {
            shape: ShapeKind::Rectangle,
            width: w,
            height: h,
            x,
            y,
            ..Default::default()
        }
    }

    fn bounds(shape: &ShapeParams) -> (f32, f32, f32, f32) {
        shape.world_bounds().unwrap()
    }

    #[test]
    fn nesting_places_multi_shape_batch_inside_workspace() {
        let mut drawing = DrawingState {
            current: ShapeParams::default(),
            shapes: vec![
                rect(30.0, 20.0, 100.0, 100.0),
                rect(25.0, 15.0, 80.0, 60.0),
                rect(20.0, 10.0, 60.0, 50.0),
                rect(15.0, 12.0, 40.0, 40.0),
            ],
        };

        let result = apply_nesting(
            &mut drawing,
            &[],
            egui::vec2(120.0, 80.0),
            NestingOptions {
                spacing_mm: 2.0,
                margin_mm: 5.0,
                allow_rotation: false,
                selection_only: false,
            },
        );

        assert_eq!(result.considered, 4);
        assert_eq!(result.placed, 4);
        for shape in &drawing.shapes {
            let (x0, y0, x1, y1) = bounds(shape);
            assert!(x0 >= 5.0 - 1e-3);
            assert!(y0 >= 5.0 - 1e-3);
            assert!(x1 <= 115.0 + 1e-3);
            assert!(y1 <= 75.0 + 1e-3);
        }
    }

    #[test]
    fn nesting_respects_workspace_limits_and_skips_overflow() {
        let mut drawing = DrawingState {
            current: ShapeParams::default(),
            shapes: vec![
                rect(40.0, 20.0, 0.0, 0.0),
                rect(40.0, 20.0, 0.0, 0.0),
                rect(40.0, 20.0, 0.0, 0.0),
            ],
        };

        let result = apply_nesting(
            &mut drawing,
            &[],
            egui::vec2(90.0, 35.0),
            NestingOptions {
                spacing_mm: 5.0,
                margin_mm: 5.0,
                allow_rotation: false,
                selection_only: false,
            },
        );

        assert_eq!(result.considered, 3);
        assert_eq!(result.placed, 1);
        assert_eq!(result.skipped, 2);
    }

    #[test]
    fn rotation_option_can_make_fit_possible() {
        let mut drawing_no_rot = DrawingState {
            current: ShapeParams::default(),
            shapes: vec![rect(80.0, 40.0, 0.0, 0.0)],
        };
        let mut drawing_rot = DrawingState {
            current: ShapeParams::default(),
            shapes: vec![rect(80.0, 40.0, 0.0, 0.0)],
        };

        let no_rot = apply_nesting(
            &mut drawing_no_rot,
            &[],
            egui::vec2(60.0, 120.0),
            NestingOptions {
                spacing_mm: 0.0,
                margin_mm: 5.0,
                allow_rotation: false,
                selection_only: false,
            },
        );
        let with_rot = apply_nesting(
            &mut drawing_rot,
            &[],
            egui::vec2(60.0, 120.0),
            NestingOptions {
                spacing_mm: 0.0,
                margin_mm: 5.0,
                allow_rotation: true,
                selection_only: false,
            },
        );

        assert_eq!(no_rot.placed, 0);
        assert_eq!(with_rot.placed, 1);
        assert_eq!(with_rot.used_rotation, 1);
    }
}
