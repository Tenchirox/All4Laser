/// Scanline Fill Generator
/// Generates a raster-style hatch fill for closed shapes (Rectangle, Circle, etc.)

use crate::ui::drawing::{ShapeParams, ShapeKind};
use crate::ui::layers_new::CutLayer;
use crate::gcode::generator::GCodeBuilder;

pub fn generate_fill(lines: &mut Vec<String>, shape: &ShapeParams, layer: &CutLayer, interval_mm: f32) {
    if interval_mm <= 0.001 { return; }

    let mut builder = GCodeBuilder::new();
    // Inherit existing lines if we wanted to reuse builder, but here we append later.

    let bounds = match shape.shape {
        ShapeKind::Rectangle => (shape.x, shape.y, shape.x + shape.width, shape.y + shape.height),
        ShapeKind::Circle => (shape.x - shape.radius, shape.y - shape.radius, shape.x + shape.radius, shape.y + shape.radius),
        ShapeKind::TextLine => return, // Text filling is complex (needs font outline poly), skipping for now
    };

    let (min_x, min_y, max_x, max_y) = bounds;
    let mut y = min_y;
    let mut left_to_right = true;

    builder.comment(&format!("Fill Scan (Layer C{:02})", layer.id));
    builder.laser_off();

    // Move to start
    builder.rapid(if left_to_right { min_x } else { max_x }, y);

    while y <= max_y {
        let (x_start, x_end) = match shape.shape {
            ShapeKind::Rectangle => (min_x, max_x),
            ShapeKind::Circle => {
                // Circle equation: (x-cx)^2 + (y-cy)^2 = r^2
                // x = cx +/- sqrt(r^2 - (y-cy)^2)
                let cy = shape.y;
                let dy = y - cy;
                if dy.abs() > shape.radius {
                    y += interval_mm;
                    continue;
                }
                let dx = (shape.radius.powi(2) - dy.powi(2)).sqrt();
                (shape.x - dx, shape.x + dx)
            }
            _ => (min_x, max_x),
        };

        if left_to_right {
            // We are at x_start (or close to it)
            // If the last move ended exactly where we start, the generator handles it.
            // But if we stepped down, we are already at the correct Y, possibly slightly offset X.
            // Rapid to start if distance is large? For scan, we usually assume connected path.
            // However, circle edges change X every line.

            // Standard scan: rapid to start of line, burn to end.
            // Bi-directional scan connects ends.

            // To ensure we start exactly at x_start:
            // But wait, previous line ended at prev_x_end. We stepped Y down.
            // We need to move from (prev_x_end, y) to (x_start, y) if not same.

            // Actually, the generator's `linear` will do G1. If we want G0 for the step-over if it's large (whitespace), we should check.
            // For simple shapes, step over is small (interval).
            // We rely on G1 F(Speed) for step over to keep motion smooth (no stop/start accel penalty of M5/G0 if possible).
            // But we must turn laser OFF for step over if we are outside the shape.
            // The step-over logic is below. Here is the burn stroke:

            builder.linear(x_end, y, layer.speed, layer.power);
        } else {
            builder.linear(x_start, y, layer.speed, layer.power);
        }

        // Turnaround / Step over
        y += interval_mm;
        if y <= max_y {
            // Turn off laser for the step down
            builder.laser_off();
            // Note: LightBurn uses "Fast Whitespace" scan which uses G0.
            // Simple scan uses G1 for smoothness.
            // Let's use rapid (G0) which automatically does M5 if needed via our builder.

            let next_x_start = if !left_to_right {
                // Next line is Left->Right, so start at Left X
                match shape.shape {
                    ShapeKind::Rectangle => min_x,
                    ShapeKind::Circle => {
                        let cy = shape.y;
                        let dy = y - cy;
                        if dy.abs() > shape.radius { shape.x } else {
                             let dx = (shape.radius.powi(2) - dy.powi(2)).sqrt();
                             shape.x - dx
                        }
                    },
                    _ => min_x
                }
            } else {
                // Next line is Right->Left, start at Right X
                 match shape.shape {
                    ShapeKind::Rectangle => max_x,
                    ShapeKind::Circle => {
                        let cy = shape.y;
                        let dy = y - cy;
                        if dy.abs() > shape.radius { shape.x } else {
                             let dx = (shape.radius.powi(2) - dy.powi(2)).sqrt();
                             shape.x + dx
                        }
                    },
                    _ => max_x
                }
            };

            // Use Rapid G0 for step over.
            builder.rapid(next_x_start, y);
        }

        left_to_right = !left_to_right;
    }

    builder.laser_off();
    lines.extend(builder.finish());
}
