#![allow(dead_code)]

use crate::ui::drawing::{PathData, ShapeKind, ShapeParams};

/// Experimental Ruida .rd importer.
///
/// Ruida RD is a binary controller format; this importer is a best-effort path
/// extractor for mixed/textual RD variants often seen in toolchain exports.
pub fn import_rd(bytes: &[u8]) -> Result<Vec<ShapeParams>, String> {
    if bytes.is_empty() {
        return Err("Empty .rd file".into());
    }

    let lossy = String::from_utf8_lossy(bytes);

    // 1) Try HPGL-like content first (PU/PD)
    if (lossy.contains("PU") || lossy.contains("PD"))
        && let Ok(shapes) = crate::imaging::hpgl::parse_hpgl(&lossy, 0)
        && !shapes.is_empty()
    {
        return Ok(shapes);
    }

    // 2) Try generic G-code-like lines (G0/G1 with X/Y)
    let mut path_points: Vec<(f32, f32)> = Vec::new();
    for line in lossy.lines() {
        let l = line.trim();
        if l.is_empty() {
            continue;
        }
        let upper = l.to_ascii_uppercase();
        if !(upper.contains("G0") || upper.contains("G1") || upper.contains('X')) {
            continue;
        }
        if let Some((x, y)) = extract_xy(&upper) {
            path_points.push((x, y));
        }
    }

    // 3) Try token stream fallback (X..Y.. in binary-adjacent text)
    if path_points.len() < 2 {
        let tokens = lossy
            .split(|c: char| c.is_whitespace() || c == ',' || c == ';' || c == '\0')
            .filter(|t| !t.is_empty())
            .collect::<Vec<_>>();
        for tok in tokens {
            if let Some((x, y)) = extract_xy(&tok.to_ascii_uppercase()) {
                path_points.push((x, y));
            }
        }
    }

    // Remove duplicate consecutive points
    path_points.dedup_by(|a, b| (a.0 - b.0).abs() < 0.0001 && (a.1 - b.1).abs() < 0.0001);

    if path_points.len() < 2 {
        return Err(
            "Unsupported .rd structure: could not extract XY path data (experimental importer)"
                .into(),
        );
    }

    let shape = ShapeParams {
        shape: ShapeKind::Path(PathData::from_points(path_points)),
        layer_idx: 0,
        ..ShapeParams::default()
    };

    Ok(vec![shape])
}

fn extract_xy(s: &str) -> Option<(f32, f32)> {
    let x_idx = s.find('X')?;
    let y_idx = s.find('Y')?;
    if y_idx <= x_idx {
        return None;
    }

    let x_raw = &s[x_idx + 1..y_idx];
    let y_raw = &s[y_idx + 1..];

    let x = parse_num_prefix(x_raw)?;
    let y = parse_num_prefix(y_raw)?;
    Some((x, y))
}

fn parse_num_prefix(s: &str) -> Option<f32> {
    let mut out = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() || c == '.' || c == '-' || c == '+' {
            out.push(c);
        } else {
            break;
        }
    }
    if out.is_empty() {
        None
    } else {
        out.parse::<f32>().ok()
    }
}
