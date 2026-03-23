//! Scene composer: takes a prompt string, parses it, generates shapes from the
//! library, scales/positions them, and returns `Vec<ShapeParams>` ready for the canvas.

use super::prompt_parser::{self, Relation};
use super::shape_library;
use crate::ui::drawing::{PathData, ShapeKind, ShapeParams};

/// Main entry point — call from the UI with the raw prompt and target dimensions.
pub fn generate_from_prompt(
    prompt: &str,
    width_mm: f32,
    height_mm: f32,
    layer_idx: usize,
) -> Vec<ShapeParams> {
    let analysis = prompt_parser::parse_prompt(prompt);
    let n = analysis.elements.len();

    if n == 0 {
        return Vec::new();
    }

    // Collect raw polyline sets per element
    let raw_sets: Vec<Vec<Vec<(f32, f32)>>> = analysis
        .elements
        .iter()
        .map(|(elem, _)| shape_library::generate_shape(&elem.subject))
        .collect();

    // Determine layout based on element count and relations
    let layouts = compute_layout(&analysis.elements, n, width_mm, height_mm);

    let mut out = Vec::new();
    let group_id = timestamp_group_id();

    for (i, polys) in raw_sets.iter().enumerate() {
        let layout = &layouts[i];
        for poly in polys {
            let scaled: Vec<(f32, f32)> = poly
                .iter()
                .map(|&(nx, ny)| {
                    (layout.ox + nx * layout.w, layout.oy + ny * layout.h)
                })
                .collect();

            if scaled.len() < 2 {
                continue;
            }

            out.push(ShapeParams {
                shape: ShapeKind::Path(PathData::from_points(scaled)),
                x: 0.0,
                y: 0.0,
                layer_idx,
                group_id: Some(group_id),
                ..ShapeParams::default()
            });
        }
    }

    out
}

struct LayoutRect {
    ox: f32,
    oy: f32,
    w: f32,
    h: f32,
}

fn compute_layout(
    elements: &[(prompt_parser::SceneElement, Option<Relation>)],
    n: usize,
    total_w: f32,
    total_h: f32,
) -> Vec<LayoutRect> {
    if n == 1 {
        // Single element: center with 90% of area, maintain aspect ratio
        let margin = 0.05;
        return vec![LayoutRect {
            ox: total_w * margin,
            oy: total_h * margin,
            w: total_w * (1.0 - 2.0 * margin),
            h: total_h * (1.0 - 2.0 * margin),
        }];
    }

    // Check if there's a dominant spatial relation
    let first_rel = elements.first().and_then(|(_, r)| r.clone());

    match first_rel {
        Some(Relation::On) => layout_stacked(n, total_w, total_h),
        Some(Relation::Inside) => layout_nested(n, total_w, total_h),
        Some(Relation::Around) => layout_radial(n, total_w, total_h),
        _ => {
            // "with", "and", or no relation: side by side or grid
            if n == 2 {
                layout_side_by_side(total_w, total_h)
            } else {
                layout_grid(n, total_w, total_h)
            }
        }
    }
}

/// "X on Y" — first element on top of second (Y takes bottom 60%, X on top 50%)
fn layout_stacked(n: usize, w: f32, h: f32) -> Vec<LayoutRect> {
    let mut layouts = Vec::new();
    let margin = w.min(h) * 0.03;

    if n >= 2 {
        // First subject (the one "on" something) — upper portion, smaller
        let sub_h = h * 0.48;
        let sub_w = w * 0.50;
        layouts.push(LayoutRect {
            ox: (w - sub_w) * 0.5,
            oy: margin,
            w: sub_w,
            h: sub_h,
        });
        // Second subject (the base) — lower portion, wider
        let base_h = h * 0.35;
        let base_w = w * 0.85;
        layouts.push(LayoutRect {
            ox: (w - base_w) * 0.5,
            oy: h * 0.55,
            w: base_w,
            h: base_h,
        });
    }

    // Extra elements get small slots below
    for i in 2..n {
        let slot_w = w / (n - 2).max(1) as f32 * 0.8;
        layouts.push(LayoutRect {
            ox: margin + (i - 2) as f32 * (slot_w + margin),
            oy: h * 0.88,
            w: slot_w.min(w * 0.3),
            h: h * 0.10,
        });
    }
    layouts
}

/// "X inside Y" — Y is large, X is centered smaller inside
fn layout_nested(n: usize, w: f32, h: f32) -> Vec<LayoutRect> {
    let mut layouts = Vec::new();
    let margin = w.min(h) * 0.03;

    // First (inner)
    let inner_w = w * 0.40;
    let inner_h = h * 0.40;
    layouts.push(LayoutRect {
        ox: (w - inner_w) * 0.5,
        oy: (h - inner_h) * 0.5,
        w: inner_w,
        h: inner_h,
    });

    // Second (outer container)
    if n >= 2 {
        layouts.push(LayoutRect {
            ox: margin,
            oy: margin,
            w: w - 2.0 * margin,
            h: h - 2.0 * margin,
        });
        // Swap so outer is drawn first (behind)
        layouts.swap(0, 1);
    }

    for i in 2..n {
        let s = (w.min(h) * 0.20).max(10.0);
        layouts.push(LayoutRect {
            ox: margin + (i - 2) as f32 * (s + margin),
            oy: h - s - margin,
            w: s,
            h: s,
        });
    }
    layouts
}

/// "X around Y" — Y centered, X elements radially placed
fn layout_radial(n: usize, w: f32, h: f32) -> Vec<LayoutRect> {
    let mut layouts = Vec::new();

    // First is the center
    let center_s = w.min(h) * 0.35;
    layouts.push(LayoutRect {
        ox: (w - center_s) * 0.5,
        oy: (h - center_s) * 0.5,
        w: center_s,
        h: center_s,
    });

    // Others around
    let satellite_count = (n - 1).max(1);
    let radius = w.min(h) * 0.30;
    let sat_size = w.min(h) * 0.20;
    for i in 0..satellite_count {
        let a = std::f32::consts::PI * 2.0 * i as f32 / satellite_count as f32
            - std::f32::consts::FRAC_PI_2;
        let cx = w * 0.5 + radius * a.cos() - sat_size * 0.5;
        let cy = h * 0.5 + radius * a.sin() - sat_size * 0.5;
        layouts.push(LayoutRect {
            ox: cx,
            oy: cy,
            w: sat_size,
            h: sat_size,
        });
    }
    layouts
}

/// Two elements side by side
fn layout_side_by_side(w: f32, h: f32) -> Vec<LayoutRect> {
    let margin = w * 0.04;
    let half_w = (w - 3.0 * margin) * 0.5;
    vec![
        LayoutRect {
            ox: margin,
            oy: h * 0.08,
            w: half_w,
            h: h * 0.84,
        },
        LayoutRect {
            ox: 2.0 * margin + half_w,
            oy: h * 0.08,
            w: half_w,
            h: h * 0.84,
        },
    ]
}

/// Grid layout for 3+ elements
fn layout_grid(n: usize, w: f32, h: f32) -> Vec<LayoutRect> {
    let cols = (n as f32).sqrt().ceil() as usize;
    let rows = (n + cols - 1) / cols;
    let margin = w.min(h) * 0.03;
    let cell_w = (w - margin * (cols + 1) as f32) / cols as f32;
    let cell_h = (h - margin * (rows + 1) as f32) / rows as f32;

    let mut layouts = Vec::new();
    for i in 0..n {
        let c = i % cols;
        let r = i / cols;
        layouts.push(LayoutRect {
            ox: margin + c as f32 * (cell_w + margin),
            oy: margin + r as f32 * (cell_h + margin),
            w: cell_w,
            h: cell_h,
        });
    }
    layouts
}

fn timestamp_group_id() -> u32 {
    (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        & 0xFFFF_FFFF) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_prompt_produces_shapes() {
        let shapes = generate_from_prompt("eagle", 100.0, 100.0, 0);
        assert!(!shapes.is_empty(), "Eagle prompt should produce shapes");
        // All shapes should be paths
        for s in &shapes {
            assert!(matches!(s.shape, ShapeKind::Path(_)));
        }
    }

    #[test]
    fn eagle_on_branch_produces_multiple_groups() {
        let shapes = generate_from_prompt("aigle sur une branche", 200.0, 150.0, 0);
        assert!(shapes.len() > 5, "Scene should have many shape paths, got {}", shapes.len());
    }

    #[test]
    fn unknown_prompt_still_works() {
        let shapes = generate_from_prompt("xyzabc", 80.0, 80.0, 0);
        assert!(!shapes.is_empty(), "Unknown prompt should still produce a default shape");
    }

    #[test]
    fn multiple_subjects_layout() {
        let shapes = generate_from_prompt("heart and star and flower", 200.0, 200.0, 0);
        assert!(shapes.len() > 3);
    }

    #[test]
    fn shapes_within_bounds() {
        let w = 100.0_f32;
        let h = 80.0;
        let shapes = generate_from_prompt("cat with flower", w, h, 0);
        for s in &shapes {
            if let ShapeKind::Path(pd) = &s.shape {
                for &(px, py) in pd.points.iter() {
                    assert!(px >= -5.0 && px <= w + 5.0, "x={} OOB", px);
                    assert!(py >= -5.0 && py <= h + 5.0, "y={} OOB", py);
                }
            }
        }
    }
}
