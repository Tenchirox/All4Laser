/// Scanline Fill Generator
/// Generates a raster-style hatch fill for closed shapes (Rectangle, Circle, etc.)

use crate::ui::drawing::{ShapeParams, ShapeKind};
use crate::ui::layers_new::CutLayer;

pub fn generate_fill(lines: &mut Vec<String>, shape: &ShapeParams, layer: &CutLayer, interval_mm: f32) {
    if interval_mm <= 0.001 { return; }

    let bounds = match shape.shape {
        ShapeKind::Rectangle => (shape.x, shape.y, shape.x + shape.width, shape.y + shape.height),
        ShapeKind::Circle => (shape.x - shape.radius, shape.y - shape.radius, shape.x + shape.radius, shape.y + shape.radius),
        ShapeKind::TextLine => return, // Text filling is complex (needs font outline poly), skipping for now
    };

    let (min_x, min_y, max_x, max_y) = bounds;
    let mut y = min_y;
    let mut left_to_right = true;

    lines.push(format!("; Fill Scan (Layer C{:02})", layer.id));
    lines.push(format!("M5"));

    // Move to start
    lines.push(format!("G0 X{:.3} Y{:.3}", if left_to_right { min_x } else { max_x }, y));

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
            // Rapid to start if not already there (should be close)
            // lines.push(format!("G0 X{:.3} Y{:.3}", x_start, y)); // Optimization: assumed connected
            lines.push(format!("M3 S{:.0}", layer.power));
            lines.push(format!("G1 X{:.3} Y{:.3} F{:.0}", x_end, y, layer.speed));
        } else {
            lines.push(format!("M3 S{:.0}", layer.power));
            lines.push(format!("G1 X{:.3} Y{:.3} F{:.0}", x_start, y, layer.speed));
        }

        // Turnaround
        y += interval_mm;
        if y <= max_y {
            lines.push("M5".into()); // Turn off for step over? Or keep on for "Overcut"? Usually OFF.
            // Rapid move Y step? Or G1?
            // LightBurn usually does G1 with Laser OFF for smooth motion, or G0 if 'Fast Whitespace' is on.
            // Let's use G1 F(Speed) to keep motion smooth.

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

            lines.push(format!("G1 X{:.3} Y{:.3} F{:.0}", next_x_start, y, layer.speed));
        }

        left_to_right = !left_to_right;
    }
    lines.push("M5".into());
}
