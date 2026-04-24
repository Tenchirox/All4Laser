#![allow(dead_code)]

/// PDF / AI file importer — extracts vector paths from PDF content streams
///
/// Handles PDF path operators: m (moveto), l (lineto), c (curveto),
/// h (closepath), re (rectangle), S/s (stroke), f/F (fill).
/// Modern Adobe Illustrator (.ai) files are PDF internally.

use crate::ui::drawing::{PathData, PathSegment, ShapeKind, ShapeParams};
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
    let mut segments: Vec<PathSegment> = Vec::new();
    let mut sub_start: (f32, f32) = (0.0, 0.0);
    let mut has_curves = false;
    let mut subpath_start: Option<(f32, f32)> = None;
    let mut cx: f32 = 0.0;
    let mut cy: f32 = 0.0;

    // CTM stack for coordinate transforms (simplified: only track scale/translate)
    let mut ctm_stack: Vec<[f32; 6]> = Vec::new();
    let mut ctm: [f32; 6] = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]; // identity

    let transform = |ctm: &[f32; 6], x: f32, y: f32| -> (f32, f32) {
        // CTM [a b c d e f] represents:
        // | a  b  0 |
        // | c  d  0 |
        // | e  f  1 |
        // For row vector [x y 1]: tx = x*a + y*c + e, ty = x*b + y*d + f
        let tx = ctm[0] * x + ctm[2] * y + ctm[4];
        let ty = ctm[1] * x + ctm[3] * y + ctm[5];
        // Convert from PDF units (1/72 inch) to mm
        (tx * PDF_UNIT_MM, ty * PDF_UNIT_MM)
    };

    let flush = |segments: &mut Vec<PathSegment>, sub_start: &(f32, f32), has_curves: &mut bool, shapes: &mut Vec<ShapeParams>, layer_idx: usize| {
        if segments.is_empty() { return; }
        let pd = if *has_curves {
            PathData::from_segments(*sub_start, std::mem::take(segments))
        } else {
            let mut pts = vec![*sub_start];
            for s in segments.iter() {
                if let PathSegment::LineTo(x, y) = s {
                    pts.push((*x, *y));
                }
            }
            segments.clear();
            PathData::from_points(pts)
        };
        if pd.points.len() >= 2 {
            let min_x = pd.points.iter().map(|p| p.0).fold(f32::MAX, f32::min);
            let min_y = pd.points.iter().map(|p| p.1).fold(f32::MAX, f32::min);
            if pd.has_curves() {
                let local_start = (pd.start.0 - min_x, pd.start.1 - min_y);
                let local_segs: Vec<PathSegment> = pd.segments.iter().map(|seg| match seg {
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
                shapes.push(ShapeParams {
                    shape: ShapeKind::Path(PathData::from_segments(local_start, local_segs)),
                    x: min_x, y: min_y, layer_idx, ..Default::default()
                });
            } else {
                let rel: Vec<(f32,f32)> = pd.points.iter().map(|&(x,y)| (x - min_x, y - min_y)).collect();
                shapes.push(ShapeParams {
                    shape: ShapeKind::Path(PathData::from_points(rel)),
                    x: min_x, y: min_y, layer_idx, ..Default::default()
                });
            }
        }
        *has_curves = false;
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
                        ctm = multiply_ctm(&new, &ctm); // Fixed order: M_new x CTM_old
                    }
                }
            }
            "m" => {
                // moveto
                if op.operands.len() >= 2 {
                    let x = obj_to_f32(&op.operands[0]).unwrap_or(0.0);
                    let y = obj_to_f32(&op.operands[1]).unwrap_or(0.0);
                    let (tx, ty) = transform(&ctm, x, y);
                    // Flush previous sub-path
                    if !segments.is_empty() {
                        flush(&mut segments, &sub_start, &mut has_curves, &mut shapes, layer_idx);
                    }
                    cx = tx;
                    cy = ty;
                    sub_start = (tx, ty);
                    subpath_start = Some((tx, ty));
                }
            }
            "l" => {
                // lineto
                if op.operands.len() >= 2 {
                    let x = obj_to_f32(&op.operands[0]).unwrap_or(0.0);
                    let y = obj_to_f32(&op.operands[1]).unwrap_or(0.0);
                    let (tx, ty) = transform(&ctm, x, y);
                    segments.push(PathSegment::LineTo(tx, ty));
                    cx = tx;
                    cy = ty;
                }
            }
            "c" => {
                // curveto (cubic bezier)
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

                    has_curves = true;
                    segments.push(PathSegment::CubicBezier {
                        c1: (tx1, ty1), c2: (tx2, ty2), end: (tx3, ty3),
                    });
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

                    has_curves = true;
                    segments.push(PathSegment::CubicBezier {
                        c1: (cx, cy), c2: (tx2, ty2), end: (tx3, ty3),
                    });
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

                    has_curves = true;
                    segments.push(PathSegment::CubicBezier {
                        c1: (tx1, ty1), c2: (tx3, ty3), end: (tx3, ty3),
                    });
                    cx = tx3;
                    cy = ty3;
                }
            }
            "h" => {
                // closepath
                if let Some((sx, sy)) = subpath_start {
                    segments.push(PathSegment::LineTo(sx, sy));
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

                    // Transform the four corners of the rectangle
                    let p1 = transform(&ctm, x, y);
                    let p2 = transform(&ctm, x + w, y);
                    let p3 = transform(&ctm, x + w, y + h);
                    let p4 = transform(&ctm, x, y + h);

                    // Check if there's significant rotation (non-axis-aligned)
                    let has_rotation = (p2.0 - p1.0).abs() < 0.001 || (p4.0 - p1.0).abs() < 0.001;
                    
                    if has_rotation {
                        // Create as path with the transformed corners
                        let min_x = p1.0.min(p2.0).min(p3.0).min(p4.0);
                        let min_y = p1.1.min(p2.1).min(p3.1).min(p4.1);
                        let local_p1 = (p1.0 - min_x, p1.1 - min_y);
                        let local_p2 = (p2.0 - min_x, p2.1 - min_y);
                        let local_p3 = (p3.0 - min_x, p3.1 - min_y);
                        let local_p4 = (p4.0 - min_x, p4.1 - min_y);
                        
                        let path = PathData::from_segments(
                            local_p1,
                            vec![
                                PathSegment::LineTo(local_p2.0, local_p2.1),
                                PathSegment::LineTo(local_p3.0, local_p3.1),
                                PathSegment::LineTo(local_p4.0, local_p4.1),
                                PathSegment::LineTo(local_p1.0, local_p1.1),
                            ],
                        );
                        shapes.push(ShapeParams {
                            shape: ShapeKind::Path(path),
                            x: min_x,
                            y: min_y,
                            layer_idx,
                            ..Default::default()
                        });
                    } else {
                        // Axis-aligned: can use Rectangle shape
                        let w_mm = (p2.0 - p1.0).abs();
                        let h_mm = (p4.1 - p1.1).abs();
                        if w_mm > 0.01 && h_mm > 0.01 {
                            shapes.push(ShapeParams {
                                shape: ShapeKind::Rectangle,
                                x: p1.0.min(p2.0),
                                y: p1.1.min(p4.1),
                                width: w_mm,
                                height: h_mm,
                                layer_idx,
                                ..Default::default()
                            });
                        }
                    }
                }
            }
            // Path painting operators — flush the current path
            "S" | "s" | "f" | "F" | "f*" | "B" | "B*" | "b" | "b*" => {
                if op.operator.as_str() == "s" || op.operator.as_str() == "b" || op.operator.as_str() == "b*" {
                    // close + stroke/fill
                    if let Some((sx, sy)) = subpath_start {
                        if !segments.is_empty() {
                            segments.push(PathSegment::LineTo(sx, sy));
                        }
                    }
                }
                flush(&mut segments, &sub_start, &mut has_curves, &mut shapes, layer_idx);
                subpath_start = None;
            }
            "n" => {
                // End path without painting (clipping path)
                segments.clear();
                has_curves = false;
                subpath_start = None;
            }
            _ => {}
        }
    }

    // Flush any remaining path
    flush(&mut segments, &sub_start, &mut has_curves, &mut shapes, layer_idx);

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
