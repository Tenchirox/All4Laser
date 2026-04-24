#![allow(dead_code)]
use crate::ui::drawing::{ImageData, PathData, PathSegment, ShapeKind, ShapeParams};
use crate::imaging::raster::RasterParams;
use serde_json::Value;
use std::sync::Arc;

/// Import an xTool Creative Space (.xcs) JSON file and return shapes.
pub fn import_xcs(content: &str) -> Result<Vec<ShapeParams>, String> {
    let root: Value =
        serde_json::from_str(content).map_err(|e| format!("XCS JSON parse error: {e}"))?;

    let active_canvas_id = root.get("canvasId").and_then(|v| v.as_str());

    let canvas = root
        .get("canvas")
        .and_then(|v| v.as_array())
        .ok_or("Missing 'canvas' array in XCS file")?;

    let selected_panels: Vec<&Value> = if let Some(active_id) = active_canvas_id {
        let matched: Vec<&Value> = canvas
            .iter()
            .filter(|panel| panel.get("id").and_then(|v| v.as_str()) == Some(active_id))
            .collect();
        if matched.is_empty() {
            canvas.iter().collect()
        } else {
            matched
        }
    } else {
        canvas.iter().collect()
    };

    let mut shapes = Vec::new();

    // Collect layerTags and assign layer indices
    // BITMAP layers get lower indices (rendered behind), PATH layers get higher
    let mut layer_map: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for panel in &selected_panels {
        if let Some(displays) = panel.get("displays").and_then(|v| v.as_array()) {
            for disp in displays {
                let tag = disp.get("layerTag").and_then(|v| v.as_str()).unwrap_or("#000000").to_string();
                let dtype = disp.get("type").and_then(|v| v.as_str()).unwrap_or("");
                // BITMAPs first (index 0, 1, ...), then PATHs
                if dtype == "BITMAP" && !layer_map.contains_key(&tag) {
                    layer_map.insert(tag, 0); // placeholder
                }
            }
            for disp in displays {
                let tag = disp.get("layerTag").and_then(|v| v.as_str()).unwrap_or("#000000").to_string();
                let dtype = disp.get("type").and_then(|v| v.as_str()).unwrap_or("");
                if dtype != "BITMAP" && !layer_map.contains_key(&tag) {
                    layer_map.insert(tag, 0); // placeholder
                }
            }
        }
    }
    // Assign sequential indices
    for (idx, (_, v)) in layer_map.iter_mut().enumerate() {
        *v = idx;
    }
    eprintln!("[XCS] layer map: {:?}", layer_map);

    eprintln!("[XCS] canvas panels imported: {}", selected_panels.len());
    for (pi, panel) in selected_panels.iter().enumerate() {
        let displays = match panel.get("displays").and_then(|v| v.as_array()) {
            Some(d) => d,
            None => { eprintln!("[XCS] panel {pi}: no displays"); continue; },
        };

        eprintln!("[XCS] panel {pi}: {} displays", displays.len());
        for (di, disp) in displays.iter().enumerate() {
            let dtype = disp
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let layer_tag = disp.get("layerTag").and_then(|v| v.as_str()).unwrap_or("#000000");
            let layer_idx = layer_map.get(layer_tag).copied().unwrap_or(0);

            eprintln!("[XCS]   display[{di}] type={dtype} layer={layer_tag} idx={layer_idx}");
            match dtype {
                "PATH" => {
                    match parse_path_display(disp, layer_idx) {
                        Some(s) => {
                            eprintln!("[XCS]     -> {} path shapes", s.len());
                            shapes.extend(s);
                        }
                        None => eprintln!("[XCS]     -> parse_path_display returned None"),
                    }
                }
                "BITMAP" => {
                    match parse_bitmap_display(disp, layer_idx) {
                        Some(s) => {
                            eprintln!("[XCS]     -> bitmap shape ok");
                            shapes.push(s);
                        }
                        None => eprintln!("[XCS]     -> parse_bitmap_display returned None"),
                    }
                }
                "LINE" => {
                    match parse_line_display(disp, layer_idx) {
                        Some(s) => {
                            eprintln!("[XCS]     -> line shape ok");
                            shapes.push(s);
                        }
                        None => eprintln!("[XCS]     -> parse_line_display returned None"),
                    }
                }
                "CIRCLE" | "ELLIPSE" => {
                    match parse_circle_display(disp, layer_idx) {
                        Some(s) => {
                            eprintln!("[XCS]     -> {} circle/ellipse shape(s)", s.len());
                            shapes.extend(s);
                        }
                        None => eprintln!("[XCS]     -> parse_circle_display returned None"),
                    }
                }
                _ => { eprintln!("[XCS]     -> skipped"); }
            }
        }
    }

    eprintln!("[XCS] total shapes: {}", shapes.len());
    if shapes.is_empty() {
        return Err("No shapes found in XCS file.".into());
    }

    // Normalize: shift all shapes so the design starts near (0, 0)
    let mut global_min_x = f32::MAX;
    let mut global_min_y = f32::MAX;
    for s in &shapes {
        global_min_x = global_min_x.min(s.x);
        global_min_y = global_min_y.min(s.y);
    }
    eprintln!("[XCS] global min: ({global_min_x}, {global_min_y})");
    for s in &mut shapes {
        s.x -= global_min_x;
        s.y -= global_min_y;
    }

    Ok(shapes)
}

/// Intermediate representation for a parsed SVG sub-path with Bézier data.
struct RawSubPath {
    start: (f32, f32),
    segments: Vec<PathSegment>,
}

impl RawSubPath {
    /// Collect all key points (start, endpoints, control points) for bounding box.
    fn all_points(&self) -> Vec<(f32, f32)> {
        let mut pts = vec![self.start];
        for seg in &self.segments {
            match seg {
                PathSegment::LineTo(x, y) => pts.push((*x, *y)),
                PathSegment::CubicBezier { c1, c2, end } => {
                    pts.push(*c1);
                    pts.push(*c2);
                    pts.push(*end);
                }
                PathSegment::QuadBezier { c, end } => {
                    pts.push(*c);
                    pts.push(*end);
                }
            }
        }
        pts
    }
}

/// Parse a PATH display element into one or more ShapeParams.
fn parse_path_display(disp: &Value, layer_idx: usize) -> Option<Vec<ShapeParams>> {
    let dpath = disp.get("dPath").and_then(|v| v.as_str())?;
    let angle = disp.get("angle").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let scale_x = disp
        .get("scale")
        .and_then(|v| v.get("x"))
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0) as f32;
    let scale_y = disp
        .get("scale")
        .and_then(|v| v.get("y"))
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0) as f32;
    let skew_x = disp
        .get("skew")
        .and_then(|v| v.get("x"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as f32;
    let skew_y = disp
        .get("skew")
        .and_then(|v| v.get("y"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as f32;
    let local_skew_x = disp
        .get("localSkew")
        .and_then(|v| v.get("x"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as f32;
    let local_skew_y = disp
        .get("localSkew")
        .and_then(|v| v.get("y"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as f32;
    let pivot_x = disp
        .get("pivot")
        .and_then(|v| v.get("x"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as f32;
    let pivot_y = disp
        .get("pivot")
        .and_then(|v| v.get("y"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as f32;
    let offset_x = disp.get("offsetX").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let offset_y = disp.get("offsetY").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;

    // Parse SVG path data into sub-paths with Bézier segments preserved
    let sub_paths = parse_svg_dpath(dpath);
    if sub_paths.is_empty() {
        return None;
    }

    // Native XCS transform order:
    // T(offset) * R(angle) * Skew(skew + localSkew) * S(scale) * T(-pivot)
    // Then convert canvas Y-down -> world Y-up.
    let angle_rad = angle.to_radians();
    let (sin_a, cos_a) = angle_rad.sin_cos();
    let tan_skew_x = (skew_x + local_skew_x).to_radians().tan();
    let tan_skew_y = (skew_y + local_skew_y).to_radians().tan();

    let xform = |px: f32, py: f32| -> (f32, f32) {
        let mut tx = px - pivot_x;
        let mut ty = py - pivot_y;

        tx *= scale_x;
        ty *= scale_y;

        if tan_skew_x.abs() > 1e-6 || tan_skew_y.abs() > 1e-6 {
            let sx = tx + tan_skew_x * ty;
            let sy = ty + tan_skew_y * tx;
            tx = sx;
            ty = sy;
        }

        let (rx, ry) = if angle_rad.abs() > 1e-6 {
            (tx * cos_a - ty * sin_a, tx * sin_a + ty * cos_a)
        } else {
            (tx, ty)
        };

        let canvas_x = rx + offset_x + pivot_x;
        let canvas_y = ry + offset_y + pivot_y;
        (canvas_x, -canvas_y)
    };

    let mut result = Vec::new();
    for sp in sub_paths {
        if sp.segments.is_empty() {
            continue;
        }

        // Transform start point and all segments (includes rotation baking)
        let t_start = xform(sp.start.0, sp.start.1);
        let t_segs: Vec<PathSegment> = sp
            .segments
            .iter()
            .map(|seg| match seg {
                PathSegment::LineTo(x, y) => {
                    let (tx, ty) = xform(*x, *y);
                    PathSegment::LineTo(tx, ty)
                }
                PathSegment::CubicBezier { c1, c2, end } => {
                    let tc1 = xform(c1.0, c1.1);
                    let tc2 = xform(c2.0, c2.1);
                    let tend = xform(end.0, end.1);
                    PathSegment::CubicBezier {
                        c1: tc1,
                        c2: tc2,
                        end: tend,
                    }
                }
                PathSegment::QuadBezier { c, end } => {
                    let tc = xform(c.0, c.1);
                    let tend = xform(end.0, end.1);
                    PathSegment::QuadBezier {
                        c: tc,
                        end: tend,
                    }
                }
            })
            .collect();

        // Create PathData with both segments and flattened points
        let path_data = PathData::from_segments(t_start, t_segs);
        if path_data.points.len() < 2 {
            continue;
        }

        // Compute bounding box from flattened points for ShapeParams offset
        let min_x = path_data.points.iter().map(|p| p.0).fold(f32::MAX, f32::min);
        let min_y = path_data.points.iter().map(|p| p.1).fold(f32::MAX, f32::min);

        // Make coordinates local (relative to bounding box origin)
        let local_start = (path_data.start.0 - min_x, path_data.start.1 - min_y);
        let local_segs: Vec<PathSegment> = path_data
            .segments
            .iter()
            .map(|seg| match seg {
                PathSegment::LineTo(x, y) => PathSegment::LineTo(x - min_x, y - min_y),
                PathSegment::CubicBezier { c1, c2, end } => PathSegment::CubicBezier {
                    c1: (c1.0 - min_x, c1.1 - min_y),
                    c2: (c2.0 - min_x, c2.1 - min_y),
                    end: (end.0 - min_x, end.1 - min_y),
                },
                PathSegment::QuadBezier { c, end } => PathSegment::QuadBezier {
                    c: (c.0 - min_x, c.1 - min_y),
                    end: (end.0 - min_x, end.1 - min_y),
                },
            })
            .collect();
        let local_path = PathData::from_segments(local_start, local_segs);

        result.push(ShapeParams {
            shape: ShapeKind::Path(local_path),
            x: min_x,
            y: min_y,
            rotation: 0.0,
            layer_idx,
            ..Default::default()
        });
    }

    Some(result)
}

/// Parse a LINE display element into a Path shape using native XCS transform.
fn parse_line_display(disp: &Value, layer_idx: usize) -> Option<ShapeParams> {
    let angle = disp.get("angle").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let offset_x = disp.get("offsetX").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let offset_y = disp.get("offsetY").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let scale_x = disp
        .get("scale")
        .and_then(|v| v.get("x"))
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0) as f32;
    let scale_y = disp
        .get("scale")
        .and_then(|v| v.get("y"))
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0) as f32;

    let end_x = disp
        .get("endPoint")
        .and_then(|v| v.get("x"))
        .and_then(|v| v.as_f64())
        .unwrap_or(disp.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0)) as f32;
    let end_y = disp
        .get("endPoint")
        .and_then(|v| v.get("y"))
        .and_then(|v| v.as_f64())
        .unwrap_or(disp.get("height").and_then(|v| v.as_f64()).unwrap_or(0.0)) as f32;

    // For LINE elements: start at (offsetX, offsetY), extend by (endPoint * scale), then rotate
    let angle_rad = angle.to_radians();
    let (sin_a, cos_a) = angle_rad.sin_cos();

    // Start point in canvas coords
    let start_canvas_x = offset_x;
    let start_canvas_y = offset_y;

    // End point is start + (endPoint * scale), then rotated
    let raw_dx = end_x * scale_x;
    let raw_dy = end_y * scale_y;

    // Apply rotation to the delta
    let rot_dx = raw_dx * cos_a - raw_dy * sin_a;
    let rot_dy = raw_dx * sin_a + raw_dy * cos_a;

    let end_canvas_x = start_canvas_x + rot_dx;
    let end_canvas_y = start_canvas_y + rot_dy;

    // Convert to world coords (Y-up)
    let p0 = (start_canvas_x, -start_canvas_y);
    let p1 = (end_canvas_x, -end_canvas_y);

    if (p0.0 - p1.0).abs() < 1e-6 && (p0.1 - p1.1).abs() < 1e-6 {
        return None;
    }

    let path_data = PathData::from_segments(p0, vec![PathSegment::LineTo(p1.0, p1.1)]);
    let min_x = path_data.points.iter().map(|p| p.0).fold(f32::MAX, f32::min);
    let min_y = path_data.points.iter().map(|p| p.1).fold(f32::MAX, f32::min);

    let local_start = (path_data.start.0 - min_x, path_data.start.1 - min_y);
    let local_segs: Vec<PathSegment> = path_data
        .segments
        .iter()
        .map(|seg| match seg {
            PathSegment::LineTo(x, y) => PathSegment::LineTo(x - min_x, y - min_y),
            PathSegment::CubicBezier { c1, c2, end } => PathSegment::CubicBezier {
                c1: (c1.0 - min_x, c1.1 - min_y),
                c2: (c2.0 - min_x, c2.1 - min_y),
                end: (end.0 - min_x, end.1 - min_y),
            },
            PathSegment::QuadBezier { c, end } => PathSegment::QuadBezier {
                c: (c.0 - min_x, c.1 - min_y),
                end: (end.0 - min_x, end.1 - min_y),
            },
        })
        .collect();
    let local_path = PathData::from_segments(local_start, local_segs);

    Some(ShapeParams {
        shape: ShapeKind::Path(local_path),
        x: min_x,
        y: min_y,
        rotation: 0.0,
        layer_idx,
        ..Default::default()
    })
}

/// Parse a BITMAP display element into a RasterImage ShapeParams.
fn parse_bitmap_display(disp: &Value, layer_idx: usize) -> Option<ShapeParams> {
    let b64_str = disp.get("base64").and_then(|v| v.as_str())?;

    // Strip "data:image/...;base64," prefix if present
    let raw_b64 = if let Some(idx) = b64_str.find(",") {
        &b64_str[idx + 1..]
    } else {
        b64_str
    };

    // Decode base64
    use base64::Engine;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(raw_b64)
        .ok()?;

    // Load image preserving alpha channel (compositing happens in preprocess_image)
    // No flip needed: the renderer UV mapping handles Y-up world coords
    let raw_img = image::load_from_memory(&bytes).ok()?;
    let img = image::DynamicImage::ImageRgba8(raw_img.to_rgba8());

    let dx = disp.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let dy = disp.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let dw = disp.get("width").and_then(|v| v.as_f64()).unwrap_or(50.0) as f32;
    let dh = disp.get("height").and_then(|v| v.as_f64()).unwrap_or(50.0) as f32;

    // Y flip: convert screen coords (Y-down) to app coords (Y-up)
    let app_y = -(dy + dh);

    let raster_params = RasterParams {
        width_mm: dw,
        height_mm: dh,
        ..Default::default()
    };

    Some(ShapeParams {
        shape: ShapeKind::RasterImage {
            data: ImageData(Arc::new(img)),
            params: raster_params,
        },
        x: dx,
        y: app_y,
        width: dw,
        height: dh,
        layer_idx,
        ..Default::default()
    })
}

/// Parse SVG path data string (M, L, C, Q, Z commands) into sub-paths with Bézier segments preserved.
fn parse_svg_dpath(d: &str) -> Vec<RawSubPath> {
    let mut paths: Vec<RawSubPath> = Vec::new();
    let mut segments: Vec<PathSegment> = Vec::new();
    let mut cx: f32 = 0.0;
    let mut cy: f32 = 0.0;
    let mut start_x: f32 = 0.0;
    let mut start_y: f32 = 0.0;
    let mut sub_start: (f32, f32) = (0.0, 0.0);
    let mut last_cubic_ctrl: Option<(f32, f32)> = None;

    let tokens = tokenize_svg_path(d);
    let mut i = 0;

    while i < tokens.len() {
        match tokens[i].as_str() {
            "M" => {
                // Flush previous sub-path
                if !segments.is_empty() {
                    paths.push(RawSubPath { start: sub_start, segments: std::mem::take(&mut segments) });
                } else {
                    segments.clear();
                }
                last_cubic_ctrl = None;
                i += 1;
                if let Some((x, y, adv)) = read_pair(&tokens, i) {
                    cx = x;
                    cy = y;
                    start_x = x;
                    start_y = y;
                    sub_start = (cx, cy);
                    i += adv;
                    // Implicit L after first M pair
                    while let Some((x2, y2, adv2)) = read_pair(&tokens, i) {
                        cx = x2;
                        cy = y2;
                        segments.push(PathSegment::LineTo(cx, cy));
                        i += adv2;
                    }
                }
            }
            "m" => {
                if !segments.is_empty() {
                    paths.push(RawSubPath { start: sub_start, segments: std::mem::take(&mut segments) });
                } else {
                    segments.clear();
                }
                last_cubic_ctrl = None;
                i += 1;
                if let Some((dx, dy, adv)) = read_pair(&tokens, i) {
                    cx += dx;
                    cy += dy;
                    start_x = cx;
                    start_y = cy;
                    sub_start = (cx, cy);
                    i += adv;
                    while let Some((dx2, dy2, adv2)) = read_pair(&tokens, i) {
                        cx += dx2;
                        cy += dy2;
                        segments.push(PathSegment::LineTo(cx, cy));
                        i += adv2;
                    }
                }
            }
            "L" => {
                last_cubic_ctrl = None;
                i += 1;
                while let Some((x, y, adv)) = read_pair(&tokens, i) {
                    cx = x;
                    cy = y;
                    segments.push(PathSegment::LineTo(cx, cy));
                    i += adv;
                }
            }
            "l" => {
                last_cubic_ctrl = None;
                i += 1;
                while let Some((dx, dy, adv)) = read_pair(&tokens, i) {
                    cx += dx;
                    cy += dy;
                    segments.push(PathSegment::LineTo(cx, cy));
                    i += adv;
                }
            }
            "H" => {
                last_cubic_ctrl = None;
                i += 1;
                while let Some(v) = read_num(&tokens, i) {
                    cx = v;
                    segments.push(PathSegment::LineTo(cx, cy));
                    i += 1;
                }
            }
            "h" => {
                last_cubic_ctrl = None;
                i += 1;
                while let Some(v) = read_num(&tokens, i) {
                    cx += v;
                    segments.push(PathSegment::LineTo(cx, cy));
                    i += 1;
                }
            }
            "V" => {
                last_cubic_ctrl = None;
                i += 1;
                while let Some(v) = read_num(&tokens, i) {
                    cy = v;
                    segments.push(PathSegment::LineTo(cx, cy));
                    i += 1;
                }
            }
            "v" => {
                last_cubic_ctrl = None;
                i += 1;
                while let Some(v) = read_num(&tokens, i) {
                    cy += v;
                    segments.push(PathSegment::LineTo(cx, cy));
                    i += 1;
                }
            }
            "C" => {
                i += 1;
                while let Some((pts6, adv)) = read_n_nums(&tokens, i, 6) {
                    let c2 = (pts6[2], pts6[3]);
                    segments.push(PathSegment::CubicBezier {
                        c1: (pts6[0], pts6[1]),
                        c2,
                        end: (pts6[4], pts6[5]),
                    });
                    last_cubic_ctrl = Some(c2);
                    cx = pts6[4];
                    cy = pts6[5];
                    i += adv;
                }
            }
            "c" => {
                i += 1;
                while let Some((pts6, adv)) = read_n_nums(&tokens, i, 6) {
                    let c2 = (cx + pts6[2], cy + pts6[3]);
                    segments.push(PathSegment::CubicBezier {
                        c1: (cx + pts6[0], cy + pts6[1]),
                        c2,
                        end: (cx + pts6[4], cy + pts6[5]),
                    });
                    last_cubic_ctrl = Some(c2);
                    cx += pts6[4];
                    cy += pts6[5];
                    i += adv;
                }
            }
            "S" => {
                i += 1;
                while let Some((pts4, adv)) = read_n_nums(&tokens, i, 4) {
                    let c1 = if let Some(prev_c2) = last_cubic_ctrl {
                        (2.0 * cx - prev_c2.0, 2.0 * cy - prev_c2.1)
                    } else {
                        (cx, cy)
                    };
                    let c2 = (pts4[0], pts4[1]);
                    let end = (pts4[2], pts4[3]);
                    segments.push(PathSegment::CubicBezier { c1, c2, end });
                    last_cubic_ctrl = Some(c2);
                    cx = end.0;
                    cy = end.1;
                    i += adv;
                }
            }
            "s" => {
                i += 1;
                while let Some((pts4, adv)) = read_n_nums(&tokens, i, 4) {
                    let c1 = if let Some(prev_c2) = last_cubic_ctrl {
                        (2.0 * cx - prev_c2.0, 2.0 * cy - prev_c2.1)
                    } else {
                        (cx, cy)
                    };
                    let c2 = (cx + pts4[0], cy + pts4[1]);
                    let end = (cx + pts4[2], cy + pts4[3]);
                    segments.push(PathSegment::CubicBezier { c1, c2, end });
                    last_cubic_ctrl = Some(c2);
                    cx = end.0;
                    cy = end.1;
                    i += adv;
                }
            }
            "Q" => {
                last_cubic_ctrl = None;
                i += 1;
                while let Some((pts4, adv)) = read_n_nums(&tokens, i, 4) {
                    segments.push(PathSegment::QuadBezier {
                        c: (pts4[0], pts4[1]),
                        end: (pts4[2], pts4[3]),
                    });
                    cx = pts4[2];
                    cy = pts4[3];
                    i += adv;
                }
            }
            "q" => {
                last_cubic_ctrl = None;
                i += 1;
                while let Some((pts4, adv)) = read_n_nums(&tokens, i, 4) {
                    segments.push(PathSegment::QuadBezier {
                        c: (cx + pts4[0], cy + pts4[1]),
                        end: (cx + pts4[2], cy + pts4[3]),
                    });
                    cx += pts4[2];
                    cy += pts4[3];
                    i += adv;
                }
            }
            "A" | "a" => {
                last_cubic_ctrl = None;
                // Arc command — convert to cubic bezier curves
                let relative = tokens[i] == "a";
                i += 1;
                while let Some((p7, adv)) = read_n_nums(&tokens, i, 7) {
                    let rx = p7[0];
                    let ry = p7[1];
                    let x_rot = p7[2];
                    let large_arc = p7[3].abs() > 0.5;
                    let sweep = p7[4].abs() > 0.5;
                    let (ex, ey) = if relative {
                        (cx + p7[5], cy + p7[6])
                    } else {
                        (p7[5], p7[6])
                    };
                    let arc_segs = arc_to_beziers(cx, cy, rx, ry, x_rot, large_arc, sweep, ex, ey);
                    segments.extend(arc_segs);
                    cx = ex;
                    cy = ey;
                    i += adv;
                }
            }
            "Z" | "z" => {
                last_cubic_ctrl = None;
                // Close path
                if !segments.is_empty() {
                    segments.push(PathSegment::LineTo(start_x, start_y));
                    cx = start_x;
                    cy = start_y;
                }
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    // Flush last sub-path
    if !segments.is_empty() {
        paths.push(RawSubPath { start: sub_start, segments });
    }

    paths
}

/// Tokenize SVG path data into command letters and number strings.
fn tokenize_svg_path(d: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    let chars: Vec<char> = d.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        if ch.is_ascii_alphabetic() {
            if !buf.is_empty() {
                tokens.push(std::mem::take(&mut buf));
            }
            tokens.push(ch.to_string());
            i += 1;
        } else if ch == '-' || ch == '+' || ch == '.' || ch.is_ascii_digit() {
            // Start of a number
            // Flush buffer if we have content and this is a new number (starts with '-' or '+' after digits)
            if !buf.is_empty()
                && (ch == '-' || ch == '+')
                && !buf.ends_with('e')
                && !buf.ends_with('E')
            {
                tokens.push(std::mem::take(&mut buf));
            }
            // Handle case where '.' starts a new number (e.g. "1.5.3" = "1.5" "0.3")
            if ch == '.' && buf.contains('.') {
                tokens.push(std::mem::take(&mut buf));
            }
            buf.push(ch);
            i += 1;
        } else {
            // Whitespace or comma — flush
            if !buf.is_empty() {
                tokens.push(std::mem::take(&mut buf));
            }
            i += 1;
        }
    }
    if !buf.is_empty() {
        tokens.push(buf);
    }
    tokens
}

fn read_num(tokens: &[String], i: usize) -> Option<f32> {
    if i >= tokens.len() {
        return None;
    }
    tokens[i].parse::<f32>().ok()
}

fn read_pair(tokens: &[String], i: usize) -> Option<(f32, f32, usize)> {
    let x = read_num(tokens, i)?;
    let y = read_num(tokens, i + 1)?;
    Some((x, y, 2))
}

fn read_n_nums(tokens: &[String], i: usize, n: usize) -> Option<(Vec<f32>, usize)> {
    if i + n > tokens.len() {
        return None;
    }
    let mut vals = Vec::with_capacity(n);
    for j in 0..n {
        match tokens[i + j].parse::<f32>() {
            Ok(v) => vals.push(v),
            Err(_) => return None,
        }
    }
    Some((vals, n))
}

/// Convert an SVG elliptical arc to cubic bézier segments.
/// Uses the SVG spec's endpoint-to-center parameterization, then approximates
/// each ≤90° arc segment with a cubic bézier curve.
fn arc_to_beziers(
    cx: f32, cy: f32,
    mut rx: f32, mut ry: f32,
    x_rotation_deg: f32,
    large_arc: bool,
    sweep: bool,
    ex: f32, ey: f32,
) -> Vec<PathSegment> {
    use std::f32::consts::{FRAC_PI_2, TAU};

    // Degenerate: same point
    if (cx - ex).abs() < 1e-6 && (cy - ey).abs() < 1e-6 {
        return Vec::new();
    }

    rx = rx.abs();
    ry = ry.abs();
    if rx < 1e-6 || ry < 1e-6 {
        return vec![PathSegment::LineTo(ex, ey)];
    }

    let phi = x_rotation_deg.to_radians();
    let (sin_phi, cos_phi) = phi.sin_cos();

    // Step 1: Compute (x1', y1') — rotated midpoint
    let dx = (cx - ex) / 2.0;
    let dy = (cy - ey) / 2.0;
    let x1p = cos_phi * dx + sin_phi * dy;
    let y1p = -sin_phi * dx + cos_phi * dy;

    // Step 2: Ensure radii are large enough
    let x1p2 = x1p * x1p;
    let y1p2 = y1p * y1p;
    let mut rx2 = rx * rx;
    let mut ry2 = ry * ry;

    let lambda = x1p2 / rx2 + y1p2 / ry2;
    if lambda > 1.0 {
        let s = lambda.sqrt();
        rx *= s;
        ry *= s;
        rx2 = rx * rx;
        ry2 = ry * ry;
    }

    // Step 3: Compute center in transformed coordinates
    let num = (rx2 * ry2 - rx2 * y1p2 - ry2 * x1p2).max(0.0);
    let den = rx2 * y1p2 + ry2 * x1p2;
    let sq = if den > 1e-10 { (num / den).sqrt() } else { 0.0 };
    let sign = if large_arc == sweep { -1.0 } else { 1.0 };
    let cxp = sign * sq * rx * y1p / ry;
    let cyp = -sign * sq * ry * x1p / rx;

    // Step 4: Compute actual center
    let ccx = cos_phi * cxp - sin_phi * cyp + (cx + ex) / 2.0;
    let ccy = sin_phi * cxp + cos_phi * cyp + (cy + ey) / 2.0;

    // Step 5: Compute start angle and delta angle
    let vangle = |ux: f32, uy: f32, vx: f32, vy: f32| -> f32 {
        let n = (ux * ux + uy * uy).sqrt() * (vx * vx + vy * vy).sqrt();
        if n < 1e-10 { return 0.0; }
        let c = ((ux * vx + uy * vy) / n).clamp(-1.0, 1.0);
        let mut a = c.acos();
        if ux * vy - uy * vx < 0.0 { a = -a; }
        a
    };

    let theta1 = vangle(1.0, 0.0, (x1p - cxp) / rx, (y1p - cyp) / ry);
    let mut dtheta = vangle(
        (x1p - cxp) / rx, (y1p - cyp) / ry,
        (-x1p - cxp) / rx, (-y1p - cyp) / ry,
    );

    if !sweep && dtheta > 0.0 {
        dtheta -= TAU;
    } else if sweep && dtheta < 0.0 {
        dtheta += TAU;
    }

    // Step 6: Split into segments ≤ 90° and convert to cubic béziers
    let n_segs = ((dtheta.abs() / FRAC_PI_2).ceil() as usize).max(1);
    let seg_angle = dtheta / n_segs as f32;
    let alpha = 4.0 / 3.0 * (seg_angle / 4.0).tan();

    let mut result = Vec::with_capacity(n_segs);
    let mut t = theta1;
    for _ in 0..n_segs {
        let t2 = t + seg_angle;
        let (sin1, cos1) = t.sin_cos();
        let (sin2, cos2) = t2.sin_cos();

        // Start point on rotated ellipse
        let spx = cos_phi * rx * cos1 - sin_phi * ry * sin1 + ccx;
        let spy = sin_phi * rx * cos1 + cos_phi * ry * sin1 + ccy;
        // End point on rotated ellipse
        let epx = cos_phi * rx * cos2 - sin_phi * ry * sin2 + ccx;
        let epy = sin_phi * rx * cos2 + cos_phi * ry * sin2 + ccy;

        // Tangent at start (derivative of parametric ellipse, rotated by phi)
        let tdx1 = cos_phi * (-rx * sin1) - sin_phi * (ry * cos1);
        let tdy1 = sin_phi * (-rx * sin1) + cos_phi * (ry * cos1);
        // Tangent at end
        let tdx2 = cos_phi * (-rx * sin2) - sin_phi * (ry * cos2);
        let tdy2 = sin_phi * (-rx * sin2) + cos_phi * (ry * cos2);

        result.push(PathSegment::CubicBezier {
            c1: (spx + alpha * tdx1, spy + alpha * tdy1),
            c2: (epx - alpha * tdx2, epy - alpha * tdy2),
            end: (epx, epy),
        });

        t = t2;
    }

    result
}

/// Parse a CIRCLE/ELLIPSE display element into ShapeParams.
/// Generates a closed ellipse path from 4 cubic bézier curves.
fn parse_circle_display(disp: &Value, layer_idx: usize) -> Option<Vec<ShapeParams>> {
    let display_type = disp.get("type").and_then(|v| v.as_str()).unwrap_or("");
    let dx = disp.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let dy = disp.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let dw = disp.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let dh = disp.get("height").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let angle = disp.get("angle").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;

    if dw < 0.001 || dh < 0.001 {
        return None;
    }

    // Ellipse parameters in canvas coords (Y-down)
    let rx = dw / 2.0;
    let ry = dh / 2.0;
    // XCS uses center anchoring for CIRCLE displays, while other ellipse-like objects
    // are represented with top-left anchoring.
    let angle_norm = angle.rem_euclid(360.0);
    let near_180 = (angle_norm - 180.0).abs() < 0.001;
    let (ccx, ccy) = if display_type == "CIRCLE" {
        if near_180 {
            (dx - rx, dy - ry)
        } else {
            (dx, dy)
        }
    } else {
        (dx + rx, dy + ry)
    };

    // Convert center to world coords (Y-up)
    let wcx = ccx;
    let wcy = -ccy;

    // κ = 4(√2 − 1)/3 ≈ 0.5522847498 — standard circle-to-bézier constant
    let kx = rx * 0.5522847498;
    let ky = ry * 0.5522847498;

    // 4 cubic bézier curves forming a closed ellipse (world Y-up)
    let start = (wcx + rx, wcy);
    let segs = vec![
        PathSegment::CubicBezier {
            c1: (wcx + rx, wcy + ky),
            c2: (wcx + kx, wcy + ry),
            end: (wcx, wcy + ry),
        },
        PathSegment::CubicBezier {
            c1: (wcx - kx, wcy + ry),
            c2: (wcx - rx, wcy + ky),
            end: (wcx - rx, wcy),
        },
        PathSegment::CubicBezier {
            c1: (wcx - rx, wcy - ky),
            c2: (wcx - kx, wcy - ry),
            end: (wcx, wcy - ry),
        },
        PathSegment::CubicBezier {
            c1: (wcx + kx, wcy - ry),
            c2: (wcx + rx, wcy - ky),
            end: (wcx + rx, wcy),
        },
    ];

    // Bake rotation if present (around ellipse center)
    let segs = if angle.abs() > 1e-6 {
        let angle_rad = angle.to_radians();
        let (sin_a, cos_a) = angle_rad.sin_cos();
        let rotate = |px: f32, py: f32| -> (f32, f32) {
            let ddx = px - wcx;
            let ddy = py - wcy;
            (wcx + ddx * cos_a - ddy * sin_a, wcy + ddx * sin_a + ddy * cos_a)
        };
        let rot_seg = |seg: &PathSegment| -> PathSegment {
            match seg {
                PathSegment::CubicBezier { c1, c2, end } => PathSegment::CubicBezier {
                    c1: rotate(c1.0, c1.1),
                    c2: rotate(c2.0, c2.1),
                    end: rotate(end.0, end.1),
                },
                PathSegment::LineTo(x, y) => {
                    let (rx, ry) = rotate(*x, *y);
                    PathSegment::LineTo(rx, ry)
                }
                other => other.clone(),
            }
        };
        segs.iter().map(rot_seg).collect()
    } else {
        segs
    };

    let start_pt = if angle.abs() > 1e-6 {
        let angle_rad = angle.to_radians();
        let (sin_a, cos_a) = angle_rad.sin_cos();
        let ddx = start.0 - wcx;
        let ddy = start.1 - wcy;
        (wcx + ddx * cos_a - ddy * sin_a, wcy + ddx * sin_a + ddy * cos_a)
    } else {
        start
    };

    let path_data = PathData::from_segments(start_pt, segs);
    if path_data.points.len() < 2 {
        return None;
    }

    // Make coordinates local
    let min_x = path_data.points.iter().map(|p| p.0).fold(f32::MAX, f32::min);
    let min_y = path_data.points.iter().map(|p| p.1).fold(f32::MAX, f32::min);
    let local_start = (path_data.start.0 - min_x, path_data.start.1 - min_y);
    let local_segs: Vec<PathSegment> = path_data.segments.iter().map(|seg| match seg {
        PathSegment::LineTo(x, y) => PathSegment::LineTo(x - min_x, y - min_y),
        PathSegment::CubicBezier { c1, c2, end } => PathSegment::CubicBezier {
            c1: (c1.0 - min_x, c1.1 - min_y),
            c2: (c2.0 - min_x, c2.1 - min_y),
            end: (end.0 - min_x, end.1 - min_y),
        },
        PathSegment::QuadBezier { c, end } => PathSegment::QuadBezier {
            c: (c.0 - min_x, c.1 - min_y),
            end: (end.0 - min_x, end.1 - min_y),
        },
    }).collect();
    let local_path = PathData::from_segments(local_start, local_segs);

    Some(vec![ShapeParams {
        shape: ShapeKind::Path(local_path),
        x: min_x,
        y: min_y,
        rotation: 0.0,
        layer_idx,
        ..Default::default()
    }])
}

