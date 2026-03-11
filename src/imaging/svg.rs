#![allow(dead_code)]

/// SVG to GCode conversion using usvg for parsing
///
/// Extracts path segments from SVG and converts them to GCode move/cut commands.

use tiny_skia::Transform;

#[derive(Clone, Debug, PartialEq)]
pub struct SvgLayer {
    pub color_ha: String,
    pub speed: f32,
    pub power: f32,
    pub enabled: bool,
}

#[derive(Clone)]
pub struct SvgParams {
    pub scale: f32, // mm per SVG unit
    pub outline: crate::imaging::raster::OutlineParams,
    pub layers: Vec<SvgLayer>,
}

impl Default for SvgParams {
    fn default() -> Self {
        Self {
            scale: 1.0,
            outline: crate::imaging::raster::OutlineParams::default(),
            layers: vec![SvgLayer {
                color_ha: "Default".into(),
                speed: 1000.0,
                power: 1000.0,
                enabled: true,
            }],
        }
    }
}

pub fn extract_layers(svg_data: &[u8]) -> Vec<SvgLayer> {
    let opts = usvg::Options::default();
    let mut colors = std::collections::HashSet::new();

    if let Ok(tree) = usvg::Tree::from_data(svg_data, &opts) {
        fn walk(node: &usvg::Node, colors: &mut std::collections::HashSet<String>) {
            match node {
                usvg::Node::Group(group) => {
                    for child in group.children() {
                        walk(&child, colors);
                    }
                }
                usvg::Node::Path(path) => {
                    if let Some(stroke) = path.stroke() {
                        if let usvg::Paint::Color(c) = stroke.paint() {
                            colors.insert(format!("#{:02X}{:02X}{:02X}", c.red, c.green, c.blue));
                        }
                    }
                    if let Some(fill) = path.fill() {
                        if let usvg::Paint::Color(c) = fill.paint() {
                            colors.insert(format!("#{:02X}{:02X}{:02X}", c.red, c.green, c.blue));
                        }
                    }
                }
                _ => {}
            }
        }
        for node in tree.root().children() {
            walk(&node, &mut colors);
        }
    }

    let mut layers: Vec<SvgLayer> = colors
        .into_iter()
        .map(|color_ha| SvgLayer {
            color_ha,
            speed: 1000.0,
            power: 1000.0,
            enabled: true,
        })
        .collect();

    // Sort layers by color hex to keep UI stable
    layers.sort_by(|a, b| a.color_ha.cmp(&b.color_ha));

    if layers.is_empty() {
        layers.push(SvgLayer {
            color_ha: "Default".into(),
            speed: 1000.0,
            power: 1000.0,
            enabled: true,
        });
    }

    layers
}

/// Convert an SVG file to GCode
pub fn svg_to_gcode(svg_data: &[u8], params: &SvgParams) -> Result<Vec<String>, String> {
    let opts = usvg::Options::default();
    let _tree =
        usvg::Tree::from_data(svg_data, &opts).map_err(|e| format!("SVG parse error: {e}"))?;

    let _ = params;

    let gcode = Vec::new();
    // ... (rest of old svg_to_gcode logic if still needed, but we'll use paths now)
    // Actually, let's keep it for compatibility if needed, but the main goal is svg_to_paths.
    Ok(gcode)
}

pub fn svg_to_paths(
    svg_data: &[u8],
    params: &SvgParams,
) -> Result<Vec<(Vec<(f32, f32)>, usize)>, String> {
    let opts = usvg::Options::default();
    let tree =
        usvg::Tree::from_data(svg_data, &opts).map_err(|e| format!("SVG parse error: {e}"))?;

    let mut out_paths = Vec::new();

    for (layer_idx, layer) in params.layers.iter().enumerate() {
        if !layer.enabled {
            continue;
        }

        let mut layer_paths = Vec::new();
        for node in tree.root().children() {
            collect_paths(&node, params, layer, &mut layer_paths);
        }

        for p in layer_paths {
            out_paths.push((p, layer_idx));
        }
    }

    Ok(out_paths)
}

fn collect_paths(
    node: &usvg::Node,
    params: &SvgParams,
    layer: &SvgLayer,
    out: &mut Vec<Vec<(f32, f32)>>,
) {
    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    fn quad_at(p0: (f32, f32), p1: (f32, f32), p2: (f32, f32), t: f32) -> (f32, f32) {
        let a = (lerp(p0.0, p1.0, t), lerp(p0.1, p1.1, t));
        let b = (lerp(p1.0, p2.0, t), lerp(p1.1, p2.1, t));
        (lerp(a.0, b.0, t), lerp(a.1, b.1, t))
    }

    fn cubic_at(
        p0: (f32, f32),
        p1: (f32, f32),
        p2: (f32, f32),
        p3: (f32, f32),
        t: f32,
    ) -> (f32, f32) {
        let a = (lerp(p0.0, p1.0, t), lerp(p0.1, p1.1, t));
        let b = (lerp(p1.0, p2.0, t), lerp(p1.1, p2.1, t));
        let c = (lerp(p2.0, p3.0, t), lerp(p2.1, p3.1, t));
        let d = (lerp(a.0, b.0, t), lerp(a.1, b.1, t));
        let e = (lerp(b.0, c.0, t), lerp(b.1, c.1, t));
        (lerp(d.0, e.0, t), lerp(d.1, e.1, t))
    }

    match node {
        usvg::Node::Group(group) => {
            for child in group.children() {
                collect_paths(&child, params, layer, out);
            }
        }
        usvg::Node::Path(path) => {
            let mut matches_layer = false;
            if layer.color_ha == "Default" {
                matches_layer = true;
            } else {
                if let Some(stroke) = path.stroke() {
                    if let usvg::Paint::Color(c) = stroke.paint() {
                        let hex = format!("#{:02X}{:02X}{:02X}", c.red, c.green, c.blue);
                        if hex == layer.color_ha {
                            matches_layer = true;
                        }
                    }
                }
                if let Some(fill) = path.fill() {
                    if let usvg::Paint::Color(c) = fill.paint() {
                        let hex = format!("#{:02X}{:02X}{:02X}", c.red, c.green, c.blue);
                        if hex == layer.color_ha {
                            matches_layer = true;
                        }
                    }
                }
            }

            if matches_layer {
                // Apply all SVG transforms (including group transforms) before extracting segments.
                // Otherwise imported geometry can be missing/misplaced.
                let abs_ts = path.abs_transform();
                let ts = Transform::from_row(
                    abs_ts.sx,
                    abs_ts.ky,
                    abs_ts.kx,
                    abs_ts.sy,
                    abs_ts.tx,
                    abs_ts.ty,
                );
                let p0 = path.data().clone();
                let p = p0.clone().transform(ts).unwrap_or(p0);

                let mut current_path = Vec::new();
                let mut current_pos: Option<(f32, f32)> = None;
                let mut subpath_first: Option<(f32, f32)> = None;
                for seg in p.segments() {
                    match seg {
                        tiny_skia::PathSegment::MoveTo(pt) => {
                            if !current_path.is_empty() {
                                out.push(current_path);
                                current_path = Vec::new();
                            }
                            let p0 = (pt.x * params.scale, pt.y * params.scale);
                            current_path.push(p0);
                            current_pos = Some(p0);
                            subpath_first = Some(p0);
                        }
                        tiny_skia::PathSegment::LineTo(pt) => {
                            let p1 = (pt.x * params.scale, pt.y * params.scale);
                            current_path.push(p1);
                            current_pos = Some(p1);
                        }
                        tiny_skia::PathSegment::QuadTo(ctrl, to) => {
                            let p0 = current_pos.unwrap_or((ctrl.x * params.scale, ctrl.y * params.scale));
                            let p1 = (ctrl.x * params.scale, ctrl.y * params.scale);
                            let p2 = (to.x * params.scale, to.y * params.scale);
                            let steps = 16;
                            for i in 1..=steps {
                                let t = i as f32 / steps as f32;
                                current_path.push(quad_at(p0, p1, p2, t));
                            }
                            current_pos = Some(p2);
                        }
                        tiny_skia::PathSegment::CubicTo(c1, c2, to) => {
                            let p0 = current_pos.unwrap_or((c1.x * params.scale, c1.y * params.scale));
                            let p1 = (c1.x * params.scale, c1.y * params.scale);
                            let p2 = (c2.x * params.scale, c2.y * params.scale);
                            let p3 = (to.x * params.scale, to.y * params.scale);
                            let steps = 16;
                            for i in 1..=steps {
                                let t = i as f32 / steps as f32;
                                current_path.push(cubic_at(p0, p1, p2, p3, t));
                            }
                            current_pos = Some(p3);
                        }
                        tiny_skia::PathSegment::Close => {
                            if !current_path.is_empty() {
                                if let (Some(first), Some(last)) =
                                    (current_path.first().copied(), current_path.last().copied())
                                {
                                    if (first.0 - last.0).abs() > 0.0001
                                        || (first.1 - last.1).abs() > 0.0001
                                    {
                                        // Prefer closing to the actual subpath start if known.
                                        current_path.push(subpath_first.unwrap_or(first));
                                    }
                                }
                                out.push(current_path);
                                current_path = Vec::new();
                            }
                            current_pos = None;
                            subpath_first = None;
                        }
                    }
                }
                if !current_path.is_empty() {
                    out.push(current_path);
                }
            }
        }
        _ => {}
    }
}

fn process_node(node: &usvg::Node, params: &SvgParams, layer: &SvgLayer, gcode: &mut Vec<String>) {
    match node {
        usvg::Node::Group(group) => {
            for child in group.children() {
                process_node(&child, params, layer, gcode);
            }
        }
        usvg::Node::Path(path) => {
            let mut matches_layer = false;

            if layer.color_ha == "Default" {
                matches_layer = true;
            } else {
                if let Some(stroke) = path.stroke() {
                    if let usvg::Paint::Color(c) = stroke.paint() {
                        let hex = format!("#{:02X}{:02X}{:02X}", c.red, c.green, c.blue);
                        if hex == layer.color_ha {
                            matches_layer = true;
                        }
                    }
                }
                if let Some(fill) = path.fill() {
                    if let usvg::Paint::Color(c) = fill.paint() {
                        let hex = format!("#{:02X}{:02X}{:02X}", c.red, c.green, c.blue);
                        if hex == layer.color_ha {
                            matches_layer = true;
                        }
                    }
                }
            }

            if matches_layer {
                process_path(path, params, layer, gcode);
            }
        }
        _ => {}
    }
}

fn process_path(path: &usvg::Path, params: &SvgParams, layer: &SvgLayer, gcode: &mut Vec<String>) {
    let mut first = true;

    for seg in path.data().segments() {
        match seg {
            tiny_skia::PathSegment::MoveTo(pt) => {
                gcode.push("M5".to_string());
                gcode.push(format!(
                    "G0X{:.3}Y{:.3}",
                    pt.x * params.scale,
                    pt.y * params.scale,
                ));
                first = false;
            }
            tiny_skia::PathSegment::LineTo(pt) => {
                if first {
                    gcode.push(format!(
                        "G0X{:.3}Y{:.3}",
                        pt.x * params.scale,
                        pt.y * params.scale,
                    ));
                    first = false;
                } else {
                    gcode.push(format!(
                        "G1X{:.3}Y{:.3}S{:.0}F{:.0}",
                        pt.x * params.scale,
                        pt.y * params.scale,
                        layer.power,
                        layer.speed,
                    ));
                }
            }
            tiny_skia::PathSegment::QuadTo(pt1, pt2) => {
                let _ = pt1;
                gcode.push(format!(
                    "G1X{:.3}Y{:.3}S{:.0}F{:.0}",
                    pt2.x * params.scale,
                    pt2.y * params.scale,
                    layer.power,
                    layer.speed,
                ));
            }
            tiny_skia::PathSegment::CubicTo(_pt1, _pt2, pt3) => {
                gcode.push(format!(
                    "G1X{:.3}Y{:.3}S{:.0}F{:.0}",
                    pt3.x * params.scale,
                    pt3.y * params.scale,
                    layer.power,
                    layer.speed,
                ));
            }
            tiny_skia::PathSegment::Close => {
                gcode.push("M5".to_string());
            }
        }
    }
}
