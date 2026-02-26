use egui::{Color32, Painter, Pos2, Rect, Stroke, Vec2};
use crate::gcode::types::PreviewSegment;
use crate::theme;
use crate::ui::drawing::{ShapeParams, ShapeKind};
use std::collections::HashSet;

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
    pub show_rapids: bool, // Toggle for G0 moves
    _drag_start: Option<Pos2>,
    pub selected_shape_idx: HashSet<usize>, // Selected indices in DrawingState (Multi-select)
}

impl Default for PreviewRenderer {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan: Vec2::ZERO,
            machine_pos: Pos2::ZERO,
            workspace_size: Vec2::new(400.0, 400.0), // conservative default
            simulation_progress: None,
            show_rapids: true,
            _drag_start: None,
            selected_shape_idx: HashSet::new(),
        }
    }
}

pub enum InteractiveAction {
    None,
    SelectShape(usize, bool), // idx, is_multi (ctrl/shift)
    Deselect,
    DragSelection { delta: Vec2 }, // Drag all selected
}

impl PreviewRenderer {
    /// Render the preview onto an egui Response area
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        segments: &[PreviewSegment],
        shapes: &[ShapeParams], // Drawing shapes
        is_light: bool,
        job_offset: Vec2,
        job_rotation_deg: f32,
        camera_state: &crate::ui::camera::CameraState,
    ) -> InteractiveAction {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());
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
            Pos2::new(rx + job_center.x + job_offset.x, ry + job_center.y + job_offset.y)
        };

        // Background
        let bg_color = if is_light { Color32::from_rgb(250, 250, 250) } else { theme::CRUST };
        painter.rect_filled(rect, 0.0, bg_color);

        // Draw camera image if available and enabled
        if camera_state.enabled {
            if let Some(texture) = &camera_state.texture {
                let calib = &camera_state.calibration;
                
                let cam_rect_w = Rect::from_min_size(
                    Pos2::new(calib.offset_x, calib.offset_y),
                    Vec2::new(400.0 * calib.scale, 400.0 * calib.scale)
                );
                
                let uv = Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0));
                let mut mesh = egui::Mesh::with_texture(texture.id());
                
                let p1 = self.world_to_screen(cam_rect_w.min.x, cam_rect_w.min.y, rect);
                let p2 = self.world_to_screen(cam_rect_w.max.x, cam_rect_w.max.y, rect);
                
                let color = Color32::from_white_alpha((camera_state.opacity * 255.0) as u8);
                
                mesh.add_rect_with_uv(Rect::from_min_max(p1, p2), uv, color);
                painter.add(mesh);
            }
        }

        // Handle generic input (Pan/Zoom) vs Selection input
        let handled_interaction = self.handle_input(ui, &response, rect, shapes);
        if let InteractiveAction::None = handled_interaction {
             // If interaction wasn't a selection/drag, proceed standard input handling
        } else {
            action = handled_interaction;
        }

        // Draw grid
        self.draw_grid(&painter, rect, is_light);

        // Draw GCode segments
        self.draw_gcode_segments(&painter, segments, rect, is_light, transform);

        // Draw interactive shapes (Overlay)
        for (idx, shape) in shapes.iter().enumerate() {
            let is_selected = self.selected_shape_idx.contains(&idx);
            self.draw_shape_overlay(&painter, shape, rect, is_selected);
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
            [Pos2::new(origin.x - 15.0, origin.y), Pos2::new(origin.x + 15.0, origin.y)],
            Stroke::new(1.0, theme::SURFACE2),
        );
        painter.line_segment(
            [Pos2::new(origin.x, origin.y - 15.0), Pos2::new(origin.x, origin.y + 15.0)],
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

    fn draw_gcode_segments<F>(&self, painter: &Painter, segments: &[PreviewSegment], rect: Rect, is_light: bool, transform: F)
    where F: Fn(f32, f32) -> Pos2
    {
        let rapid_stroke = if is_light {
            Stroke::new(0.5, Color32::from_rgba_premultiplied(200, 200, 200, 150))
        } else {
            Stroke::new(0.5, Color32::from_rgba_premultiplied(69, 71, 90, 180))
        };

        let mut current_accumulated_line: Option<(Pos2, Pos2, f32, usize)> = None;

        let flush_accumulated = |painter: &egui::Painter, accum: &mut Option<(Pos2, Pos2, f32, usize)>| {
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

                let color = if is_light {
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
                    if let Some((acc_p1, acc_p2, total_power, count)) = current_accumulated_line.as_mut() {
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

    fn draw_shape_overlay(&self, painter: &Painter, shape: &ShapeParams, rect: Rect, is_selected: bool) {
        // Calculate shape bounds in world
        let bounds = match shape.shape {
            ShapeKind::Rectangle => Rect::from_min_size(
                Pos2::new(shape.x, shape.y),
                Vec2::new(shape.width, shape.height)
            ),
            ShapeKind::Circle => Rect::from_min_size(
                Pos2::new(shape.x - shape.radius, shape.y - shape.radius),
                Vec2::new(shape.radius * 2.0, shape.radius * 2.0)
            ),
            ShapeKind::TextLine => {
                // Rough estimation for text bounds
                let char_w = shape.font_size_mm * 0.6;
                let w = shape.text.len() as f32 * char_w;
                Rect::from_min_size(
                    Pos2::new(shape.x, shape.y),
                    Vec2::new(w, shape.font_size_mm)
                )
            }
        };

        // Correct Y flipping: world Y goes up, screen Y goes down.
        // world_to_screen flips Y.

        let p1 = self.world_to_screen(bounds.min.x, bounds.min.y, rect);
        let p2 = self.world_to_screen(bounds.max.x, bounds.max.y, rect);
        let screen_rect = Rect::from_min_max(p1, p2);

        if is_selected {
            // Draw marching ants or dashed line
            painter.rect_stroke(screen_rect, 0.0, Stroke::new(1.0, theme::BLUE), egui::StrokeKind::Middle);
            // Handles
            let handle_size = 6.0;
            let corners = [screen_rect.min, screen_rect.max, screen_rect.left_bottom(), screen_rect.right_top()];
            for c in corners {
                painter.rect_filled(Rect::from_center_size(c, Vec2::splat(handle_size)), 1.0, theme::BLUE);
            }
        }
    }

    /// Auto-fit all segments in view
    pub fn auto_fit(&mut self, segments: &[PreviewSegment], rect: Rect, job_offset: Vec2, job_rotation_deg: f32) {
        if segments.is_empty() {
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
            Pos2::new(rx + job_center.x + job_offset.x, ry + job_center.y + job_offset.y)
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

        let data_w = (max_x - min_x).max(0.001);
        let data_h = (max_y - min_y).max(0.001);

        let margin = 40.0;
        let view_w = rect.width() - margin * 2.0;
        let view_h = rect.height() - margin * 2.0;

        self.zoom = (view_w / data_w).min(view_h / data_h);
        let center_x = (min_x + max_x) / 2.0;
        let center_y = (min_y + max_y) / 2.0;

        self.pan = Vec2::new(
            rect.center().x - center_x * self.zoom,
            rect.center().y + center_y * self.zoom,
        );
    }

    pub fn zoom_in(&mut self) {
        self.zoom *= 1.3;
    }

    pub fn zoom_out(&mut self) {
        self.zoom /= 1.3;
    }

    fn handle_input(&mut self, ui: &egui::Ui, response: &egui::Response, rect: Rect, shapes: &[ShapeParams]) -> InteractiveAction {
        let mut action = InteractiveAction::None;

        if let Some(hover) = response.hover_pos() {
            // Zoom logic
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                let old_zoom = self.zoom;
                let factor = if scroll > 0.0 { 1.1 } else { 1.0 / 1.1 };
                self.zoom = (self.zoom * factor).clamp(0.01, 10000.0);
                self.pan.x = hover.x - (hover.x - self.pan.x) * (self.zoom / old_zoom);
                self.pan.y = hover.y - (hover.y - self.pan.y) * (self.zoom / old_zoom);
            }

            // Selection Logic (Click)
            if response.clicked() {
                // Convert screen hover to world
                let wx = (hover.x - self.pan.x) / self.zoom;
                let wy = -(hover.y - self.pan.y) / self.zoom;

                let is_multi = ui.input(|i| i.modifiers.ctrl || i.modifiers.shift);

                let mut hit_idx = None;
                // Simple hit testing against shape bounds
                for (idx, shape) in shapes.iter().enumerate() {
                    let bounds = match shape.shape {
                        ShapeKind::Rectangle => Rect::from_min_size(Pos2::new(shape.x, shape.y), Vec2::new(shape.width, shape.height)),
                        ShapeKind::Circle => Rect::from_min_size(Pos2::new(shape.x - shape.radius, shape.y - shape.radius), Vec2::new(shape.radius * 2.0, shape.radius * 2.0)),
                        ShapeKind::TextLine => {
                             let char_w = shape.font_size_mm * 0.6;
                             let w = shape.text.len() as f32 * char_w;
                             Rect::from_min_size(Pos2::new(shape.x, shape.y), Vec2::new(w, shape.font_size_mm))
                        }
                    };

                    if bounds.contains(Pos2::new(wx, wy)) {
                        hit_idx = Some(idx);
                    }
                }

                if let Some(idx) = hit_idx {
                    if is_multi {
                        if self.selected_shape_idx.contains(&idx) {
                            self.selected_shape_idx.remove(&idx);
                        } else {
                            self.selected_shape_idx.insert(idx);
                        }
                    } else {
                        self.selected_shape_idx.clear();
                        self.selected_shape_idx.insert(idx);
                    }
                    action = InteractiveAction::SelectShape(idx, is_multi);
                } else if !is_multi {
                    self.selected_shape_idx.clear();
                    action = InteractiveAction::Deselect;
                }
            }
        }

        // Dragging Logic
        // If we have a selection, and we are dragging, move the shapes
        if response.dragged() {
            if !self.selected_shape_idx.is_empty() {
                let delta = response.drag_delta();
                // Convert screen delta to world delta
                let world_dx = delta.x / self.zoom;
                let world_dy = -delta.y / self.zoom; // Flip Y

                action = InteractiveAction::DragSelection {
                    delta: Vec2::new(world_dx, world_dy)
                };
            } else {
                // Pan if no selection
                self.pan += response.drag_delta();
            }
        }

        action
    }

    fn world_to_screen(&self, wx: f32, wy: f32, _rect: Rect) -> Pos2 {
        Pos2::new(
            wx * self.zoom + self.pan.x,
            -wy * self.zoom + self.pan.y,
        )
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
        let workspace_stroke = Stroke::new(1.5, Color32::from_rgba_premultiplied(166, 227, 161, 120));
        let label_color = if is_light { Color32::from_rgb(100, 100, 120) } else { theme::OVERLAY0 };

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

        let min_wx = (rect.left() - self.pan.x) / self.zoom;
        let max_wx = (rect.right() - self.pan.x) / self.zoom;
        let min_wy = (self.pan.y - rect.bottom()) / self.zoom;
        let max_wy = (self.pan.y - rect.top()) / self.zoom;

        // Vertical lines
        let start_x = (min_wx / minor_step).floor() * minor_step;
        let mut wx = start_x;
        while wx <= max_wx {
            let sx = wx * self.zoom + self.pan.x;
            let is_major = (wx / major_step).round() * major_step == wx.round();
            painter.line_segment(
                [Pos2::new(sx, rect.top()), Pos2::new(sx, rect.bottom())],
                if is_major { major_stroke } else { minor_stroke },
            );
            // Ruler label on major lines
            if is_major && sx >= rect.left() + 4.0 {
                painter.text(
                    Pos2::new(sx + 2.0, rect.top() + 2.0),
                    egui::Align2::LEFT_TOP,
                    format!("{:.0}", wx),
                    egui::FontId::monospace(9.0),
                    label_color,
                );
            }
            wx += minor_step;
        }

        // Horizontal lines
        let start_y = (min_wy / minor_step).floor() * minor_step;
        let mut wy = start_y;
        while wy <= max_wy {
            let sy = -wy * self.zoom + self.pan.y;
            let is_major = (wy / major_step).round() * major_step == wy.round();
            painter.line_segment(
                [Pos2::new(rect.left(), sy), Pos2::new(rect.right(), sy)],
                if is_major { major_stroke } else { minor_stroke },
            );
            // Ruler label on major lines
            if is_major && sy >= rect.top() + 14.0 && sy <= rect.bottom() - 4.0 {
                painter.text(
                    Pos2::new(rect.left() + 2.0, sy - 9.0),
                    egui::Align2::LEFT_TOP,
                    format!("{:.0}", wy),
                    egui::FontId::monospace(9.0),
                    label_color,
                );
            }
            wy += minor_step;
        }

        // Axis lines through origin
        let origin = Pos2::new(self.pan.x, self.pan.y);
        if origin.x >= rect.left() && origin.x <= rect.right() {
            painter.line_segment(
                [Pos2::new(origin.x, rect.top()), Pos2::new(origin.x, rect.bottom())],
                axis_stroke,
            );
        }
        if origin.y >= rect.top() && origin.y <= rect.bottom() {
            painter.line_segment(
                [Pos2::new(rect.left(), origin.y), Pos2::new(rect.right(), origin.y)],
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
