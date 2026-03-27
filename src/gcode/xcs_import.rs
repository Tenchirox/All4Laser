#![allow(dead_code)]
use crate::ui::drawing::{ImageData, PathData, PathSegment, ShapeKind, ShapeParams};
use crate::imaging::raster::RasterParams;
use serde_json::Value;
use std::sync::Arc;

/// Import an xTool Creative Space (.xcs) JSON file and return shapes.
pub fn import_xcs(content: &str) -> Result<Vec<ShapeParams>, String> {
    let root: Value =
        serde_json::from_str(content).map_err(|e| format!("XCS JSON parse error: {e}"))?;

    let canvas = root
        .get("canvas")
        .and_then(|v| v.as_array())
        .ok_or("Missing 'canvas' array in XCS file")?;

    let mut shapes = Vec::new();

    // Collect layerTags and assign layer indices
    // BITMAP layers get lower indices (rendered behind), PATH layers get higher
    let mut layer_map: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for panel in canvas {
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

    eprintln!("[XCS] canvas panels: {}", canvas.len());
    for (pi, panel) in canvas.iter().enumerate() {
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

    // Display bounding box in canvas coords (screen Y-down)
    let disp_x = disp.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let disp_y = disp.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let disp_w = disp.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let disp_h = disp.get("height").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;

    // Parse SVG path data into sub-paths with Bézier segments preserved
    let sub_paths = parse_svg_dpath(dpath);
    if sub_paths.is_empty() {
        return None;
    }

    // Compute raw bounding box across ALL sub-paths (using control points for conservative bbox)
    let mut raw_min_x = f32::MAX;
    let mut raw_min_y = f32::MAX;
    let mut raw_max_x = f32::MIN;
    let mut raw_max_y = f32::MIN;
    for sp in &sub_paths {
        for (px, py) in sp.all_points() {
            raw_min_x = raw_min_x.min(px);
            raw_min_y = raw_min_y.min(py);
            raw_max_x = raw_max_x.max(px);
            raw_max_y = raw_max_y.max(py);
        }
    }
    let raw_w = raw_max_x - raw_min_x;
    let raw_h = raw_max_y - raw_min_y;

    // Coordinate transform: map raw dPath coords to display bbox, then flip Y
    // Then bake in XCS rotation so the shape geometry is final (no ShapeParams.rotation needed).
    let angle_rad = angle.to_radians();
    let (sin_a, cos_a) = angle_rad.sin_cos();
    // Rotation center in world coords (center of the display element)
    let rot_cx = disp_x + disp_w * 0.5;
    let rot_cy = -(disp_y + disp_h * 0.5);

    let xform = |px: f32, py: f32| -> (f32, f32) {
        let nx = if raw_w > 0.001 {
            (px - raw_min_x) / raw_w * disp_w + disp_x
        } else {
            disp_x
        };
        let canvas_y = if raw_h > 0.001 {
            (py - raw_min_y) / raw_h * disp_h + disp_y
        } else {
            disp_y
        };
        let wx = nx;
        let wy = -canvas_y;
        // Bake rotation around display element center
        if angle_rad.abs() > 1e-6 {
            let dx = wx - rot_cx;
            let dy = wy - rot_cy;
            (rot_cx + dx * cos_a - dy * sin_a,
             rot_cy + dx * sin_a + dy * cos_a)
        } else {
            (wx, wy)
        }
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
                i += 1;
                while let Some((x, y, adv)) = read_pair(&tokens, i) {
                    cx = x;
                    cy = y;
                    segments.push(PathSegment::LineTo(cx, cy));
                    i += adv;
                }
            }
            "l" => {
                i += 1;
                while let Some((dx, dy, adv)) = read_pair(&tokens, i) {
                    cx += dx;
                    cy += dy;
                    segments.push(PathSegment::LineTo(cx, cy));
                    i += adv;
                }
            }
            "H" => {
                i += 1;
                while let Some(v) = read_num(&tokens, i) {
                    cx = v;
                    segments.push(PathSegment::LineTo(cx, cy));
                    i += 1;
                }
            }
            "h" => {
                i += 1;
                while let Some(v) = read_num(&tokens, i) {
                    cx += v;
                    segments.push(PathSegment::LineTo(cx, cy));
                    i += 1;
                }
            }
            "V" => {
                i += 1;
                while let Some(v) = read_num(&tokens, i) {
                    cy = v;
                    segments.push(PathSegment::LineTo(cx, cy));
                    i += 1;
                }
            }
            "v" => {
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
                    segments.push(PathSegment::CubicBezier {
                        c1: (pts6[0], pts6[1]),
                        c2: (pts6[2], pts6[3]),
                        end: (pts6[4], pts6[5]),
                    });
                    cx = pts6[4];
                    cy = pts6[5];
                    i += adv;
                }
            }
            "c" => {
                i += 1;
                while let Some((pts6, adv)) = read_n_nums(&tokens, i, 6) {
                    segments.push(PathSegment::CubicBezier {
                        c1: (cx + pts6[0], cy + pts6[1]),
                        c2: (cx + pts6[2], cy + pts6[3]),
                        end: (cx + pts6[4], cy + pts6[5]),
                    });
                    cx += pts6[4];
                    cy += pts6[5];
                    i += adv;
                }
            }
            "Q" => {
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
                // Arc command — approximate with line to endpoint
                let relative = tokens[i] == "a";
                i += 1;
                while let Some((p7, adv)) = read_n_nums(&tokens, i, 7) {
                    if relative {
                        cx += p7[5];
                        cy += p7[6];
                    } else {
                        cx = p7[5];
                        cy = p7[6];
                    }
                    segments.push(PathSegment::LineTo(cx, cy));
                    i += adv;
                }
            }
            "Z" | "z" => {
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

