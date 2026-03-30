/// Camera marker detection logic (pure functions, no app state dependency)
/// Extracted from app.rs for maintainability.

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct MarkerComponent {
    pub center_x: f32,
    pub center_y: f32,
    pub fill_ratio: f32,
    pub area: usize,
    pub aspect_ratio: f32,
}

#[allow(dead_code)]
pub fn detect_marker_components(rgba: &[u8], width: usize, height: usize) -> Vec<MarkerComponent> {
    if width == 0 || height == 0 || rgba.len() < width.saturating_mul(height).saturating_mul(4) {
        return Vec::new();
    }

    let mut dark = vec![false; width * height];
    for y in 0..height {
        for x in 0..width {
            let i = (y * width + x) * 4;
            let lum = (rgba[i] as u16 + rgba[i + 1] as u16 + rgba[i + 2] as u16) / 3;
            dark[y * width + x] = lum < 80;
        }
    }

    let mut visited = vec![false; width * height];
    let mut out = Vec::new();
    let min_area = ((width * height) as f32 * 0.00025).max(16.0) as usize;

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            if !dark[idx] || visited[idx] {
                continue;
            }

            let mut stack = vec![idx];
            visited[idx] = true;
            let mut area = 0usize;
            let mut sum_x = 0usize;
            let mut sum_y = 0usize;
            let mut min_x = x;
            let mut max_x = x;
            let mut min_y = y;
            let mut max_y = y;

            while let Some(cur) = stack.pop() {
                let cx = cur % width;
                let cy = cur / width;
                area += 1;
                sum_x += cx;
                sum_y += cy;
                min_x = min_x.min(cx);
                max_x = max_x.max(cx);
                min_y = min_y.min(cy);
                max_y = max_y.max(cy);

                if cx > 0 {
                    let n = cur - 1;
                    if dark[n] && !visited[n] {
                        visited[n] = true;
                        stack.push(n);
                    }
                }
                if cx + 1 < width {
                    let n = cur + 1;
                    if dark[n] && !visited[n] {
                        visited[n] = true;
                        stack.push(n);
                    }
                }
                if cy > 0 {
                    let n = cur - width;
                    if dark[n] && !visited[n] {
                        visited[n] = true;
                        stack.push(n);
                    }
                }
                if cy + 1 < height {
                    let n = cur + width;
                    if dark[n] && !visited[n] {
                        visited[n] = true;
                        stack.push(n);
                    }
                }
            }

            if area < min_area {
                continue;
            }

            let bw = (max_x - min_x + 1) as f32;
            let bh = (max_y - min_y + 1) as f32;
            let bbox_area = (bw * bh).max(1.0);
            let fill_ratio = area as f32 / bbox_area;
            let aspect_ratio = if bh > 0.0 { bw / bh } else { 0.0 };
            let center_x = sum_x as f32 / area as f32;
            let center_y = sum_y as f32 / area as f32;

            out.push(MarkerComponent {
                center_x,
                center_y,
                fill_ratio,
                area,
                aspect_ratio,
            });
        }
    }

    out
}

#[allow(dead_code)]
pub fn detect_cross_and_circle_markers(
    rgba: &[u8],
    width: usize,
    height: usize,
) -> Option<((f32, f32), (f32, f32))> {
    let mut comps = detect_marker_components(rgba, width, height)
        .into_iter()
        .filter(|c| c.aspect_ratio > 0.55 && c.aspect_ratio < 1.8)
        .collect::<Vec<_>>();

    if comps.len() < 2 {
        return None;
    }

    comps.sort_by(|a, b| {
        b.area.cmp(&a.area).then_with(|| {
            a.fill_ratio
                .partial_cmp(&b.fill_ratio)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    });

    let candidates = comps.into_iter().take(12).collect::<Vec<_>>();
    let mut best_pair: Option<(MarkerComponent, MarkerComponent, f32)> = None;
    for i in 0..candidates.len() {
        for j in 0..candidates.len() {
            if i == j {
                continue;
            }
            let cross = candidates[i];
            let circle = candidates[j];
            let fill_gap = circle.fill_ratio - cross.fill_ratio;
            if fill_gap < 0.08 {
                continue;
            }
            if cross.fill_ratio > 0.70 || circle.fill_ratio < 0.30 {
                continue;
            }
            let score = fill_gap * 100.0 + (cross.area.min(circle.area) as f32).sqrt();
            match best_pair {
                Some((_, _, best)) if score <= best => {}
                _ => best_pair = Some((cross, circle, score)),
            }
        }
    }

    let (cross, circle, _) = best_pair?;
    Some((
        (cross.center_x, cross.center_y),
        (circle.center_x, circle.center_y),
    ))
}
