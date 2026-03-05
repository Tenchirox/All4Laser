use crate::gcode::types::GCodeLine;

/// A group of GCode lines that form a continuous burn (M3 on -> moves -> M5 off)
#[derive(Debug, Clone)]
pub struct BurnPath {
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub lines: Vec<GCodeLine>,
    pub nesting_level: usize,
}

pub fn optimize(lines: &[GCodeLine]) -> Vec<GCodeLine> {
    if lines.is_empty() {
        return Vec::new();
    }

    let mut optimized = Vec::new();
    let mut header = Vec::new();
    let mut footer = Vec::new();
    let mut burn_paths = Vec::new();

    // 1. Group lines into burn paths and isolate header/footer
    let mut current_path: Option<BurnPath> = None;
    let mut cur_x = 0.0;
    let mut cur_y = 0.0;

    // We assume absolute mode for simplicity of distance calculation
    for line in lines {
        if let Some(m) = line.m_code {
            if matches!(m, 3 | 4) {
                current_path = Some(BurnPath {
                    start_x: cur_x,
                    start_y: cur_y,
                    end_x: cur_x,
                    end_y: cur_y,
                    min_x: cur_x,
                    min_y: cur_y,
                    max_x: cur_x,
                    max_y: cur_y,
                    lines: vec![line.clone()],
                    nesting_level: 0,
                });
                continue;
            }
            if m == 5 {
                if let Some(mut path) = current_path.take() {
                    path.lines.push(line.clone());
                    burn_paths.push(path);
                }
                continue;
            }
        }

        if let Some(path) = current_path.as_mut() {
            path.lines.push(line.clone());
            if let Some(x) = line.x {
                path.end_x = x;
                cur_x = x;
                path.min_x = path.min_x.min(x);
                path.max_x = path.max_x.max(x);
            }
            if let Some(y) = line.y {
                path.end_y = y;
                cur_y = y;
                path.min_y = path.min_y.min(y);
                path.max_y = path.max_y.max(y);
            }
        } else if !burn_paths.is_empty() {
            // Footer logic: everything after the last M5
            footer.push(line.clone());
        } else {
            // Header logic: everything before the first M3
            header.push(line.clone());
            if let Some(x) = line.x {
                cur_x = x;
            }
            if let Some(y) = line.y {
                cur_y = y;
            }
        }
    }

    // 2. Calculate nesting levels (Bounding Box Inclusion)
    for i in 0..burn_paths.len() {
        let mut level = 0;
        for j in 0..burn_paths.len() {
            if i == j {
                continue;
            }
            let a = &burn_paths[i];
            let b = &burn_paths[j];
            // If A is inside B
            if a.min_x >= b.min_x && a.max_x <= b.max_x && a.min_y >= b.min_y && a.max_y <= b.max_y
            {
                level += 1;
            }
        }
        burn_paths[i].nesting_level = level;
    }

    // 3. Greedy sorting with nesting priority
    optimized.extend(header);

    // Group remaining paths by nesting level using BTreeMap of Vecs
    let mut paths_by_level: std::collections::BTreeMap<usize, Vec<BurnPath>> = std::collections::BTreeMap::new();
    for path in burn_paths {
        paths_by_level.entry(path.nesting_level).or_default().push(path);
    }

    let mut last_x = cur_x;
    let mut last_y = cur_y;

    while !remaining.is_empty() {
        // Find max nesting level remaining
        let max_nesting = remaining.iter().map(|p| p.nesting_level).max().unwrap_or(0);

        let mut best_index = None;
        let mut min_dist_sq = f32::MAX;
    while let Some((_, mut level_paths)) = paths_by_level.pop_last() {
        while !level_paths.is_empty() {
            let mut best_index = 0;
            let mut min_dist_sq = f32::MAX;

            for (i, path) in level_paths.iter().enumerate() {
                let dist_sq = (path.start_x - last_x).powi(2) + (path.start_y - last_y).powi(2);
                if dist_sq < min_dist_sq {
                    min_dist_sq = dist_sq;
                    best_index = i;
                }
            }

            // Remove the nearest path and process it
            // swap_remove is O(1) compared to remove which is O(N).
            let best_path = level_paths.swap_remove(best_index);
            last_x = best_path.end_x;
            last_y = best_path.end_y;
            optimized.extend(best_path.lines);
        }
    }

    optimized.extend(footer);
    optimized
}
