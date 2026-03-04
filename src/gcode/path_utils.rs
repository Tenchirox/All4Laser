use crate::gcode::generator::GCodeBuilder;
use crate::ui::layers_new::CutLayer;

/// Compute the turning angle (degrees) at point path[i] between segments (i-1,i) and (i,i+1).
/// Returns 180 for straight lines, 0 for perfect U-turns.
fn turning_angle_deg(path: &[(f32, f32)], i: usize) -> f32 {
    if i == 0 || i >= path.len() - 1 {
        return 180.0;
    }
    let (ax, ay) = (path[i].0 - path[i - 1].0, path[i].1 - path[i - 1].1);
    let (bx, by) = (path[i + 1].0 - path[i].0, path[i + 1].1 - path[i].1);
    let la = (ax * ax + ay * ay).sqrt();
    let lb = (bx * bx + by * by).sqrt();
    if la < 1e-6 || lb < 1e-6 {
        return 180.0;
    }
    let dot = ax * bx + ay * by;
    let cos_a = (dot / (la * lb)).clamp(-1.0, 1.0);
    cos_a.acos().to_degrees()
}

/// Compute total path length
fn path_total_length(path: &[(f32, f32)]) -> f32 {
    let mut total = 0.0f32;
    for seg in path.windows(2) {
        let dx = seg[1].0 - seg[0].0;
        let dy = seg[1].1 - seg[0].1;
        total += (dx * dx + dy * dy).sqrt();
    }
    total
}

/// Compute cumulative distance up to point i
fn cumulative_distance(path: &[(f32, f32)], i: usize) -> f32 {
    let mut d = 0.0f32;
    for j in 0..i.min(path.len().saturating_sub(1)) {
        let dx = path[j + 1].0 - path[j].0;
        let dy = path[j + 1].1 - path[j].1;
        d += (dx * dx + dy * dy).sqrt();
    }
    d
}

/// Compute ramped power for point i (F12)
fn ramp_adjusted_power(path: &[(f32, f32)], i: usize, layer: &CutLayer) -> f32 {
    if !layer.ramp_enabled || layer.ramp_length_mm <= 0.0 {
        return layer.power;
    }
    let total = path_total_length(path);
    let dist = cumulative_distance(path, i);
    let ramp = layer.ramp_length_mm;
    let min_pwr = layer.power * layer.ramp_start_pct / 100.0;

    // Ramp up at start
    if dist < ramp {
        let t = dist / ramp;
        return min_pwr + (layer.power - min_pwr) * t;
    }
    // Ramp down at end
    let dist_from_end = total - dist;
    if dist_from_end < ramp {
        let t = dist_from_end / ramp;
        return min_pwr + (layer.power - min_pwr) * t;
    }
    layer.power
}

/// Compute power for point i considering corner power reduction (F40).
fn corner_adjusted_power(path: &[(f32, f32)], i: usize, layer: &CutLayer) -> f32 {
    if !layer.corner_power_enabled || i == 0 || i >= path.len() - 1 {
        return layer.power;
    }
    let angle = turning_angle_deg(path, i);
    if angle < layer.corner_angle_threshold {
        // Interpolate: at 0° → corner_power_pct%, at threshold → 100%
        let ratio = (angle / layer.corner_angle_threshold).clamp(0.0, 1.0);
        let pct = layer.corner_power_pct + (100.0 - layer.corner_power_pct) * ratio;
        (layer.power * pct / 100.0).max(0.0)
    } else {
        layer.power
    }
}

fn first_dir(path: &[(f32, f32)]) -> Option<(f32, f32)> {
    for seg in path.windows(2) {
        let dx = seg[1].0 - seg[0].0;
        let dy = seg[1].1 - seg[0].1;
        let len = (dx * dx + dy * dy).sqrt();
        if len > 0.000_1 {
            return Some((dx / len, dy / len));
        }
    }
    None
}

fn last_dir(path: &[(f32, f32)]) -> Option<(f32, f32)> {
    for seg in path.windows(2).rev() {
        let dx = seg[1].0 - seg[0].0;
        let dy = seg[1].1 - seg[0].1;
        let len = (dx * dx + dy * dy).sqrt();
        if len > 0.000_1 {
            return Some((dx / len, dy / len));
        }
    }
    None
}

/// Applies tabs (bridges) to a path by breaking it into segments with gaps.
pub fn apply_tabs(
    builder: &mut GCodeBuilder,
    path: &[(f32, f32)],
    layer: &CutLayer,
) {
    if path.len() < 2 {
        return;
    }

    // Perforation mode (F33): reuse the tab/bridge algorithm with perforation params
    if layer.perforation_enabled && !layer.tab_enabled {
        let cut_len = layer.perforation_cut_mm.max(0.1);
        let gap_len = layer.perforation_gap_mm.max(0.1);

        builder.laser_off();
        builder.rapid(path[0].0, path[0].1);

        let mut dist_in_phase = 0.0f32;
        let mut cutting = true; // start in cut phase

        for i in 0..path.len() - 1 {
            let p1 = path[i];
            let p2 = path[i + 1];
            let dx = p2.0 - p1.0;
            let dy = p2.1 - p1.1;
            let seg_len = (dx * dx + dy * dy).sqrt();
            if seg_len < 0.0001 { continue; }

            let mut covered = 0.0f32;
            while covered < seg_len - 0.0001 {
                let phase_len = if cutting { cut_len } else { gap_len };
                let remaining_phase = phase_len - dist_in_phase;
                let remaining_seg = seg_len - covered;
                let step = remaining_phase.min(remaining_seg);

                let t = ((covered + step) / seg_len).min(1.0);
                let tx = p1.0 + dx * t;
                let ty = p1.1 + dy * t;

                if cutting {
                    builder.linear(tx, ty, layer.speed, layer.power);
                } else {
                    builder.laser_off();
                    builder.rapid(tx, ty);
                }

                covered += step;
                dist_in_phase += step;

                if dist_in_phase >= phase_len - 0.0001 {
                    cutting = !cutting;
                    dist_in_phase = 0.0;
                }
            }
        }
        builder.laser_off();
        return;
    }

    if !layer.tab_enabled {
        let start = path[0];
        let end = path[path.len() - 1];
        let lead_in = layer.lead_in_mm.max(0.0);
        let lead_out = layer.lead_out_mm.max(0.0);

        let start_pos = if lead_in > 0.0 {
            if let Some((ux, uy)) = first_dir(path) {
                (start.0 - ux * lead_in, start.1 - uy * lead_in)
            } else {
                start
            }
        } else {
            start
        };

        let end_pos = if lead_out > 0.0 {
            if let Some((ux, uy)) = last_dir(path) {
                (end.0 + ux * lead_out, end.1 + uy * lead_out)
            } else {
                end
            }
        } else {
            end
        };

        builder.laser_off();
        builder.rapid(start_pos.0, start_pos.1);
        if lead_in > 0.0 {
            builder.linear(start.0, start.1, layer.speed, layer.power);
        }
        for (i, p) in path[1..].iter().enumerate() {
            let idx = i + 1;
            let mut pwr = corner_adjusted_power(path, idx, layer);
            // Apply ramping on top of corner adjustment (F12)
            if layer.ramp_enabled {
                let ramp_pwr = ramp_adjusted_power(path, idx, layer);
                pwr = pwr.min(ramp_pwr);
            }
            builder.linear(p.0, p.1, layer.speed, pwr);
        }
        if lead_out > 0.0 {
            builder.linear(end_pos.0, end_pos.1, layer.speed, layer.power);
        }
        builder.laser_off();
        return;
    }

    let tab_spacing = layer.tab_spacing;
    let tab_size = layer.tab_size;
    let lead_in = layer.lead_in_mm.max(0.0);
    let lead_out = layer.lead_out_mm.max(0.0);
    let mut dist_since_last_tab = 0.0;

    let start = path[0];
    let end = path[path.len() - 1];
    let start_pos = if lead_in > 0.0 {
        if let Some((ux, uy)) = first_dir(path) {
            (start.0 - ux * lead_in, start.1 - uy * lead_in)
        } else {
            start
        }
    } else {
        start
    };
    let end_pos = if lead_out > 0.0 {
        if let Some((ux, uy)) = last_dir(path) {
            (end.0 + ux * lead_out, end.1 + uy * lead_out)
        } else {
            end
        }
    } else {
        end
    };

    builder.laser_off();
    builder.rapid(start_pos.0, start_pos.1);
    if lead_in > 0.0 {
        builder.linear(start.0, start.1, layer.speed, layer.power);
    }

    let mut laser_is_on = false;

    for i in 0..path.len() - 1 {
        let p1 = path[i];
        let p2 = path[i+1];
        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;
        let seg_len = (dx * dx + dy * dy).sqrt();

        if seg_len == 0.0 { continue; }

        let mut covered = 0.0;
        while covered < seg_len {
            let remaining_seg = seg_len - covered;
            
            if !laser_is_on {
                // We are in a "cut" zone
                // How much more can we cut before a tab?
                let can_cut = tab_spacing - dist_since_last_tab;
                let move_dist = can_cut.min(remaining_seg);
                
                let t = (covered + move_dist) / seg_len;
                let tx = p1.0 + dx * t;
                let ty = p1.1 + dy * t;
                
                builder.linear(tx, ty, layer.speed, layer.power);
                covered += move_dist;
                dist_since_last_tab += move_dist;

                if dist_since_last_tab >= tab_spacing {
                    // Start a tab
                    builder.laser_off();
                    laser_is_on = true;
                    dist_since_last_tab = 0.0;
                }
            } else {
                // We are in a "tab" (gap) zone
                // How much more gap before we resume cutting?
                let can_gap = tab_size - dist_since_last_tab;
                let move_dist = can_gap.min(remaining_seg);
                
                let t = (covered + move_dist) / seg_len;
                let tx = p1.0 + dx * t;
                let ty = p1.1 + dy * t;
                
                builder.rapid(tx, ty); // Move without laser
                covered += move_dist;
                dist_since_last_tab += move_dist;

                if dist_since_last_tab >= tab_size {
                    // End tab, resume cutting
                    laser_is_on = false;
                    dist_since_last_tab = 0.0;
                    // Note: GCodeBuilder will handle M3 when next linear() is called
                }
            }
        }
    }

    if lead_out > 0.0 {
        builder.linear(end_pos.0, end_pos.1, layer.speed, layer.power);
    }
    builder.laser_off();
}
