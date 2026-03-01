/// Scanline Fill Generator
/// Generates a raster-style hatch fill for closed shapes (Rectangle, Circle, etc.)

use crate::ui::drawing::{ShapeParams, ShapeKind};
use crate::ui::layers_new::CutLayer;
use crate::gcode::generator::GCodeBuilder;

pub fn generate_fill(lines: &mut Vec<String>, shape: &ShapeParams, layer: &CutLayer) {
    let interval_mm = layer.fill_interval_mm.max(0.01);
    let overscan_mm = layer.fill_overscan_mm.max(0.0);
    let min_power = layer.min_power.clamp(0.0, layer.power);

    let mut builder = GCodeBuilder::new();

    let bounds = match shape.shape {
        ShapeKind::Rectangle => (shape.x, shape.y, shape.x + shape.width, shape.y + shape.height),
        ShapeKind::Circle => (shape.x - shape.radius, shape.y - shape.radius, shape.x + shape.radius, shape.y + shape.radius),
        _ => return, // Text/Path filling is complex (needs polygon logic), skipping for now
    };

    let (min_x, min_y, max_x, max_y) = bounds;
    let mut y = min_y;
    let mut left_to_right = true;

    builder.comment(&format!("Fill Scan (Layer C{:02})", layer.id));
    builder.laser_off();

    while y <= max_y + 0.0001 {
        let span = match shape.shape {
            ShapeKind::Rectangle => (min_x, max_x),
            ShapeKind::Circle => {
                // Circle equation: (x-cx)^2 + (y-cy)^2 = r^2
                // x = cx +/- sqrt(r^2 - (y-cy)^2)
                let cy = shape.y;
                let dy = y - cy;
                if dy.abs() > shape.radius {
                    y += interval_mm;
                    if layer.fill_bidirectional {
                        left_to_right = !left_to_right;
                    }
                    continue;
                }
                let dx = (shape.radius.powi(2) - dy.powi(2)).sqrt();
                (shape.x - dx, shape.x + dx)
            }
            _ => (min_x, max_x),
        };

        let (x_start, x_end) = span;
        let line_left_to_right = if layer.fill_bidirectional { left_to_right } else { true };

        let (entry_x, body_start, body_end, exit_x) = if line_left_to_right {
            (
                x_start - overscan_mm,
                x_start,
                x_end,
                x_end + overscan_mm,
            )
        } else {
            (
                x_end + overscan_mm,
                x_end,
                x_start,
                x_start - overscan_mm,
            )
        };

        // Entry: rapid to overscan edge, then optional low-power pre-fire into shape
        builder.rapid(entry_x, y);
        if overscan_mm > 0.0 {
            if min_power > 0.0 {
                builder.linear(body_start, y, layer.speed, min_power);
            } else {
                builder.rapid(body_start, y);
            }
        }

        // Main scan stroke at target power
        builder.linear(body_end, y, layer.speed, layer.power);

        // Exit overscan tail
        if overscan_mm > 0.0 {
            if min_power > 0.0 {
                builder.linear(exit_x, y, layer.speed, min_power);
            } else {
                builder.rapid(exit_x, y);
            }
        }

        y += interval_mm;
        if layer.fill_bidirectional {
            left_to_right = !left_to_right;
        }
    }

    builder.laser_off();
    lines.extend(builder.finish());
}
