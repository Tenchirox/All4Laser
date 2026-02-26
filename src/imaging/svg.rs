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
    gcode.push("; Generated from SVG by All4Laser".to_string());
    gcode.push("G90".to_string());
    gcode.push("G21".to_string());
    gcode.push("M4".to_string());

    // For each layer, walk the node tree and only process nodes matching its color
    for layer in &params.layers {
        if !layer.enabled { continue; }

        gcode.push(format!("; --- Layer: {} ---", layer.color_ha));
        gcode.push(format!("; Speed: {} mm/min, Power: S{}", layer.speed, layer.power));
        
        for node in tree.root().children() {
            process_node(&node, params, layer, &mut gcode);
        }
    }

    // --- Cutting Frame (Outline) ---
    if params.outline.enabled && params.outline.passes > 0 {
        gcode.push("; Cutting Frame".to_string());
        let size = tree.size();
        let w = size.width() * params.scale;
        let h = size.height() * params.scale;
        let s = params.outline.speed;
        let p = params.outline.power;

        for i in 0..params.outline.passes {
            gcode.push(format!("; Pass {}", i + 1));
            gcode.push(format!("G0X0Y0S0"));
            gcode.push(format!("G1X{w:.3}Y0S{p}F{s}"));
            gcode.push(format!("G1X{w:.3}Y{h:.3}"));
            gcode.push(format!("G1X0Y{h:.3}"));
            gcode.push(format!("G1X0Y0"));
        }
        gcode.push("M5".to_string());
    }

    gcode.push("M5".to_string());
    gcode.push("G0X0Y0".to_string());
    Ok(gcode)
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
