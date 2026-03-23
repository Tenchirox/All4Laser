//! Robust SVG path `d` attribute parser for LLM output.
//! Cleans up markdown/text noise, extracts paths, auto-normalizes coordinates.
//! Supports layer classification via class="cut|engrave|fine" attributes.

/// Layer classification for a path element.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathLayer {
    /// Outer contour — laser cut
    Cut,
    /// Medium-detail engraving
    Engrave,
    /// Fine detail / hatching / contrast
    Fine,
}

/// A parsed path with its layer classification and polyline points.
#[derive(Debug, Clone)]
pub struct LayeredPath {
    pub layer: PathLayer,
    pub points: Vec<(f32, f32)>,
}

/// Parse all `<path d="..."/>` from raw LLM text and return polylines in [0,100] coords.
/// Handles markdown code blocks, text noise, and auto-normalizes coordinates.
pub fn extract_paths_from_svg(raw: &str) -> Vec<Vec<(f32, f32)>> {
    extract_layered_paths(raw)
        .into_iter()
        .map(|lp| lp.points)
        .collect()
}

/// Parse paths with layer classification from raw LLM output.
/// Returns `LayeredPath` with class info (cut/engrave/fine).
pub fn extract_layered_paths(raw: &str) -> Vec<LayeredPath> {
    let cleaned = clean_llm_output(raw);
    let mut paths: Vec<LayeredPath> = Vec::new();

    // Extract <path ...> tags and parse class + d attributes
    extract_path_tags(&cleaned, &mut paths);

    // If no <path> tags found, try raw path data (defaults to Cut layer)
    if paths.is_empty() {
        let trimmed = cleaned.trim();
        if looks_like_path_data(trimmed) {
            if let Some(poly) = parse_d_attribute(trimmed) {
                if poly.len() >= 2 {
                    paths.push(LayeredPath { layer: PathLayer::Cut, points: poly });
                }
            }
        }
    }

    // Auto-normalize all points to [0,100]
    {
        let mut all_points: Vec<&mut Vec<(f32, f32)>> = paths.iter_mut().map(|lp| &mut lp.points).collect();
        normalize_to_viewbox_multi(&mut all_points);
    }

    // Remove degenerate paths
    paths.retain(|lp| {
        if lp.points.len() < 2 {
            return false;
        }
        let (fx, fy) = lp.points[0];
        lp.points.iter().any(|(x, y)| (x - fx).abs() > 0.5 || (y - fy).abs() > 0.5)
    });

    // Smooth jagged paths (Chaikin subdivision) for paths with many short segments
    for lp in paths.iter_mut() {
        if lp.points.len() >= 6 {
            lp.points = chaikin_smooth(&lp.points, 2);
        }
    }

    paths
}

/// Chaikin's corner-cutting subdivision for smoothing jagged polylines.
/// Each iteration replaces sharp corners with smoother curves.
fn chaikin_smooth(points: &[(f32, f32)], iterations: usize) -> Vec<(f32, f32)> {
    if points.len() < 3 {
        return points.to_vec();
    }

    // Detect if path is closed (first ≈ last point)
    let is_closed = {
        let (fx, fy) = points[0];
        let (lx, ly) = points[points.len() - 1];
        (fx - lx).abs() < 1.0 && (fy - ly).abs() < 1.0
    };

    let mut pts = points.to_vec();
    for _ in 0..iterations {
        if pts.len() < 3 {
            break;
        }
        let mut smooth = Vec::with_capacity(pts.len() * 2);

        if !is_closed {
            // Keep first point for open paths
            smooth.push(pts[0]);
        }

        for i in 0..pts.len() - 1 {
            let (x0, y0) = pts[i];
            let (x1, y1) = pts[i + 1];
            // Q point at 25%
            smooth.push((0.75 * x0 + 0.25 * x1, 0.75 * y0 + 0.25 * y1));
            // R point at 75%
            smooth.push((0.25 * x0 + 0.75 * x1, 0.25 * y0 + 0.75 * y1));
        }

        if is_closed {
            // Close the loop by connecting last to first
            let (x0, y0) = pts[pts.len() - 1];
            let (x1, y1) = pts[0];
            smooth.push((0.75 * x0 + 0.25 * x1, 0.75 * y0 + 0.25 * y1));
            smooth.push((0.25 * x0 + 0.75 * x1, 0.25 * y0 + 0.75 * y1));
        } else {
            // Keep last point for open paths
            smooth.push(pts[pts.len() - 1]);
        }

        pts = smooth;
    }
    pts
}

/// Strip markdown code blocks, explanatory text, XML headers from LLM output.
fn clean_llm_output(raw: &str) -> String {
    let mut s = raw.to_string();

    // Remove markdown code fences
    s = s.replace("```xml", "").replace("```svg", "").replace("```html", "").replace("```", "");

    // Remove XML headers
    if let Some(pos) = s.find("<?xml") {
        if let Some(end) = s[pos..].find("?>") {
            s = format!("{}{}", &s[..pos], &s[pos + end + 2..]);
        }
    }

    // Remove <svg ...> and </svg> wrappers but keep inner content
    while let Some(start) = s.find("<svg") {
        if let Some(end) = s[start..].find('>') {
            s = format!("{}{}", &s[..start], &s[start + end + 1..]);
        } else {
            break;
        }
    }
    s = s.replace("</svg>", "");

    // Remove lines that don't contain path data (explanatory text)
    let lines: Vec<&str> = s.lines().collect();
    if lines.len() > 1 {
        let filtered: Vec<&str> = lines
            .into_iter()
            .filter(|line| {
                let t = line.trim();
                t.is_empty()
                    || t.contains("<path")
                    || t.contains("d=\"")
                    || t.contains("d='")
                    || looks_like_path_data(t)
            })
            .collect();
        if !filtered.is_empty() {
            s = filtered.join("\n");
        }
    }

    s
}

/// Check if a string looks like SVG path data (starts with M/m and has numbers).
fn looks_like_path_data(s: &str) -> bool {
    let t = s.trim();
    if t.len() < 3 {
        return false;
    }
    let first = t.chars().next().unwrap_or(' ');
    (first == 'M' || first == 'm') && t.chars().any(|c| c.is_ascii_digit())
}

/// Extract full `<path ...>` tags, parsing both `class` and `d` attributes.
fn extract_path_tags(text: &str, paths: &mut Vec<LayeredPath>) {
    let mut search = text;
    while let Some(pos) = search.find("<path") {
        let rest = &search[pos..];
        // Find the end of this tag
        let tag_end = rest.find("/>").or_else(|| rest.find('>'));
        let tag = if let Some(end) = tag_end {
            &rest[..end + if rest[end..].starts_with("/>") { 2 } else { 1 }]
        } else {
            search = &search[pos + 5..];
            continue;
        };

        // Extract class attribute
        let layer = extract_attr_value(tag, "class")
            .map(|c| match c.trim() {
                "engrave" => PathLayer::Engrave,
                "fine" => PathLayer::Fine,
                _ => PathLayer::Cut,
            })
            .unwrap_or(PathLayer::Cut);

        // Extract d attribute
        if let Some(d_value) = extract_attr_value(tag, "d") {
            if let Some(polyline) = parse_d_attribute(d_value) {
                if polyline.len() >= 2 {
                    paths.push(LayeredPath { layer, points: polyline });
                }
            }
        }

        search = &search[pos + tag.len()..];
    }
}

/// Extract an attribute value from an HTML/SVG tag string.
/// Supports both double and single quotes.
fn extract_attr_value<'a>(tag: &'a str, attr: &str) -> Option<&'a str> {
    // Try attr="..."
    let dq_prefix = format!("{}=\"", attr);
    if let Some(pos) = tag.find(&dq_prefix) {
        let start = pos + dq_prefix.len();
        if let Some(end) = tag[start..].find('"') {
            return Some(&tag[start..start + end]);
        }
    }
    // Try attr='...'
    let sq_prefix = format!("{}='", attr);
    if let Some(pos) = tag.find(&sq_prefix) {
        let start = pos + sq_prefix.len();
        if let Some(end) = tag[start..].find('\'') {
            return Some(&tag[start..start + end]);
        }
    }
    None
}

/// Auto-detect the bounding box of all paths and remap coordinates to [0,100].
fn normalize_to_viewbox_multi(paths: &mut Vec<&mut Vec<(f32, f32)>>) {
    if paths.is_empty() {
        return;
    }

    // Find global bounding box
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;

    for path in paths.iter() {
        for &(x, y) in path.iter() {
            if x.is_finite() {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
            }
            if y.is_finite() {
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }

    // Filter out non-finite points first
    for path in paths.iter_mut() {
        path.retain(|&(x, y)| x.is_finite() && y.is_finite());
    }

    let range_x = max_x - min_x;
    let range_y = max_y - min_y;

    // If already roughly in [0,100], skip normalization
    if min_x >= -5.0 && max_x <= 105.0 && min_y >= -5.0 && max_y <= 105.0 && range_x > 1.0 && range_y > 1.0 {
        // Just clamp to [0, 100]
        for path in paths.iter_mut() {
            for pt in path.iter_mut() {
                pt.0 = pt.0.clamp(0.0, 100.0);
                pt.1 = pt.1.clamp(0.0, 100.0);
            }
        }
        return;
    }

    // Need to normalize — fit into [2, 98] with margin
    if range_x < 0.001 || range_y < 0.001 {
        return;
    }

    // Preserve aspect ratio
    let scale = 96.0 / range_x.max(range_y);
    let offset_x = 2.0 + (96.0 - range_x * scale) * 0.5;
    let offset_y = 2.0 + (96.0 - range_y * scale) * 0.5;

    for path in paths.iter_mut() {
        for pt in path.iter_mut() {
            pt.0 = ((pt.0 - min_x) * scale + offset_x).clamp(0.0, 100.0);
            pt.1 = ((pt.1 - min_y) * scale + offset_y).clamp(0.0, 100.0);
        }
    }
}

/// Parse a single SVG path `d` attribute string into a polyline.
/// Supports: M, L, C, Q, Z (absolute only for simplicity; relative handled too).
fn parse_d_attribute(d: &str) -> Option<Vec<(f32, f32)>> {
    let tokens = tokenize(d);
    if tokens.is_empty() {
        return None;
    }

    let mut pts = Vec::new();
    let mut cx = 0.0_f32;
    let mut cy = 0.0_f32;
    let mut start_x = 0.0_f32;
    let mut start_y = 0.0_f32;
    let mut i = 0;

    while i < tokens.len() {
        match tokens[i].as_str() {
            "M" => {
                if let Some((x, y, adv)) = read_pair(&tokens, i + 1) {
                    cx = x;
                    cy = y;
                    start_x = x;
                    start_y = y;
                    pts.push((cx, cy));
                    i += 1 + adv;
                    // Implicit L after M
                    while let Some((x2, y2, adv2)) = read_pair(&tokens, i) {
                        cx = x2;
                        cy = y2;
                        pts.push((cx, cy));
                        i += adv2;
                    }
                } else {
                    i += 1;
                }
            }
            "m" => {
                if let Some((dx, dy, adv)) = read_pair(&tokens, i + 1) {
                    cx += dx;
                    cy += dy;
                    start_x = cx;
                    start_y = cy;
                    pts.push((cx, cy));
                    i += 1 + adv;
                    while let Some((dx2, dy2, adv2)) = read_pair(&tokens, i) {
                        cx += dx2;
                        cy += dy2;
                        pts.push((cx, cy));
                        i += adv2;
                    }
                } else {
                    i += 1;
                }
            }
            "L" => {
                i += 1;
                while let Some((x, y, adv)) = read_pair(&tokens, i) {
                    cx = x;
                    cy = y;
                    pts.push((cx, cy));
                    i += adv;
                }
            }
            "l" => {
                i += 1;
                while let Some((dx, dy, adv)) = read_pair(&tokens, i) {
                    cx += dx;
                    cy += dy;
                    pts.push((cx, cy));
                    i += adv;
                }
            }
            "H" => {
                i += 1;
                if let Some(x) = read_num(&tokens, i) {
                    cx = x;
                    pts.push((cx, cy));
                    i += 1;
                }
            }
            "h" => {
                i += 1;
                if let Some(dx) = read_num(&tokens, i) {
                    cx += dx;
                    pts.push((cx, cy));
                    i += 1;
                }
            }
            "V" => {
                i += 1;
                if let Some(y) = read_num(&tokens, i) {
                    cy = y;
                    pts.push((cx, cy));
                    i += 1;
                }
            }
            "v" => {
                i += 1;
                if let Some(dy) = read_num(&tokens, i) {
                    cy += dy;
                    pts.push((cx, cy));
                    i += 1;
                }
            }
            "C" => {
                i += 1;
                // Cubic bezier: 3 pairs
                while let Some(((c1x, c1y), (c2x, c2y), (ex, ey), adv)) = read_cubic(&tokens, i) {
                    flatten_cubic(&mut pts, cx, cy, c1x, c1y, c2x, c2y, ex, ey, 16);
                    cx = ex;
                    cy = ey;
                    i += adv;
                }
            }
            "c" => {
                i += 1;
                while let Some(((dc1x, dc1y), (dc2x, dc2y), (dex, dey), adv)) = read_cubic(&tokens, i) {
                    let c1x = cx + dc1x;
                    let c1y = cy + dc1y;
                    let c2x = cx + dc2x;
                    let c2y = cy + dc2y;
                    let ex = cx + dex;
                    let ey = cy + dey;
                    flatten_cubic(&mut pts, cx, cy, c1x, c1y, c2x, c2y, ex, ey, 16);
                    cx = ex;
                    cy = ey;
                    i += adv;
                }
            }
            "Q" => {
                i += 1;
                while let Some(((qx, qy), (ex, ey), adv)) = read_quad(&tokens, i) {
                    flatten_quad(&mut pts, cx, cy, qx, qy, ex, ey, 12);
                    cx = ex;
                    cy = ey;
                    i += adv;
                }
            }
            "q" => {
                i += 1;
                while let Some(((dqx, dqy), (dex, dey), adv)) = read_quad(&tokens, i) {
                    let qx = cx + dqx;
                    let qy = cy + dqy;
                    let ex = cx + dex;
                    let ey = cy + dey;
                    flatten_quad(&mut pts, cx, cy, qx, qy, ex, ey, 12);
                    cx = ex;
                    cy = ey;
                    i += adv;
                }
            }
            "A" | "a" => {
                // Arc: skip for now, just move to endpoint
                let relative = tokens[i] == "a";
                i += 1;
                // Read 7 values: rx ry x-rotation large-arc sweep ex ey
                if i + 6 < tokens.len() {
                    if let (Some(ex), Some(ey)) = (read_num(&tokens, i + 5), read_num(&tokens, i + 6)) {
                        if relative {
                            cx += ex;
                            cy += ey;
                        } else {
                            cx = ex;
                            cy = ey;
                        }
                        pts.push((cx, cy));
                        i += 7;
                    } else {
                        i += 7;
                    }
                } else {
                    i += 1;
                }
            }
            "Z" | "z" => {
                // Close path
                if (cx - start_x).abs() > 0.01 || (cy - start_y).abs() > 0.01 {
                    pts.push((start_x, start_y));
                }
                cx = start_x;
                cy = start_y;
                i += 1;
            }
            _ => {
                // Unknown or number without command — try to skip
                i += 1;
            }
        }
    }

    if pts.len() >= 2 {
        Some(pts)
    } else {
        None
    }
}

// ── tokenizer ───────────────────────────────────────────────────────────

fn tokenize(d: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut buf = String::new();

    for ch in d.chars() {
        if ch.is_ascii_alphabetic() {
            if !buf.is_empty() {
                tokens.push(buf.clone());
                buf.clear();
            }
            tokens.push(ch.to_string());
        } else if ch == ',' || ch.is_ascii_whitespace() {
            if !buf.is_empty() {
                tokens.push(buf.clone());
                buf.clear();
            }
        } else if ch == '-' && !buf.is_empty() && !buf.ends_with('e') && !buf.ends_with('E') {
            // Negative sign starts new number
            tokens.push(buf.clone());
            buf.clear();
            buf.push(ch);
        } else {
            buf.push(ch);
        }
    }
    if !buf.is_empty() {
        tokens.push(buf);
    }
    tokens
}

fn read_num(tokens: &[String], idx: usize) -> Option<f32> {
    tokens.get(idx)?.parse::<f32>().ok()
}

fn read_pair(tokens: &[String], idx: usize) -> Option<(f32, f32, usize)> {
    let x = read_num(tokens, idx)?;
    let y = read_num(tokens, idx + 1)?;
    Some((x, y, 2))
}

fn read_cubic(tokens: &[String], idx: usize) -> Option<((f32, f32), (f32, f32), (f32, f32), usize)> {
    let c1x = read_num(tokens, idx)?;
    let c1y = read_num(tokens, idx + 1)?;
    let c2x = read_num(tokens, idx + 2)?;
    let c2y = read_num(tokens, idx + 3)?;
    let ex = read_num(tokens, idx + 4)?;
    let ey = read_num(tokens, idx + 5)?;
    Some(((c1x, c1y), (c2x, c2y), (ex, ey), 6))
}

fn read_quad(tokens: &[String], idx: usize) -> Option<((f32, f32), (f32, f32), usize)> {
    let qx = read_num(tokens, idx)?;
    let qy = read_num(tokens, idx + 1)?;
    let ex = read_num(tokens, idx + 2)?;
    let ey = read_num(tokens, idx + 3)?;
    Some(((qx, qy), (ex, ey), 4))
}

// ── curve flattening ────────────────────────────────────────────────────

fn flatten_cubic(
    pts: &mut Vec<(f32, f32)>,
    x0: f32, y0: f32,
    c1x: f32, c1y: f32,
    c2x: f32, c2y: f32,
    x3: f32, y3: f32,
    steps: usize,
) {
    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        let it = 1.0 - t;
        let x = it * it * it * x0 + 3.0 * it * it * t * c1x + 3.0 * it * t * t * c2x + t * t * t * x3;
        let y = it * it * it * y0 + 3.0 * it * it * t * c1y + 3.0 * it * t * t * c2y + t * t * t * y3;
        pts.push((x, y));
    }
}

fn flatten_quad(
    pts: &mut Vec<(f32, f32)>,
    x0: f32, y0: f32,
    qx: f32, qy: f32,
    x2: f32, y2: f32,
    steps: usize,
) {
    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        let it = 1.0 - t;
        let x = it * it * x0 + 2.0 * it * t * qx + t * t * x2;
        let y = it * it * y0 + 2.0 * it * t * qy + t * t * y2;
        pts.push((x, y));
    }
}

/// Scale paths from [0,100] viewBox to target mm dimensions.
pub fn scale_paths(paths: Vec<Vec<(f32, f32)>>, width_mm: f32, height_mm: f32) -> Vec<Vec<(f32, f32)>> {
    let sx = width_mm / 100.0;
    let sy = height_mm / 100.0;
    paths
        .into_iter()
        .map(|poly| poly.into_iter().map(|(x, y)| (x * sx, y * sy)).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_star() {
        let svg = r#"<path d="M50,5 L61,35 L95,35 L68,57 L79,90 L50,70 L21,90 L32,57 L5,35 L39,35 Z" />"#;
        let paths = extract_paths_from_svg(svg);
        assert_eq!(paths.len(), 1);
        assert!(paths[0].len() >= 10);
    }

    #[test]
    fn parse_multiple_paths() {
        let svg = r#"
            <path d="M10,10 L90,10 L90,90 L10,90 Z" />
            <path d="M50,20 L50,80" />
        "#;
        let paths = extract_paths_from_svg(svg);
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn parse_cubic_bezier() {
        let svg = r#"<path d="M10,80 C40,10 65,10 95,80" />"#;
        let paths = extract_paths_from_svg(svg);
        assert_eq!(paths.len(), 1);
        assert!(paths[0].len() > 2);
    }

    #[test]
    fn parse_quadratic_bezier() {
        let svg = r#"<path d="M10,80 Q52.5,10 95,80" />"#;
        let paths = extract_paths_from_svg(svg);
        assert_eq!(paths.len(), 1);
        assert!(paths[0].len() > 2);
    }

    #[test]
    fn parse_relative_commands() {
        let svg = r#"<path d="m10,10 l80,0 l0,80 l-80,0 z" />"#;
        let paths = extract_paths_from_svg(svg);
        assert_eq!(paths.len(), 1);
        assert!(paths[0].len() >= 4);
    }

    #[test]
    fn scale_to_mm() {
        let paths = vec![vec![(0.0, 0.0), (100.0, 100.0)]];
        let scaled = scale_paths(paths, 200.0, 150.0);
        assert!((scaled[0][1].0 - 200.0).abs() < 0.01);
        assert!((scaled[0][1].1 - 150.0).abs() < 0.01);
    }

    #[test]
    fn handles_garbage_gracefully() {
        let paths = extract_paths_from_svg("This is not SVG at all");
        assert!(paths.is_empty());
    }

    #[test]
    fn handles_empty() {
        let paths = extract_paths_from_svg("");
        assert!(paths.is_empty());
    }

    #[test]
    fn handles_markdown_wrapped_svg() {
        let llm_output = r#"Here is your SVG:

```svg
<path d="M50,5 L61,35 L95,35 L68,57 L79,90 L50,70 L21,90 L32,57 L5,35 L39,35 Z" />
```

This creates a star shape."#;
        let paths = extract_paths_from_svg(llm_output);
        assert_eq!(paths.len(), 1);
        assert!(paths[0].len() >= 10);
    }

    #[test]
    fn handles_svg_wrapper() {
        let llm_output = r#"<?xml version="1.0"?>
<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
  <path d="M10,10 L90,10 L90,90 L10,90 Z" />
  <path d="M50,20 L50,80" />
</svg>"#;
        let paths = extract_paths_from_svg(llm_output);
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn auto_normalizes_large_coords() {
        // Coords in 0-500 range should be normalized to 0-100
        let svg = r#"<path d="M250,25 L305,175 L475,175 L340,285 L395,450 L250,350 L105,450 L160,285 L25,175 L195,175 Z" />"#;
        let paths = extract_paths_from_svg(svg);
        assert_eq!(paths.len(), 1);
        // All points should now be in [0, 100]
        for &(x, y) in &paths[0] {
            assert!(x >= 0.0 && x <= 100.0, "x={} out of range", x);
            assert!(y >= 0.0 && y <= 100.0, "y={} out of range", y);
        }
    }

    #[test]
    fn auto_normalizes_negative_coords() {
        let svg = r#"<path d="M-50,-50 L50,-50 L50,50 L-50,50 Z" />"#;
        let paths = extract_paths_from_svg(svg);
        assert_eq!(paths.len(), 1);
        for &(x, y) in &paths[0] {
            assert!(x >= 0.0 && x <= 100.0, "x={} out of range", x);
            assert!(y >= 0.0 && y <= 100.0, "y={} out of range", y);
        }
    }

    #[test]
    fn filters_degenerate_paths() {
        // A path where all points are the same should be filtered out
        let svg = r#"<path d="M50,50 L50,50 L50,50 Z" />"#;
        let paths = extract_paths_from_svg(svg);
        assert!(paths.is_empty());
    }

    #[test]
    fn handles_raw_path_data_without_tags() {
        let raw = "M50,5 L61,35 L95,35 L68,57 L79,90 L50,70 L21,90 L32,57 L5,35 L39,35 Z";
        let paths = extract_paths_from_svg(raw);
        assert_eq!(paths.len(), 1);
    }

    #[test]
    fn extracts_layer_classes() {
        let svg = r#"
<path class="cut" d="M10,10 L90,10 L90,90 L10,90 Z" />
<path class="engrave" d="M30,30 L70,30 L70,70 L30,70 Z" />
<path class="fine" d="M45,40 L55,40 L55,60 L45,60 Z" />
<path d="M20,20 L80,20" />
"#;
        let layered = extract_layered_paths(svg);
        assert_eq!(layered.len(), 4);
        assert_eq!(layered[0].layer, PathLayer::Cut);
        assert_eq!(layered[1].layer, PathLayer::Engrave);
        assert_eq!(layered[2].layer, PathLayer::Fine);
        assert_eq!(layered[3].layer, PathLayer::Cut); // default when no class
    }

    #[test]
    fn layered_paths_preserve_all_layers() {
        let svg = r#"
<path class="cut" d="M50,5 C20,5 5,30 5,55 C5,80 25,95 50,95 C75,95 95,80 95,55 C95,30 80,5 50,5 Z" />
<path class="engrave" d="M30,45 C30,35 45,35 45,45 C45,55 30,55 30,45 Z" />
<path class="engrave" d="M55,45 C55,35 70,35 70,45 C70,55 55,55 55,45 Z" />
<path class="fine" d="M25,50 L25,70 M30,55 L28,72" />
"#;
        let layered = extract_layered_paths(svg);
        let n_cut = layered.iter().filter(|lp| lp.layer == PathLayer::Cut).count();
        let n_eng = layered.iter().filter(|lp| lp.layer == PathLayer::Engrave).count();
        let n_fine = layered.iter().filter(|lp| lp.layer == PathLayer::Fine).count();
        assert_eq!(n_cut, 1);
        assert_eq!(n_eng, 2);
        assert!(n_fine >= 1);
    }

    #[test]
    fn all_points_clamped_to_viewbox() {
        let svg = r#"<path d="M5,5 L95,5 L95,95 L5,95 Z" />"#;
        let paths = extract_paths_from_svg(svg);
        for path in &paths {
            for &(x, y) in path {
                assert!(x >= 0.0 && x <= 100.0);
                assert!(y >= 0.0 && y <= 100.0);
            }
        }
    }
}
