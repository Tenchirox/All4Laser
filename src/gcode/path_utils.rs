use crate::gcode::generator::GCodeBuilder;
use crate::ui::layers_new::CutLayer;

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
        for p in &path[1..] {
            builder.linear(p.0, p.1, layer.speed, layer.power);
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
