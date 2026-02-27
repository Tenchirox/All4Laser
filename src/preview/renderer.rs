use egui::{Color32, Painter, Pos2, Rect, Stroke, Vec2, Ui};
use crate::gcode::types::PreviewSegment;
use crate::theme;
use crate::ui::drawing::{ShapeParams, ShapeKind, DrawingState};
use crate::ui::layers_new::CutLayer;
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
    pub realistic_preview: bool, // Toggle for realistic material texture
    _drag_start: Option<Pos2>,
    pub selected_shape_idx: HashSet<usize>, // Selected indices in DrawingState (Multi-select)
    pub node_edit_mode: bool,
    pub selected_node: Option<(usize, usize)>, // (shape_idx, point_idx)
    pub hover_shape_idx: Option<usize>,
    pub dragging_rotation: Option<usize>, // shape_idx being rotated
    pub image_textures: std::collections::HashMap<usize, egui::TextureHandle>, // shape_idx -> texture
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
            realistic_preview: false,
            _drag_start: None,
            selected_shape_idx: HashSet::new(),
            node_edit_mode: false,
            selected_node: None,
            hover_shape_idx: None,
            dragging_rotation: None,
            image_textures: std::collections::HashMap::new(),
        }
    }
}

pub enum InteractiveAction {
    None,
    SelectShape(usize, bool), // idx, is_multi (ctrl/shift)
    Deselect,
    DragSelection { delta: Vec2 }, // Drag all selected
    RotateSelection { shape_idx: usize, delta_deg: f32 },
    MoveNode { shape_idx: usize, node_idx: usize, new_pos: Pos2 },
}

impl PreviewRenderer {
    /// Render the preview onto an egui Response area
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        segments: &[PreviewSegment],
        shapes: &[ShapeParams], // Drawing shapes
        _layers: &[CutLayer],   // New: to get colors
        is_light: bool,
        job_offset: Vec2,
        job_rotation_deg: f32,
        camera_state: &crate::ui::camera::CameraState,
    ) -> InteractiveAction {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
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
            self.draw_shape_overlay(ui, &painter, shape, rect, is_selected, idx, _layers);
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
        let rapid_stroke = if is_light || self.realistic_preview {
            Stroke::new(0.5, Color32::from_rgba_premultiplied(200, 200, 200, 150))
        } else {
            Stroke::new(0.5, Color32::from_rgba_premultiplied(69, 71, 90, 180))
        };

        let mut current_accumulated_line: Option<(Pos2, Pos2, f32, usize)> = None;

        let realistic = self.realistic_preview;

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

    fn draw_shape_overlay(&mut self, ui: &Ui, painter: &Painter, shape: &ShapeParams, rect: Rect, is_selected: bool, idx: usize, layers: &[CutLayer]) {
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
                            let wx = shape.x + pts[i+1].0 * angle.cos() - pts[i+1].1 * angle.sin();
                            let wy = shape.y + pts[i+1].0 * angle.sin() + pts[i+1].1 * angle.cos();
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
                        let processed = crate::imaging::raster::preprocess_image(&data.0, params);
                        let rgba = processed.to_rgba8();
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            [rgba.width() as _, rgba.height() as _],
                            rgba.as_flat_samples().as_slice(),
                        );
                        ui.ctx().load_texture(
                            format!("shape_{}", idx),
                            color_image,
                            Default::default()
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
                    let wx = shape.x + params.width_mm * angle.cos() - params.height_mm * angle.sin();
                    let wy = shape.y + params.width_mm * angle.sin() + params.height_mm * angle.cos();
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
                
                mesh.vertices.push(egui::epaint::Vertex { pos: p1, uv: egui::pos2(0.0, 0.0), color: Color32::WHITE });
                mesh.vertices.push(egui::epaint::Vertex { pos: p2, uv: egui::pos2(1.0, 0.0), color: Color32::WHITE });
                mesh.vertices.push(egui::epaint::Vertex { pos: p3, uv: egui::pos2(1.0, 1.0), color: Color32::WHITE });
                mesh.vertices.push(egui::epaint::Vertex { pos: p4, uv: egui::pos2(0.0, 1.0), color: Color32::WHITE });

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
            painter.add(egui::Shape::line(points.clone(), Stroke::new(1.2, theme::BLUE)));
            
            // Arrows at ends of arc
            let draw_arrow = |p: Pos2, angle: f32| {
                let head_len = 2.5;
                let head_angle = 0.5;
                painter.line_segment([p, p - Vec2::angled(angle + head_angle) * head_len], Stroke::new(1.2, theme::BLUE));
                painter.line_segment([p, p - Vec2::angled(angle - head_angle) * head_len], Stroke::new(1.2, theme::BLUE));
            };
            
            draw_arrow(*(points.last().unwrap()), angle_base + arc_span/2.0 + std::f32::consts::FRAC_PI_2);
            draw_arrow(*(points.first().unwrap()), angle_base - arc_span/2.0 - std::f32::consts::FRAC_PI_2);

            painter.circle_filled(s_handle, 3.5, theme::BLUE);
            painter.circle_stroke(s_handle, 5.0, Stroke::new(1.0, Color32::WHITE));

            // Small square handles at corners for movement feedback (placeholder for full transform handles)
            let handle_size = 4.0;
            painter.rect_filled(Rect::from_center_size(s_center, Vec2::splat(handle_size)), 1.0, theme::BLUE);
            
            // Node Editing Handles
            if self.node_edit_mode {
                if let ShapeKind::Path(pts) = &shape.shape {
                    for (v_idx, p) in pts.iter().enumerate() {
                        let vp = {
                            let wx = shape.x + p.0 * angle.cos() - p.1 * angle.sin();
                            let wy = shape.y + p.0 * angle.sin() + p.1 * angle.cos();
                            self.world_to_screen(wx, wy, rect)
                        };
                        let is_node_sel = self.selected_node == Some((idx, v_idx));
                        let color = if is_node_sel { theme::RED } else { theme::GREEN };
                        painter.circle_filled(vp, 4.0, color);
                    }
                }
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

    fn handle_input(&mut self, ui: &egui::Ui, response: &egui::Response, _rect: Rect, shapes: &[ShapeParams]) -> InteractiveAction {
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

            // Convert screen hover to world
            let wx = (hover.x - self.pan.x) / self.zoom;
            let wy = -(hover.y - self.pan.y) / self.zoom;
            let is_multi = ui.input(|i| i.modifiers.ctrl || i.modifiers.shift);

            // 1. Detect Hover
            self.hover_shape_idx = None;
            for (idx, shape) in shapes.iter().enumerate() {
                if self.is_point_in_shape(wx, wy, shape) {
                    self.hover_shape_idx = Some(idx);
                }
            }

            // 2. Click/Drag Selection Logic
            if response.drag_started() {
                // Check for rotation handle first if a single shape is selected
                if self.selected_shape_idx.len() == 1 {
                    let s_idx = *self.selected_shape_idx.iter().next().unwrap();
                    if let Some(shape) = shapes.get(s_idx) {
                        let handle_pos = self.get_rotation_handle_pos(shape);
                        if (handle_pos.x - wx).abs() < 8.0 / self.zoom && (handle_pos.y - wy).abs() < 8.0 / self.zoom {
                            self.dragging_rotation = Some(s_idx);
                        }
                    }
                }
            }

            if response.clicked() && self.dragging_rotation.is_none() {
                if let Some(idx) = self.hover_shape_idx {
                    if self.node_edit_mode {
                        let mut node_hit = None;
                        if let Some(shape) = shapes.get(idx) {
                            if let ShapeKind::Path(pts) = &shape.shape {
                                for (v_idx, p) in pts.iter().enumerate() {
                                    let vp = Pos2::new(shape.x + p.0, shape.y + p.1);
                                    if (vp.x - wx).abs() < 4.0 / self.zoom && (vp.y - wy).abs() < 4.0 / self.zoom {
                                        node_hit = Some(v_idx);
                                        break;
                                    }
                                }
                            }
                        }
                        self.selected_node = node_hit.map(|n| (idx, n));
                    }

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
                    self.selected_node = None;
                    action = InteractiveAction::Deselect;
                }
            }
        }

        if response.dragged() {
            if let Some(s_idx) = self.dragging_rotation {
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
                                let mut max_x: f32 = 0.0; let mut max_y: f32 = 0.0;
                                for p in pts {
                                    max_x = max_x.max(p.0);
                                    max_y = max_y.max(p.1);
                                }
                                (max_x, max_y)
                            }
                            ShapeKind::RasterImage { params, .. } => (params.width_mm, params.height_mm),
                        };
                        
                        // Angle from center to top-right handle in local space
                        let base_angle = (local_ly + offset - lcy).atan2(local_lx + offset - lcx).to_degrees();
                        let current_angle = dy.atan2(dx).to_degrees();
                        let new_rotation = current_angle - base_angle;
                        let delta = new_rotation - shape.rotation;
                        
                        action = InteractiveAction::RotateSelection { shape_idx: s_idx, delta_deg: delta };
                    }
                }
            } else if self.node_edit_mode && self.selected_node.is_some() {
                 let (s_idx, v_idx) = self.selected_node.unwrap();
                 if let Some(pos) = response.interact_pointer_pos() {
                     let dx = (pos.x - self.pan.x) / self.zoom;
                     let dy = -(pos.y - self.pan.y) / self.zoom;
                     action = InteractiveAction::MoveNode { shape_idx: s_idx, node_idx: v_idx, new_pos: Pos2::new(dx, dy) };
                 }
            } else if !self.selected_shape_idx.is_empty() {
                let delta = response.drag_delta();
                action = InteractiveAction::DragSelection {
                    delta: Vec2::new(delta.x / self.zoom, -delta.y / self.zoom)
                };
            } else {
                self.pan += response.drag_delta();
            }
        } else {
            self.dragging_rotation = None;
        }

        // Set cursor
        if self.hover_shape_idx.is_some() || self.dragging_rotation.is_some() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        action
    }

    fn is_point_in_shape(&self, wx: f32, wy: f32, shape: &ShapeParams) -> bool {
        let (lx, ly) = self.world_to_local(wx, wy, shape);
        match &shape.shape {
            ShapeKind::Rectangle => lx >= 0.0 && lx <= shape.width && ly >= 0.0 && ly <= shape.height,
            ShapeKind::Circle => (lx*lx + ly*ly).sqrt() <= shape.radius,
            ShapeKind::TextLine => {
                let char_w = shape.font_size_mm * 0.6;
                let w = shape.text.len() as f32 * char_w;
                lx >= 0.0 && lx <= w && ly >= 0.0 && ly <= shape.font_size_mm
            }
            ShapeKind::Path(pts) => {
                // Re-use bounding box for hit test for paths (simplified)
                let mut min_x = f32::MAX; let mut max_x = f32::MIN;
                let mut min_y = f32::MAX; let mut max_y = f32::MIN;
                for p in pts {
                    min_x = min_x.min(p.0); max_x = max_x.max(p.0);
                    min_y = min_y.min(p.1); max_y = max_y.max(p.1);
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
                    [Pos2::new(sx, s_y_start.max(rect.top())), Pos2::new(sx, s_y_end.min(rect.bottom()))],
                    if is_major { major_stroke } else { minor_stroke },
                );

                // Ruler label on major lines (Top edge of workspace)
                if is_major {
                    // Stay on top edge of workspace, but clamp to visible screen area
                    let label_y = s_y_start.max(rect.top() + 4.0);
                    if label_y <= s_y_end - 10.0 { // Only draw if we're not squishing into the bottom edge
                        painter.text(
                            Pos2::new(sx + 2.0, label_y),
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
                    [Pos2::new(s_x_start.max(rect.left()), sy), Pos2::new(s_x_end.min(rect.right()), sy)],
                    if is_major { major_stroke } else { minor_stroke },
                );

                // Ruler label on major lines (Left edge of workspace)
                if is_major {
                    // Stay on left edge of workspace, but clamp to visible screen area
                    let label_x = s_x_start.max(rect.left() + 2.0);
                    if label_x <= s_x_end - 20.0 { // Only draw if we're not squishing into the right edge
                        painter.text(
                            Pos2::new(label_x, sy - 9.0),
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
                [Pos2::new(origin.x, s_y_start.max(rect.top())), Pos2::new(origin.x, origin.y.min(rect.bottom()))],
                axis_stroke,
            );
        }
        if origin.y >= rect.top() && origin.y <= rect.bottom() {
            painter.line_segment(
                [Pos2::new(origin.x.max(rect.left()), origin.y), Pos2::new(s_x_end.min(rect.right()), origin.y)],
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
