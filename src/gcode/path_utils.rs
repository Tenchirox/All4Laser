use crate::gcode::generator::GCodeBuilder;
use crate::ui::layers_new::CutLayer;

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
        // Just draw as usual
        builder.laser_off();
        builder.rapid(path[0].0, path[0].1);
        for p in &path[1..] {
            builder.linear(p.0, p.1, layer.speed, layer.power);
        }
        builder.laser_off();
        return;
    }

    let tab_spacing = layer.tab_spacing;
    let tab_size = layer.tab_size;
    let mut dist_since_last_tab = 0.0;
    
    builder.laser_off();
    builder.rapid(path[0].0, path[0].1);
    
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
    builder.laser_off();
}
