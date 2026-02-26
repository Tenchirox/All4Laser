/// DXF file importer — converts LINE, ARC, CIRCLE, LWPOLYLINE to GCode

pub struct DxfParams {
    pub speed: f32,
    pub power: f32,
    pub scale: f32, // mm per DXF unit
    pub passes: u32,
}

impl Default for DxfParams {
    fn default() -> Self {
        Self { speed: 800.0, power: 800.0, scale: 1.0, passes: 1 }
    }
}

pub fn dxf_to_gcode(data: &str, params: &DxfParams) -> Result<Vec<String>, String> {
    let mut lines = Vec::new();
    lines.push("; DXF Import — All4Laser".into());
    lines.push("G90 G21 G17".into());
    lines.push(format!("M3 S{:.0}", params.power));

    for _ in 0..params.passes {
        let entities = parse_entities(data);
        for entity in &entities {
            match entity {
                DxfEntity::Line { x1, y1, x2, y2 } => {
                    let (x1, y1, x2, y2) = (x1 * params.scale, y1 * params.scale, x2 * params.scale, y2 * params.scale);
                    lines.push("M5".into());
                    lines.push(format!("G0 X{:.3} Y{:.3} F3000", x1, y1));
                    lines.push(format!("M3 S{:.0}", params.power));
                    lines.push(format!("G1 X{:.3} Y{:.3} F{:.0}", x2, y2, params.speed));
                }
                DxfEntity::Circle { cx, cy, r } => {
                    let (cx, cy, r) = (cx * params.scale, cy * params.scale, r * params.scale);
                    // Approximate circle with 36 segments
                    let start_x = cx + r;
                    let start_y = cy;
                    lines.push("M5".into());
                    lines.push(format!("G0 X{:.3} Y{:.3} F3000", start_x, start_y));
                    lines.push(format!("M3 S{:.0}", params.power));
                    let n = 36usize;
                    for i in 1..=n {
                        let angle = 2.0 * std::f32::consts::PI * i as f32 / n as f32;
                        let x = cx + r * angle.cos();
                        let y = cy + r * angle.sin();
                        lines.push(format!("G1 X{:.3} Y{:.3} F{:.0}", x, y, params.speed));
                    }
                }
                DxfEntity::Polyline(pts) if pts.len() >= 2 => {
                    let (sx, sy) = pts[0];
                    lines.push("M5".into());
                    lines.push(format!("G0 X{:.3} Y{:.3} F3000", sx * params.scale, sy * params.scale));
                    lines.push(format!("M3 S{:.0}", params.power));
                    for &(px, py) in pts.iter().skip(1) {
                        lines.push(format!("G1 X{:.3} Y{:.3} F{:.0}", px * params.scale, py * params.scale, params.speed));
                    }
                }
                DxfEntity::Arc { cx, cy, r, start_angle, end_angle } => {
                    let (cx, cy, r) = (cx * params.scale, cy * params.scale, r * params.scale);
                    let sa = start_angle.to_radians();
                    let mut ea = end_angle.to_radians();
                    if ea <= sa { ea += 2.0 * std::f32::consts::PI; }
                    let sx = cx + r * sa.cos();
                    let sy = cy + r * sa.sin();
                    lines.push("M5".into());
                    lines.push(format!("G0 X{:.3} Y{:.3} F3000", sx, sy));
                    lines.push(format!("M3 S{:.0}", params.power));
                    let n = 36usize;
                    for i in 1..=n {
                        let angle = sa + (ea - sa) * i as f32 / n as f32;
                        let x = cx + r * angle.cos();
                        let y = cy + r * angle.sin();
                        lines.push(format!("G1 X{:.3} Y{:.3} F{:.0}", x, y, params.speed));
                    }
                }
                _ => {}
            }
        }
    }

    lines.push("M5".into());
    lines.push("G0 X0 Y0 F3000".into());
    Ok(lines)
}

#[derive(Debug)]
enum DxfEntity {
    Line { x1: f32, y1: f32, x2: f32, y2: f32 },
    Circle { cx: f32, cy: f32, r: f32 },
    Arc { cx: f32, cy: f32, r: f32, start_angle: f32, end_angle: f32 },
    Polyline(Vec<(f32, f32)>),
}

fn parse_entities(data: &str) -> Vec<DxfEntity> {
    let mut entities = Vec::new();
    let mut lines_iter = data.lines().peekable();

    // Fast scan: find ENTITIES section
    while let Some(line) = lines_iter.next() {
        if line.trim() == "ENTITIES" { break; }
    }

    while let Some(code_line) = lines_iter.next() {
        let code = code_line.trim().parse::<i32>().unwrap_or(-1);
        let value = lines_iter.next().map(|l| l.trim()).unwrap_or("").to_string();

        if code == 0 {
            match value.as_str() {
                "LINE" => {
                    if let Some(e) = read_line(&mut lines_iter) { entities.push(e); }
                }
                "CIRCLE" => {
                    if let Some(e) = read_circle(&mut lines_iter) { entities.push(e); }
                }
                "ARC" => {
                    if let Some(e) = read_arc(&mut lines_iter) { entities.push(e); }
                }
                "LWPOLYLINE" | "POLYLINE" => {
                    if let Some(e) = read_polyline(&mut lines_iter) { entities.push(e); }
                }
                "ENDSEC" | "EOF" => break,
                _ => {}
            }
        }
    }

    entities
}

fn read_pairs<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Vec<(i32, String)> {
    let mut pairs = Vec::new();
    loop {
        let code_str = match iter.next() { Some(s) => s.trim().to_string(), None => break };
        let val_str = match iter.next() { Some(s) => s.trim().to_string(), None => break };
        let code: i32 = code_str.parse().unwrap_or(-1);
        if code == 0 { break; } // next entity
        pairs.push((code, val_str));
    }
    pairs
}

fn parse_f<'a>(pairs: &[(i32, String)], group: i32) -> f32 {
    pairs.iter().find(|(c, _)| *c == group).and_then(|(_, v)| v.parse::<f32>().ok()).unwrap_or(0.0)
}

fn read_line<'a>(iter: &mut std::iter::Peekable<impl Iterator<Item = &'a str>>) -> Option<DxfEntity> {
    let flat: Vec<String> = read_flat_pairs(iter);
    let pairs = parse_flat(&flat);
    Some(DxfEntity::Line {
        x1: parse_f(&pairs, 10), y1: parse_f(&pairs, 20),
        x2: parse_f(&pairs, 11), y2: parse_f(&pairs, 21),
    })
}

fn read_circle<'a>(iter: &mut std::iter::Peekable<impl Iterator<Item = &'a str>>) -> Option<DxfEntity> {
    let flat = read_flat_pairs(iter);
    let pairs = parse_flat(&flat);
    Some(DxfEntity::Circle { cx: parse_f(&pairs, 10), cy: parse_f(&pairs, 20), r: parse_f(&pairs, 40) })
}

fn read_arc<'a>(iter: &mut std::iter::Peekable<impl Iterator<Item = &'a str>>) -> Option<DxfEntity> {
    let flat = read_flat_pairs(iter);
    let pairs = parse_flat(&flat);
    Some(DxfEntity::Arc {
        cx: parse_f(&pairs, 10), cy: parse_f(&pairs, 20),
        r: parse_f(&pairs, 40),
        start_angle: parse_f(&pairs, 50),
        end_angle: parse_f(&pairs, 51),
    })
}

fn read_polyline<'a>(iter: &mut std::iter::Peekable<impl Iterator<Item = &'a str>>) -> Option<DxfEntity> {
    let flat = read_flat_pairs(iter);
    let pairs = parse_flat(&flat);
    
    // Collect sequential x/y pairs (group 10 and 20 repeat)
    let mut pts = Vec::new();
    let mut x_acc: Vec<f32> = Vec::new();
    let mut y_acc: Vec<f32> = Vec::new();
    for (code, val) in &pairs {
        if *code == 10 { x_acc.push(val.parse().unwrap_or(0.0)); }
        if *code == 20 { y_acc.push(val.parse().unwrap_or(0.0)); }
    }
    for (x, y) in x_acc.into_iter().zip(y_acc.into_iter()) {
        pts.push((x, y));
    }
    if pts.len() < 2 { return None; }
    Some(DxfEntity::Polyline(pts))
}

fn read_flat_pairs<'a>(iter: &mut std::iter::Peekable<impl Iterator<Item = &'a str>>) -> Vec<String> {
    let mut flat = Vec::new();
    loop {
        let peeked = iter.peek().copied().unwrap_or("").trim();
        let code: i32 = peeked.parse().unwrap_or(-1);
        if code == 0 { break; }
        if let Some(code_line) = iter.next() {
            flat.push(code_line.trim().to_string());
            if let Some(val_line) = iter.next() {
                flat.push(val_line.trim().to_string());
            }
        } else {
            break;
        }
    }
    flat
}

fn parse_flat(flat: &[String]) -> Vec<(i32, String)> {
    flat.chunks(2)
        .filter_map(|c| if c.len() == 2 { Some((c[0].parse().unwrap_or(-1), c[1].clone())) } else { None })
        .collect()
}
