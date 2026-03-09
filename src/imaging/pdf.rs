#![allow(dead_code)]

/// PDF / AI file importer — extracts vector paths from PDF content streams
///
/// Handles PDF path operators: m (moveto), l (lineto), c (curveto),
/// h (closepath), re (rectangle), S/s (stroke), f/F (fill).
/// Modern Adobe Illustrator (.ai) files are PDF internally.

use crate::ui::drawing::{ShapeKind, ShapeParams};
use lopdf::Document;

/// Default PDF unit is 1/72 inch = 0.3528 mm
const PDF_UNIT_MM: f32 = 0.3528;

pub fn parse_pdf(data: &[u8], layer_idx: usize) -> Result<Vec<ShapeParams>, String> {
    let doc = Document::load_mem(data).map_err(|e| format!("PDF parse error: {e}"))?;

    let mut all_shapes: Vec<ShapeParams> = Vec::new();

    for page_id in doc.page_iter() {
        let content = match doc.get_page_content(page_id) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let ops = match lopdf::content::Content::decode(&content) {
            Ok(c) => c.operations,
            Err(_) => continue,
        };

        let mut shapes = extract_paths_from_ops(&ops, layer_idx);
        all_shapes.append(&mut shapes);
    }

    if all_shapes.is_empty() {
        return Err("No vector geometry found in PDF".into());
    }

    normalize_shapes(&mut all_shapes);
    Ok(all_shapes)
}

fn extract_paths_from_ops(
    ops: &[lopdf::content::Operation],
    layer_idx: usize,
) -> Vec<ShapeParams> {
    let mut shapes: Vec<ShapeParams> = Vec::new();
    let mut current_path: Vec<(f32, f32)> = Vec::new();
    let mut subpath_start: Option<(f32, f32)> = None;
    let mut cx: f32 = 0.0;
    let mut cy: f32 = 0.0;

    // CTM stack for coordinate transforms (simplified: only track scale/translate)
    let mut ctm_stack: Vec<[f32; 6]> = Vec::new();
    let mut ctm: [f32; 6] = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]; // identity

    let transform = |ctm: &[f32; 6], x: f32, y: f32| -> (f32, f32) {
        let tx = ctm[0] * x + ctm[2] * y + ctm[4];
        let ty = ctm[1] * x + ctm[3] * y + ctm[5];
        (tx * PDF_UNIT_MM, ty * PDF_UNIT_MM)
    };

    for op in ops {
        match op.operator.as_str() {
            "q" => {
                ctm_stack.push(ctm);
            }
            "Q" => {
                if let Some(saved) = ctm_stack.pop() {
                    ctm = saved;
                }
            }
            "cm" => {
                // Concatenate matrix
                if op.operands.len() >= 6 {
                    let vals: Vec<f32> = op
                        .operands
                        .iter()
                        .filter_map(|o| obj_to_f32(o))
                        .collect();
                    if vals.len() >= 6 {
                        let new = [vals[0], vals[1], vals[2], vals[3], vals[4], vals[5]];
                        ctm = multiply_ctm(&ctm, &new);
                    }
                }
            }
            "m" => {
                // moveto
                if op.operands.len() >= 2 {
                    let x = obj_to_f32(&op.operands[0]).unwrap_or(0.0);
                    let y = obj_to_f32(&op.operands[1]).unwrap_or(0.0);
                    let (tx, ty) = transform(&ctm, x, y);
                    // If there's a current path, flush it before starting a new subpath
                    if current_path.len() >= 2 {
                        flush_path(&mut current_path, &mut shapes, layer_idx);
                    } else {
                        current_path.clear();
                    }
                    cx = tx;
                    cy = ty;
                    subpath_start = Some((tx, ty));
                    current_path.push((tx, ty));
                }
            }
            "l" => {
                // lineto
                if op.operands.len() >= 2 {
                    let x = obj_to_f32(&op.operands[0]).unwrap_or(0.0);
                    let y = obj_to_f32(&op.operands[1]).unwrap_or(0.0);
                    let (tx, ty) = transform(&ctm, x, y);
                    if current_path.is_empty() {
                        current_path.push((cx, cy));
                    }
                    current_path.push((tx, ty));
                    cx = tx;
                    cy = ty;
                }
            }
            "c" => {
                // curveto (cubic bezier) — approximate with line segments
                if op.operands.len() >= 6 {
                    let x1 = obj_to_f32(&op.operands[0]).unwrap_or(0.0);
                    let y1 = obj_to_f32(&op.operands[1]).unwrap_or(0.0);
                    let x2 = obj_to_f32(&op.operands[2]).unwrap_or(0.0);
                    let y2 = obj_to_f32(&op.operands[3]).unwrap_or(0.0);
                    let x3 = obj_to_f32(&op.operands[4]).unwrap_or(0.0);
                    let y3 = obj_to_f32(&op.operands[5]).unwrap_or(0.0);

                    let (tx1, ty1) = transform(&ctm, x1, y1);
                    let (tx2, ty2) = transform(&ctm, x2, y2);
                    let (tx3, ty3) = transform(&ctm, x3, y3);

                    if current_path.is_empty() {
                        current_path.push((cx, cy));
                    }

                    // Flatten cubic bezier with 16 segments
                    let n = 16;
                    for i in 1..=n {
                        let t = i as f32 / n as f32;
                        let mt = 1.0 - t;
                        let px = mt * mt * mt * cx
                            + 3.0 * mt * mt * t * tx1
                            + 3.0 * mt * t * t * tx2
                            + t * t * t * tx3;
                        let py = mt * mt * mt * cy
                            + 3.0 * mt * mt * t * ty1
                            + 3.0 * mt * t * t * ty2
                            + t * t * t * ty3;
                        current_path.push((px, py));
                    }
                    cx = tx3;
                    cy = ty3;
                }
            }
            "v" => {
                // curveto with first control point = current point
                if op.operands.len() >= 4 {
                    let x2 = obj_to_f32(&op.operands[0]).unwrap_or(0.0);
                    let y2 = obj_to_f32(&op.operands[1]).unwrap_or(0.0);
                    let x3 = obj_to_f32(&op.operands[2]).unwrap_or(0.0);
                    let y3 = obj_to_f32(&op.operands[3]).unwrap_or(0.0);

                    let (tx2, ty2) = transform(&ctm, x2, y2);
                    let (tx3, ty3) = transform(&ctm, x3, y3);

                    if current_path.is_empty() {
                        current_path.push((cx, cy));
                    }
                    let n = 16;
                    for i in 1..=n {
                        let t = i as f32 / n as f32;
                        let mt = 1.0 - t;
                        let px = mt * mt * mt * cx
                            + 3.0 * mt * mt * t * cx
                            + 3.0 * mt * t * t * tx2
                            + t * t * t * tx3;
                        let py = mt * mt * mt * cy
                            + 3.0 * mt * mt * t * cy
                            + 3.0 * mt * t * t * ty2
                            + t * t * t * ty3;
                        current_path.push((px, py));
                    }
                    cx = tx3;
                    cy = ty3;
                }
            }
            "y" => {
                // curveto with last control point = endpoint
                if op.operands.len() >= 4 {
                    let x1 = obj_to_f32(&op.operands[0]).unwrap_or(0.0);
                    let y1 = obj_to_f32(&op.operands[1]).unwrap_or(0.0);
                    let x3 = obj_to_f32(&op.operands[2]).unwrap_or(0.0);
                    let y3 = obj_to_f32(&op.operands[3]).unwrap_or(0.0);

                    let (tx1, ty1) = transform(&ctm, x1, y1);
                    let (tx3, ty3) = transform(&ctm, x3, y3);

                    if current_path.is_empty() {
                        current_path.push((cx, cy));
                    }
                    let n = 16;
                    for i in 1..=n {
                        let t = i as f32 / n as f32;
                        let mt = 1.0 - t;
                        let px = mt * mt * mt * cx
                            + 3.0 * mt * mt * t * tx1
                            + 3.0 * mt * t * t * tx3
                            + t * t * t * tx3;
                        let py = mt * mt * mt * cy
                            + 3.0 * mt * mt * t * ty1
                            + 3.0 * mt * t * t * ty3
                            + t * t * t * ty3;
                        current_path.push((px, py));
                    }
                    cx = tx3;
                    cy = ty3;
                }
            }
            "h" => {
                // closepath
                if let Some((sx, sy)) = subpath_start {
                    if current_path.is_empty() {
                        current_path.push((cx, cy));
                    }
                    current_path.push((sx, sy));
                    cx = sx;
                    cy = sy;
                }
            }
            "re" => {
                // rectangle: x y w h
                if op.operands.len() >= 4 {
                    let x = obj_to_f32(&op.operands[0]).unwrap_or(0.0);
                    let y = obj_to_f32(&op.operands[1]).unwrap_or(0.0);
                    let w = obj_to_f32(&op.operands[2]).unwrap_or(0.0);
                    let h = obj_to_f32(&op.operands[3]).unwrap_or(0.0);

                    let (tx, ty) = transform(&ctm, x, y);
                    let w_mm = w * ctm[0].abs() * PDF_UNIT_MM;
                    let h_mm = h * ctm[3].abs() * PDF_UNIT_MM;

                    if w_mm > 0.01 && h_mm > 0.01 {
                        shapes.push(ShapeParams {
                            shape: ShapeKind::Rectangle,
                            x: tx,
                            y: ty,
                            width: w_mm,
                            height: h_mm,
                            layer_idx,
                            ..Default::default()
                        });
                    }
                }
            }
            // Path painting operators — flush the current path
            "S" | "s" | "f" | "F" | "f*" | "B" | "B*" | "b" | "b*" => {
                if op.operator.as_str() == "s" || op.operator.as_str() == "b" || op.operator.as_str() == "b*" {
                    // close + stroke/fill
                    if let Some((sx, sy)) = subpath_start {
                        if !current_path.is_empty() {
                            current_path.push((sx, sy));
                        }
                    }
                }
                flush_path(&mut current_path, &mut shapes, layer_idx);
                subpath_start = None;
            }
            "n" => {
                // End path without painting (clipping path)
                current_path.clear();
                subpath_start = None;
            }
            _ => {}
        }
    }

    // Flush any remaining path
    flush_path(&mut current_path, &mut shapes, layer_idx);

    shapes
}

fn obj_to_f32(obj: &lopdf::Object) -> Option<f32> {
    match obj {
        lopdf::Object::Integer(i) => Some(*i as f32),
        lopdf::Object::Real(f) => Some(*f as f32),
        _ => None,
    }
}

fn multiply_ctm(a: &[f32; 6], b: &[f32; 6]) -> [f32; 6] {
    [
        a[0] * b[0] + a[1] * b[2],
        a[0] * b[1] + a[1] * b[3],
        a[2] * b[0] + a[3] * b[2],
        a[2] * b[1] + a[3] * b[3],
        a[4] * b[0] + a[5] * b[2] + b[4],
        a[4] * b[1] + a[5] * b[3] + b[5],
    ]
}

fn flush_path(
    current_path: &mut Vec<(f32, f32)>,
    shapes: &mut Vec<ShapeParams>,
    layer_idx: usize,
) {
    if current_path.len() >= 2 {
        let min_x = current_path.iter().map(|p| p.0).fold(f32::MAX, f32::min);
        let min_y = current_path.iter().map(|p| p.1).fold(f32::MAX, f32::min);

        let rel_points: Vec<(f32, f32)> = current_path
            .iter()
            .map(|&(x, y)| (x - min_x, y - min_y))
            .collect();

        shapes.push(ShapeParams {
            shape: ShapeKind::Path(rel_points),
            x: min_x,
            y: min_y,
            layer_idx,
            ..Default::default()
        });
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
    fn test_obj_to_f32() {
        assert_eq!(obj_to_f32(&lopdf::Object::Integer(42)), Some(42.0));
        assert_eq!(obj_to_f32(&lopdf::Object::Real(3.14)), Some(3.14));
        assert_eq!(obj_to_f32(&lopdf::Object::Boolean(true)), None);
    }

    #[test]
    fn test_ctm_multiply_identity() {
        let id = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        let m = [2.0, 0.0, 0.0, 3.0, 10.0, 20.0];
        let result = multiply_ctm(&id, &m);
        assert_eq!(result, m);
    }
}
