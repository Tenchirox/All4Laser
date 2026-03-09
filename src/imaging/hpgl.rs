#![allow(dead_code)]

/// HPGL / PLT file importer — converts pen plotter commands to drawing shapes
///
/// Supported commands: IN, SP, PU, PD, PA, CI, LT
/// Coordinates are in plotter units (1 unit = 0.025 mm by default).

use crate::ui::drawing::{ShapeKind, ShapeParams};

const HPGL_UNIT_MM: f32 = 0.025; // 1 HPGL unit = 0.025 mm (40 units per mm)

pub fn parse_hpgl(data: &str, layer_idx: usize) -> Result<Vec<ShapeParams>, String> {
    let mut shapes: Vec<ShapeParams> = Vec::new();
    let mut pen_down = false;
    let mut current_path: Vec<(f32, f32)> = Vec::new();
    let mut cx: f32 = 0.0;
    let mut cy: f32 = 0.0;

    // Normalize: HPGL commands are separated by ';' and may span lines
    let normalized = data.replace('\n', "").replace('\r', "");

    for cmd_str in normalized.split(';') {
        let cmd_str = cmd_str.trim();
        if cmd_str.is_empty() {
            continue;
        }

        // Extract the 2-letter command and the rest as parameters
        let (op, params_str) = if cmd_str.len() >= 2 && cmd_str.as_bytes()[0].is_ascii_alphabetic() {
            let op_end = cmd_str
                .char_indices()
                .find(|(_, c)| !c.is_ascii_alphabetic())
                .map(|(i, _)| i)
                .unwrap_or(cmd_str.len());
            let op = &cmd_str[..op_end];
            let rest = &cmd_str[op_end..];
            (op.to_uppercase(), rest.to_string())
        } else {
            continue;
        };

        match op.as_str() {
            "IN" | "SP" | "LT" | "VS" | "FS" | "WU" | "IP" | "SC" => {
                // Initialize / select pen / line type — no geometry action needed
            }
            "PU" => {
                // Pen Up — flush current path, then move
                flush_path(&mut current_path, &mut shapes, layer_idx);
                pen_down = false;
                let coords = parse_coord_pairs(&params_str);
                if let Some(&(x, y)) = coords.last() {
                    cx = x * HPGL_UNIT_MM;
                    cy = y * HPGL_UNIT_MM;
                }
            }
            "PD" => {
                // Pen Down — start drawing, process coordinate pairs
                pen_down = true;
                let coords = parse_coord_pairs(&params_str);
                if coords.is_empty() {
                    // PD with no coords just sets pen down
                    if current_path.is_empty() {
                        current_path.push((cx, cy));
                    }
                } else {
                    if current_path.is_empty() {
                        current_path.push((cx, cy));
                    }
                    for &(x, y) in &coords {
                        cx = x * HPGL_UNIT_MM;
                        cy = y * HPGL_UNIT_MM;
                        current_path.push((cx, cy));
                    }
                }
            }
            "PA" => {
                // Plot Absolute — move/draw to coordinates
                let coords = parse_coord_pairs(&params_str);
                for &(x, y) in &coords {
                    let nx = x * HPGL_UNIT_MM;
                    let ny = y * HPGL_UNIT_MM;
                    if pen_down {
                        if current_path.is_empty() {
                            current_path.push((cx, cy));
                        }
                        current_path.push((nx, ny));
                    }
                    cx = nx;
                    cy = ny;
                }
            }
            "CI" => {
                // Circle — CI radius[,chord_angle]
                flush_path(&mut current_path, &mut shapes, layer_idx);
                let parts: Vec<&str> = params_str.split(',').collect();
                if let Some(r_str) = parts.first() {
                    if let Ok(r) = r_str.trim().parse::<f32>() {
                        let r_mm = r * HPGL_UNIT_MM;
                        let shape = ShapeParams {
                            shape: ShapeKind::Circle,
                            x: cx,
                            y: cy,
                            width: r_mm * 2.0,
                            height: r_mm * 2.0,
                            layer_idx,
                            ..Default::default()
                        };
                        shapes.push(shape);
                    }
                }
            }
            "PR" => {
                // Plot Relative
                let coords = parse_coord_pairs(&params_str);
                for &(dx, dy) in &coords {
                    let nx = cx + dx * HPGL_UNIT_MM;
                    let ny = cy + dy * HPGL_UNIT_MM;
                    if pen_down {
                        if current_path.is_empty() {
                            current_path.push((cx, cy));
                        }
                        current_path.push((nx, ny));
                    }
                    cx = nx;
                    cy = ny;
                }
            }
            _ => {
                // Unknown command — skip
            }
        }
    }

    // Flush any remaining path
    flush_path(&mut current_path, &mut shapes, layer_idx);

    if shapes.is_empty() {
        return Err("No geometry found in HPGL file".into());
    }

    // Normalize: shift all shapes so minimum is near origin
    normalize_shapes(&mut shapes);

    Ok(shapes)
}

fn parse_coord_pairs(s: &str) -> Vec<(f32, f32)> {
    let mut result = Vec::new();
    let nums: Vec<f32> = s
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|t| !t.is_empty())
        .filter_map(|t| t.trim().parse::<f32>().ok())
        .collect();

    for chunk in nums.chunks(2) {
        if chunk.len() == 2 {
            result.push((chunk[0], chunk[1]));
        }
    }
    result
}

fn flush_path(
    current_path: &mut Vec<(f32, f32)>,
    shapes: &mut Vec<ShapeParams>,
    layer_idx: usize,
) {
    if current_path.len() >= 2 {
        // Compute bounding box to get origin
        let min_x = current_path.iter().map(|p| p.0).fold(f32::MAX, f32::min);
        let min_y = current_path.iter().map(|p| p.1).fold(f32::MAX, f32::min);

        // Make path relative to its bounding box origin
        let rel_points: Vec<(f32, f32)> = current_path
            .iter()
            .map(|&(x, y)| (x - min_x, y - min_y))
            .collect();

        let shape = ShapeParams {
            shape: ShapeKind::Path(rel_points),
            x: min_x,
            y: min_y,
            layer_idx,
            ..Default::default()
        };
        shapes.push(shape);
    }
    current_path.clear();
}

fn normalize_shapes(shapes: &mut [ShapeParams]) {
    if shapes.is_empty() {
        return;
    }
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    for s in shapes.iter() {
        min_x = min_x.min(s.x);
        min_y = min_y.min(s.y);
    }
    if min_x != 0.0 || min_y != 0.0 {
        for s in shapes.iter_mut() {
            s.x -= min_x;
            s.y -= min_y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_hpgl_line() {
        let hpgl = "IN;SP1;PU0,0;PD1000,0;PD1000,1000;PU;";
        let shapes = parse_hpgl(hpgl, 0).unwrap();
        assert!(!shapes.is_empty());
        // Should produce a single path with 3 points
        if let ShapeKind::Path(pts) = &shapes[0].shape {
            assert_eq!(pts.len(), 3);
        } else {
            panic!("Expected Path shape");
        }
    }

    #[test]
    fn test_hpgl_circle() {
        let hpgl = "IN;SP1;PU2000,2000;CI500;";
        let shapes = parse_hpgl(hpgl, 0).unwrap();
        assert_eq!(shapes.len(), 1);
        assert!(matches!(shapes[0].shape, ShapeKind::Circle));
    }

    #[test]
    fn test_empty_hpgl() {
        let hpgl = "IN;SP1;";
        assert!(parse_hpgl(hpgl, 0).is_err());
    }
}
