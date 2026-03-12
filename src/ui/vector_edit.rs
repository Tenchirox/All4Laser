#![allow(dead_code)]

use crate::ui::drawing::{DrawingState, PathData, ShapeKind};

const JOIN_EPS: f32 = 1e-3;

fn dist2(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    dx * dx + dy * dy
}

fn is_closed_path(pts: &[(f32, f32)]) -> bool {
    pts.len() > 2 && dist2(pts[0], *pts.last().unwrap_or(&pts[0])) <= JOIN_EPS
}

fn point_segment_distance_sq(p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    let abx = b.0 - a.0;
    let aby = b.1 - a.1;
    let apx = p.0 - a.0;
    let apy = p.1 - a.1;
    let ab_len_sq = abx * abx + aby * aby;
    if ab_len_sq <= f32::EPSILON {
        return dist2(p, a);
    }
    let t = ((apx * abx + apy * aby) / ab_len_sq).clamp(0.0, 1.0);
    let proj = (a.0 + abx * t, a.1 + aby * t);
    dist2(p, proj)
}

fn rdp_mark(pts: &[(f32, f32)], keep: &mut [bool], start: usize, end: usize, tol_sq: f32) {
    if end <= start + 1 {
        return;
    }

    let a = pts[start];
    let b = pts[end];
    let mut max_d_sq = -1.0_f32;
    let mut max_idx = start;
    for (i, p) in pts.iter().enumerate().take(end).skip(start + 1) {
        let d_sq = point_segment_distance_sq(*p, a, b);
        if d_sq > max_d_sq {
            max_d_sq = d_sq;
            max_idx = i;
        }
    }

    if max_d_sq > tol_sq {
        keep[max_idx] = true;
        rdp_mark(pts, keep, start, max_idx, tol_sq);
        rdp_mark(pts, keep, max_idx, end, tol_sq);
    }
}

fn max_deviation_to_polyline(points: &[(f32, f32)], polyline: &[(f32, f32)]) -> f32 {
    if polyline.len() < 2 || points.is_empty() {
        return 0.0;
    }
    let mut max_d_sq = 0.0_f32;
    for p in points {
        let mut best = f32::MAX;
        for i in 0..(polyline.len() - 1) {
            best = best.min(point_segment_distance_sq(*p, polyline[i], polyline[i + 1]));
        }
        max_d_sq = max_d_sq.max(best);
    }
    max_d_sq.sqrt()
}

pub fn insert_node_on_segment(
    drawing: &mut DrawingState,
    shape_idx: usize,
    insert_after: usize,
    local_point: (f32, f32),
) -> Result<usize, String> {
    let Some(shape) = drawing.shapes.get_mut(shape_idx) else {
        return Err("Invalid shape selection".into());
    };
    let ShapeKind::Path(pts) = &mut shape.shape else {
        return Err("Selected shape is not a path".into());
    };
    if pts.len() < 2 || insert_after >= pts.len() - 1 {
        return Err("Cannot insert node on this segment".into());
    }
    let insert_idx = insert_after + 1;
    pts.insert(insert_idx, local_point);
    Ok(insert_idx)
}

pub fn insert_midpoint_after(
    drawing: &mut DrawingState,
    shape_idx: usize,
    node_idx: usize,
) -> Result<usize, String> {
    let Some(shape) = drawing.shapes.get_mut(shape_idx) else {
        return Err("Invalid shape selection".into());
    };
    let ShapeKind::Path(pts) = &mut shape.shape else {
        return Err("Selected shape is not a path".into());
    };
    if pts.len() < 2 || node_idx >= pts.len() - 1 {
        return Err("Need a node with a following segment".into());
    }
    let a = pts[node_idx];
    let b = pts[node_idx + 1];
    let m = ((a.0 + b.0) * 0.5, (a.1 + b.1) * 0.5);
    let insert_idx = node_idx + 1;
    pts.insert(insert_idx, m);
    Ok(insert_idx)
}

pub fn delete_node(
    drawing: &mut DrawingState,
    shape_idx: usize,
    node_idx: usize,
) -> Result<(), String> {
    let Some(shape) = drawing.shapes.get_mut(shape_idx) else {
        return Err("Invalid shape selection".into());
    };
    let ShapeKind::Path(pts) = &mut shape.shape else {
        return Err("Selected shape is not a path".into());
    };

    if pts.len() <= 2 {
        return Err("Path must keep at least 2 nodes".into());
    }
    if node_idx >= pts.len() {
        return Err("Invalid node".into());
    }

    let closed = pts.len() > 2 && dist2(pts[0], *pts.last().unwrap_or(&pts[0])) < JOIN_EPS;
    pts.remove(node_idx);

    if closed {
        if pts.is_empty() {
            return Err("Path became empty".into());
        }
        let first = pts[0];
        if let Some(last) = pts.last_mut() {
            *last = first;
        }
    }

    Ok(())
}

pub fn delete_nodes(
    drawing: &mut DrawingState,
    shape_idx: usize,
    node_indices: &[usize],
) -> Result<(), String> {
    let Some(shape) = drawing.shapes.get_mut(shape_idx) else {
        return Err("Invalid shape selection".into());
    };
    let ShapeKind::Path(pts) = &mut shape.shape else {
        return Err("Selected shape is not a path".into());
    };

    if node_indices.is_empty() {
        return Err("No nodes selected".into());
    }

    let mut indices: Vec<usize> = node_indices
        .iter()
        .copied()
        .filter(|i| *i < pts.len())
        .collect();
    indices.sort_unstable();
    indices.dedup();
    if indices.is_empty() {
        return Err("Invalid node selection".into());
    }

    let closed = is_closed_path(pts);
    let min_remaining = if closed { 4 } else { 2 };
    if pts.len().saturating_sub(indices.len()) < min_remaining {
        return Err("Path would become invalid after deleting selected nodes".into());
    }

    for idx in indices.into_iter().rev() {
        pts.remove(idx);
    }

    if closed {
        let first = pts[0];
        if let Some(last) = pts.last_mut() {
            *last = first;
        }
    }

    Ok(())
}

pub fn split_path_at_node(
    drawing: &mut DrawingState,
    shape_idx: usize,
    node_idx: usize,
) -> Result<usize, String> {
    let Some(base_shape) = drawing.shapes.get(shape_idx).cloned() else {
        return Err("Invalid shape selection".into());
    };

    let ShapeKind::Path(pts) = base_shape.shape.clone() else {
        return Err("Selected shape is not a path".into());
    };

    if pts.len() < 4 {
        return Err("Path too short to split".into());
    }
    if node_idx == 0 || node_idx >= pts.len() - 1 {
        return Err("Select an interior node to split".into());
    }

    let left = pts[..=node_idx].to_vec();
    let right = pts[node_idx..].to_vec();

    if left.len() < 2 || right.len() < 2 {
        return Err("Split produced invalid path".into());
    }

    if let Some(shape) = drawing.shapes.get_mut(shape_idx) {
        shape.shape = ShapeKind::Path(PathData::from_points(left));
    }

    let mut second = base_shape;
    second.shape = ShapeKind::Path(PathData::from_points(right));
    let insert_idx = shape_idx + 1;
    drawing.shapes.insert(insert_idx, second);
    Ok(insert_idx)
}

pub fn smooth_node(
    drawing: &mut DrawingState,
    shape_idx: usize,
    node_idx: usize,
) -> Result<(), String> {
    smooth_node_weighted(drawing, shape_idx, node_idx, 0.7)
}

pub fn smooth_node_weighted(
    drawing: &mut DrawingState,
    shape_idx: usize,
    node_idx: usize,
    strength: f32,
) -> Result<(), String> {
    let Some(shape) = drawing.shapes.get_mut(shape_idx) else {
        return Err("Invalid shape selection".into());
    };
    let ShapeKind::Path(pts) = &mut shape.shape else {
        return Err("Selected shape is not a path".into());
    };
    if pts.len() < 3 || node_idx == 0 || node_idx >= pts.len() - 1 {
        return Err("Need an interior node".into());
    }

    let prev = pts[node_idx - 1];
    let next = pts[node_idx + 1];
    let cur = pts[node_idx];
    let avg = ((prev.0 + next.0) * 0.5, (prev.1 + next.1) * 0.5);

    let k = strength.clamp(0.0, 1.0);
    pts[node_idx] = (cur.0 * (1.0 - k) + avg.0 * k, cur.1 * (1.0 - k) + avg.1 * k);
    Ok(())
}

pub fn corner_node(
    drawing: &mut DrawingState,
    shape_idx: usize,
    node_idx: usize,
) -> Result<(), String> {
    corner_node_weighted(drawing, shape_idx, node_idx, 0.7)
}

pub fn corner_node_weighted(
    drawing: &mut DrawingState,
    shape_idx: usize,
    node_idx: usize,
    strength: f32,
) -> Result<(), String> {
    let Some(shape) = drawing.shapes.get_mut(shape_idx) else {
        return Err("Invalid shape selection".into());
    };
    let ShapeKind::Path(pts) = &mut shape.shape else {
        return Err("Selected shape is not a path".into());
    };
    if pts.len() < 3 || node_idx == 0 || node_idx >= pts.len() - 1 {
        return Err("Need an interior node".into());
    }

    let prev = pts[node_idx - 1];
    let next = pts[node_idx + 1];
    let cur = pts[node_idx];
    let avg = ((prev.0 + next.0) * 0.5, (prev.1 + next.1) * 0.5);

    let vx = cur.0 - avg.0;
    let vy = cur.1 - avg.1;
    let k = strength.clamp(0.0, 1.5);
    pts[node_idx] = (cur.0 + vx * k, cur.1 + vy * k);
    Ok(())
}

pub fn smooth_nodes_weighted(
    drawing: &mut DrawingState,
    shape_idx: usize,
    node_indices: &[usize],
    strength: f32,
) -> Result<(), String> {
    if node_indices.is_empty() {
        return Err("No nodes selected".into());
    }
    let mut indices = node_indices.to_vec();
    indices.sort_unstable();
    indices.dedup();
    for idx in indices {
        let _ = smooth_node_weighted(drawing, shape_idx, idx, strength);
    }
    Ok(())
}

pub fn corner_nodes_weighted(
    drawing: &mut DrawingState,
    shape_idx: usize,
    node_indices: &[usize],
    strength: f32,
) -> Result<(), String> {
    if node_indices.is_empty() {
        return Err("No nodes selected".into());
    }
    let mut indices = node_indices.to_vec();
    indices.sort_unstable();
    indices.dedup();
    for idx in indices {
        let _ = corner_node_weighted(drawing, shape_idx, idx, strength);
    }
    Ok(())
}

pub fn simplify_path(
    drawing: &mut DrawingState,
    shape_idx: usize,
    tolerance: f32,
) -> Result<usize, String> {
    let Some(shape) = drawing.shapes.get_mut(shape_idx) else {
        return Err("Invalid shape selection".into());
    };
    let ShapeKind::Path(pts) = &mut shape.shape else {
        return Err("Selected shape is not a path".into());
    };
    if pts.len() < 3 {
        return Err("Path is too short to simplify".into());
    }

    let tol = tolerance.max(0.001);
    let closed = is_closed_path(pts);

    let mut working = pts.clone();
    if closed {
        working.pop();
    }
    if working.len() < 3 {
        return Err("Path is too short to simplify".into());
    }

    let mut keep = vec![false; working.len()];
    keep[0] = true;
    keep[working.len() - 1] = true;
    rdp_mark(&working, &mut keep, 0, working.len() - 1, tol * tol);
    let mut simplified: Vec<(f32, f32)> = working
        .iter()
        .enumerate()
        .filter(|(i, _)| keep[*i])
        .map(|(_, p)| *p)
        .collect();

    let min_nodes = if closed { 3 } else { 2 };
    if simplified.len() < min_nodes {
        return Err("Simplification removed too many nodes".into());
    }

    if closed {
        simplified.push(simplified[0]);
    }

    let max_dev = max_deviation_to_polyline(&working, &simplified);
    if max_dev > tol * 2.5 {
        return Err("Simplification exceeds quality tolerance".into());
    }

    let removed = pts.len().saturating_sub(simplified.len());
    pts.points = simplified;
    Ok(removed)
}

pub fn smooth_path(
    drawing: &mut DrawingState,
    shape_idx: usize,
    iterations: usize,
    strength: f32,
) -> Result<(), String> {
    let Some(shape) = drawing.shapes.get_mut(shape_idx) else {
        return Err("Invalid shape selection".into());
    };
    let ShapeKind::Path(pts) = &mut shape.shape else {
        return Err("Selected shape is not a path".into());
    };
    if pts.len() < 3 {
        return Err("Path is too short to smooth".into());
    }
    if iterations == 0 {
        return Ok(());
    }

    let closed = is_closed_path(pts);
    let k = strength.clamp(0.0, 1.0);

    let mut core: Vec<(f32, f32)> = pts.points.clone();
    if closed {
        core.pop();
    }
    if core.len() < 3 {
        return Ok(());
    }

    for _ in 0..iterations {
        let prev = core.clone();
        if closed {
            let n = prev.len();
            for i in 0..n {
                let p_prev = prev[(i + n - 1) % n];
                let p_cur = prev[i];
                let p_next = prev[(i + 1) % n];
                let avg = ((p_prev.0 + p_next.0) * 0.5, (p_prev.1 + p_next.1) * 0.5);
                core[i] = (
                    p_cur.0 * (1.0 - k) + avg.0 * k,
                    p_cur.1 * (1.0 - k) + avg.1 * k,
                );
            }
        } else {
            for i in 1..(prev.len() - 1) {
                let p_prev = prev[i - 1];
                let p_cur = prev[i];
                let p_next = prev[i + 1];
                let avg = ((p_prev.0 + p_next.0) * 0.5, (p_prev.1 + p_next.1) * 0.5);
                core[i] = (
                    p_cur.0 * (1.0 - k) + avg.0 * k,
                    p_cur.1 * (1.0 - k) + avg.1 * k,
                );
            }
        }
    }

    if closed {
        let first = core[0];
        core.push(first);
    }
    pts.points = core;
    Ok(())
}

pub fn join_selected_paths(
    drawing: &mut DrawingState,
    selection: &[usize],
) -> Result<usize, String> {
    if selection.len() < 2 {
        return Err("Select at least 2 paths to join".into());
    }

    let a_idx = selection[0];
    let b_idx = selection[1];
    if a_idx == b_idx {
        return Err("Select two different paths".into());
    }

    let Some(a_shape) = drawing.shapes.get(a_idx).cloned() else {
        return Err("Invalid first path".into());
    };
    let Some(b_shape) = drawing.shapes.get(b_idx).cloned() else {
        return Err("Invalid second path".into());
    };

    let ShapeKind::Path(a_data) = a_shape.shape.clone() else {
        return Err("First selection is not a path".into());
    };
    let ShapeKind::Path(b_data) = b_shape.shape.clone() else {
        return Err("Second selection is not a path".into());
    };
    let mut a_pts = a_data.points;
    let mut b_pts = b_data.points;

    if a_pts.len() < 2 || b_pts.len() < 2 {
        return Err("Each path needs at least 2 nodes".into());
    }

    let d_end_start = dist2(*a_pts.last().unwrap(), b_pts[0]);
    let d_end_end = dist2(*a_pts.last().unwrap(), *b_pts.last().unwrap());
    let d_start_start = dist2(a_pts[0], b_pts[0]);
    let d_start_end = dist2(a_pts[0], *b_pts.last().unwrap());

    let mut best = d_end_start;
    let mut mode = 0;
    if d_end_end < best {
        best = d_end_end;
        mode = 1;
    }
    if d_start_start < best {
        best = d_start_start;
        mode = 2;
    }
    if d_start_end < best {
        mode = 3;
    }

    match mode {
        1 => b_pts.reverse(),
        2 => a_pts.reverse(),
        3 => {
            a_pts.reverse();
            b_pts.reverse();
        }
        _ => {}
    }

    let mut merged = a_pts;
    if dist2(*merged.last().unwrap(), b_pts[0]) > JOIN_EPS {
        merged.push(b_pts[0]);
    }
    merged.extend_from_slice(&b_pts[1..]);

    let mut new_shape = a_shape;
    new_shape.shape = ShapeKind::Path(PathData::from_points(merged));


    let mut to_remove = [a_idx, b_idx];
    to_remove.sort_unstable();
    drawing.shapes.remove(to_remove[1]);
    drawing.shapes.remove(to_remove[0]);
    drawing.shapes.push(new_shape);

    Ok(drawing.shapes.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::drawing::{DrawingState, ShapeKind, ShapeParams};

    fn path_shape(points: Vec<(f32, f32)>) -> ShapeParams {
        ShapeParams {
            shape: ShapeKind::Path(PathData::from_points(points)),
            ..ShapeParams::default()
        }
    }

    #[test]
    fn simplify_path_reduces_collinear_points() {
        let mut drawing = DrawingState {
            current: ShapeParams::default(),
            shapes: vec![path_shape(vec![
                (0.0, 0.0),
                (1.0, 0.0),
                (2.0, 0.0),
                (3.0, 0.0),
            ])],
        };

        let removed = simplify_path(&mut drawing, 0, 0.01).unwrap();
        assert!(removed >= 2);
        let ShapeKind::Path(pts) = &drawing.shapes[0].shape else {
            panic!("expected path")
        };
        assert_eq!(pts.len(), 2);
    }

    #[test]
    fn smooth_path_keeps_open_endpoints() {
        let mut drawing = DrawingState {
            current: ShapeParams::default(),
            shapes: vec![path_shape(vec![(0.0, 0.0), (1.0, 2.0), (2.0, 0.0)])],
        };

        smooth_path(&mut drawing, 0, 3, 0.6).unwrap();
        let ShapeKind::Path(pts) = &drawing.shapes[0].shape else {
            panic!("expected path")
        };
        assert_eq!(pts.first().copied(), Some((0.0, 0.0)));
        assert_eq!(pts.last().copied(), Some((2.0, 0.0)));
    }

    #[test]
    fn delete_nodes_multi_keeps_path_valid() {
        let mut drawing = DrawingState {
            current: ShapeParams::default(),
            shapes: vec![path_shape(vec![
                (0.0, 0.0),
                (1.0, 0.5),
                (2.0, 1.0),
                (3.0, 0.5),
                (4.0, 0.0),
            ])],
        };

        delete_nodes(&mut drawing, 0, &[1, 3]).unwrap();
        let ShapeKind::Path(pts) = &drawing.shapes[0].shape else {
            panic!("expected path")
        };
        assert_eq!(pts.len(), 3);
        assert_eq!(pts[0], (0.0, 0.0));
        assert_eq!(pts[2], (4.0, 0.0));
    }
}
