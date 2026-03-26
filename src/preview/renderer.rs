#![allow(dead_code)]

use crate::gcode::types::PreviewSegment;
use crate::i18n::tr;
use crate::theme;
use crate::ui::drawing::{ShapeKind, ShapeParams};
use crate::ui::layers_new::{CutLayer, CutMode};
use egui::{Color32, Painter, Pos2, Rect, Stroke, StrokeKind, Ui, Vec2};
use indexmap::IndexSet;
use std::collections::HashMap;

/// An object visible in the preview that can be interacted with.
#[derive(Clone)]
pub enum PreviewObject {
    /// A raw GCode segment (from file)
    GCode(PreviewSegment),
    /// A structured shape (from drawing tools)
    Shape {
        params: ShapeParams,
        idx: usize, // Index in the DrawingState shapes list
    },
}

/// 2D GCode preview renderer with pan/zoom and interactivity
pub struct PreviewRenderer {
    pub zoom: f32,
    pub pan: Vec2,
    pub machine_pos: Pos2,
    /// Machine workspace in mm (from GRBL $130, $131)
    pub workspace_size: Vec2,
    pub simulation_progress: Option<f32>, // 0.0 to 1.0
    pub simulation_playing: bool,         // Auto-play simulation
    pub show_rapids: bool,                // Toggle for G0 moves
    pub show_fill_preview: bool,          // Overlay predicted hatch for fill layers
    pub show_thermal_risk: bool,          // Overlay thermal overburn risk heatmap
    pub risk_threshold: f32,              // Alert threshold for accumulated energy score
    pub risk_cell_mm: f32,                // Grid cell size in mm used by thermal risk map
    pub last_risk_alert_cells: usize,     // Number of currently highlighted risk cells
    pub realistic_preview: bool,          // Toggle for realistic material texture
    _drag_start: Option<Pos2>,
    pub selected_shape_idx: IndexSet<usize>, // Selected indices in DrawingState (Multi-select). Ordered so last selected is last.
    pub node_edit_mode: bool,
    pub selected_node: Option<(usize, usize)>, // (shape_idx, point_idx)
    pub selected_nodes: IndexSet<(usize, usize)>,
    pub hover_shape_idx: Option<usize>,
    pub dragging_rotation: Option<usize>, // shape_idx being rotated
    dragging_node_handle: Option<(usize, usize, NodeHandleKind)>,
    selection_box_start: Option<Pos2>,
    selection_box_end: Option<Pos2>,
    selection_box_additive: bool,
    pub image_textures: std::collections::HashMap<usize, egui::TextureHandle>, // shape_idx -> texture
    pub initial_fit_done: bool,
    // Measurement tool (F50)
    pub measure_mode: bool,
    pub measure_start: Option<(f32, f32)>, // world coords mm
    pub measure_end: Option<(f32, f32)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeHandleKind {
    In,
    Out,
}

impl Default for PreviewRenderer {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan: Vec2::ZERO,
            machine_pos: Pos2::ZERO,
            workspace_size: Vec2::new(400.0, 400.0), // conservative default
            simulation_progress: None,
            simulation_playing: false,
            show_rapids: true,
            show_fill_preview: true,
            show_thermal_risk: false,
            risk_threshold: 10.0,
            risk_cell_mm: 2.0,
            last_risk_alert_cells: 0,
            realistic_preview: false,
            _drag_start: None,
            selected_shape_idx: IndexSet::new(),
            node_edit_mode: false,
            selected_node: None,
            selected_nodes: IndexSet::new(),
            hover_shape_idx: None,
            dragging_rotation: None,
            dragging_node_handle: None,
            selection_box_start: None,
            selection_box_end: None,
            selection_box_additive: false,
            image_textures: std::collections::HashMap::new(),
            initial_fit_done: false,
            measure_mode: false,
            measure_start: None,
            measure_end: None,
        }
    }
}

pub enum InteractiveAction {
    None,
    SelectShape(usize, bool), // idx, is_multi (ctrl/shift)
    Deselect,
    DragSelection {
        delta: Vec2,
    }, // Drag all selected
    RotateSelection {
        shape_idx: usize,
        delta_deg: f32,
    },
    MoveNode {
        shape_idx: usize,
        node_idx: usize,
        new_pos: Pos2,
    },
    MoveNodeHandle {
        shape_idx: usize,
        node_idx: usize,
        handle: NodeHandleKind,
        new_pos: Pos2,
    },
    AddNode {
        shape_idx: usize,
        insert_after: usize,
        new_pos: Pos2,
    },
    CameraPickPoint(Pos2),
    // Context menu actions (right-click)
    ContextCopy,
    ContextCut,
    ContextPaste,
    ContextDeleteSelection,
    ContextDuplicateSelection,
    ContextGroupSelection,
    ContextUngroupSelection,
    ContextSelectAll,
    GroupSelection,
    UngroupSelection,
}

impl PreviewRenderer {
    /// Render the preview onto an egui Response area
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        segments: &[PreviewSegment],
        shapes: &[ShapeParams], // Drawing shapes
        layers: &[CutLayer],
        is_light: bool,
        job_offset: Vec2,
        job_rotation_deg: f32,
        camera_state: &crate::ui::camera::CameraState,
    ) -> InteractiveAction {
        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
        let rect = response.rect;
        let mut action = InteractiveAction::None;

        // Bounding box for rotation center (calculated from GCode segments for now)
        let mut job_min = Pos2::new(f32::MAX, f32::MAX);
        let mut job_max = Pos2::new(f32::MIN, f32::MIN);
        if !segments.is_empty() {
            for seg in segments {
                job_min.x = job_min.x.min(seg.x1).min(seg.x2);
                job_min.y = job_min.y.min(seg.y1).min(seg.y2);
                job_max.x = job_max.x.max(seg.x1).max(seg.x2);
                job_max.y = job_max.y.max(seg.y1).max(seg.y2);
            }
        } else {
            // Default center if no GCode
            job_min = Pos2::ZERO;
            job_max = Pos2::ZERO;
        }
        let job_center = job_min + (job_max - job_min) * 0.5;
        let angle = job_rotation_deg.to_radians();
        let (sin_a, cos_a) = angle.sin_cos();

        let transform = |x: f32, y: f32| -> Pos2 {
            // 1. Center
            let dx = x - job_center.x;
            let dy = y - job_center.y;
            // 2. Rotate
            let rx = dx * cos_a - dy * sin_a;
            let ry = dx * sin_a + dy * cos_a;
            // 3. Re-center + Offset
            Pos2::new(
                rx + job_center.x + job_offset.x,
                ry + job_center.y + job_offset.y,
            )
        };

        if !self.initial_fit_done {
            self.auto_fit_workspace(rect);
            self.initial_fit_done = true;
        }

        // Background
        let bg_color = if self.realistic_preview {
            Color32::from_rgb(222, 184, 135) // Wood texture color (Burlywood)
        } else if is_light {
            Color32::from_rgb(250, 250, 250)
        } else {
            theme::CRUST
        };
        painter.rect_filled(rect, 0.0, bg_color);

        // Draw camera image if available and enabled
        if camera_state.enabled {
            if let Some(texture) = &camera_state.texture {
                let calib = &camera_state.calibration;
                let scale = calib.scale.max(0.01);
                let base_w = self.workspace_size.x.max(1.0);
                let base_h = self.workspace_size.y.max(1.0);
                let w = base_w * scale;
                let h = base_h * scale;
                let (sin_a, cos_a) = calib.rotation.to_radians().sin_cos();
                let cx = calib.offset_x + w * 0.5;
                let cy = calib.offset_y + h * 0.5;

                let rotate_world = |x: f32, y: f32| -> Pos2 {
                    let dx = x - cx;
                    let dy = y - cy;
                    Pos2::new(cx + dx * cos_a - dy * sin_a, cy + dx * sin_a + dy * cos_a)
                };

                let w0 = rotate_world(calib.offset_x, calib.offset_y);
                let w1 = rotate_world(calib.offset_x + w, calib.offset_y);
                let w2 = rotate_world(calib.offset_x + w, calib.offset_y + h);
                let w3 = rotate_world(calib.offset_x, calib.offset_y + h);

                let p0 = self.world_to_screen(w0.x, w0.y, rect);
                let p1 = self.world_to_screen(w1.x, w1.y, rect);
                let p2 = self.world_to_screen(w2.x, w2.y, rect);
                let p3 = self.world_to_screen(w3.x, w3.y, rect);

                let color = Color32::from_white_alpha((camera_state.opacity * 255.0) as u8);

                let mut mesh = egui::Mesh::with_texture(texture.id());
                let base_idx = mesh.vertices.len() as u32;
                mesh.add_triangle(base_idx, base_idx + 1, base_idx + 2);
                mesh.add_triangle(base_idx, base_idx + 2, base_idx + 3);
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: p0,
                    uv: egui::pos2(0.0, 0.0),
                    color,
                });
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: p1,
                    uv: egui::pos2(1.0, 0.0),
                    color,
                });
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: p2,
                    uv: egui::pos2(1.0, 1.0),
                    color,
                });
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: p3,
                    uv: egui::pos2(0.0, 1.0),
                    color,
                });
                painter.add(mesh);

                let frame = Stroke::new(1.0, Color32::from_rgba_premultiplied(137, 180, 250, 220));
                painter.line_segment([p0, p1], frame);
                painter.line_segment([p1, p2], frame);
                painter.line_segment([p2, p3], frame);
                painter.line_segment([p3, p0], frame);
            }
        }

        // Handle generic input (Pan/Zoom) vs Selection input
        let camera_pick_active =
            camera_state.calibration_wizard_active || camera_state.point_align_active;
        let handled_interaction = self.handle_input(
            ui,
            &response,
            rect,
            shapes,
            camera_state,
            camera_pick_active,
        );
        if let InteractiveAction::None = handled_interaction {
            // If interaction wasn't a selection/drag, proceed standard input handling
        } else {
            action = handled_interaction;
        }

        // Right-click context menu
        let has_selection = !self.selected_shape_idx.is_empty();
        let ctx_action = self.show_context_menu(ui, &response, has_selection);
        if !matches!(ctx_action, InteractiveAction::None) {
            action = ctx_action;
        }

        // Draw grid
        self.draw_grid(&painter, rect, is_light);

        // Draw GCode segments
        self.draw_gcode_segments(&painter, segments, rect, is_light, transform);

        // Draw thermal risk overlay (advanced simulation)
        self.draw_thermal_risk_overlay(&painter, segments, rect, &transform);

        if self.show_fill_preview {
            self.draw_fill_overlay(&painter, rect, shapes, layers, is_light);
        }

        // Draw interactive shapes (Overlay)
        for (idx, shape) in shapes.iter().enumerate() {
            let is_selected = self.selected_shape_idx.contains(&idx);
            self.draw_shape_overlay(ui, &painter, shape, rect, is_selected, idx, layers);
        }

        if let (Some(start), Some(end)) = (self.selection_box_start, self.selection_box_end) {
            let a = self.world_to_screen(start.x, start.y, rect);
            let b = self.world_to_screen(end.x, end.y, rect);
            let sel_rect = Rect::from_two_pos(a, b);
            let fill = Color32::from_rgba_premultiplied(
                theme::BLUE.r(),
                theme::BLUE.g(),
                theme::BLUE.b(),
                28,
            );
            painter.rect_filled(sel_rect, 0.0, fill);
            painter.rect_stroke(
                sel_rect,
                0.0,
                Stroke::new(1.0, theme::BLUE),
                StrokeKind::Middle,
            );
        }

        // Draw machine position
        let machine_screen = self.world_to_screen(self.machine_pos.x, self.machine_pos.y, rect);
        painter.circle_filled(machine_screen, 5.0, theme::RED);

        // Draw Simulation Playhead
        if let Some(progress) = self.simulation_progress {
            if !segments.is_empty() {
                let idx = ((segments.len() - 1) as f32 * progress) as usize;
                let seg = &segments[idx % segments.len()];
                let p = self.world_to_screen(seg.x2, seg.y2, rect);
                painter.circle_filled(p, 4.0, Color32::from_rgb(255, 100, 0));
                painter.circle_stroke(p, 6.0, Stroke::new(1.0, Color32::WHITE));
            }
        }

        // Draw origin crosshair
        let origin = self.world_to_screen(0.0, 0.0, rect);
        painter.line_segment(
            [
                Pos2::new(origin.x - 15.0, origin.y),
                Pos2::new(origin.x + 15.0, origin.y),
            ],
            Stroke::new(1.0, theme::SURFACE2),
        );
        painter.line_segment(
            [
                Pos2::new(origin.x, origin.y - 15.0),
                Pos2::new(origin.x, origin.y + 15.0),
            ],
            Stroke::new(1.0, theme::SURFACE2),
        );

        if segments.is_empty() && shapes.is_empty() {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Load a GCode file or draw shapes to preview",
                egui::FontId::proportional(16.0),
                theme::OVERLAY0,
            );
        }

        action
    }

    fn is_point_in_camera_overlay(
        &self,
        wx: f32,
        wy: f32,
        camera_state: &crate::ui::camera::CameraState,
    ) -> bool {
        if !camera_state.enabled || camera_state.texture.is_none() {
            return false;
        }

        let calib = &camera_state.calibration;
        let scale = calib.scale.max(0.01);
        let w = self.workspace_size.x.max(1.0) * scale;
        let h = self.workspace_size.y.max(1.0) * scale;
        let (sin_a, cos_a) = calib.rotation.to_radians().sin_cos();
        let cx = calib.offset_x + w * 0.5;
        let cy = calib.offset_y + h * 0.5;

        let dx = wx - cx;
        let dy = wy - cy;
        let ux = cx + dx * cos_a + dy * sin_a;
        let uy = cy - dx * sin_a + dy * cos_a;

        ux >= calib.offset_x
            && ux <= calib.offset_x + w
            && uy >= calib.offset_y
            && uy <= calib.offset_y + h
    }

    fn draw_thermal_risk_overlay<F>(
        &mut self,
        painter: &Painter,
        segments: &[PreviewSegment],
        rect: Rect,
        transform: &F,
    ) where
        F: Fn(f32, f32) -> Pos2,
    {
        if !self.show_thermal_risk || segments.is_empty() {
            self.last_risk_alert_cells = 0;
            return;
        }

        let heat = thermal_risk_heatmap(
            segments,
            self.risk_cell_mm.max(0.5),
            self.simulation_progress,
            transform,
        );

        let threshold = self.risk_threshold.max(0.1);
        let cell = self.risk_cell_mm.max(0.5);
        let mut alert_count = 0usize;

        for ((cx, cy), score) in heat {
            if score < threshold {
                continue;
            }
            alert_count += 1;

            let x0 = cx as f32 * cell;
            let y0 = cy as f32 * cell;
            let x1 = x0 + cell;
            let y1 = y0 + cell;

            let p0 = self.world_to_screen(x0, y0, rect);
            let p1 = self.world_to_screen(x1, y1, rect);
            let overlay_rect = Rect::from_two_pos(p0, p1);

            if !rect.intersects(overlay_rect) {
                continue;
            }

            let intensity = ((score - threshold) / (threshold * 2.0)).clamp(0.15, 1.0);
            let alpha = (55.0 + intensity * 145.0) as u8;
            let fill = Color32::from_rgba_premultiplied(255, 64, 32, alpha);
            let stroke = Color32::from_rgba_premultiplied(255, 140, 80, (alpha as f32 * 0.9) as u8);

            painter.rect_filled(overlay_rect, 0.0, fill);
            painter.rect_stroke(
                overlay_rect,
                0.0,
                Stroke::new(0.7, stroke),
                StrokeKind::Inside,
            );
        }

        self.last_risk_alert_cells = alert_count;
    }

    fn draw_gcode_segments<F>(
        &self,
        painter: &Painter,
        segments: &[PreviewSegment],
        rect: Rect,
        is_light: bool,
        transform: F,
    ) where
        F: Fn(f32, f32) -> Pos2,
    {
        let rapid_stroke = if is_light || self.realistic_preview {
            Stroke::new(0.5, Color32::from_rgba_premultiplied(200, 200, 200, 150))
        } else {
            Stroke::new(0.5, Color32::from_rgba_premultiplied(69, 71, 90, 180))
        };

        let mut current_accumulated_line: Option<(Pos2, Pos2, f32, usize)> = None;

        let realistic = self.realistic_preview;

        let flush_accumulated =
            |painter: &egui::Painter, accum: &mut Option<(Pos2, Pos2, f32, usize)>| {
                if let Some((p1, p2, total_power, count)) = accum.take() {
                    let avg_power = total_power / (count as f32);
                    let base_alpha = 40.0 + 215.0 * avg_power;
                    let mut stroke_width = 0.1 * self.zoom;
                    let mut final_alpha = base_alpha;

                    if stroke_width < 1.0 {
                        final_alpha *= stroke_width;
                        stroke_width = 1.0;
                    }

                    stroke_width = stroke_width.min(2.5);

                    let color = if realistic {
                        // Burn effect (Dark Brown)
                        Color32::from_rgba_premultiplied(60, 30, 10, (avg_power * 255.0) as u8)
                    } else if is_light {
                        Color32::from_rgba_premultiplied(0, 0, 0, final_alpha as u8)
                    } else {
                        Color32::from_rgba_premultiplied(255, 255, 255, final_alpha as u8)
                    };
                    painter.line_segment([p1, p2], Stroke::new(stroke_width, color));
                }
            };

        for seg in segments {
            let p1_w = transform(seg.x1, seg.y1);
            let p2_w = transform(seg.x2, seg.y2);
            let p1 = self.world_to_screen(p1_w.x, p1_w.y, rect);
            let p2 = self.world_to_screen(p2_w.x, p2_w.y, rect);

            if !rect_contains_approx(rect, p1) && !rect_contains_approx(rect, p2) {
                flush_accumulated(&painter, &mut current_accumulated_line);
                continue;
            }

            if seg.laser_on {
                let is_horizontal = (p1.y - p2.y).abs() < 0.5;
                let length_sq = (p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2);

                if is_horizontal && length_sq < 4.0 {
                    if let Some((acc_p1, acc_p2, total_power, count)) =
                        current_accumulated_line.as_mut()
                    {
                        if (acc_p1.y - p1.y).abs() < 1.0 && (acc_p2.x - p1.x).abs() < 2.0 {
                            acc_p2.x = p2.x;
                            *total_power += seg.power;
                            *count += 1;
                        } else {
                            flush_accumulated(&painter, &mut current_accumulated_line);
                            current_accumulated_line = Some((p1, p2, seg.power, 1));
                        }
                    } else {
                        current_accumulated_line = Some((p1, p2, seg.power, 1));
                    }
                } else {
                    flush_accumulated(&painter, &mut current_accumulated_line);

                    let base_alpha = 40.0 + 215.0 * seg.power;
                    let mut stroke_width = 0.1 * self.zoom;
                    let mut final_alpha = base_alpha;

                    if stroke_width < 1.0 {
                        final_alpha *= stroke_width;
                        stroke_width = 1.0;
                    }

                    stroke_width = stroke_width.min(2.5);

                    let color = if is_light {
                        Color32::from_rgba_premultiplied(0, 0, 0, final_alpha as u8)
                    } else {
                        Color32::from_rgba_premultiplied(255, 255, 255, final_alpha as u8)
                    };
                    painter.line_segment([p1, p2], Stroke::new(stroke_width, color));
                }
            } else if self.show_rapids {
                flush_accumulated(&painter, &mut current_accumulated_line);
                let dist_sq = (seg.x1 - seg.x2).powi(2) + (seg.y1 - seg.y2).powi(2);
                if dist_sq > 25.0 {
                    painter.line_segment([p1, p2], rapid_stroke);
                }
            }
        }
        flush_accumulated(&painter, &mut current_accumulated_line);
    }

    fn draw_fill_overlay(
        &self,
        painter: &Painter,
        rect: Rect,
        shapes: &[ShapeParams],
        layers: &[CutLayer],
        is_light: bool,
    ) {
        if shapes.is_empty() {
            return;
        }

        const MAX_SEGMENTS_TOTAL: usize = 6000;
        const MAX_SEGMENTS_PER_LAYER: usize = 1500;

        let mut per_layer_indices: HashMap<usize, Vec<usize>> = HashMap::new();
        for (shape_idx, shape) in shapes.iter().enumerate() {
            per_layer_indices
                .entry(shape.layer_idx)
                .or_default()
                .push(shape_idx);
        }

        let mut layer_indices: Vec<usize> = per_layer_indices.keys().copied().collect();
        layer_indices.sort_by_key(|&layer_idx| {
            if let Some(layer) = layers.get(layer_idx) {
                (layer.output_order, layer.id as i32)
            } else {
                (i32::MAX, layer_idx as i32)
            }
        });

        let mut remaining = MAX_SEGMENTS_TOTAL;
        for layer_idx in layer_indices {
            if remaining == 0 {
                break;
            }

            let Some(layer) = layers.get(layer_idx) else {
                continue;
            };
            if !layer.visible {
                continue;
            }
            if !matches!(
                layer.mode,
                CutMode::Fill | CutMode::FillAndLine | CutMode::Offset
            ) {
                continue;
            }

            let Some(shape_indices) = per_layer_indices.get(&layer_idx) else {
                continue;
            };

            let layer_shapes: Vec<&ShapeParams> = shape_indices
                .iter()
                .filter_map(|&idx| shapes.get(idx))
                .collect();
            if layer_shapes.is_empty() {
                continue;
            }

            let request_count = remaining.min(MAX_SEGMENTS_PER_LAYER);
            let segments = crate::gcode::fill::preview_fill_segments_group(
                &layer_shapes,
                layer,
                request_count,
            );
            if segments.is_empty() {
                continue;
            }

            let alpha = if is_light { 80 } else { 100 };
            let fill_color = Color32::from_rgba_premultiplied(
                layer.color.r(),
                layer.color.g(),
                layer.color.b(),
                alpha,
            );
            let stroke = Stroke::new(0.9, fill_color);

            for (start, end) in segments {
                let p1 = self.world_to_screen(start.0, start.1, rect);
                let p2 = self.world_to_screen(end.0, end.1, rect);
                if rect_contains_approx(rect, p1) || rect_contains_approx(rect, p2) {
                    painter.line_segment([p1, p2], stroke);
                }
            }

            remaining = remaining.saturating_sub(request_count);
        }
    }

    fn draw_shape_overlay(
        &mut self,
        ui: &Ui,
        painter: &Painter,
        shape: &ShapeParams,
        rect: Rect,
        is_selected: bool,
        idx: usize,
        layers: &[CutLayer],
    ) {
        let is_hovered = self.hover_shape_idx == Some(idx);

        let stroke_color = if is_selected {
            theme::BLUE
        } else if is_hovered {
            theme::LAVENDER
        } else {
            // Use layer color if available
            if let Some(layer) = layers.get(shape.layer_idx) {
                layer.color
            } else {
                theme::TEXT.linear_multiply(0.5)
            }
        };

        let stroke = Stroke::new(1.0, stroke_color);
        let angle = shape.rotation.to_radians();

        // Draw the shape itself
        match &shape.shape {
            ShapeKind::Rectangle => {
                let p1 = {
                    let wx = shape.x + 0.0 * angle.cos() - 0.0 * angle.sin();
                    let wy = shape.y + 0.0 * angle.sin() + 0.0 * angle.cos();
                    self.world_to_screen(wx, wy, rect)
                };
                let p2 = {
                    let wx = shape.x + shape.width * angle.cos() - 0.0 * angle.sin();
                    let wy = shape.y + shape.width * angle.sin() + 0.0 * angle.cos();
                    self.world_to_screen(wx, wy, rect)
                };
                let p3 = {
                    let wx = shape.x + shape.width * angle.cos() - shape.height * angle.sin();
                    let wy = shape.y + shape.width * angle.sin() + shape.height * angle.cos();
                    self.world_to_screen(wx, wy, rect)
                };
                let p4 = {
                    let wx = shape.x + 0.0 * angle.cos() - shape.height * angle.sin();
                    let wy = shape.y + 0.0 * angle.sin() + shape.height * angle.cos();
                    self.world_to_screen(wx, wy, rect)
                };
                painter.line_segment([p1, p2], stroke);
                painter.line_segment([p2, p3], stroke);
                painter.line_segment([p3, p4], stroke);
                painter.line_segment([p4, p1], stroke);
            }
            ShapeKind::Circle => {
                let center = self.world_to_screen(shape.x, shape.y, rect);
                painter.circle_stroke(center, shape.radius * self.zoom, stroke);
            }
            ShapeKind::TextLine => {
                let center = self.world_to_screen(shape.x, shape.y, rect);
                painter.text(
                    center,
                    egui::Align2::LEFT_BOTTOM,
                    &shape.text,
                    egui::FontId::proportional(shape.font_size_mm * self.zoom),
                    stroke_color,
                );
            }
            ShapeKind::Path(pts) => {
                if pts.len() > 1 {
                    for i in 0..pts.len() - 1 {
                        let p1 = {
                            let wx = shape.x + pts[i].0 * angle.cos() - pts[i].1 * angle.sin();
                            let wy = shape.y + pts[i].0 * angle.sin() + pts[i].1 * angle.cos();
                            self.world_to_screen(wx, wy, rect)
                        };
                        let p2 = {
                            let wx =
                                shape.x + pts[i + 1].0 * angle.cos() - pts[i + 1].1 * angle.sin();
                            let wy =
                                shape.y + pts[i + 1].0 * angle.sin() + pts[i + 1].1 * angle.cos();
                            self.world_to_screen(wx, wy, rect)
                        };
                        painter.line_segment([p1, p2], stroke);
                    }
                }
            }
            ShapeKind::RasterImage { data, params } => {
                let texture_id = {
                    // Get or Create texture
                    let texture = self.image_textures.entry(idx).or_insert_with(|| {
                        let processed = crate::imaging::raster::preprocess_image_rgba(&data.0, params);
                        let rgba = processed.to_rgba8();
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            [rgba.width() as _, rgba.height() as _],
                            rgba.as_flat_samples().as_slice(),
                        );
                        ui.ctx().load_texture(
                            format!("shape_{}", idx),
                            color_image,
                            Default::default(),
                        )
                    });
                    texture.id()
                };

                // Calculate the rotated quad
                let p1 = {
                    let wx = shape.x + 0.0 * angle.cos() - 0.0 * angle.sin();
                    let wy = shape.y + 0.0 * angle.sin() + 0.0 * angle.cos();
                    self.world_to_screen(wx, wy, rect)
                };
                let p2 = {
                    let wx = shape.x + params.width_mm * angle.cos() - 0.0 * angle.sin();
                    let wy = shape.y + params.width_mm * angle.sin() + 0.0 * angle.cos();
                    self.world_to_screen(wx, wy, rect)
                };
                let p3 = {
                    let wx =
                        shape.x + params.width_mm * angle.cos() - params.height_mm * angle.sin();
                    let wy =
                        shape.y + params.width_mm * angle.sin() + params.height_mm * angle.cos();
                    self.world_to_screen(wx, wy, rect)
                };
                let p4 = {
                    let wx = shape.x + 0.0 * angle.cos() - params.height_mm * angle.sin();
                    let wy = shape.y + 0.0 * angle.sin() + params.height_mm * angle.cos();
                    self.world_to_screen(wx, wy, rect)
                };

                let mut mesh = egui::Mesh::with_texture(texture_id);
                let base_idx = mesh.vertices.len() as u32;
                mesh.add_triangle(base_idx, base_idx + 1, base_idx + 2);
                mesh.add_triangle(base_idx, base_idx + 2, base_idx + 3);

                mesh.vertices.push(egui::epaint::Vertex {
                    pos: p1,
                    uv: egui::pos2(0.0, 1.0),
                    color: Color32::WHITE,
                });
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: p2,
                    uv: egui::pos2(1.0, 1.0),
                    color: Color32::WHITE,
                });
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: p3,
                    uv: egui::pos2(1.0, 0.0),
                    color: Color32::WHITE,
                });
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: p4,
                    uv: egui::pos2(0.0, 0.0),
                    color: Color32::WHITE,
                });

                painter.add(mesh);

                // Draw outline for clarity
                painter.line_segment([p1, p2], stroke);
                painter.line_segment([p2, p3], stroke);
                painter.line_segment([p3, p4], stroke);
                painter.line_segment([p4, p1], stroke);
            }
        }

        // Draw Selection UI
        if is_selected {
            // Rotation Handle
            let handle_pos = self.get_rotation_handle_pos(shape);
            let s_handle = self.world_to_screen(handle_pos.x, handle_pos.y, rect);
            let s_center = self.world_to_screen(shape.x, shape.y, rect);

            // Top-right corner in local space
            let (tr_lx, tr_ly) = match &shape.shape {
                ShapeKind::Rectangle => (shape.width, shape.height),
                ShapeKind::Circle => (shape.radius, shape.radius), // "corner" of square bounding box
                _ => (0.0, 0.0),
            };

            // World pos of top-right corner
            let angle = shape.rotation.to_radians();
            let tr_wx = shape.x + tr_lx * angle.cos() - tr_ly * angle.sin();
            let tr_wy = shape.y + tr_lx * angle.sin() + tr_ly * angle.cos();
            let s_tr = self.world_to_screen(tr_wx, tr_wy, rect);

            // Draw connection line from top-right corner to handle
            painter.line_segment([s_tr, s_handle], Stroke::new(1.0, theme::SUBTEXT));

            // Draw an arc with arrows (Visual indicator for rotation)
            let radius = s_center.distance(s_handle);
            let angle_base = (s_handle - s_center).angle();
            let arc_span = 0.2; // radians

            let mut points = vec![];
            for i in -5..=5 {
                let a = angle_base + (i as f32 * arc_span / 10.0);
                points.push(s_center + Vec2::angled(a) * radius);
            }
            painter.add(egui::Shape::line(
                points.clone(),
                Stroke::new(1.2, theme::BLUE),
            ));

            // Arrows at ends of arc
            let draw_arrow = |p: Pos2, angle: f32| {
                let head_len = 2.5;
                let head_angle = 0.5;
                painter.line_segment(
                    [p, p - Vec2::angled(angle + head_angle) * head_len],
                    Stroke::new(1.2, theme::BLUE),
                );
                painter.line_segment(
                    [p, p - Vec2::angled(angle - head_angle) * head_len],
                    Stroke::new(1.2, theme::BLUE),
                );
            };

            draw_arrow(
                *(points.last().unwrap()),
                angle_base + arc_span / 2.0 + std::f32::consts::FRAC_PI_2,
            );
            draw_arrow(
                *(points.first().unwrap()),
                angle_base - arc_span / 2.0 - std::f32::consts::FRAC_PI_2,
            );

            painter.circle_filled(s_handle, 3.5, theme::BLUE);
            painter.circle_stroke(s_handle, 5.0, Stroke::new(1.0, Color32::WHITE));

            // Small square handles at corners for movement feedback (placeholder for full transform handles)
            let handle_size = 4.0;
            painter.rect_filled(
                Rect::from_center_size(s_center, Vec2::splat(handle_size)),
                1.0,
                theme::BLUE,
            );

            // Node Editing Handles
            if self.node_edit_mode {
                if let ShapeKind::Path(pts) = &shape.shape {
                    for (v_idx, p) in pts.iter().enumerate() {
                        let vp = {
                            let wx = shape.x + p.0 * angle.cos() - p.1 * angle.sin();
                            let wy = shape.y + p.0 * angle.sin() + p.1 * angle.cos();
                            self.world_to_screen(wx, wy, rect)
                        };
                        let is_node_sel = self.selected_nodes.contains(&(idx, v_idx))
                            || self.selected_node == Some((idx, v_idx));
                        let color = if is_node_sel {
                            theme::RED
                        } else {
                            theme::GREEN
                        };
                        painter.circle_filled(vp, 4.0, color);
                    }

                    if let Some((s_idx, node_idx)) = self.selected_node {
                        if s_idx == idx {
                            if let Some(cur) = pts.get(node_idx) {
                                let cur_w = {
                                    let wx = shape.x + cur.0 * angle.cos() - cur.1 * angle.sin();
                                    let wy = shape.y + cur.0 * angle.sin() + cur.1 * angle.cos();
                                    self.world_to_screen(wx, wy, rect)
                                };

                                if node_idx > 0 {
                                    if let Some(prev) = pts.get(node_idx - 1) {
                                        let prev_w = {
                                            let wx = shape.x + prev.0 * angle.cos()
                                                - prev.1 * angle.sin();
                                            let wy = shape.y
                                                + prev.0 * angle.sin()
                                                + prev.1 * angle.cos();
                                            self.world_to_screen(wx, wy, rect)
                                        };
                                        painter.line_segment(
                                            [cur_w, prev_w],
                                            Stroke::new(1.0, Color32::from_rgb(100, 170, 255)),
                                        );
                                        painter.rect_filled(
                                            Rect::from_center_size(prev_w, Vec2::splat(6.0)),
                                            1.0,
                                            Color32::from_rgb(100, 170, 255),
                                        );
                                    }
                                }

                                if node_idx + 1 < pts.len() {
                                    if let Some(next) = pts.get(node_idx + 1) {
                                        let next_w = {
                                            let wx = shape.x + next.0 * angle.cos()
                                                - next.1 * angle.sin();
                                            let wy = shape.y
                                                + next.0 * angle.sin()
                                                + next.1 * angle.cos();
                                            self.world_to_screen(wx, wy, rect)
                                        };
                                        painter.line_segment(
                                            [cur_w, next_w],
                                            Stroke::new(1.0, Color32::from_rgb(255, 170, 100)),
                                        );
                                        painter.rect_filled(
                                            Rect::from_center_size(next_w, Vec2::splat(6.0)),
                                            1.0,
                                            Color32::from_rgb(255, 170, 100),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Auto-fit the machine workspace to the view
    pub fn auto_fit_workspace(&mut self, rect: Rect) {
        if self.workspace_size.x <= 0.0 || self.workspace_size.y <= 0.0 {
            return;
        }

        let margin = 40.0;
        let view_w = rect.width() - margin * 2.0;
        let view_h = rect.height() - margin * 2.0;

        self.zoom = (view_w / self.workspace_size.x)
            .min(view_h / self.workspace_size.y)
            .max(0.01);

        let center_x = self.workspace_size.x / 2.0;
        let center_y = self.workspace_size.y / 2.0;

        self.pan = Vec2::new(
            rect.center().x - center_x * self.zoom,
            rect.center().y + center_y * self.zoom,
        );
    }

    pub fn fit_world_bounds(&mut self, rect: Rect, min_x: f32, min_y: f32, max_x: f32, max_y: f32) {
        let data_w = (max_x - min_x).max(0.001);
        let data_h = (max_y - min_y).max(0.001);

        let margin = 40.0;
        let view_w = rect.width() - margin * 2.0;
        let view_h = rect.height() - margin * 2.0;

        self.zoom = (view_w / data_w).min(view_h / data_h).max(0.01);
        let center_x = (min_x + max_x) / 2.0;
        let center_y = (min_y + max_y) / 2.0;

        self.pan = Vec2::new(
            rect.center().x - center_x * self.zoom,
            rect.center().y + center_y * self.zoom,
        );
    }

    /// Auto-fit all segments in view
    pub fn auto_fit(
        &mut self,
        segments: &[PreviewSegment],
        rect: Rect,
        job_offset: Vec2,
        job_rotation_deg: f32,
    ) {
        if segments.is_empty() {
            self.auto_fit_workspace(rect);
            return;
        }

        let mut job_min = Pos2::new(f32::MAX, f32::MAX);
        let mut job_max = Pos2::new(f32::MIN, f32::MIN);
        for seg in segments {
            job_min.x = job_min.x.min(seg.x1).min(seg.x2);
            job_min.y = job_min.y.min(seg.y1).min(seg.y2);
            job_max.x = job_max.x.max(seg.x1).max(seg.x2);
            job_max.y = job_max.y.max(seg.y1).max(seg.y2);
        }
        let job_center = job_min + (job_max - job_min) * 0.5;
        let angle = job_rotation_deg.to_radians();
        let (sin_a, cos_a) = angle.sin_cos();

        let transform = |x: f32, y: f32| -> Pos2 {
            let dx = x - job_center.x;
            let dy = y - job_center.y;
            let rx = dx * cos_a - dy * sin_a;
            let ry = dx * sin_a + dy * cos_a;
            Pos2::new(
                rx + job_center.x + job_offset.x,
                ry + job_center.y + job_offset.y,
            )
        };

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for seg in segments {
            let p1 = transform(seg.x1, seg.y1);
            let p2 = transform(seg.x2, seg.y2);
            min_x = min_x.min(p1.x).min(p2.x);
            min_y = min_y.min(p1.y).min(p2.y);
            max_x = max_x.max(p1.x).max(p2.x);
            max_y = max_y.max(p1.y).max(p2.y);
        }

        self.fit_world_bounds(rect, min_x, min_y, max_x, max_y);
    }

    /// Convert screen coordinates to world coordinates (mm)
    pub fn screen_to_world(&self, screen: Pos2, _rect: Rect) -> (f32, f32) {
        let wx = (screen.x - self.pan.x) / self.zoom;
        let wy = -(screen.y - self.pan.y) / self.zoom;
        (wx, wy)
    }

    /// Draw measurement overlay (F50)
    pub fn draw_measurement(&self, painter: &Painter, rect: Rect) {
        if !self.measure_mode {
            return;
        }
        if let (Some(start), Some(end)) = (self.measure_start, self.measure_end) {
            let p1 = self.world_to_screen(start.0, start.1, rect);
            let p2 = self.world_to_screen(end.0, end.1, rect);
            let dx = end.0 - start.0;
            let dy = end.1 - start.1;
            let dist = (dx * dx + dy * dy).sqrt();

            painter.line_segment([p1, p2], Stroke::new(2.0, Color32::from_rgb(0, 200, 255)));
            painter.circle_filled(p1, 4.0, Color32::from_rgb(0, 200, 255));
            painter.circle_filled(p2, 4.0, Color32::from_rgb(0, 200, 255));

            let mid = Pos2::new((p1.x + p2.x) / 2.0, (p1.y + p2.y) / 2.0 - 12.0);
            let label = format!("{dist:.2} mm");
            painter.text(
                mid,
                egui::Align2::CENTER_BOTTOM,
                label,
                egui::FontId::proportional(13.0),
                Color32::from_rgb(0, 200, 255),
            );

            // Show dx/dy
            let detail = format!("dx={dx:.2} dy={dy:.2}");
            painter.text(
                Pos2::new(mid.x, mid.y + 14.0),
                egui::Align2::CENTER_TOP,
                detail,
                egui::FontId::proportional(10.0),
                Color32::from_rgb(0, 180, 220),
            );
        }
    }

    pub fn zoom_in(&mut self) {
        self.zoom *= 1.3;
    }

    pub fn zoom_out(&mut self) {
        self.zoom /= 1.3;
    }

    fn handle_input(
        &mut self,
        ui: &egui::Ui,
        response: &egui::Response,
        _rect: Rect,
        shapes: &[ShapeParams],
        camera_state: &crate::ui::camera::CameraState,
        camera_pick_active: bool,
    ) -> InteractiveAction {
        let mut action = InteractiveAction::None;

        // ── Middle mouse button: ALWAYS pan, regardless of selection ──
        if response.dragged_by(egui::PointerButton::Middle) {
            let delta = response.drag_delta();
            self.pan += delta;
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
            return action;
        }

        if let Some(hover) = response.hover_pos() {
            // Zoom logic (scroll wheel)
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                let old_zoom = self.zoom;
                let factor = if scroll > 0.0 { 1.1 } else { 1.0 / 1.1 };
                self.zoom = (self.zoom * factor).clamp(0.01, 10000.0);
                self.pan.x = hover.x - (hover.x - self.pan.x) * (self.zoom / old_zoom);
                self.pan.y = hover.y - (hover.y - self.pan.y) * (self.zoom / old_zoom);
            }

            // Convert screen hover to world
            let wx = (hover.x - self.pan.x) / self.zoom;
            let wy = -(hover.y - self.pan.y) / self.zoom;
            let is_multi = ui.input(|i| i.modifiers.ctrl || i.modifiers.shift);

            if response.clicked_by(egui::PointerButton::Primary)
                && self.dragging_rotation.is_none()
                && camera_pick_active
            {
                if self.is_point_in_camera_overlay(wx, wy, camera_state) {
                    return InteractiveAction::CameraPickPoint(Pos2::new(wx, wy));
                }
                return InteractiveAction::None;
            }

            // ── Measurement tool (F50): intercept clicks before selection ──
            if self.measure_mode {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
            }
            if self.measure_mode && response.clicked_by(egui::PointerButton::Primary) {
                if self.measure_start.is_none() {
                    self.measure_start = Some((wx, wy));
                } else if self.measure_end.is_none() {
                    self.measure_end = Some((wx, wy));
                } else {
                    // Third click: start a new measurement
                    self.measure_start = Some((wx, wy));
                    self.measure_end = None;
                }
                return action;
            }

            if response.clicked_by(egui::PointerButton::Secondary) {
                // Right click actions could be handled here or returned as a new InteractiveAction type later
            }

            // 1. Detect Hover
            self.hover_shape_idx = None;
            for (idx, shape) in shapes.iter().enumerate() {
                if self.is_point_in_shape(wx, wy, shape) {
                    self.hover_shape_idx = Some(idx);
                }
            }

            // 2. Left-click/Drag Selection Logic
            if response.drag_started_by(egui::PointerButton::Primary) {
                if self.node_edit_mode {
                    if let Some((shape_idx, node_idx)) = self.selected_node {
                        if let Some(shape) = shapes.get(shape_idx) {
                            if let ShapeKind::Path(pts) = &shape.shape {
                                let angle = shape.rotation.to_radians();
                                let (sin_a, cos_a) = angle.sin_cos();

                                if node_idx > 0 {
                                    let p = pts[node_idx - 1];
                                    let hp = Pos2::new(
                                        shape.x + p.0 * cos_a - p.1 * sin_a,
                                        shape.y + p.0 * sin_a + p.1 * cos_a,
                                    );
                                    if hp.distance(Pos2::new(wx, wy)) <= 6.0 / self.zoom {
                                        self.dragging_node_handle =
                                            Some((shape_idx, node_idx, NodeHandleKind::In));
                                    }
                                }
                                if self.dragging_node_handle.is_none() && node_idx + 1 < pts.len() {
                                    let p = pts[node_idx + 1];
                                    let hp = Pos2::new(
                                        shape.x + p.0 * cos_a - p.1 * sin_a,
                                        shape.y + p.0 * sin_a + p.1 * cos_a,
                                    );
                                    if hp.distance(Pos2::new(wx, wy)) <= 6.0 / self.zoom {
                                        self.dragging_node_handle =
                                            Some((shape_idx, node_idx, NodeHandleKind::Out));
                                    }
                                }
                            }
                        }
                    }
                }

                // Check for rotation handle first if a single shape is selected
                if self.selected_shape_idx.len() == 1 && self.dragging_node_handle.is_none() {
                    let s_idx = *self.selected_shape_idx.iter().next().unwrap();
                    if let Some(shape) = shapes.get(s_idx) {
                        let handle_pos = self.get_rotation_handle_pos(shape);
                        if (handle_pos.x - wx).abs() < 8.0 / self.zoom
                            && (handle_pos.y - wy).abs() < 8.0 / self.zoom
                        {
                            self.dragging_rotation = Some(s_idx);
                        }
                    }
                }

                if self.dragging_rotation.is_none()
                    && self.dragging_node_handle.is_none()
                    && self.hover_shape_idx.is_none()
                    && !self.node_edit_mode
                {
                    if is_multi {
                        // Ctrl/Shift + drag on empty space → selection box
                        self.selection_box_start = Some(Pos2::new(wx, wy));
                        self.selection_box_end = Some(Pos2::new(wx, wy));
                        self.selection_box_additive = is_multi;
                    } else {
                        // Plain drag on empty space → pan the view
                        self._drag_start = Some(Pos2::new(wx, wy));
                    }
                }
            }

            if response.clicked_by(egui::PointerButton::Primary) && self.dragging_rotation.is_none()
            {
                if let Some(idx) = self.hover_shape_idx {
                    let mut add_node_action: Option<InteractiveAction> = None;
                    if self.node_edit_mode {
                        let mut node_hit = None;
                        if let Some(shape) = shapes.get(idx) {
                            if let ShapeKind::Path(pts) = &shape.shape {
                                let angle = shape.rotation.to_radians();
                                let (sin_a, cos_a) = angle.sin_cos();
                                for (v_idx, p) in pts.iter().enumerate() {
                                    let vp = Pos2::new(
                                        shape.x + p.0 * cos_a - p.1 * sin_a,
                                        shape.y + p.0 * sin_a + p.1 * cos_a,
                                    );
                                    if (vp.x - wx).abs() < 4.0 / self.zoom
                                        && (vp.y - wy).abs() < 4.0 / self.zoom
                                    {
                                        node_hit = Some(v_idx);
                                        break;
                                    }
                                }

                                if node_hit.is_none() && pts.len() > 1 {
                                    let click = Pos2::new(wx, wy);
                                    let mut best_dist = f32::MAX;
                                    let mut best_seg = 0usize;
                                    let mut best_point = click;
                                    for i in 0..(pts.len() - 1) {
                                        let a = Pos2::new(
                                            shape.x + pts[i].0 * cos_a - pts[i].1 * sin_a,
                                            shape.y + pts[i].0 * sin_a + pts[i].1 * cos_a,
                                        );
                                        let b = Pos2::new(
                                            shape.x + pts[i + 1].0 * cos_a - pts[i + 1].1 * sin_a,
                                            shape.y + pts[i + 1].0 * sin_a + pts[i + 1].1 * cos_a,
                                        );
                                        let (dist, proj) = point_segment_distance(click, a, b);
                                        if dist < best_dist {
                                            best_dist = dist;
                                            best_seg = i;
                                            best_point = proj;
                                        }
                                    }

                                    if best_dist <= 8.0 / self.zoom {
                                        add_node_action = Some(InteractiveAction::AddNode {
                                            shape_idx: idx,
                                            insert_after: best_seg,
                                            new_pos: best_point,
                                        });
                                    }
                                }
                            }
                        }

                        if let Some(node_idx) = node_hit {
                            self.selected_node = Some((idx, node_idx));
                            if is_multi {
                                let key = (idx, node_idx);
                                if self.selected_nodes.contains(&key) {
                                    self.selected_nodes.shift_remove(&key);
                                } else {
                                    self.selected_nodes.insert(key);
                                }
                            } else {
                                self.selected_nodes.clear();
                                self.selected_nodes.insert((idx, node_idx));
                            }
                        } else if !is_multi {
                            self.selected_node = None;
                            self.selected_nodes.clear();
                        }
                    }

                    if is_multi {
                        if self.selected_shape_idx.contains(&idx) {
                            self.selected_shape_idx.shift_remove(&idx);
                        } else {
                            self.selected_shape_idx.insert(idx);
                        }
                    } else {
                        self.selected_shape_idx.clear();
                        self.selected_shape_idx.insert(idx);
                    }
                    action =
                        add_node_action.unwrap_or(InteractiveAction::SelectShape(idx, is_multi));
                } else if !is_multi {
                    self.selected_shape_idx.clear();
                    self.selected_node = None;
                    self.selected_nodes.clear();
                    action = InteractiveAction::Deselect;
                }
            }
        }

        // ── Left mouse drag: pan / shape/node operations ──
        if response.dragged_by(egui::PointerButton::Primary) {
            if self._drag_start.is_some() {
                // Pan the view (left-drag on empty space)
                let delta = response.drag_delta();
                self.pan += delta;
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
            } else if self.selection_box_start.is_some() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let ex = (pos.x - self.pan.x) / self.zoom;
                    let ey = -(pos.y - self.pan.y) / self.zoom;
                    self.selection_box_end = Some(Pos2::new(ex, ey));
                }
            } else if let Some((shape_idx, node_idx, handle)) = self.dragging_node_handle {
                if let Some(pos) = response.interact_pointer_pos() {
                    let dx = (pos.x - self.pan.x) / self.zoom;
                    let dy = -(pos.y - self.pan.y) / self.zoom;
                    action = InteractiveAction::MoveNodeHandle {
                        shape_idx,
                        node_idx,
                        handle,
                        new_pos: Pos2::new(dx, dy),
                    };
                }
            } else if let Some(s_idx) = self.dragging_rotation {
                if let Some(pos) = response.interact_pointer_pos() {
                    let wx = (pos.x - self.pan.x) / self.zoom;
                    let wy = -(pos.y - self.pan.y) / self.zoom;
                    if let Some(shape) = shapes.get(s_idx) {
                        let (lcx, lcy) = shape.local_center();
                        let (wcx, wcy) = shape.world_pos(lcx, lcy);
                        let dx = wx - wcx;
                        let dy = wy - wcy;

                        // Handle is at (width + offset, height + offset) in local coords
                        let offset = 2.0;
                        let (local_lx, local_ly) = match &shape.shape {
                            ShapeKind::Rectangle => (shape.width, shape.height),
                            ShapeKind::Circle => (shape.radius, shape.radius),
                            ShapeKind::TextLine => {
                                let char_w = shape.font_size_mm * 0.6;
                                (shape.text.len() as f32 * char_w, shape.font_size_mm)
                            }
                            ShapeKind::Path(pts) => {
                                let mut max_x: f32 = 0.0;
                                let mut max_y: f32 = 0.0;
                                for p in pts {
                                    max_x = max_x.max(p.0);
                                    max_y = max_y.max(p.1);
                                }
                                (max_x, max_y)
                            }
                            ShapeKind::RasterImage { params, .. } => {
                                (params.width_mm, params.height_mm)
                            }
                        };

                        // Angle from center to top-right handle in local space
                        let base_angle = (local_ly + offset - lcy)
                            .atan2(local_lx + offset - lcx)
                            .to_degrees();
                        let current_angle = dy.atan2(dx).to_degrees();
                        let new_rotation = current_angle - base_angle;
                        let delta = new_rotation - shape.rotation;

                        action = InteractiveAction::RotateSelection {
                            shape_idx: s_idx,
                            delta_deg: delta,
                        };
                    }
                }
            } else if self.node_edit_mode && self.selected_node.is_some() {
                let (s_idx, v_idx) = self.selected_node.unwrap();
                if let Some(pos) = response.interact_pointer_pos() {
                    let dx = (pos.x - self.pan.x) / self.zoom;
                    let dy = -(pos.y - self.pan.y) / self.zoom;
                    action = InteractiveAction::MoveNode {
                        shape_idx: s_idx,
                        node_idx: v_idx,
                        new_pos: Pos2::new(dx, dy),
                    };
                }
            } else if !self.selected_shape_idx.is_empty() {
                let delta = response.drag_delta();
                action = InteractiveAction::DragSelection {
                    delta: Vec2::new(delta.x / self.zoom, -delta.y / self.zoom),
                };
            }
        } else if !response.dragged_by(egui::PointerButton::Middle) {
            if response.drag_stopped_by(egui::PointerButton::Primary) {
                if let (Some(start), Some(end)) = (
                    self.selection_box_start.take(),
                    self.selection_box_end.take(),
                ) {
                    let hit_indices = self.shapes_in_world_rect(start, end, shapes);
                    if !self.selection_box_additive {
                        self.selected_shape_idx.clear();
                    }
                    for idx in hit_indices {
                        self.selected_shape_idx.insert(idx);
                    }
                    self.selected_node = None;
                    self.selected_nodes.clear();

                    if let Some(last) = self.selected_shape_idx.iter().last().copied() {
                        action = InteractiveAction::SelectShape(last, true);
                    } else if !self.selection_box_additive {
                        action = InteractiveAction::Deselect;
                    }
                    self.selection_box_additive = false;
                }
            }
            self._drag_start = None;
            self.dragging_rotation = None;
            self.dragging_node_handle = None;
        }

        // Set cursor
        if self.hover_shape_idx.is_some() || self.dragging_rotation.is_some() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        action
    }

    fn show_context_menu(
        &self,
        _ui: &egui::Ui,
        response: &egui::Response,
        has_selection: bool,
    ) -> InteractiveAction {
        let mut ctx_action = InteractiveAction::None;
        response.clone().context_menu(|ui| {
            if has_selection {
                if ui.button(format!("�  {}", tr("Copy"))).clicked() {
                    ctx_action = InteractiveAction::ContextCopy;
                    ui.close_menu();
                }
                if ui.button(format!("✂  {}", tr("Cut"))).clicked() {
                    ctx_action = InteractiveAction::ContextCut;
                    ui.close_menu();
                }
            }
            if ui.button(format!("📌  {}", tr("Paste"))).clicked() {
                ctx_action = InteractiveAction::ContextPaste;
                ui.close_menu();
            }
            if has_selection {
                ui.separator();
                if ui.button(format!("📋  {}", tr("Duplicate"))).clicked() {
                    ctx_action = InteractiveAction::ContextDuplicateSelection;
                    ui.close_menu();
                }
                if ui.button(format!("�  {}", tr("Delete"))).clicked() {
                    ctx_action = InteractiveAction::ContextDeleteSelection;
                    ui.close_menu();
                }
                ui.separator();
                if ui.button(format!("🔗  {}", tr("Group"))).clicked() {
                    ctx_action = InteractiveAction::ContextGroupSelection;
                    ui.close_menu();
                }
                if ui.button(format!("✂  {}", tr("Ungroup"))).clicked() {
                    ctx_action = InteractiveAction::ContextUngroupSelection;
                    ui.close_menu();
                }
            }
            ui.separator();
            if ui.button(format!("☑  {}", tr("Select All"))).clicked() {
                ctx_action = InteractiveAction::ContextSelectAll;
                ui.close_menu();
            }
        });
        ctx_action
    }

    fn is_point_in_shape(&self, wx: f32, wy: f32, shape: &ShapeParams) -> bool {
        let (lx, ly) = self.world_to_local(wx, wy, shape);
        match &shape.shape {
            ShapeKind::Rectangle => {
                lx >= 0.0 && lx <= shape.width && ly >= 0.0 && ly <= shape.height
            }
            ShapeKind::Circle => (lx * lx + ly * ly).sqrt() <= shape.radius,
            ShapeKind::TextLine => {
                let char_w = shape.font_size_mm * 0.6;
                let w = shape.text.len() as f32 * char_w;
                lx >= 0.0 && lx <= w && ly >= 0.0 && ly <= shape.font_size_mm
            }
            ShapeKind::Path(pts) => {
                // Re-use bounding box for hit test for paths (simplified)
                let mut min_x = f32::MAX;
                let mut max_x = f32::MIN;
                let mut min_y = f32::MAX;
                let mut max_y = f32::MIN;
                for p in pts {
                    min_x = min_x.min(p.0);
                    max_x = max_x.max(p.0);
                    min_y = min_y.min(p.1);
                    max_y = max_y.max(p.1);
                }
                lx >= min_x && lx <= max_x && ly >= min_y && ly <= max_y
            }
            ShapeKind::RasterImage { params, .. } => {
                lx >= 0.0 && lx <= params.width_mm && ly >= 0.0 && ly <= params.height_mm
            }
        }
    }

    fn get_rotation_handle_pos(&self, shape: &ShapeParams) -> Pos2 {
        let angle = shape.rotation.to_radians();
        let offset = 2.0; // World units (mm) away from the corner

        // Find local top-right corner
        let (lx, ly) = match &shape.shape {
            ShapeKind::Rectangle => (shape.width, shape.height),
            ShapeKind::Circle => (shape.radius, shape.radius),
            ShapeKind::TextLine => {
                let char_w = shape.font_size_mm * 0.6;
                (shape.text.len() as f32 * char_w, shape.font_size_mm)
            }
            ShapeKind::Path(pts) => {
                let mut max_x: f32 = 0.0;
                let mut max_y: f32 = 0.0;
                for p in pts {
                    max_x = max_x.max(p.0);
                    max_y = max_y.max(p.1);
                }
                (max_x, max_y)
            }
            ShapeKind::RasterImage { params, .. } => (params.width_mm, params.height_mm),
        };

        // Add offset in local coordinates
        let lx_f = lx + offset;
        let ly_f = ly + offset;

        let wx = shape.x + lx_f * angle.cos() - ly_f * angle.sin();
        let wy = shape.y + lx_f * angle.sin() + ly_f * angle.cos();
        Pos2::new(wx, wy)
    }

    fn world_to_local(&self, wx: f32, wy: f32, shape: &ShapeParams) -> (f32, f32) {
        let dx = wx - shape.x;
        let dy = wy - shape.y;
        let angle = -shape.rotation.to_radians();
        let lx = dx * angle.cos() - dy * angle.sin();
        let ly = dx * angle.sin() + dy * angle.cos();
        (lx, ly)
    }

    fn shapes_in_world_rect(&self, start: Pos2, end: Pos2, shapes: &[ShapeParams]) -> Vec<usize> {
        let min_x = start.x.min(end.x);
        let min_y = start.y.min(end.y);
        let max_x = start.x.max(end.x);
        let max_y = start.y.max(end.y);

        shapes
            .iter()
            .enumerate()
            .filter_map(|(idx, shape)| {
                let (sx0, sy0, sx1, sy1) = self.shape_world_bounds(shape)?;
                let intersects = sx1 >= min_x && sx0 <= max_x && sy1 >= min_y && sy0 <= max_y;
                if intersects { Some(idx) } else { None }
            })
            .collect()
    }

    fn shape_world_bounds(&self, shape: &ShapeParams) -> Option<(f32, f32, f32, f32)> {
        fn transform(shape: &ShapeParams, lx: f32, ly: f32) -> (f32, f32) {
            let angle = shape.rotation.to_radians();
            let (sin_a, cos_a) = angle.sin_cos();
            (
                shape.x + lx * cos_a - ly * sin_a,
                shape.y + lx * sin_a + ly * cos_a,
            )
        }

        let points: Vec<(f32, f32)> = match &shape.shape {
            ShapeKind::Rectangle => vec![
                transform(shape, 0.0, 0.0),
                transform(shape, shape.width, 0.0),
                transform(shape, shape.width, shape.height),
                transform(shape, 0.0, shape.height),
            ],
            ShapeKind::Circle => vec![
                (shape.x - shape.radius, shape.y - shape.radius),
                (shape.x + shape.radius, shape.y + shape.radius),
            ],
            ShapeKind::TextLine => {
                let char_w = shape.font_size_mm * 0.6;
                let w = shape.text.len() as f32 * char_w;
                vec![
                    transform(shape, 0.0, 0.0),
                    transform(shape, w, 0.0),
                    transform(shape, w, shape.font_size_mm),
                    transform(shape, 0.0, shape.font_size_mm),
                ]
            }
            ShapeKind::Path(pts) => {
                if pts.is_empty() {
                    return None;
                }
                pts.iter()
                    .map(|(lx, ly)| transform(shape, *lx, *ly))
                    .collect()
            }
            ShapeKind::RasterImage { params, .. } => vec![
                transform(shape, 0.0, 0.0),
                transform(shape, params.width_mm, 0.0),
                transform(shape, params.width_mm, params.height_mm),
                transform(shape, 0.0, params.height_mm),
            ],
        };

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;
        for (x, y) in points {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
        Some((min_x, min_y, max_x, max_y))
    }

    fn world_to_screen(&self, wx: f32, wy: f32, _rect: Rect) -> Pos2 {
        Pos2::new(wx * self.zoom + self.pan.x, -wy * self.zoom + self.pan.y)
    }

    fn draw_grid(&self, painter: &Painter, rect: Rect, is_light: bool) {
        let minor_stroke = if is_light {
            Stroke::new(0.5, Color32::from_rgba_premultiplied(210, 210, 215, 255))
        } else {
            Stroke::new(0.5, Color32::from_rgba_premultiplied(35, 37, 48, 255))
        };
        let major_stroke = if is_light {
            Stroke::new(0.8, Color32::from_rgba_premultiplied(180, 180, 190, 255))
        } else {
            Stroke::new(0.8, Color32::from_rgba_premultiplied(58, 60, 75, 255))
        };
        let axis_stroke = if is_light {
            Stroke::new(1.5, Color32::from_rgb(140, 140, 150))
        } else {
            Stroke::new(1.5, theme::SURFACE2)
        };
        let workspace_stroke =
            Stroke::new(1.5, Color32::from_rgba_premultiplied(166, 227, 161, 120));
        let label_color = if is_light {
            Color32::from_rgb(100, 100, 120)
        } else {
            theme::OVERLAY0
        };

        // Adaptive grid spacing
        let (minor_step, major_step) = if self.zoom < 0.3 {
            (50.0_f32, 100.0_f32)
        } else if self.zoom < 1.0 {
            (10.0, 50.0)
        } else if self.zoom < 5.0 {
            (5.0, 10.0)
        } else {
            (1.0, 5.0)
        };

        let _min_wx = (rect.left() - self.pan.x) / self.zoom;
        let _max_wx = (rect.right() - self.pan.x) / self.zoom;
        let _min_wy = (self.pan.y - rect.bottom()) / self.zoom;
        let _max_wy = (self.pan.y - rect.top()) / self.zoom;

        // Vertical lines (X axis) constrained to workspace
        let mut wx = 0.0;
        while wx <= self.workspace_size.x {
            let sx = wx * self.zoom + self.pan.x;
            if sx >= rect.left() && sx <= rect.right() {
                let is_major = (wx / major_step).round() * major_step == wx.round();

                // Screen Y bounds for vertical lines
                let s_y_start = self.pan.y - self.workspace_size.y * self.zoom;
                let s_y_end = self.pan.y;

                painter.line_segment(
                    [
                        Pos2::new(sx, s_y_start.max(rect.top())),
                        Pos2::new(sx, s_y_end.min(rect.bottom())),
                    ],
                    if is_major { major_stroke } else { minor_stroke },
                );

                // Ruler label on major lines (Top edge of workspace)
                if is_major {
                    // Stay on top edge of workspace, but clamp to visible screen area
                    let label_y = s_y_start.max(rect.top() + 4.0);
                    if label_y <= s_y_end - 10.0 {
                        // Only draw if we're not squishing into the bottom edge
                        painter.text(
                            Pos2::new(sx + 2.0, label_y - 12.0),
                            egui::Align2::LEFT_TOP,
                            format!("{:.0}", wx),
                            egui::FontId::monospace(9.0),
                            label_color,
                        );
                    }
                }
            }
            wx += minor_step;
        }

        // Horizontal lines (Y axis) constrained to workspace
        let mut wy = 0.0;
        while wy <= self.workspace_size.y {
            let sy = -wy * self.zoom + self.pan.y;
            if sy >= rect.top() && sy <= rect.bottom() {
                let is_major = (wy / major_step).round() * major_step == wy.round();

                // Screen X bounds for horizontal lines
                let s_x_start = self.pan.x;
                let s_x_end = self.pan.x + self.workspace_size.x * self.zoom;

                painter.line_segment(
                    [
                        Pos2::new(s_x_start.max(rect.left()), sy),
                        Pos2::new(s_x_end.min(rect.right()), sy),
                    ],
                    if is_major { major_stroke } else { minor_stroke },
                );

                // Ruler label on major lines (Left edge of workspace)
                if is_major {
                    // Stay on left edge of workspace, but clamp to visible screen area
                    let label_x = s_x_start.max(rect.left() + 2.0);
                    if label_x <= s_x_end - 20.0 {
                        // Only draw if we're not squishing into the right edge
                        painter.text(
                            Pos2::new(label_x - 22.0, sy - 9.0),
                            egui::Align2::LEFT_TOP,
                            format!("{:.0}", wy),
                            egui::FontId::monospace(9.0),
                            label_color,
                        );
                    }
                }
            }
            wy += minor_step;
        }

        // Axis lines through origin (Only where they intersect workspace)
        let origin = Pos2::new(self.pan.x, self.pan.y);
        let s_y_start = self.pan.y - self.workspace_size.y * self.zoom;
        let s_x_end = self.pan.x + self.workspace_size.x * self.zoom;

        if origin.x >= rect.left() && origin.x <= rect.right() {
            painter.line_segment(
                [
                    Pos2::new(origin.x, s_y_start.max(rect.top())),
                    Pos2::new(origin.x, origin.y.min(rect.bottom())),
                ],
                axis_stroke,
            );
        }
        if origin.y >= rect.top() && origin.y <= rect.bottom() {
            painter.line_segment(
                [
                    Pos2::new(origin.x.max(rect.left()), origin.y),
                    Pos2::new(s_x_end.min(rect.right()), origin.y),
                ],
                axis_stroke,
            );
        }

        // Draw machine workspace boundary rect
        let ws_tl = Pos2::new(
            0.0 * self.zoom + self.pan.x,
            -self.workspace_size.y * self.zoom + self.pan.y,
        );
        let ws_br = Pos2::new(
            self.workspace_size.x * self.zoom + self.pan.x,
            0.0 * self.zoom + self.pan.y,
        );
        let ws_rect = Rect::from_min_max(ws_tl, ws_br);
        painter.rect_stroke(ws_rect, 0.0, workspace_stroke, egui::StrokeKind::Outside);

        // Corner label
        painter.text(
            ws_br + Vec2::new(-4.0, -14.0),
            egui::Align2::RIGHT_TOP,
            format!("{}×{}mm", self.workspace_size.x, self.workspace_size.y),
            egui::FontId::monospace(10.0),
            Color32::from_rgba_premultiplied(166, 227, 161, 180),
        );
    }
}

fn rect_contains_approx(rect: Rect, pos: Pos2) -> bool {
    let margin = 100.0;
    pos.x >= rect.left() - margin
        && pos.x <= rect.right() + margin
        && pos.y >= rect.top() - margin
        && pos.y <= rect.bottom() + margin
}

fn point_segment_distance(p: Pos2, a: Pos2, b: Pos2) -> (f32, Pos2) {
    let ab = b - a;
    let ap = p - a;
    let ab_len_sq = ab.x * ab.x + ab.y * ab.y;
    if ab_len_sq <= f32::EPSILON {
        return (p.distance(a), a);
    }

    let t = ((ap.x * ab.x + ap.y * ab.y) / ab_len_sq).clamp(0.0, 1.0);
    let proj = a + ab * t;
    (p.distance(proj), proj)
}

fn thermal_risk_heatmap<F>(
    segments: &[PreviewSegment],
    cell_mm: f32,
    progress: Option<f32>,
    transform: &F,
) -> HashMap<(i32, i32), f32>
where
    F: Fn(f32, f32) -> Pos2,
{
    let mut out = HashMap::new();
    if segments.is_empty() {
        return out;
    }

    let max_idx = if let Some(p) = progress {
        let p = p.clamp(0.0, 1.0);
        (((segments.len() - 1) as f32 * p).round() as usize + 1).min(segments.len())
    } else {
        segments.len()
    };

    for seg in segments.iter().take(max_idx) {
        if !seg.laser_on {
            continue;
        }

        let len = (seg.x2 - seg.x1).hypot(seg.y2 - seg.y1).max(0.01);
        let samples = ((len / cell_mm.max(0.5)).ceil() as i32).max(1);
        let sample_weight = seg.power.max(0.01) * len / samples as f32;

        for i in 0..=samples {
            let t = i as f32 / samples as f32;
            let x = seg.x1 + (seg.x2 - seg.x1) * t;
            let y = seg.y1 + (seg.y2 - seg.y1) * t;
            let pt = transform(x, y);
            let cx = (pt.x / cell_mm).floor() as i32;
            let cy = (pt.y / cell_mm).floor() as i32;
            *out.entry((cx, cy)).or_insert(0.0) += sample_weight;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(x1: f32, y1: f32, x2: f32, y2: f32, power: f32) -> PreviewSegment {
        PreviewSegment {
            x1,
            y1,
            x2,
            y2,
            power,
            layer_id: 0,
            laser_on: true,
        }
    }

    #[test]
    fn thermal_risk_detects_overlapped_paths() {
        let segments = vec![
            seg(0.0, 0.0, 20.0, 0.0, 1.0),
            seg(0.0, 0.0, 20.0, 0.0, 1.0),
            seg(0.0, 0.0, 20.0, 0.0, 1.0),
        ];

        let heat = thermal_risk_heatmap(&segments, 2.0, None, &|x, y| egui::pos2(x, y));
        let risky_cells = heat.values().filter(|score| **score >= 4.0).count();
        assert!(risky_cells > 0);
    }

    #[test]
    fn thermal_risk_progress_limits_alerts() {
        let segments = vec![
            seg(0.0, 0.0, 20.0, 0.0, 1.0),
            seg(0.0, 2.0, 20.0, 2.0, 1.0),
            seg(0.0, 4.0, 20.0, 4.0, 1.0),
            seg(0.0, 6.0, 20.0, 6.0, 1.0),
        ];

        let full = thermal_risk_heatmap(&segments, 2.0, None, &|x, y| egui::pos2(x, y));
        let half = thermal_risk_heatmap(&segments, 2.0, Some(0.25), &|x, y| egui::pos2(x, y));

        let full_energy: f32 = full.values().copied().sum();
        let half_energy: f32 = half.values().copied().sum();
        assert!(half_energy < full_energy);
    }
}
