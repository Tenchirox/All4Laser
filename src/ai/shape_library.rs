//! Parametric shape generators for the embedded AI.
//! Each generator returns a `Vec<Vec<(f32, f32)>>` — a list of closed/open polylines
//! in normalised coordinates [0,1]×[0,1].

use super::prompt_parser::Subject;
use std::f32::consts::PI;

/// Main dispatch: returns polylines for a given subject, normalised to [0,1]².
pub fn generate_shape(subject: &Subject) -> Vec<Vec<(f32, f32)>> {
    match subject {
        Subject::Bird | Subject::Eagle => eagle(),
        Subject::Cat => cat(),
        Subject::Dog => dog(),
        Subject::Fish => fish(),
        Subject::Butterfly => butterfly(),
        Subject::Horse => horse(),
        Subject::Tree => tree(),
        Subject::Flower => flower(),
        Subject::Star => star(5),
        Subject::Moon => moon(),
        Subject::Sun => sun(),
        Subject::Mountain => mountain(),
        Subject::Leaf => leaf(),
        Subject::Heart => heart(),
        Subject::House => house(),
        Subject::Gear => gear(12),
        Subject::Arrow => arrow(),
        Subject::Key => key(),
        Subject::Crown => crown(),
        Subject::Snowflake => snowflake(),
        Subject::Spiral => spiral(),
        Subject::Diamond => diamond(),
        Subject::Shield => shield(),
        Subject::Anchor => anchor(),
        Subject::Lightning => lightning(),
        Subject::Skull => skull(),
        Subject::Paw => paw(),
        Subject::Music => music_note(),
        Subject::Flame => flame(),
    }
}

// ── helpers ──────────────────────────────────────────────────────────────

fn circle_pts(cx: f32, cy: f32, rx: f32, ry: f32, n: usize) -> Vec<(f32, f32)> {
    let mut pts: Vec<(f32, f32)> = (0..n)
        .map(|i| {
            let a = 2.0 * PI * i as f32 / n as f32;
            (cx + rx * a.cos(), cy + ry * a.sin())
        })
        .collect();
    pts.push(pts[0]); // close
    pts
}

fn close(pts: &mut Vec<(f32, f32)>) {
    if let (Some(&first), Some(&last)) = (pts.first(), pts.last()) {
        if (first.0 - last.0).abs() > 0.001 || (first.1 - last.1).abs() > 0.001 {
            pts.push(first);
        }
    }
}

// ── shapes ───────────────────────────────────────────────────────────────

fn eagle() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Body
    out.push(circle_pts(0.48, 0.52, 0.14, 0.18, 32));
    // Head
    out.push(circle_pts(0.52, 0.32, 0.08, 0.08, 24));
    // Beak
    let mut bk = vec![(0.60, 0.30), (0.70, 0.32), (0.60, 0.36)];
    close(&mut bk);
    out.push(bk);
    // Eye
    out.push(circle_pts(0.55, 0.30, 0.015, 0.015, 12));
    // Left wing
    let mut lw = vec![
        (0.34, 0.44), (0.18, 0.28), (0.08, 0.30), (0.12, 0.42),
        (0.06, 0.38), (0.10, 0.50), (0.28, 0.54),
    ];
    close(&mut lw);
    out.push(lw);
    // Right wing
    let mut rw = vec![
        (0.62, 0.44), (0.78, 0.28), (0.88, 0.30), (0.84, 0.42),
        (0.90, 0.38), (0.86, 0.50), (0.68, 0.54),
    ];
    close(&mut rw);
    out.push(rw);
    // Tail feathers
    let mut tf = vec![
        (0.38, 0.68), (0.30, 0.82), (0.40, 0.78),
        (0.48, 0.86), (0.56, 0.78), (0.58, 0.68),
    ];
    close(&mut tf);
    out.push(tf);
    // Talons left
    out.push(vec![(0.42, 0.70), (0.38, 0.80), (0.36, 0.84)]);
    out.push(vec![(0.38, 0.80), (0.34, 0.82)]);
    // Talons right
    out.push(vec![(0.54, 0.70), (0.58, 0.80), (0.60, 0.84)]);
    out.push(vec![(0.58, 0.80), (0.62, 0.82)]);
    out
}

fn cat() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Body
    out.push(circle_pts(0.50, 0.58, 0.18, 0.22, 32));
    // Head
    out.push(circle_pts(0.50, 0.28, 0.12, 0.11, 28));
    // Left ear
    let mut le = vec![(0.40, 0.20), (0.36, 0.08), (0.44, 0.18)];
    close(&mut le);
    out.push(le);
    // Right ear
    let mut re = vec![(0.56, 0.18), (0.64, 0.08), (0.60, 0.20)];
    close(&mut re);
    out.push(re);
    // Eyes
    out.push(circle_pts(0.44, 0.26, 0.02, 0.025, 12));
    out.push(circle_pts(0.56, 0.26, 0.02, 0.025, 12));
    // Nose
    let mut nose = vec![(0.50, 0.30), (0.48, 0.33), (0.52, 0.33)];
    close(&mut nose);
    out.push(nose);
    // Whiskers
    out.push(vec![(0.38, 0.30), (0.22, 0.28)]);
    out.push(vec![(0.38, 0.32), (0.22, 0.34)]);
    out.push(vec![(0.62, 0.30), (0.78, 0.28)]);
    out.push(vec![(0.62, 0.32), (0.78, 0.34)]);
    // Tail
    out.push(vec![
        (0.32, 0.64), (0.22, 0.70), (0.18, 0.62), (0.14, 0.56), (0.18, 0.50),
    ]);
    // Paws
    out.push(circle_pts(0.40, 0.78, 0.04, 0.03, 12));
    out.push(circle_pts(0.60, 0.78, 0.04, 0.03, 12));
    out
}

fn dog() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    out.push(circle_pts(0.50, 0.55, 0.20, 0.22, 32));
    out.push(circle_pts(0.50, 0.28, 0.12, 0.11, 28));
    // Floppy ears
    let mut le = vec![(0.38, 0.22), (0.30, 0.18), (0.28, 0.34), (0.36, 0.32)];
    close(&mut le);
    out.push(le);
    let mut re = vec![(0.62, 0.22), (0.70, 0.18), (0.72, 0.34), (0.64, 0.32)];
    close(&mut re);
    out.push(re);
    // Eyes
    out.push(circle_pts(0.44, 0.26, 0.02, 0.02, 12));
    out.push(circle_pts(0.56, 0.26, 0.02, 0.02, 12));
    // Nose
    out.push(circle_pts(0.50, 0.33, 0.025, 0.02, 12));
    // Tongue
    out.push(vec![(0.50, 0.36), (0.48, 0.42), (0.52, 0.42), (0.50, 0.36)]);
    // Tail
    out.push(vec![(0.68, 0.50), (0.78, 0.42), (0.82, 0.36), (0.80, 0.30)]);
    // Paws
    out.push(circle_pts(0.38, 0.76, 0.04, 0.03, 12));
    out.push(circle_pts(0.62, 0.76, 0.04, 0.03, 12));
    out
}

fn fish() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Body (horizontal ellipse)
    out.push(circle_pts(0.45, 0.50, 0.25, 0.15, 32));
    // Tail
    let mut tail = vec![(0.70, 0.50), (0.85, 0.32), (0.90, 0.50), (0.85, 0.68), (0.70, 0.50)];
    close(&mut tail);
    out.push(tail);
    // Eye
    out.push(circle_pts(0.28, 0.46, 0.025, 0.025, 12));
    // Dorsal fin
    let mut df = vec![(0.38, 0.36), (0.50, 0.22), (0.58, 0.36)];
    close(&mut df);
    out.push(df);
    // Ventral fin
    let mut vf = vec![(0.42, 0.64), (0.48, 0.74), (0.54, 0.64)];
    close(&mut vf);
    out.push(vf);
    // Scales (decorative arcs)
    out.push(vec![(0.36, 0.44), (0.40, 0.50), (0.36, 0.56)]);
    out.push(vec![(0.46, 0.42), (0.50, 0.50), (0.46, 0.58)]);
    out.push(vec![(0.56, 0.44), (0.60, 0.50), (0.56, 0.56)]);
    out
}

fn butterfly() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Body (thin vertical ellipse)
    out.push(circle_pts(0.50, 0.50, 0.02, 0.20, 16));
    // Upper left wing
    let mut ulw: Vec<(f32, f32)> = (0..24).map(|i| {
        let a = PI * 0.5 + PI * i as f32 / 23.0;
        let r = 0.22 + 0.06 * (3.0 * a).sin();
        (0.50 + r * a.cos() * 0.9, 0.38 + r * a.sin() * 0.8)
    }).collect();
    close(&mut ulw);
    out.push(ulw);
    // Upper right wing (mirror)
    let mut urw: Vec<(f32, f32)> = (0..24).map(|i| {
        let a = -(PI * 0.5) - PI * i as f32 / 23.0;
        let r = 0.22 + 0.06 * (3.0 * a).sin().abs();
        (0.50 + r * a.cos() * 0.9, 0.38 - r * a.sin() * 0.8)
    }).collect();
    close(&mut urw);
    out.push(urw);
    // Lower left wing
    let mut llw: Vec<(f32, f32)> = (0..18).map(|i| {
        let a = PI * 0.5 + PI * 0.8 * i as f32 / 17.0;
        let r = 0.16;
        (0.50 + r * a.cos(), 0.62 + r * a.sin() * 0.7)
    }).collect();
    close(&mut llw);
    out.push(llw);
    // Lower right wing
    let mut lrw: Vec<(f32, f32)> = (0..18).map(|i| {
        let a = -(PI * 0.5) - PI * 0.8 * i as f32 / 17.0;
        let r = 0.16;
        (0.50 + r * a.cos(), 0.62 - r * a.sin() * 0.7)
    }).collect();
    close(&mut lrw);
    out.push(lrw);
    // Antennae
    out.push(vec![(0.49, 0.30), (0.42, 0.16), (0.40, 0.12)]);
    out.push(vec![(0.51, 0.30), (0.58, 0.16), (0.60, 0.12)]);
    // Antenna dots
    out.push(circle_pts(0.40, 0.11, 0.012, 0.012, 8));
    out.push(circle_pts(0.60, 0.11, 0.012, 0.012, 8));
    out
}

fn horse() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Body
    out.push(circle_pts(0.46, 0.50, 0.22, 0.16, 32));
    // Neck
    let mut neck = vec![(0.60, 0.38), (0.64, 0.24), (0.58, 0.20), (0.54, 0.36)];
    close(&mut neck);
    out.push(neck);
    // Head
    out.push(circle_pts(0.62, 0.18, 0.06, 0.07, 20));
    // Ear
    let mut ear = vec![(0.60, 0.12), (0.62, 0.06), (0.64, 0.12)];
    close(&mut ear);
    out.push(ear);
    // Eye
    out.push(circle_pts(0.64, 0.16, 0.012, 0.012, 10));
    // Mane
    out.push(vec![
        (0.58, 0.14), (0.56, 0.20), (0.54, 0.26), (0.52, 0.32), (0.54, 0.38),
    ]);
    // Legs
    out.push(vec![(0.36, 0.64), (0.34, 0.82), (0.32, 0.88)]);
    out.push(vec![(0.42, 0.64), (0.40, 0.84), (0.38, 0.88)]);
    out.push(vec![(0.56, 0.64), (0.58, 0.84), (0.60, 0.88)]);
    out.push(vec![(0.52, 0.64), (0.54, 0.82), (0.56, 0.88)]);
    // Hooves
    out.push(vec![(0.30, 0.88), (0.34, 0.88)]);
    out.push(vec![(0.36, 0.88), (0.40, 0.88)]);
    out.push(vec![(0.58, 0.88), (0.62, 0.88)]);
    out.push(vec![(0.54, 0.88), (0.58, 0.88)]);
    // Tail
    out.push(vec![
        (0.24, 0.44), (0.18, 0.48), (0.14, 0.54), (0.16, 0.60), (0.20, 0.56),
    ]);
    out
}

fn tree() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Trunk
    let mut trunk = vec![(0.44, 0.60), (0.44, 0.90), (0.56, 0.90), (0.56, 0.60)];
    close(&mut trunk);
    out.push(trunk);
    // Crown — layered circles for a natural look
    out.push(circle_pts(0.50, 0.38, 0.22, 0.20, 32));
    out.push(circle_pts(0.36, 0.44, 0.14, 0.12, 24));
    out.push(circle_pts(0.64, 0.44, 0.14, 0.12, 24));
    out.push(circle_pts(0.50, 0.26, 0.14, 0.12, 24));
    // Branch hints
    out.push(vec![(0.36, 0.56), (0.26, 0.50)]);
    out.push(vec![(0.64, 0.56), (0.74, 0.50)]);
    // Roots
    out.push(vec![(0.44, 0.90), (0.38, 0.96)]);
    out.push(vec![(0.56, 0.90), (0.62, 0.96)]);
    out
}

fn flower() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Center
    out.push(circle_pts(0.50, 0.40, 0.06, 0.06, 20));
    // Petals
    for i in 0..6 {
        let a = 2.0 * PI * i as f32 / 6.0 - PI / 2.0;
        let cx = 0.50 + 0.14 * a.cos();
        let cy = 0.40 + 0.14 * a.sin();
        out.push(circle_pts(cx, cy, 0.07, 0.05, 20));
    }
    // Stem
    out.push(vec![(0.50, 0.46), (0.50, 0.88)]);
    // Leaves
    let mut ll = vec![(0.50, 0.62), (0.38, 0.56), (0.36, 0.62), (0.44, 0.66)];
    close(&mut ll);
    out.push(ll);
    let mut rl = vec![(0.50, 0.72), (0.62, 0.66), (0.64, 0.72), (0.56, 0.76)];
    close(&mut rl);
    out.push(rl);
    out
}

fn star(n: usize) -> Vec<Vec<(f32, f32)>> {
    let mut pts = Vec::new();
    let outer = 0.40;
    let inner = 0.16;
    for i in 0..(n * 2) {
        let a = PI / 2.0 + 2.0 * PI * i as f32 / (n * 2) as f32;
        let r = if i % 2 == 0 { outer } else { inner };
        pts.push((0.50 + r * a.cos(), 0.50 - r * a.sin()));
    }
    close(&mut pts);
    vec![pts]
}

fn moon() -> Vec<Vec<(f32, f32)>> {
    let n = 40;
    let mut pts = Vec::new();
    // Outer arc (full circle left half)
    for i in 0..=n {
        let a = -PI / 2.0 + PI * i as f32 / n as f32;
        pts.push((0.50 + 0.30 * a.cos(), 0.50 + 0.30 * a.sin()));
    }
    // Inner arc (creates crescent)
    for i in (0..=n).rev() {
        let a = -PI / 2.0 + PI * i as f32 / n as f32;
        pts.push((0.62 + 0.22 * a.cos(), 0.50 + 0.28 * a.sin()));
    }
    close(&mut pts);
    vec![pts]
}

fn sun() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Central disc
    out.push(circle_pts(0.50, 0.50, 0.14, 0.14, 32));
    // Rays
    for i in 0..12 {
        let a = 2.0 * PI * i as f32 / 12.0;
        let x0 = 0.50 + 0.18 * a.cos();
        let y0 = 0.50 + 0.18 * a.sin();
        let x1 = 0.50 + 0.34 * a.cos();
        let y1 = 0.50 + 0.34 * a.sin();
        out.push(vec![(x0, y0), (x1, y1)]);
    }
    out
}

fn mountain() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Main range
    let mut m = vec![
        (0.05, 0.85), (0.22, 0.30), (0.35, 0.55), (0.50, 0.18),
        (0.65, 0.50), (0.78, 0.25), (0.95, 0.85),
    ];
    close(&mut m);
    out.push(m);
    // Snow cap on main peak
    let mut snow = vec![(0.44, 0.30), (0.50, 0.18), (0.56, 0.30), (0.52, 0.32), (0.48, 0.32)];
    close(&mut snow);
    out.push(snow);
    // Sun
    out.push(circle_pts(0.82, 0.16, 0.06, 0.06, 16));
    out
}

fn leaf() -> Vec<Vec<(f32, f32)>> {
    let n = 24;
    let mut pts: Vec<(f32, f32)> = (0..=n).map(|i| {
        let t = i as f32 / n as f32;
        let a = t * 2.0 * PI;
        let r = 0.35 * (0.5 * a).sin();
        (0.50 + r * a.cos() * 0.6, 0.50 + r * a.sin())
    }).collect();
    close(&mut pts);
    let mut out = vec![pts];
    // Central vein
    out.push(vec![(0.50, 0.16), (0.50, 0.84)]);
    // Side veins
    out.push(vec![(0.50, 0.34), (0.38, 0.28)]);
    out.push(vec![(0.50, 0.34), (0.62, 0.28)]);
    out.push(vec![(0.50, 0.50), (0.36, 0.44)]);
    out.push(vec![(0.50, 0.50), (0.64, 0.44)]);
    out.push(vec![(0.50, 0.66), (0.40, 0.60)]);
    out.push(vec![(0.50, 0.66), (0.60, 0.60)]);
    out
}

fn heart() -> Vec<Vec<(f32, f32)>> {
    let n = 40;
    let mut pts: Vec<(f32, f32)> = (0..=n).map(|i| {
        let t = 2.0 * PI * i as f32 / n as f32;
        let x = 16.0 * t.sin().powi(3);
        let y = 13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos();
        // Normalise to [0,1]
        (0.50 + x / 40.0, 0.50 - y / 40.0)
    }).collect();
    close(&mut pts);
    vec![pts]
}

fn house() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Walls
    let mut walls = vec![(0.20, 0.50), (0.20, 0.90), (0.80, 0.90), (0.80, 0.50)];
    close(&mut walls);
    out.push(walls);
    // Roof
    let mut roof = vec![(0.15, 0.50), (0.50, 0.12), (0.85, 0.50)];
    close(&mut roof);
    out.push(roof);
    // Door
    let mut door = vec![(0.42, 0.62), (0.42, 0.90), (0.58, 0.90), (0.58, 0.62)];
    close(&mut door);
    out.push(door);
    // Door knob
    out.push(circle_pts(0.55, 0.76, 0.012, 0.012, 8));
    // Windows
    let mut wl = vec![(0.26, 0.56), (0.26, 0.68), (0.38, 0.68), (0.38, 0.56)];
    close(&mut wl);
    out.push(wl);
    let mut wr = vec![(0.62, 0.56), (0.62, 0.68), (0.74, 0.68), (0.74, 0.56)];
    close(&mut wr);
    out.push(wr);
    // Window crosses
    out.push(vec![(0.32, 0.56), (0.32, 0.68)]);
    out.push(vec![(0.26, 0.62), (0.38, 0.62)]);
    out.push(vec![(0.68, 0.56), (0.68, 0.68)]);
    out.push(vec![(0.62, 0.62), (0.74, 0.62)]);
    // Chimney
    let mut ch = vec![(0.64, 0.14), (0.64, 0.30), (0.74, 0.24), (0.74, 0.14)];
    close(&mut ch);
    out.push(ch);
    out
}

fn gear(teeth: usize) -> Vec<Vec<(f32, f32)>> {
    let mut pts = Vec::new();
    let outer_r = 0.40;
    let inner_r = 0.32;
    let tooth_half = PI / (teeth * 2) as f32;
    for i in 0..teeth {
        let base_a = 2.0 * PI * i as f32 / teeth as f32;
        // Outer tooth
        pts.push((0.50 + outer_r * (base_a - tooth_half).cos(), 0.50 + outer_r * (base_a - tooth_half).sin()));
        pts.push((0.50 + outer_r * (base_a + tooth_half).cos(), 0.50 + outer_r * (base_a + tooth_half).sin()));
        // Valley
        let mid_a = base_a + PI / teeth as f32;
        pts.push((0.50 + inner_r * (mid_a - tooth_half).cos(), 0.50 + inner_r * (mid_a - tooth_half).sin()));
        pts.push((0.50 + inner_r * (mid_a + tooth_half).cos(), 0.50 + inner_r * (mid_a + tooth_half).sin()));
    }
    close(&mut pts);
    let mut out = vec![pts];
    // Center hole
    out.push(circle_pts(0.50, 0.50, 0.08, 0.08, 20));
    out
}

fn arrow() -> Vec<Vec<(f32, f32)>> {
    let mut pts = vec![
        (0.10, 0.46), (0.10, 0.54),  // shaft bottom-left
        (0.60, 0.54), (0.60, 0.68),  // shaft to head base right
        (0.92, 0.50),                  // tip
        (0.60, 0.32), (0.60, 0.46),  // head base left back to shaft
    ];
    close(&mut pts);
    vec![pts]
}

fn key() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Handle (ring)
    out.push(circle_pts(0.28, 0.50, 0.14, 0.14, 28));
    // Inner hole
    out.push(circle_pts(0.28, 0.50, 0.06, 0.06, 16));
    // Shaft
    let mut shaft = vec![(0.42, 0.47), (0.82, 0.47), (0.82, 0.53), (0.42, 0.53)];
    close(&mut shaft);
    out.push(shaft);
    // Teeth
    let mut t1 = vec![(0.72, 0.53), (0.72, 0.62), (0.76, 0.62), (0.76, 0.53)];
    close(&mut t1);
    out.push(t1);
    let mut t2 = vec![(0.80, 0.53), (0.80, 0.58), (0.84, 0.58), (0.84, 0.53)];
    close(&mut t2);
    out.push(t2);
    out
}

fn crown() -> Vec<Vec<(f32, f32)>> {
    let mut pts = vec![
        (0.15, 0.75), (0.15, 0.45), (0.25, 0.58),
        (0.35, 0.30), (0.45, 0.55),
        (0.50, 0.22), (0.55, 0.55),
        (0.65, 0.30), (0.75, 0.58),
        (0.85, 0.45), (0.85, 0.75),
    ];
    close(&mut pts);
    let mut out = vec![pts];
    // Jewels
    out.push(circle_pts(0.35, 0.32, 0.02, 0.02, 10));
    out.push(circle_pts(0.50, 0.24, 0.025, 0.025, 10));
    out.push(circle_pts(0.65, 0.32, 0.02, 0.02, 10));
    // Band
    out.push(vec![(0.15, 0.70), (0.85, 0.70)]);
    out
}

fn snowflake() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    for i in 0..6 {
        let a = PI / 2.0 + 2.0 * PI * i as f32 / 6.0;
        let (ca, sa) = (a.cos(), a.sin());
        // Main arm
        out.push(vec![(0.50, 0.50), (0.50 + 0.36 * ca, 0.50 - 0.36 * sa)]);
        // Side branches
        let blen = 0.10;
        for &frac in &[0.4, 0.7] {
            let bx = 0.50 + 0.36 * frac * ca;
            let by = 0.50 - 0.36 * frac * sa;
            let ba = a + PI / 4.0;
            out.push(vec![(bx, by), (bx + blen * ba.cos(), by - blen * ba.sin())]);
            let ba2 = a - PI / 4.0;
            out.push(vec![(bx, by), (bx + blen * ba2.cos(), by - blen * ba2.sin())]);
        }
    }
    out
}

fn spiral() -> Vec<Vec<(f32, f32)>> {
    let n = 200;
    let turns = 4.0;
    let pts: Vec<(f32, f32)> = (0..=n).map(|i| {
        let t = i as f32 / n as f32;
        let a = turns * 2.0 * PI * t;
        let r = 0.02 + 0.36 * t;
        (0.50 + r * a.cos(), 0.50 + r * a.sin())
    }).collect();
    vec![pts]
}

fn diamond() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Outer shape
    let mut outer = vec![
        (0.50, 0.08), (0.80, 0.38), (0.50, 0.92), (0.20, 0.38),
    ];
    close(&mut outer);
    out.push(outer);
    // Crown line
    out.push(vec![(0.20, 0.38), (0.80, 0.38)]);
    // Facets
    out.push(vec![(0.35, 0.38), (0.50, 0.08)]);
    out.push(vec![(0.65, 0.38), (0.50, 0.08)]);
    out.push(vec![(0.35, 0.38), (0.50, 0.92)]);
    out.push(vec![(0.65, 0.38), (0.50, 0.92)]);
    out.push(vec![(0.50, 0.38), (0.50, 0.92)]);
    out
}

fn shield() -> Vec<Vec<(f32, f32)>> {
    let n = 20;
    let mut pts = vec![(0.18, 0.12), (0.18, 0.52)];
    // Bottom curve
    for i in 0..=n {
        let t = i as f32 / n as f32;
        let x = 0.18 + 0.64 * t;
        let y = 0.52 + 0.38 * (PI * t).sin();
        pts.push((x, y));
    }
    pts.push((0.82, 0.52));
    pts.push((0.82, 0.12));
    close(&mut pts);
    let mut out = vec![pts];
    // Cross emblem
    out.push(vec![(0.50, 0.20), (0.50, 0.60)]);
    out.push(vec![(0.34, 0.38), (0.66, 0.38)]);
    out
}

fn anchor() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Ring at top
    out.push(circle_pts(0.50, 0.16, 0.06, 0.06, 16));
    // Shaft
    out.push(vec![(0.50, 0.22), (0.50, 0.78)]);
    // Cross bar
    out.push(vec![(0.36, 0.34), (0.64, 0.34)]);
    // Flukes (curved arms)
    let n = 12;
    let mut left: Vec<(f32, f32)> = (0..=n).map(|i| {
        let t = i as f32 / n as f32;
        let a = PI * 0.5 * t;
        (0.50 - 0.24 * a.sin(), 0.78 - 0.18 * (1.0 - a.cos()))
    }).collect();
    left.push((0.20, 0.70));
    out.push(left);
    let mut right: Vec<(f32, f32)> = (0..=n).map(|i| {
        let t = i as f32 / n as f32;
        let a = PI * 0.5 * t;
        (0.50 + 0.24 * a.sin(), 0.78 - 0.18 * (1.0 - a.cos()))
    }).collect();
    right.push((0.80, 0.70));
    out.push(right);
    // Arrow tips on flukes
    out.push(vec![(0.22, 0.66), (0.20, 0.70), (0.26, 0.72)]);
    out.push(vec![(0.78, 0.66), (0.80, 0.70), (0.74, 0.72)]);
    out
}

fn lightning() -> Vec<Vec<(f32, f32)>> {
    let mut pts = vec![
        (0.42, 0.06), (0.28, 0.46), (0.46, 0.42),
        (0.34, 0.94), (0.72, 0.38), (0.52, 0.42),
        (0.64, 0.06),
    ];
    close(&mut pts);
    vec![pts]
}

fn skull() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Cranium (top half circle + jaw)
    let n = 24;
    let mut cranium: Vec<(f32, f32)> = (0..=n).map(|i| {
        let a = PI + PI * i as f32 / n as f32;
        (0.50 + 0.28 * a.cos(), 0.40 + 0.28 * a.sin())
    }).collect();
    // Jaw
    cranium.push((0.78, 0.44));
    cranium.push((0.74, 0.62));
    cranium.push((0.64, 0.70));
    cranium.push((0.36, 0.70));
    cranium.push((0.26, 0.62));
    cranium.push((0.22, 0.44));
    close(&mut cranium);
    out.push(cranium);
    // Eyes
    out.push(circle_pts(0.40, 0.38, 0.06, 0.07, 16));
    out.push(circle_pts(0.60, 0.38, 0.06, 0.07, 16));
    // Nose
    let mut nose = vec![(0.50, 0.48), (0.46, 0.56), (0.54, 0.56)];
    close(&mut nose);
    out.push(nose);
    // Teeth
    for i in 0..5 {
        let x = 0.38 + i as f32 * 0.06;
        out.push(vec![(x, 0.64), (x, 0.70)]);
    }
    out
}

fn paw() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Main pad
    out.push(circle_pts(0.50, 0.62, 0.16, 0.14, 24));
    // Toes (4 circles)
    out.push(circle_pts(0.30, 0.40, 0.07, 0.08, 16));
    out.push(circle_pts(0.42, 0.30, 0.07, 0.08, 16));
    out.push(circle_pts(0.58, 0.30, 0.07, 0.08, 16));
    out.push(circle_pts(0.70, 0.40, 0.07, 0.08, 16));
    out
}

fn music_note() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Note head (left)
    out.push(circle_pts(0.32, 0.72, 0.10, 0.07, 20));
    // Note head (right)
    out.push(circle_pts(0.62, 0.66, 0.10, 0.07, 20));
    // Stems
    out.push(vec![(0.42, 0.72), (0.42, 0.20)]);
    out.push(vec![(0.72, 0.66), (0.72, 0.14)]);
    // Beam
    let mut beam = vec![(0.42, 0.20), (0.72, 0.14), (0.72, 0.22), (0.42, 0.28)];
    close(&mut beam);
    out.push(beam);
    // Flag/second beam
    let mut beam2 = vec![(0.42, 0.30), (0.72, 0.24), (0.72, 0.32), (0.42, 0.38)];
    close(&mut beam2);
    out.push(beam2);
    out
}

fn flame() -> Vec<Vec<(f32, f32)>> {
    let mut out = Vec::new();
    // Outer flame
    let mut outer = vec![
        (0.50, 0.06), (0.38, 0.28), (0.32, 0.46),
        (0.28, 0.62), (0.30, 0.76), (0.38, 0.86),
        (0.50, 0.92), (0.62, 0.86), (0.70, 0.76),
        (0.72, 0.62), (0.68, 0.46), (0.62, 0.28),
    ];
    close(&mut outer);
    out.push(outer);
    // Inner flame
    let mut inner = vec![
        (0.50, 0.30), (0.44, 0.46), (0.40, 0.60),
        (0.42, 0.72), (0.50, 0.80),
        (0.58, 0.72), (0.60, 0.60), (0.56, 0.46),
    ];
    close(&mut inner);
    out.push(inner);
    // Core
    out.push(circle_pts(0.50, 0.68, 0.05, 0.08, 12));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_shapes_produce_output() {
        let subjects = [
            Subject::Eagle, Subject::Bird, Subject::Cat, Subject::Dog,
            Subject::Fish, Subject::Butterfly, Subject::Horse, Subject::Tree,
            Subject::Flower, Subject::Star, Subject::Moon, Subject::Sun,
            Subject::Mountain, Subject::Leaf, Subject::Heart, Subject::House,
            Subject::Gear, Subject::Arrow, Subject::Key, Subject::Crown,
            Subject::Snowflake, Subject::Spiral, Subject::Diamond, Subject::Shield,
            Subject::Anchor, Subject::Lightning, Subject::Skull, Subject::Paw,
            Subject::Music, Subject::Flame,
        ];
        for s in &subjects {
            let polys = generate_shape(s);
            assert!(!polys.is_empty(), "Shape {:?} produced no polylines", s);
            for (i, poly) in polys.iter().enumerate() {
                assert!(poly.len() >= 2, "Shape {:?} polyline {} has <2 pts", s, i);
                for &(x, y) in poly {
                    assert!(x >= -0.1 && x <= 1.1 && y >= -0.1 && y <= 1.1,
                        "Shape {:?} polyline {} has OOB point ({}, {})", s, i, x, y);
                }
            }
        }
    }
}
