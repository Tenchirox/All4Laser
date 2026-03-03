/// Scanline Fill Generator
/// Generates a raster-style hatch fill for closed shapes (Rectangle, Circle, etc.)

use crate::ui::drawing::{ShapeParams, ShapeKind};
use crate::ui::layers_new::CutLayer;
use crate::gcode::generator::GCodeBuilder;

pub fn generate_fill(lines: &mut Vec<String>, shape: &ShapeParams, layer: &CutLayer) {
    generate_fill_group(lines, &[shape], layer);
}

pub fn generate_fill_group(lines: &mut Vec<String>, shapes: &[&ShapeParams], layer: &CutLayer) {
    if shapes.is_empty() {
        return;
    }

    let interval_mm = layer.fill_interval_mm.max(0.01);
    let overscan_mm = layer.fill_overscan_mm.max(0.0);
    let min_power = layer.min_power.clamp(0.0, layer.power);
    let angle_rad = layer.fill_angle_deg.to_radians();

    let mut builder = GCodeBuilder::new();
    builder.comment(&format!(
        "Fill Scan (Layer C{:02}, angle {:.1}°)",
        layer.id,
        layer.fill_angle_deg
    ));
    builder.laser_off();

    let paths: Vec<Vec<(f32, f32)>> = shapes
        .iter()
        .filter_map(|shape| shape_world_path(shape))
        .collect();

    if paths.is_empty() {
        return;
    }

    let components = fill_components(&paths);

    for component_paths in components {
        fill_paths_world(
            &mut builder,
            &component_paths,
            layer,
            interval_mm,
            overscan_mm,
            min_power,
            angle_rad,
        );
    }

    builder.laser_off();
    lines.extend(builder.finish());
}

pub fn preview_fill_segments_group(
    shapes: &[&ShapeParams],
    layer: &CutLayer,
    max_segments: usize,
) -> Vec<((f32, f32), (f32, f32))> {
    if shapes.is_empty() || max_segments == 0 {
        return Vec::new();
    }

    let interval_mm = layer.fill_interval_mm.max(0.01);
    let angle_rad = layer.fill_angle_deg.to_radians();

    let paths: Vec<Vec<(f32, f32)>> = shapes
        .iter()
        .filter_map(|shape| shape_world_path(shape))
        .collect();

    if paths.is_empty() {
        return Vec::new();
    }

    let components = fill_components(&paths);
    let mut out: Vec<((f32, f32), (f32, f32))> = Vec::new();

    for component_paths in components {
        if out.len() >= max_segments {
            break;
        }
        let remaining = max_segments - out.len();
        let mut segments = collect_fill_segments_world_angle(
            &component_paths,
            interval_mm,
            angle_rad,
            layer.fill_bidirectional,
            remaining,
        );
        out.append(&mut segments);
    }

    out
}

fn fill_components(paths: &[Vec<(f32, f32)>]) -> Vec<Vec<Vec<(f32, f32)>>> {
    if paths.is_empty() {
        return Vec::new();
    }
    if paths.len() == 1 {
        return vec![vec![paths[0].clone()]];
    }

    let mut parent: Vec<Option<usize>> = vec![None; paths.len()];
    let areas: Vec<f32> = paths.iter().map(|p| polygon_signed_area(p).abs()).collect();

    for i in 0..paths.len() {
        let Some(sample) = representative_point(&paths[i]) else {
            continue;
        };

        let mut best_parent: Option<usize> = None;
        let mut best_parent_area = f32::INFINITY;

        for j in 0..paths.len() {
            if i == j {
                continue;
            }

            if areas[j] <= areas[i] + 0.0001 {
                continue;
            }

            if point_in_polygon(sample, &paths[j]) && areas[j] < best_parent_area {
                best_parent = Some(j);
                best_parent_area = areas[j];
            }
        }

        parent[i] = best_parent;
    }

    let mut children: Vec<Vec<usize>> = vec![Vec::new(); paths.len()];
    let mut roots: Vec<usize> = Vec::new();

    for (idx, maybe_parent) in parent.iter().enumerate() {
        if let Some(parent_idx) = maybe_parent {
            children[*parent_idx].push(idx);
        } else {
            roots.push(idx);
        }
    }

    fn collect_subtree(idx: usize, children: &[Vec<usize>], out: &mut Vec<usize>) {
        out.push(idx);
        for &child in &children[idx] {
            collect_subtree(child, children, out);
        }
    }

    let mut components: Vec<Vec<Vec<(f32, f32)>>> = Vec::new();
    for root in roots {
        let mut indices = Vec::new();
        collect_subtree(root, &children, &mut indices);
        let component_paths: Vec<Vec<(f32, f32)>> = indices.into_iter().map(|i| paths[i].clone()).collect();
        if !component_paths.is_empty() {
            components.push(component_paths);
        }
    }

    components
}

fn polygon_signed_area(path: &[(f32, f32)]) -> f32 {
    if path.len() < 3 {
        return 0.0;
    }

    let mut acc = 0.0f32;
    for edge in path.windows(2) {
        let (x1, y1) = edge[0];
        let (x2, y2) = edge[1];
        acc += x1 * y2 - x2 * y1;
    }

    acc * 0.5
}

fn representative_point(path: &[(f32, f32)]) -> Option<(f32, f32)> {
    if path.len() < 4 {
        return None;
    }

    let mut sx = 0.0f32;
    let mut sy = 0.0f32;
    let mut n = 0usize;

    for &(x, y) in &path[..path.len() - 1] {
        sx += x;
        sy += y;
        n += 1;
    }

    if n == 0 {
        None
    } else {
        Some((sx / n as f32, sy / n as f32))
    }
}

fn point_in_polygon(point: (f32, f32), path: &[(f32, f32)]) -> bool {
    let (px, py) = point;
    let mut inside = false;

    for edge in path.windows(2) {
        let (x1, y1) = edge[0];
        let (x2, y2) = edge[1];
        let intersects = ((y1 > py) != (y2 > py))
            && (px < (x2 - x1) * (py - y1) / (y2 - y1 + 1e-12) + x1);
        if intersects {
            inside = !inside;
        }
    }

    inside
}

fn shape_world_path(shape: &ShapeParams) -> Option<Vec<(f32, f32)>> {
    match &shape.shape {
        ShapeKind::Rectangle => Some(rectangle_world_path(shape)),
        ShapeKind::Circle => Some(circle_world_path(shape, 96)),
        ShapeKind::Path(points) => closed_world_path(points, shape),
        _ => None,
    }
}

fn fill_paths_world(
    builder: &mut GCodeBuilder,
    paths: &[Vec<(f32, f32)>],
    layer: &CutLayer,
    interval_mm: f32,
    overscan_mm: f32,
    min_power: f32,
    angle_rad: f32,
) {
    fill_paths_world_angle(builder, paths, layer, interval_mm, overscan_mm, min_power, angle_rad);
}

fn fill_paths_world_angle(
    builder: &mut GCodeBuilder,
    paths: &[Vec<(f32, f32)>],
    layer: &CutLayer,
    interval_mm: f32,
    overscan_mm: f32,
    min_power: f32,
    angle_rad: f32,
) {
    if paths.is_empty() {
        return;
    }

    let Some((min_x, min_y, max_x, max_y)) = paths_bounds(paths) else {
        return;
    };
    let center = ((min_x + max_x) * 0.5, (min_y + max_y) * 0.5);

    let rotated_paths: Vec<Vec<(f32, f32)>> = paths
        .iter()
        .map(|path| {
            path
                .iter()
                .map(|(x, y)| rotate_around(*x, *y, center.0, center.1, -angle_rad))
                .collect()
        })
        .collect();

    let Some((_scan_min_x, _scan_min_y, _scan_max_x, _scan_max_y)) = paths_bounds(&rotated_paths) else {
        return;
    };
    let segments = collect_fill_segments_rotated(
        &rotated_paths,
        center,
        angle_rad,
        interval_mm,
        layer.fill_bidirectional,
        usize::MAX,
    );

    for (start, end) in segments {
        emit_scan_segment(builder, start, end, layer, overscan_mm, min_power);
    }
}

fn collect_fill_segments_world_angle(
    paths: &[Vec<(f32, f32)>],
    interval_mm: f32,
    angle_rad: f32,
    bidirectional: bool,
    max_segments: usize,
) -> Vec<((f32, f32), (f32, f32))> {
    if paths.is_empty() || max_segments == 0 {
        return Vec::new();
    }

    let Some((min_x, min_y, max_x, max_y)) = paths_bounds(paths) else {
        return Vec::new();
    };
    let center = ((min_x + max_x) * 0.5, (min_y + max_y) * 0.5);

    let rotated_paths: Vec<Vec<(f32, f32)>> = paths
        .iter()
        .map(|path| {
            path
                .iter()
                .map(|(x, y)| rotate_around(*x, *y, center.0, center.1, -angle_rad))
                .collect()
        })
        .collect();

    collect_fill_segments_rotated(
        &rotated_paths,
        center,
        angle_rad,
        interval_mm,
        bidirectional,
        max_segments,
    )
}

fn collect_fill_segments_rotated(
    rotated_paths: &[Vec<(f32, f32)>],
    center: (f32, f32),
    angle_rad: f32,
    interval_mm: f32,
    bidirectional: bool,
    max_segments: usize,
) -> Vec<((f32, f32), (f32, f32))> {
    let Some((_, scan_min_y, _, scan_max_y)) = paths_bounds(rotated_paths) else {
        return Vec::new();
    };

    let mut segments = Vec::new();
    let mut y = scan_min_y;
    let mut left_to_right = true;

    while y <= scan_max_y + 0.0001 {
        if segments.len() >= max_segments {
            break;
        }

        let mut spans = multi_polygon_scanline_spans(rotated_paths, y);
        if !spans.is_empty() {
            let line_left_to_right = if bidirectional { left_to_right } else { true };
            if !line_left_to_right {
                spans.reverse();
            }

            for (x_start, x_end) in spans {
                if segments.len() >= max_segments {
                    break;
                }

                let (sx, ex) = if line_left_to_right {
                    (x_start, x_end)
                } else {
                    (x_end, x_start)
                };
                let start = rotate_around(sx, y, center.0, center.1, angle_rad);
                let end = rotate_around(ex, y, center.0, center.1, angle_rad);
                segments.push((start, end));
            }
        }

        y += interval_mm;
        if bidirectional {
            left_to_right = !left_to_right;
        }
    }

    segments
}

fn emit_scan_segment(
    builder: &mut GCodeBuilder,
    start: (f32, f32),
    end: (f32, f32),
    layer: &CutLayer,
    overscan_mm: f32,
    min_power: f32,
) {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.0001 {
        return;
    }

    let ux = dx / len;
    let uy = dy / len;

    let entry_x = start.0 - ux * overscan_mm;
    let entry_y = start.1 - uy * overscan_mm;
    let exit_x = end.0 + ux * overscan_mm;
    let exit_y = end.1 + uy * overscan_mm;

    builder.rapid(entry_x, entry_y);
    if overscan_mm > 0.0 {
        if min_power > 0.0 {
            builder.linear(start.0, start.1, layer.speed, min_power);
        } else {
            builder.rapid(start.0, start.1);
        }
    }

    builder.linear(end.0, end.1, layer.speed, layer.power);

    if overscan_mm > 0.0 {
        if min_power > 0.0 {
            builder.linear(exit_x, exit_y, layer.speed, min_power);
        } else {
            builder.rapid(exit_x, exit_y);
        }
    }
}

fn rectangle_world_path(shape: &ShapeParams) -> Vec<(f32, f32)> {
    let pts = [
        (0.0, 0.0),
        (shape.width, 0.0),
        (shape.width, shape.height),
        (0.0, shape.height),
        (0.0, 0.0),
    ];
    pts.into_iter()
        .map(|(x, y)| rotate_point(x, y, shape))
        .collect()
}

fn circle_world_path(shape: &ShapeParams, steps: usize) -> Vec<(f32, f32)> {
    let mut out = Vec::with_capacity(steps + 1);
    for i in 0..=steps {
        let t = (i as f32) / (steps as f32);
        let a = std::f32::consts::TAU * t;
        out.push((shape.x + shape.radius * a.cos(), shape.y + shape.radius * a.sin()));
    }
    out
}

fn closed_world_path(points: &[(f32, f32)], shape: &ShapeParams) -> Option<Vec<(f32, f32)>> {
    if points.len() < 3 {
        return None;
    }

    let mut out: Vec<(f32, f32)> = points.iter().map(|(x, y)| rotate_point(*x, *y, shape)).collect();
    if let (Some(first), Some(last)) = (out.first().copied(), out.last().copied()) {
        let dx = first.0 - last.0;
        let dy = first.1 - last.1;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > 0.05 {
            return None;
        }
        if dist > 0.0001 {
            out.push(first);
        }
    }

    if out.len() < 4 {
        return None;
    }

    Some(out)
}

fn rotate_point(lx: f32, ly: f32, s: &ShapeParams) -> (f32, f32) {
    let angle = s.rotation.to_radians();
    let rx = lx * angle.cos() - ly * angle.sin();
    let ry = lx * angle.sin() + ly * angle.cos();
    (s.x + rx, s.y + ry)
}

fn rotate_around(x: f32, y: f32, cx: f32, cy: f32, angle: f32) -> (f32, f32) {
    let tx = x - cx;
    let ty = y - cy;
    let rx = tx * angle.cos() - ty * angle.sin();
    let ry = tx * angle.sin() + ty * angle.cos();
    (rx + cx, ry + cy)
}

fn path_bounds(path: &[(f32, f32)]) -> (f32, f32, f32, f32) {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for (x, y) in path {
        min_x = min_x.min(*x);
        min_y = min_y.min(*y);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }

    (min_x, min_y, max_x, max_y)
}

fn paths_bounds(paths: &[Vec<(f32, f32)>]) -> Option<(f32, f32, f32, f32)> {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut any = false;

    for path in paths {
        if path.len() < 2 {
            continue;
        }
        let (px0, py0, px1, py1) = path_bounds(path);
        min_x = min_x.min(px0);
        min_y = min_y.min(py0);
        max_x = max_x.max(px1);
        max_y = max_y.max(py1);
        any = true;
    }

    if any {
        Some((min_x, min_y, max_x, max_y))
    } else {
        None
    }
}

fn polygon_scanline_intersections(path: &[(f32, f32)], y: f32, intersections: &mut Vec<f32>) {
    for edge in path.windows(2) {
        let (x1, y1) = edge[0];
        let (x2, y2) = edge[1];

        if (y1 <= y && y2 > y) || (y2 <= y && y1 > y) {
            let t = (y - y1) / (y2 - y1);
            let x = x1 + t * (x2 - x1);
            intersections.push(x);
        }
    }
}

fn spans_from_intersections(mut intersections: Vec<f32>) -> Vec<(f32, f32)> {
    intersections.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let mut spans = Vec::new();
    for pair in intersections.chunks(2) {
        if pair.len() == 2 {
            let x_start = pair[0].min(pair[1]);
            let x_end = pair[0].max(pair[1]);
            if x_end - x_start > 0.0001 {
                spans.push((x_start, x_end));
            }
        }
    }

    spans
}

fn multi_polygon_scanline_spans(paths: &[Vec<(f32, f32)>], y: f32) -> Vec<(f32, f32)> {
    let mut intersections: Vec<f32> = Vec::new();
    for path in paths {
        polygon_scanline_intersections(path, y, &mut intersections);
    }
    spans_from_intersections(intersections)
}

fn polygon_scanline_spans(path: &[(f32, f32)], y: f32) -> Vec<(f32, f32)> {
    let mut intersections: Vec<f32> = Vec::new();
    polygon_scanline_intersections(path, y, &mut intersections);
    spans_from_intersections(intersections)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::drawing::{ShapeKind, ShapeParams};
    use crate::ui::layers_new::CutLayer;

    #[test]
    fn polygon_scanline_spans_square_has_single_span() {
        let square = vec![
            (0.0, 0.0),
            (10.0, 0.0),
            (10.0, 10.0),
            (0.0, 10.0),
            (0.0, 0.0),
        ];

        let spans = polygon_scanline_spans(&square, 5.0);
        assert_eq!(spans.len(), 1);
        assert!((spans[0].0 - 0.0).abs() < 0.001);
        assert!((spans[0].1 - 10.0).abs() < 0.001);
    }

    #[test]
    fn multi_polygon_scanline_spans_respects_inner_hole() {
        let outer = vec![
            (0.0, 0.0),
            (10.0, 0.0),
            (10.0, 10.0),
            (0.0, 10.0),
            (0.0, 0.0),
        ];
        let inner = vec![
            (3.0, 3.0),
            (7.0, 3.0),
            (7.0, 7.0),
            (3.0, 7.0),
            (3.0, 3.0),
        ];

        let spans = multi_polygon_scanline_spans(&[outer, inner], 5.0);
        assert_eq!(spans.len(), 2);
        assert!((spans[0].0 - 0.0).abs() < 0.001);
        assert!((spans[0].1 - 3.0).abs() < 0.001);
        assert!((spans[1].0 - 7.0).abs() < 0.001);
        assert!((spans[1].1 - 10.0).abs() < 0.001);
    }

    #[test]
    fn fill_components_keeps_overlapping_outers_separate() {
        let a = vec![
            (0.0, 0.0),
            (10.0, 0.0),
            (10.0, 10.0),
            (0.0, 10.0),
            (0.0, 0.0),
        ];
        let b = vec![
            (6.0, 0.0),
            (16.0, 0.0),
            (16.0, 10.0),
            (6.0, 10.0),
            (6.0, 0.0),
        ];

        let components = fill_components(&[a, b]);
        assert_eq!(components.len(), 2, "overlapping independent outers must not be XOR-grouped");
    }

    #[test]
    fn preview_fill_segments_group_returns_segments() {
        let shape = ShapeParams {
            shape: ShapeKind::Rectangle,
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
            ..Default::default()
        };
        let mut layer = CutLayer::default_palette()[0].clone();
        layer.fill_interval_mm = 2.0;

        let segments = preview_fill_segments_group(&[&shape], &layer, 128);
        assert!(!segments.is_empty());
    }

    #[test]
    fn generate_fill_supports_closed_path() {
        let shape = ShapeParams {
            shape: ShapeKind::Path(vec![
                (0.0, 0.0),
                (10.0, 0.0),
                (10.0, 10.0),
                (0.0, 10.0),
                (0.0, 0.0),
            ]),
            ..Default::default()
        };
        let mut layer = CutLayer::default_palette()[0].clone();
        layer.fill_interval_mm = 2.0;
        layer.fill_overscan_mm = 0.0;

        let mut lines = Vec::new();
        generate_fill(&mut lines, &shape, &layer);

        assert!(lines.iter().any(|l| l.starts_with("; Fill Scan")));
        assert!(lines.iter().any(|l| l.starts_with("G1 X")));
    }

    fn extract_xy(line: &str) -> Option<(f32, f32)> {
        if !line.starts_with("G1") {
            return None;
        }
        let mut x = None;
        let mut y = None;
        for part in line.split_whitespace() {
            if let Some(v) = part.strip_prefix('X') {
                x = v.parse::<f32>().ok();
            }
            if let Some(v) = part.strip_prefix('Y') {
                y = v.parse::<f32>().ok();
            }
        }
        match (x, y) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        }
    }

    #[test]
    fn angled_fill_produces_diagonal_segments() {
        let shape = ShapeParams {
            shape: ShapeKind::Rectangle,
            x: 0.0,
            y: 0.0,
            width: 20.0,
            height: 10.0,
            ..Default::default()
        };
        let mut layer = CutLayer::default_palette()[0].clone();
        layer.fill_interval_mm = 1.0;
        layer.fill_overscan_mm = 0.0;
        layer.fill_bidirectional = false;
        layer.fill_angle_deg = 45.0;

        let mut lines = Vec::new();
        generate_fill(&mut lines, &shape, &layer);

        let mut points: Vec<(f32, f32)> = Vec::new();
        for line in &lines {
            if let Some(pt) = extract_xy(line) {
                points.push(pt);
            }
        }
        assert!(points.len() >= 2, "expected at least two G1 points");

        let mut has_diagonal = false;
        for pair in points.windows(2) {
            let dx = (pair[1].0 - pair[0].0).abs();
            let dy = (pair[1].1 - pair[0].1).abs();
            if dx > 0.1 && dy > 0.1 {
                has_diagonal = true;
                break;
            }
        }

        assert!(has_diagonal, "expected angled fill to emit diagonal scan segments");
    }
}
