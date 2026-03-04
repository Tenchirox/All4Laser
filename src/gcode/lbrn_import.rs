/// LightBurn .lbrn2 import/export (F9)
/// .lbrn2 files are XML-based with shape and layer definitions

use crate::ui::drawing::{ShapeParams, ShapeKind};
use crate::ui::layers_new::{CutLayer, CutMode};

/// Parse a .lbrn2 XML file and extract shapes + layer overrides
pub fn import_lbrn2(content: &str) -> Result<(Vec<ShapeParams>, Vec<LbrnLayerOverride>), String> {
    let mut shapes = Vec::new();
    let mut layer_overrides = Vec::new();

    // Simple XML tag parser for LightBurn format
    for line in content.lines() {
        let trimmed = line.trim();

        // Parse CutSetting entries (layer definitions)
        if trimmed.starts_with("<CutSetting type=") {
            if let Some(ovr) = parse_cut_setting_line(trimmed) {
                layer_overrides.push(ovr);
            }
        }

        // Parse Shape entries — rectangles
        if trimmed.starts_with("<Shape Type=\"Rect\"") || trimmed.starts_with("<Shape Type=\"0\"") {
            if let Some(shape) = parse_rect_shape(trimmed) {
                shapes.push(shape);
            }
        }

        // Parse Shape entries — ellipses/circles
        if trimmed.starts_with("<Shape Type=\"Ellipse\"") || trimmed.starts_with("<Shape Type=\"1\"") {
            if let Some(shape) = parse_ellipse_shape(trimmed) {
                shapes.push(shape);
            }
        }

        // Parse path vertices
        if trimmed.starts_with("<V vx=") {
            // Vertices are collected by a higher-level path parser
            // For now we skip raw vertex parsing
        }
    }

    Ok((shapes, layer_overrides))
}

#[derive(Clone, Debug)]
pub struct LbrnLayerOverride {
    pub index: usize,
    pub speed: f32,
    pub power: f32, // 0-100 in LightBurn, we convert to 0-1000
    pub mode: CutMode,
    pub passes: u32,
}

fn parse_cut_setting_line(line: &str) -> Option<LbrnLayerOverride> {
    let index = extract_attr(line, "index")?.parse::<usize>().ok()?;
    let speed = extract_attr(line, "speed").and_then(|s| s.parse::<f32>().ok()).unwrap_or(1000.0);
    let max_power = extract_attr(line, "maxPower").and_then(|s| s.parse::<f32>().ok()).unwrap_or(50.0);
    let mode_str = extract_attr(line, "type").unwrap_or_default();
    let mode = match mode_str.as_str() {
        "Cut" | "00" => CutMode::Line,
        "Scan" | "01" => CutMode::Fill,
        "Scan+Cut" | "02" => CutMode::FillAndLine,
        _ => CutMode::Line,
    };
    Some(LbrnLayerOverride {
        index,
        speed,
        power: max_power * 10.0, // Convert 0-100% to 0-1000 S-value
        mode,
        passes: 1,
    })
}

fn parse_rect_shape(line: &str) -> Option<ShapeParams> {
    let x = extract_attr(line, "X")?.parse::<f32>().ok()?;
    let y = extract_attr(line, "Y")?.parse::<f32>().ok()?;
    let w = extract_attr(line, "W").and_then(|s| s.parse::<f32>().ok()).unwrap_or(10.0);
    let h = extract_attr(line, "H").and_then(|s| s.parse::<f32>().ok()).unwrap_or(10.0);
    Some(ShapeParams {
        shape: ShapeKind::Rectangle,
        x, y,
        width: w, height: h,
        radius: 0.0,
        layer_idx: 0,
        text: String::new(),
        font_size_mm: 0.0,
        rotation: 0.0,
        group_id: None,
    })
}

fn parse_ellipse_shape(line: &str) -> Option<ShapeParams> {
    let cx = extract_attr(line, "CX")?.parse::<f32>().ok()?;
    let cy = extract_attr(line, "CY")?.parse::<f32>().ok()?;
    let rx = extract_attr(line, "Rx").and_then(|s| s.parse::<f32>().ok()).unwrap_or(10.0);
    Some(ShapeParams {
        shape: ShapeKind::Circle,
        x: cx, y: cy,
        width: 0.0, height: 0.0,
        radius: rx,
        layer_idx: 0,
        text: String::new(),
        font_size_mm: 0.0,
        rotation: 0.0,
        group_id: None,
    })
}

fn extract_attr(line: &str, attr: &str) -> Option<String> {
    let pattern = format!("{}=\"", attr);
    let start = line.find(&pattern)? + pattern.len();
    let rest = &line[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

/// Export shapes to a basic .lbrn2 XML format (F9)
pub fn export_lbrn2(shapes: &[ShapeParams], layers: &[CutLayer]) -> String {
    let mut xml = String::new();
    xml += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    xml += "<LightBurnProject AppVersion=\"All4Laser Export\">\n";

    // Export cut settings
    for (i, layer) in layers.iter().enumerate() {
        let mode_str = match layer.mode {
            CutMode::Line => "Cut",
            CutMode::Fill => "Scan",
            CutMode::FillAndLine => "Scan+Cut",
            CutMode::Offset => "Cut",
        };
        xml += &format!(
            "  <CutSetting type=\"{mode_str}\" index=\"{i}\" speed=\"{:.1}\" maxPower=\"{:.1}\" passes=\"{}\"/>\n",
            layer.speed, layer.power / 10.0, layer.passes
        );
    }

    // Export shapes
    for s in shapes {
        match &s.shape {
            ShapeKind::Rectangle => {
                xml += &format!(
                    "  <Shape Type=\"Rect\" X=\"{:.3}\" Y=\"{:.3}\" W=\"{:.3}\" H=\"{:.3}\" Layer=\"{}\"/>\n",
                    s.x, s.y, s.width, s.height, s.layer_idx
                );
            }
            ShapeKind::Circle => {
                xml += &format!(
                    "  <Shape Type=\"Ellipse\" CX=\"{:.3}\" CY=\"{:.3}\" Rx=\"{:.3}\" Layer=\"{}\"/>\n",
                    s.x, s.y, s.radius, s.layer_idx
                );
            }
            ShapeKind::Path(pts) if pts.len() >= 2 => {
                xml += &format!("  <Shape Type=\"Path\" Layer=\"{}\">\n", s.layer_idx);
                for (i, p) in pts.iter().enumerate() {
                    let (wx, wy) = s.world_pos(p.0, p.1);
                    let cmd = if i == 0 { "M" } else { "L" };
                    xml += &format!("    <V vx=\"{wx:.3}\" vy=\"{wy:.3}\" c=\"{cmd}\"/>\n");
                }
                xml += "  </Shape>\n";
            }
            _ => {}
        }
    }

    xml += "</LightBurnProject>\n";
    xml
}
