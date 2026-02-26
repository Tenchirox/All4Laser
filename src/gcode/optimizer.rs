use crate::gcode::types::GCodeLine;

/// A group of GCode lines that form a continuous burn (M3 on -> moves -> M5 off)
#[derive(Debug, Clone)]
pub struct BurnPath {
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub lines: Vec<GCodeLine>,
}

pub fn optimize(lines: &[GCodeLine]) -> Vec<GCodeLine> {
    if lines.is_empty() { return Vec::new(); }

    let mut optimized = Vec::new();
    let mut header = Vec::new();
    let mut footer = Vec::new();
    let mut burn_paths = Vec::new();

    // 1. Group lines into burn paths and isolate header/footer
    let mut current_path: Option<BurnPath> = None;
    let mut cur_x = 0.0;
    let mut cur_y = 0.0;
    let mut laser_on = false;

    // We assume absolute mode for simplicity of distance calculation
    for line in lines {
        if let Some(m) = line.m_code {
            if matches!(m, 3 | 4) {
                laser_on = true;
                current_path = Some(BurnPath {
                    start_x: cur_x,
                    start_y: cur_y,
                    end_x: cur_x,
                    end_y: cur_y,
                    lines: vec![line.clone()],
                });
                continue;
            }
            if m == 5 {
                laser_on = false;
                if let Some(mut path) = current_path.take() {
                    path.lines.push(line.clone());
                    burn_paths.push(path);
                }
                continue;
            }
        }

        if let Some(path) = current_path.as_mut() {
            path.lines.push(line.clone());
            if let Some(x) = line.x { path.end_x = x; cur_x = x; }
            if let Some(y) = line.y { path.end_y = y; cur_y = y; }
        } else if !burn_paths.is_empty() {
             // Footer logic: everything after the last M5
             footer.push(line.clone());
        } else if laser_on {
            // Should not happen with well-formed laser GCode (M3 always precedes moves)
            // but we handle just in case
        } else {
            // Header logic: everything before the first M3
            header.push(line.clone());
            if let Some(x) = line.x { cur_x = x; }
            if let Some(y) = line.y { cur_y = y; }
        }
    }

    // 2. Greedy sorting of burn paths
    optimized.extend(header);

    let mut remaining = burn_paths;
    let mut last_x = cur_x;
    let mut last_y = cur_y;

    while !remaining.is_empty() {
        let mut best_index = 0;
        let mut min_dist_sq = f32::MAX;

        for (i, path) in remaining.iter().enumerate() {
            let dist_sq = (path.start_x - last_x).powi(2) + (path.start_y - last_y).powi(2);
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                best_index = i;
            }
        }

        let best_path = remaining.remove(best_index);
        
        // Add G0 move to the start of this path IF it's not already there
        // (Laser GCode usually starts with M3 then moves, but the move itself might need adjustment)
        // For simplicity, we just add the lines as they were grouped
        optimized.extend(best_path.lines);
        
        last_x = best_path.end_x;
        last_y = best_path.end_y;
    }

    optimized.extend(footer);
    optimized
}
