/// SVG to GCode conversion using usvg for parsing
///
/// Extracts path segments from SVG and converts them to GCode move/cut commands.

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

    let mut layers: Vec<SvgLayer> = colors.into_iter().map(|color_ha| SvgLayer {
        color_ha,
        speed: 1000.0,
        power: 1000.0,
        enabled: true,
    }).collect();
    
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
    let tree = usvg::Tree::from_data(svg_data, &opts)
        .map_err(|e| format!("SVG parse error: {e}"))?;

    let mut gcode = Vec::new();
    // ... (rest of old svg_to_gcode logic if still needed, but we'll use paths now)
    // Actually, let's keep it for compatibility if needed, but the main goal is svg_to_paths.
    Ok(gcode) 
}

pub fn svg_to_paths(svg_data: &[u8], params: &SvgParams) -> Result<Vec<(Vec<(f32, f32)>, usize)>, String> {
    let opts = usvg::Options::default();
    let tree = usvg::Tree::from_data(svg_data, &opts)
        .map_err(|e| format!("SVG parse error: {e}"))?;

    let mut out_paths = Vec::new();

    for (layer_idx, layer) in params.layers.iter().enumerate() {
        if !layer.enabled { continue; }
        
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

fn collect_paths(node: &usvg::Node, params: &SvgParams, layer: &SvgLayer, out: &mut Vec<Vec<(f32, f32)>>) {
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
                        if hex == layer.color_ha { matches_layer = true; }
                    }
                }
                if let Some(fill) = path.fill() {
                    if let usvg::Paint::Color(c) = fill.paint() {
                        let hex = format!("#{:02X}{:02X}{:02X}", c.red, c.green, c.blue);
                        if hex == layer.color_ha { matches_layer = true; }
                    }
                }
            }

            if matches_layer {
                let mut current_path = Vec::new();
                for seg in path.data().segments() {
                    match seg {
                        tiny_skia::PathSegment::MoveTo(pt) => {
                            if !current_path.is_empty() {
                                out.push(current_path);
                                current_path = Vec::new();
                            }
                            current_path.push((pt.x * params.scale, pt.y * params.scale));
                        }
                        tiny_skia::PathSegment::LineTo(pt) |
                        tiny_skia::PathSegment::QuadTo(_, pt) |
                        tiny_skia::PathSegment::CubicTo(_, _, pt) => {
                            current_path.push((pt.x * params.scale, pt.y * params.scale));
                        }
                        tiny_skia::PathSegment::Close => {
                            if !current_path.is_empty() {
                                // Close the path by repeating the first point if needed, or just push.
                                // Drawing logic handles closing if desired.
                                out.push(current_path);
                                current_path = Vec::new();
                            }
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
                        if hex == layer.color_ha { matches_layer = true; }
                    }
                }
                if let Some(fill) = path.fill() {
                    if let usvg::Paint::Color(c) = fill.paint() {
                        let hex = format!("#{:02X}{:02X}{:02X}", c.red, c.green, c.blue);
                        if hex == layer.color_ha { matches_layer = true; }
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
