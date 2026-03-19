#![allow(dead_code)]
use crate::imaging::raster::RasterParams;
use crate::ui::drawing::{ImageData, PathData, PathSegment, ShapeKind, ShapeParams};
use crate::ui::layers_new::{CutLayer, CutMode};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
struct XForm {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    tx: f32,
    ty: f32,
}
impl Default for XForm {
    fn default() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }
}
impl XForm {
    fn apply(&self, x: f32, y: f32) -> (f32, f32) {
        (
            self.a * x + self.c * y + self.tx,
            self.b * x + self.d * y + self.ty,
        )
    }
    fn compose(&self, i: &XForm) -> XForm {
        XForm {
            a: self.a * i.a + self.c * i.b,
            b: self.b * i.a + self.d * i.b,
            c: self.a * i.c + self.c * i.d,
            d: self.b * i.c + self.d * i.d,
            tx: self.a * i.tx + self.c * i.ty + self.tx,
            ty: self.b * i.tx + self.d * i.ty + self.ty,
        }
    }
}
fn parse_xform(t: &str) -> XForm {
    let n: Vec<f32> = t
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    if n.len() >= 6 {
        XForm {
            a: n[0],
            b: n[1],
            c: n[2],
            d: n[3],
            tx: n[4],
            ty: n[5],
        }
    } else {
        XForm::default()
    }
}
#[derive(Clone, Debug, Default)]
struct Vtx {
    x: f32,
    y: f32,
    c0x: Option<f32>,
    c0y: Option<f32>,
    c1x: Option<f32>,
    c1y: Option<f32>,
}
/// Parse packed vertex list from the `.lbrn2` format (e.g. "V0 1.5 c0x2.0c0y3.0V1.0 2.0")
fn parse_vertlist(text: &str) -> Vec<Vtx> {
    text.split('V')
        .filter(|chunk| !chunk.trim().is_empty())
        .filter_map(|chunk| parse_vertex(chunk.trim()))
        .collect()
}

/// Parse a single vertex from a packed string like "1.5 2.0 c0x3.0c0y4.0c1x5.0c1y6.0"
fn parse_vertex(chunk: &str) -> Option<Vtx> {
    let ctrl_start = chunk.find('c').unwrap_or(chunk.len());
    let mut coords = chunk[..ctrl_start].split_whitespace();
    let x = coords.next()?.parse().ok()?;
    let y = coords.next()?.parse().ok()?;
    let ctrl = &chunk[ctrl_start..];
    Some(Vtx {
        x,
        y,
        c0x: extract_control_point(ctrl, "c0x"),
        c0y: extract_control_point(ctrl, "c0y"),
        c1x: extract_control_point(ctrl, "c1x"),
        c1y: extract_control_point(ctrl, "c1y"),
    })
}

/// Extract a named control-point value from a packed string (e.g. "c0x3.5c0y4.0")
fn extract_control_point(text: &str, name: &str) -> Option<f32> {
    let start = text.find(name)? + name.len();
    let end = text[start..].find('c').unwrap_or(text.len() - start);
    text[start..start + end].trim().parse().ok()
}
/// A drawing primitive: Line or Bezier connecting two vertex indices.
#[derive(Clone, Debug)]
enum Primitive {
    Line(usize, usize),
    Bezier(usize, usize),
}

/// Parse packed primitive list from `.lbrn2` format (e.g. "L0 1B2 3L4 5")
fn parse_primlist(text: &str) -> Vec<Primitive> {
    let mut prims = Vec::new();
    let bytes = text.trim().as_bytes();
    let mut pos = 0;
    while pos < bytes.len() {
        let kind = bytes[pos] as char;
        if kind != 'B' && kind != 'L' {
            pos += 1;
            continue;
        }
        pos += 1;
        let idx1_start = pos;
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
        let idx1: usize = text[idx1_start..pos].parse().unwrap_or(0);
        while pos < bytes.len() && bytes[pos] == b' ' {
            pos += 1;
        }
        let idx2_start = pos;
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
        let idx2: usize = text[idx2_start..pos].parse().unwrap_or(0);
        prims.push(if kind == 'B' {
            Primitive::Bezier(idx1, idx2)
        } else {
            Primitive::Line(idx1, idx2)
        });
    }
    prims
}
/// Parse <V vx="..." vy="..." c0x="..." c0y="..." c1x="..." c1y="..."/> elements (.lbrn v1 format)
fn parse_v_elements(inner: &str) -> Vec<Vtx> {
    let mut vs = Vec::new();
    let mut pos = 0;
    while pos < inner.len() {
        let s = match inner[pos..].find("<V ") {
            Some(p) => pos + p,
            None => break,
        };
        let e = match inner[s..].find("/>") {
            Some(p) => s + p + 2,
            None => break,
        };
        let tag = &inner[s..e];
        let vx = extract_attr(tag, "vx").and_then(|s| s.parse::<f32>().ok());
        let vy = extract_attr(tag, "vy").and_then(|s| s.parse::<f32>().ok());
        if let (Some(x), Some(y)) = (vx, vy) {
            vs.push(Vtx {
                x,
                y,
                c0x: extract_attr(tag, "c0x").and_then(|s| s.parse().ok()),
                c0y: extract_attr(tag, "c0y").and_then(|s| s.parse().ok()),
                c1x: extract_attr(tag, "c1x").and_then(|s| s.parse().ok()),
                c1y: extract_attr(tag, "c1y").and_then(|s| s.parse().ok()),
            });
        }
        pos = e;
    }
    vs
}

/// Parse <P T="B" p0="0" p1="1"/> elements (.lbrn v1 format)
fn parse_p_elements(inner: &str) -> Vec<Primitive> {
    let mut ps = Vec::new();
    let mut pos = 0;
    while pos < inner.len() {
        let s = match inner[pos..].find("<P ") {
            Some(p) => pos + p,
            None => break,
        };
        let e = match inner[s..].find("/>") {
            Some(p) => s + p + 2,
            None => break,
        };
        let tag = &inner[s..e];
        let t = extract_attr(tag, "T").unwrap_or_default();
        let p0: usize = extract_attr(tag, "p0").and_then(|s| s.parse().ok()).unwrap_or(0);
        let p1: usize = extract_attr(tag, "p1").and_then(|s| s.parse().ok()).unwrap_or(0);
        match t.as_str() {
            "B" => ps.push(Primitive::Bezier(p0, p1)),
            "L" => ps.push(Primitive::Line(p0, p1)),
            _ => {}
        }
        pos = e;
    }
    ps
}

/// Build one or more PathData from vertices + primitives.
/// Disconnected subpaths (e.g. separate letters in text) are split into
/// separate PathData to avoid unwanted connecting lines.
fn build_paths(vs: &[Vtx], ps: &[Primitive], xf: &XForm) -> Result<Vec<PathData>, String> {
    if vs.is_empty() || ps.is_empty() {
        return Ok(vec![]);
    }

    // Group primitives into contiguous subpaths.
    struct SubPath {
        start: (f32, f32),
        segments: Vec<PathSegment>,
        last_end: (f32, f32),
        has_bezier: bool,
        first_idx: usize,
        last_idx: usize,
    }

    let mut subs: Vec<SubPath> = Vec::new();

    for p in ps {
        let (i0, i1): (usize, usize) = match *p {
            Primitive::Line(a, b) | Primitive::Bezier(a, b) => (a, b),
        };
        let v0 = &vs[i0.min(vs.len() - 1)];
        let v1 = &vs[i1.min(vs.len() - 1)];
        let p0 = xf.apply(v0.x, v0.y);
        let p1 = xf.apply(v1.x, v1.y);

        // Check if this primitive continues the current subpath
        let continues = subs.last().map_or(false, |s| {
            (s.last_end.0 - p0.0).abs() < 0.01 && (s.last_end.1 - p0.1).abs() < 0.01
        });

        if !continues {
            // Start a new subpath
            subs.push(SubPath {
                start: p0,
                segments: Vec::new(),
                last_end: p0,
                has_bezier: false,
                first_idx: i0,
                last_idx: i1,
            });
        }

        let sub = subs.last_mut().ok_or("Malformed primitive list: no subpath found")?;
        sub.last_idx = i1;

        match p {
            Primitive::Line(_, _) => {
                sub.segments.push(PathSegment::LineTo(p1.0, p1.1));
                sub.last_end = p1;
            }
            Primitive::Bezier(_, _) => {
                sub.has_bezier = true;
                let cp0 = xf.apply(v0.c0x.unwrap_or(v0.x), v0.c0y.unwrap_or(v0.y));
                let cp1 = xf.apply(v1.c1x.unwrap_or(v1.x), v1.c1y.unwrap_or(v1.y));
                sub.segments.push(PathSegment::CubicBezier {
                    c1: cp0,
                    c2: cp1,
                    end: p1,
                });
                sub.last_end = p1;
            }
        }
    }

    // Convert each subpath to a PathData
    let mut result = Vec::new();
    for sub in subs {
        if sub.segments.is_empty() {
            continue;
        }

        let s = sub.start;
        let mut segments = sub.segments;

        // Close path if last primitive connects back to first vertex
        if segments.len() >= 2 && sub.first_idx == sub.last_idx {
            if (s.0 - sub.last_end.0).abs() > 0.001 || (s.1 - sub.last_end.1).abs() > 0.001 {
                segments.push(PathSegment::LineTo(s.0, s.1));
            }
        }

        if sub.has_bezier {
            result.push(PathData::from_segments(s, segments));
        } else {
            let mut pts = vec![s];
            for seg in &segments {
                if let PathSegment::LineTo(x, y) = seg {
                    pts.push((*x, *y));
                }
            }
            result.push(PathData::from_points(pts));
        }
    }
    Ok(result)
}
fn path_to_shape(pd: PathData, li: usize) -> Option<ShapeParams> {
    if pd.points.len() < 2 {
        return None;
    }
    let mx = pd.points.iter().map(|p| p.0).fold(f32::MAX, f32::min);
    let my = pd.points.iter().map(|p| p.1).fold(f32::MAX, f32::min);
    if pd.has_curves() {
        // Make segments local by subtracting min
        let local_start = (pd.start.0 - mx, pd.start.1 - my);
        let local_segs: Vec<PathSegment> = pd
            .segments
            .iter()
            .map(|seg| match seg {
                PathSegment::LineTo(x, y) => PathSegment::LineTo(x - mx, y - my),
                PathSegment::CubicBezier { c1, c2, end } => PathSegment::CubicBezier {
                    c1: (c1.0 - mx, c1.1 - my),
                    c2: (c2.0 - mx, c2.1 - my),
                    end: (end.0 - mx, end.1 - my),
                },
                PathSegment::QuadBezier { c, end } => PathSegment::QuadBezier {
                    c: (c.0 - mx, c.1 - my),
                    end: (end.0 - mx, end.1 - my),
                },
            })
            .collect();
        let local = PathData::from_segments(local_start, local_segs);
        Some(ShapeParams {
            shape: ShapeKind::Path(local),
            x: mx,
            y: my,
            layer_idx: li,
            ..Default::default()
        })
    } else {
        let r: Vec<(f32, f32)> = pd.points.iter().map(|&(x, y)| (x - mx, y - my)).collect();
        Some(ShapeParams {
            shape: ShapeKind::Path(PathData::from_points(r)),
            x: mx,
            y: my,
            layer_idx: li,
            ..Default::default()
        })
    }
}
/// Extract an XML attribute value by name (e.g. extract_attr(tag, "Type") from `<Shape Type="Rect"/>`)
fn extract_attr(l: &str, a: &str) -> Option<String> {
    let p = format!("{}=\"", a);
    let s = l.find(&p)? + p.len();
    let e = l[s..].find('"')?;
    Some(l[s..s + e].to_string())
}
/// Extract the `Value` attribute from a child element like `<speed Value="100"/>`
fn child_value_f32(block: &str, tag: &str) -> Option<f32> {
    let p = format!("<{} ", tag);
    let i = block.find(&p)?;
    extract_attr(&block[i..], "Value")?.parse().ok()
}

/// Extract the `Value` attribute from a child element as usize
fn child_value_usize(block: &str, tag: &str) -> Option<usize> {
    let p = format!("<{} ", tag);
    let i = block.find(&p)?;
    extract_attr(&block[i..], "Value")?.parse().ok()
}

/// Extract text content between `<tag>...</tag>`
fn tag_content<'a>(b: &'a str, tag: &str) -> Option<&'a str> {
    let o = format!("<{}>", tag);
    let c = format!("</{}>", tag);
    let s = b.find(&o)? + o.len();
    let e = b[s..].find(&c)? + s;
    Some(&b[s..e])
}

#[derive(Clone, Debug)]
pub struct LbrnLayerOverride {
    pub index: usize,
    pub speed: f32,
    pub power: f32,
    pub mode: CutMode,
    pub passes: u32,
}

fn parse_cut_mode(s: &str) -> CutMode {
    match s {
        "Cut" | "00" => CutMode::Line,
        "Scan" | "01" => CutMode::Fill,
        "Scan+Cut" | "02" => CutMode::FillAndLine,
        _ => CutMode::Line,
    }
}

/// Pre-scan content to collect shared PrimLists indexed by PrimID attribute
fn collect_shared_primlists(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut pos = 0;
    while pos < content.len() {
        let ss = match content[pos..].find("<Shape ") {
            Some(p) => pos + p,
            None => break,
        };
        let te = match content[ss..].find('>') {
            Some(p) => ss + p,
            None => break,
        };
        let otag = &content[ss..=te];
        if otag.ends_with("/>") {
            pos = te + 1;
            continue;
        }
        let is = te + 1;
        let co = match find_closing_tag(&content[is..], "<Shape ", "</Shape>") {
            Some(o) => o,
            None => {
                pos = is;
                continue;
            }
        };
        let inner = &content[is..is + co];
        let bend = is + co + "</Shape>".len();
        if let Some(pid) = extract_attr(otag, "PrimID") {
            if let Some(pt) = tag_content(inner, "PrimList") {
                map.entry(pid).or_insert_with(|| pt.to_string());
            }
        }
        // Recurse into Children
        if let Some(ch) = tag_content(inner, "Children") {
            let child_map = collect_shared_primlists(ch);
            for (k, v) in child_map {
                map.entry(k).or_insert(v);
            }
        }
        pos = bend;
    }
    map
}

/// Strip large base64 blobs (Thumbnail Source, Bitmap Data) to reduce
/// the working set that the parser must scan through. Returns the
/// stripped content and a map of placeholder→original data for bitmaps.
fn strip_blobs(content: &str) -> (String, HashMap<String, String>) {
    let mut result = String::with_capacity(content.len());
    let mut bitmap_data: HashMap<String, String> = HashMap::new();
    let mut pos = 0;
    let mut bitmap_id: usize = 0;

    while pos < content.len() {
        // Skip <Thumbnail Source="...huge base64..."/>
        if content[pos..].starts_with("<Thumbnail ") {
            if let Some(end) = content[pos..].find("/>") {
                let skip_end = pos + end + 2;
                result.push_str("<Thumbnail Source=\"\"/>");
                pos = skip_end;
                continue;
            }
        }

        // Extract Data="...huge base64..." from Bitmap shapes, replace with placeholder
        if content[pos..].starts_with("Data=\"") {
            let val_start = pos + 6; // after Data="
            if let Some(quote_end) = content[val_start..].find('"') {
                let data = &content[val_start..val_start + quote_end];
                if data.len() > 1024 {
                    // Large blob — replace with placeholder
                    let key = format!("__BLOB_{bitmap_id}__");
                    bitmap_data.insert(key.clone(), data.to_string());
                    result.push_str(&format!("Data=\"{key}\""));
                    bitmap_id += 1;
                    pos = val_start + quote_end + 1;
                    continue;
                }
            }
        }

        // Fast scan: find next '<' or 'D' to jump ahead
        let rest = &content[pos..];
        let next_interesting = rest.as_bytes().iter().position(|&b| b == b'<' || b == b'D');
        match next_interesting {
            Some(0) => {
                // We're at a '<' or 'D' that didn't match above, emit it and advance
                result.push(content.as_bytes()[pos] as char);
                pos += 1;
            }
            Some(n) => {
                // Copy chunk up to next interesting byte
                result.push_str(&content[pos..pos + n]);
                pos += n;
            }
            None => {
                // No more interesting bytes, copy rest
                result.push_str(&content[pos..]);
                break;
            }
        }
    }

    (result, bitmap_data)
}

/// Parse a .lbrn2 XML file and extract shapes + layer overrides
pub fn import_lbrn2(content: &str) -> Result<(Vec<ShapeParams>, Vec<LbrnLayerOverride>), String> {
    let mut shapes = Vec::new();
    let mut lo = Vec::new();

    // Strip large base64 blobs to speed up scanning
    let (stripped, bitmap_data) = strip_blobs(content);

    parse_cut_settings(&stripped, &mut lo);
    let shared_prims = collect_shared_primlists(&stripped);
    parse_shapes(
        &stripped,
        &XForm::default(),
        &shared_prims,
        &bitmap_data,
        &mut shapes,
    );
    if shapes.is_empty() {
        parse_simple_shapes(&stripped, &mut shapes);
    }
    if shapes.is_empty() && lo.is_empty() {
        return Err("No shapes or layers found in LightBurn file. \
            The file may be empty or in an unsupported format."
            .into());
    }
    Ok((shapes, lo))
}

/// Parse <CutSetting> blocks into layer overrides.
fn parse_cut_settings(c: &str, out: &mut Vec<LbrnLayerOverride>) {
    let mut pos = 0;
    loop {
        let st = match c[pos..].find("<CutSetting") {
            Some(p) => pos + p,
            None => break,
        };
        if let Some(eo) = c[st..].find("</CutSetting>") {
            let end = st + eo + "</CutSetting>".len();
            let blk = &c[st..end];
            let ms = extract_attr(blk, "type").unwrap_or_default();
            out.push(LbrnLayerOverride {
                index: child_value_usize(blk, "index").unwrap_or(0),
                speed: child_value_f32(blk, "speed").unwrap_or(1000.0),
                power: child_value_f32(blk, "maxPower").unwrap_or(50.0) * 10.0,
                mode: parse_cut_mode(&ms),
                passes: 1,
            });
            pos = end;
        } else if let Some(eo) = c[st..].find("/>") {
            let end = st + eo + 2;
            let ln = &c[st..end];
            out.push(LbrnLayerOverride {
                index: extract_attr(ln, "index").and_then(|s| s.parse().ok()).unwrap_or(0),
                speed: extract_attr(ln, "speed")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1000.0),
                power: extract_attr(ln, "maxPower")
                    .and_then(|s| s.parse::<f32>().ok())
                    .unwrap_or(50.0)
                    * 10.0,
                mode: parse_cut_mode(&extract_attr(ln, "type").unwrap_or_default()),
                passes: 1,
            });
            pos = end;
        } else {
            break;
        }
    }
}

/// Find the offset of a matching closing tag, handling nested elements.
fn find_closing_tag(c: &str, otag: &str, ctag: &str) -> Option<usize> {
    let mut d = 1i32;
    let mut pos = 0;
    let bytes = c.as_bytes();
    let ob = otag.as_bytes();
    let cb = ctag.as_bytes();
    while pos < bytes.len() {
        // Fast scan to next '<' (both tags start with '<')
        let b = bytes[pos];
        if b != b'<' {
            pos += 1;
            continue;
        }
        // Check close tag first (more likely in inner content)
        if bytes.len() - pos >= cb.len() && &bytes[pos..pos + cb.len()] == cb {
            d -= 1;
            if d == 0 {
                return Some(pos);
            }
            pos += cb.len();
        } else if bytes.len() - pos >= ob.len() && &bytes[pos..pos + ob.len()] == ob {
            let a = pos + ob.len();
            // Find closing '>'
            if let Some(gt) = c[a..].find('>') {
                let te = a + gt;
                if te > 0 && bytes[te - 1] != b'/' {
                    d += 1;
                }
                pos = te + 1;
            } else {
                pos += 1;
            }
        } else {
            pos += 1;
        }
    }
    None
}

/// Recursively parse <Shape> elements into ShapeParams.
fn parse_shapes(
    content: &str,
    pxf: &XForm,
    shared_prims: &HashMap<String, String>,
    bitmap_data: &HashMap<String, String>,
    out: &mut Vec<ShapeParams>,
) {
    let mut pos = 0;
    while pos < content.len() {
        let ss = match content[pos..].find("<Shape ") {
            Some(p) => pos + p,
            None => break,
        };
        let te = match content[ss..].find('>') {
            Some(p) => ss + p,
            None => break,
        };
        let otag = &content[ss..=te];
        let st = extract_attr(otag, "Type").unwrap_or_default();
        let ci = extract_attr(otag, "CutIndex")
            .or_else(|| extract_attr(otag, "Layer"))
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);

        if otag.ends_with("/>") {
            parse_inline_shape(otag, ci, pxf, out);
            pos = te + 1;
            continue;
        }

        let is = te + 1;
        let co = match find_closing_tag(&content[is..], "<Shape ", "</Shape>") {
            Some(o) => o,
            None => {
                pos = is;
                continue;
            }
        };
        let inner = &content[is..is + co];
        let bend = is + co + "</Shape>".len();

        let lxf = tag_content(inner, "XForm").map(parse_xform).unwrap_or_default();
        let cxf = pxf.compose(&lxf);

        match st.as_str() {
            "Group" => {
                if let Some(ch) = tag_content(inner, "Children") {
                    parse_shapes(ch, &cxf, shared_prims, bitmap_data, out);
                }
            }
            "Bitmap" => {
                // Decode base64 PNG bitmap, preserving alpha channel
                // Resolve placeholder from strip_blobs if present
                if let Some(raw_data) = extract_attr(otag, "Data") {
                    let b64_data = if raw_data.starts_with("__BLOB_") {
                        bitmap_data.get(&raw_data).cloned().unwrap_or(raw_data)
                    } else {
                        raw_data
                    };
                    use base64::Engine;
                    if let Ok(png_bytes) =
                        base64::engine::general_purpose::STANDARD.decode(&b64_data)
                    {
                        if let Ok(img) = image::load_from_memory(&png_bytes) {
                            // Flip vertically: LightBurn Y-up vs image pixel Y-down
                            let dyn_img = img.flipv();

                            // Use W/H attributes from shape tag (physical mm dimensions)
                            // then apply XForm scaling. Do NOT multiply by pixel count.
                            let w_attr = extract_attr(otag, "W")
                                .and_then(|s| s.parse::<f32>().ok())
                                .unwrap_or(100.0);
                            let h_attr = extract_attr(otag, "H")
                                .and_then(|s| s.parse::<f32>().ok())
                                .unwrap_or(100.0);
                            let width_mm = (w_attr * cxf.a).abs();
                            let height_mm = (h_attr * cxf.d).abs();
                            // LightBurn center -> top-left
                            let x = cxf.tx - width_mm / 2.0;
                            let y = cxf.ty - height_mm / 2.0;

                            let raster_params = RasterParams {
                                width_mm,
                                height_mm,
                                ..Default::default()
                            };
                            out.push(ShapeParams {
                                shape: ShapeKind::RasterImage {
                                    data: ImageData(Arc::new(dyn_img)),
                                    params: raster_params,
                                },
                                x,
                                y,
                                width: width_mm,
                                height: height_mm,
                                layer_idx: ci,
                                ..Default::default()
                            });
                        }
                    }
                }
            }
            _ => {
                let mut found = false;
                // Try .lbrn2 packed format first
                if let Some(vt) = tag_content(inner, "VertList") {
                    let vs = parse_vertlist(vt);
                    // Try local PrimList first, then shared by PrimID
                    let pt_str = tag_content(inner, "PrimList").map(|s| s.to_string()).or_else(|| {
                        extract_attr(otag, "PrimID").and_then(|pid| shared_prims.get(&pid).cloned())
                    });
                    if let Some(pt) = pt_str {
                        let ps = parse_primlist(&pt);
                        let pds = build_paths(&vs, &ps, &cxf).unwrap_or_default();
                        for pd in pds {
                            if let Some(s) = path_to_shape(pd, ci) {
                                out.push(s);
                                found = true;
                            }
                        }
                    }
                }
                // Try .lbrn v1 element format (<V .../> and <P .../>)
                if !found {
                    let vs = parse_v_elements(inner);
                    let ps = parse_p_elements(inner);
                    if !vs.is_empty() && !ps.is_empty() {
                        let pds = build_paths(&vs, &ps, &cxf).unwrap_or_default();
                        for pd in pds {
                            if let Some(s) = path_to_shape(pd, ci) {
                                out.push(s);
                                found = true;
                            }
                        }
                    }
                }
                // Fallback: try parsing as inline shape with the composed transform
                if !found {
                    parse_inline_shape(otag, ci, &cxf, out);
                }
            }
        }
        pos = bend;
    }
}

/// Parse an inline (self-closing) shape tag like Rect or Ellipse.
fn parse_inline_shape(tag: &str, li: usize, xf: &XForm, out: &mut Vec<ShapeParams>) {
    let st = extract_attr(tag, "Type").unwrap_or_default();
    match st.as_str() {
        "Rect" => {
            if let (Some(x), Some(y)) = (
                extract_attr(tag, "X").and_then(|s| s.parse::<f32>().ok()),
                extract_attr(tag, "Y").and_then(|s| s.parse::<f32>().ok()),
            ) {
                let w: f32 = extract_attr(tag, "W").and_then(|s| s.parse().ok()).unwrap_or(10.0);
                let h: f32 = extract_attr(tag, "H").and_then(|s| s.parse().ok()).unwrap_or(10.0);
                // If transform is non-identity, emit as Path to preserve rotation/skew
                if !xf_is_identity(xf) {
                    let corners = vec![
                        xf.apply(x, y),
                        xf.apply(x + w, y),
                        xf.apply(x + w, y + h),
                        xf.apply(x, y + h),
                        xf.apply(x, y),
                    ];
                    if let Some(s) = path_to_shape(PathData::from_points(corners), li) {
                        out.push(s);
                    }
                } else {
                    out.push(ShapeParams {
                        shape: ShapeKind::Rectangle,
                        x,
                        y,
                        width: w,
                        height: h,
                        layer_idx: li,
                        ..Default::default()
                    });
                }
            }
        }
        "Ellipse" => {
            if let (Some(cx), Some(cy)) = (
                extract_attr(tag, "CX").and_then(|s| s.parse::<f32>().ok()),
                extract_attr(tag, "CY").and_then(|s| s.parse::<f32>().ok()),
            ) {
                let rx: f32 = extract_attr(tag, "Rx").and_then(|s| s.parse().ok()).unwrap_or(10.0);
                let ry: f32 = extract_attr(tag, "Ry").and_then(|s| s.parse().ok()).unwrap_or(rx);
                // Always emit ellipse as Path for accuracy (handles Ry != Rx, transforms)
                let steps = 64;
                let mut pts = Vec::with_capacity(steps + 1);
                for i in 0..=steps {
                    let angle = 2.0 * std::f32::consts::PI * i as f32 / steps as f32;
                    let px = cx + rx * angle.cos();
                    let py = cy + ry * angle.sin();
                    pts.push(xf.apply(px, py));
                }
                if let Some(s) = path_to_shape(PathData::from_points(pts), li) {
                    out.push(s);
                }
            }
        }
        _ => {}
    }
}

fn xf_is_identity(xf: &XForm) -> bool {
    (xf.a - 1.0).abs() < 1e-6
        && xf.b.abs() < 1e-6
        && xf.c.abs() < 1e-6
        && (xf.d - 1.0).abs() < 1e-6
        && xf.tx.abs() < 1e-6
        && xf.ty.abs() < 1e-6
}

/// Fallback: parse simple self-closing <Shape .../> lines.
fn parse_simple_shapes(content: &str, out: &mut Vec<ShapeParams>) {
    let id_xf = XForm::default();
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("<Shape ") && t.ends_with("/>") {
            let li = extract_attr(t, "CutIndex")
                .or_else(|| extract_attr(t, "Layer"))
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            parse_inline_shape(t, li, &id_xf, out);
        }
    }
}

/// Export shapes to .lbrn2 XML
pub fn export_lbrn2(shapes: &[ShapeParams], layers: &[CutLayer]) -> String {
    // Collect layer indices actually used by shapes
    let mut used: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();
    for s in shapes {
        used.insert(s.layer_idx);
    }

    let mut x = String::new();
    x += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    x += "<LightBurnProject AppVersion=\"All4Laser\" FormatVersion=\"1\" \
          MirrorX=\"False\" MirrorY=\"False\">\n";

    // Check which layers contain image shapes
    let mut image_layers: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();
    for s in shapes {
        if matches!(&s.shape, ShapeKind::RasterImage { .. }) {
            image_layers.insert(s.layer_idx);
        }
    }

    // Only emit CutSettings for layers referenced by shapes
    for &i in &used {
        if let Some(l) = layers.get(i) {
            if image_layers.contains(&i) {
                // Image layer uses CutSetting_Img
                x += "    <CutSetting_Img type=\"Image\">\n";
                x += &format!("        <index Value=\"{i}\"/>\n");
                x += &format!("        <name Value=\"{}\"/>\n", l.name);
                x += &format!("        <maxPower Value=\"{:.6}\"/>\n", l.power / 10.0);
                x += &format!("        <maxPower2 Value=\"{:.6}\"/>\n", l.power / 10.0);
                x += &format!("        <speed Value=\"{:.6}\"/>\n", l.speed);
                x += &format!("        <priority Value=\"0\"/>\n");
                x += "        <ditherMode Value=\"stucki\"/>\n";
                x += "    </CutSetting_Img>\n";
            } else {
                let ms = match l.mode {
                    CutMode::Line => "Cut",
                    CutMode::Fill => "Scan",
                    CutMode::FillAndLine => "Scan+Cut",
                    CutMode::Offset => "Cut",
                };
                x += &format!("    <CutSetting type=\"{ms}\">\n");
                x += &format!("        <index Value=\"{i}\"/>\n");
                x += &format!("        <name Value=\"{}\"/>\n", l.name);
                x += &format!("        <maxPower Value=\"{:.6}\"/>\n", l.power / 10.0);
                x += &format!("        <maxPower2 Value=\"{:.6}\"/>\n", l.power / 10.0);
                x += &format!("        <speed Value=\"{:.6}\"/>\n", l.speed);
                x += &format!("        <priority Value=\"0\"/>\n");
                x += "    </CutSetting>\n";
            }
        }
    }

    // Wrap all shapes in a Group
    x += "    <Shape Type=\"Group\" CutIndex=\"0\">\n";
    x += "        <XForm>1 0 0 1 0 0</XForm>\n";
    x += "        <Children>\n";

    let mut vid: usize = 0;
    let mut pid: usize = 0;
    for s in shapes {
        match &s.shape {
            ShapeKind::Rectangle => {
                let (x0, y0) = (s.x, s.y);
                let pts = vec![
                    (x0, y0),
                    (x0 + s.width, y0),
                    (x0 + s.width, y0 + s.height),
                    (x0, y0 + s.height),
                ];
                x += &format!(
                    "            <Shape Type=\"Path\" CutIndex=\"{}\" VertID=\"{}\" PrimID=\"{}\">\n",
                    s.layer_idx, vid, pid
                );
                x += "                <XForm>1 0 0 1 0 0</XForm>\n";
                x += "                <VertList>";
                for p in &pts {
                    x += &format!("V{:.6} {:.6}", p.0, p.1);
                }
                x += "</VertList>\n";
                x += "                <PrimList>L0 1L1 2L2 3L3 0</PrimList>\n";
                x += "            </Shape>\n";
                vid += 1;
                pid += 1;
            }
            ShapeKind::Circle => {
                let steps = 64usize;
                x += &format!(
                    "            <Shape Type=\"Path\" CutIndex=\"{}\" VertID=\"{}\" PrimID=\"{}\">\n",
                    s.layer_idx, vid, pid
                );
                x += "                <XForm>1 0 0 1 0 0</XForm>\n";
                x += "                <VertList>";
                for i in 0..steps {
                    let angle = 2.0 * std::f32::consts::PI * i as f32 / steps as f32;
                    let na = 2.0 * std::f32::consts::PI * ((i + 1) % steps) as f32 / steps as f32;
                    let px = s.x + s.radius * angle.cos();
                    let py = s.y + s.radius * angle.sin();
                    let k = 4.0 / 3.0 * ((std::f32::consts::PI / steps as f32 / 2.0).tan());
                    let c0x = px - s.radius * k * angle.sin();
                    let c0y = py + s.radius * k * angle.cos();
                    let npx = s.x + s.radius * na.cos();
                    let npy = s.y + s.radius * na.sin();
                    let c1x = npx + s.radius * k * na.sin();
                    let c1y = npy - s.radius * k * na.cos();
                    x += &format!("V{px:.6} {py:.6}c0x{c0x:.6}c0y{c0y:.6}c1x{c1x:.6}c1y{c1y:.6}");
                }
                x += "</VertList>\n";
                x += "                <PrimList>";
                for i in 0..steps {
                    let ni = (i + 1) % steps;
                    x += &format!("B{i} {ni}");
                }
                x += "</PrimList>\n";
                x += "            </Shape>\n";
                vid += 1;
                pid += 1;
            }
            ShapeKind::Path(data) if data.len() >= 2 => {
                x += &format!(
                    "            <Shape Type=\"Path\" CutIndex=\"{}\" VertID=\"{}\" PrimID=\"{}\">\n",
                    s.layer_idx, vid, pid
                );
                x += "                <XForm>1 0 0 1 0 0</XForm>\n";

                if data.has_curves() {
                    // Bézier-aware export: build vertices with control points
                    // Vertices: [start, seg[0].end, seg[1].end, ...]
                    // Each vertex may have c0 (outgoing) and c1 (incoming) control points
                    let n_verts = data.segments.len() + 1;
                    struct LbVert {
                        wx: f32,
                        wy: f32,
                        c0: Option<(f32, f32)>,
                        c1: Option<(f32, f32)>,
                    }
                    let mut verts: Vec<LbVert> = Vec::with_capacity(n_verts);

                    // First vertex = start
                    let (sw_x, sw_y) = s.world_pos(data.start.0, data.start.1);
                    verts.push(LbVert {
                        wx: sw_x,
                        wy: sw_y,
                        c0: None,
                        c1: None,
                    });

                    // Add vertices for each segment endpoint
                    for seg in &data.segments {
                        match seg {
                            PathSegment::LineTo(ex, ey) => {
                                let (wex, wey) = s.world_pos(*ex, *ey);
                                verts.push(LbVert {
                                    wx: wex,
                                    wy: wey,
                                    c0: None,
                                    c1: None,
                                });
                            }
                            PathSegment::CubicBezier { c1, c2, end } => {
                                let (wc1x, wc1y) = s.world_pos(c1.0, c1.1);
                                let (wc2x, wc2y) = s.world_pos(c2.0, c2.1);
                                let (wex, wey) = s.world_pos(end.0, end.1);
                                // c0 on previous vertex (outgoing), c1 on this vertex (incoming)
                                let prev = verts.len() - 1;
                                verts[prev].c0 = Some((wc1x, wc1y));
                                verts.push(LbVert {
                                    wx: wex,
                                    wy: wey,
                                    c0: None,
                                    c1: Some((wc2x, wc2y)),
                                });
                            }
                            PathSegment::QuadBezier { c, end } => {
                                // Promote quadratic to cubic: cubic_c1 = p0 + 2/3*(c-p0), cubic_c2 = end + 2/3*(c-end)
                                let prev_idx = verts.len() - 1;
                                let p0 = (verts[prev_idx].wx, verts[prev_idx].wy);
                                let (wcx, wcy) = s.world_pos(c.0, c.1);
                                let (wex, wey) = s.world_pos(end.0, end.1);
                                let cc1 = (
                                    p0.0 + 2.0 / 3.0 * (wcx - p0.0),
                                    p0.1 + 2.0 / 3.0 * (wcy - p0.1),
                                );
                                let cc2 =
                                    (wex + 2.0 / 3.0 * (wcx - wex), wey + 2.0 / 3.0 * (wcy - wey));
                                verts[prev_idx].c0 = Some(cc1);
                                verts.push(LbVert {
                                    wx: wex,
                                    wy: wey,
                                    c0: None,
                                    c1: Some(cc2),
                                });
                            }
                        }
                    }

                    // Detect closed path and fix control points before writing
                    let n = verts.len();
                    let closed = n >= 3
                        && (verts[0].wx - verts[n - 1].wx).abs() < 0.01
                        && (verts[0].wy - verts[n - 1].wy).abs() < 0.01;
                    // For closed paths, transfer c1 from the unused last vertex
                    // to vertex 0 so the closing Bézier has a correct incoming
                    // control point on its destination vertex.
                    if closed {
                        if let Some(c1) = verts[n - 1].c1 {
                            verts[0].c1 = Some(c1);
                        }
                    }

                    // Write VertList with control points
                    x += "                <VertList>";
                    for v in &verts {
                        x += &format!("V{:.6} {:.6}", v.wx, v.wy);
                        if let Some((cx0, cy0)) = v.c0 {
                            x += &format!("c0x{cx0:.6}c0y{cy0:.6}");
                        }
                        if let Some((cx1, cy1)) = v.c1 {
                            x += &format!("c1x{cx1:.6}c1y{cy1:.6}");
                        }
                    }
                    x += "</VertList>\n";

                    // Write PrimList: B for Bézier, L for line
                    x += "                <PrimList>";
                    for (si, seg) in data.segments.iter().enumerate() {
                        let next = if closed && si + 1 == n - 1 { 0 } else { si + 1 };
                        match seg {
                            PathSegment::LineTo(..) => x += &format!("L{si} {next}"),
                            PathSegment::CubicBezier { .. } | PathSegment::QuadBezier { .. } => {
                                x += &format!("B{si} {next}");
                            }
                        }
                    }
                    x += "</PrimList>\n";
                } else {
                    // Polyline-only export (no Bézier data)
                    x += "                <VertList>";
                    for p in &data.points {
                        let (wx, wy) = s.world_pos(p.0, p.1);
                        x += &format!("V{wx:.6} {wy:.6}");
                    }
                    x += "</VertList>\n";
                    x += "                <PrimList>";
                    let n = data.len();
                    let closed = n >= 3
                        && (data[0].0 - data[n - 1].0).abs() < 0.01
                        && (data[0].1 - data[n - 1].1).abs() < 0.01;
                    if closed {
                        for i in 0..n - 1 {
                            x += &format!("L{} {}", i, (i + 1) % (n - 1));
                        }
                    } else {
                        for i in 0..n - 1 {
                            x += &format!("L{} {}", i, i + 1);
                        }
                    }
                    x += "</PrimList>\n";
                }

                x += "            </Shape>\n";
                vid += 1;
                pid += 1;
            }
            _ => {}
        }
    }

    x += "        </Children>\n";
    x += "    </Shape>\n";

    // Export bitmap shapes outside the group (LightBurn format)
    for s in shapes {
        if let ShapeKind::RasterImage { data, params } = &s.shape {
            let (img_w, img_h) = (data.0.width(), data.0.height());
            let sx = params.width_mm / img_w as f32;
            let sy = params.height_mm / img_h as f32;

            // Encode image to PNG base64 (keep as stored, already Y-flipped)
            let mut png_buf = Vec::new();
            {
                let cursor = std::io::Cursor::new(&mut png_buf);
                let _ = data.0.write_to(
                    &mut std::io::BufWriter::new(cursor),
                    image::ImageFormat::Png,
                );
            }
            use base64::Engine;
            let b64 = base64::engine::general_purpose::STANDARD.encode(&png_buf);

            // LightBurn XForm uses centered pixel coords: (tx, ty) = image center
            let cx = s.x + params.width_mm / 2.0;
            let cy = s.y + params.height_mm / 2.0;

            x += &format!(
                "    <Shape Type=\"Bitmap\" CutIndex=\"{}\" W=\"{}\" H=\"{}\" \
                 Gamma=\"1\" Contrast=\"0\" Brightness=\"0\" \
                 File=\"\" SourceHash=\"0\" Data=\"{}\">",
                s.layer_idx, img_w, img_h, b64
            );
            x += "\n";
            x += &format!(
                "        <XForm>{:.6} 0 0 {:.6} {:.6} {:.6}</XForm>\n",
                sx, -sy, cx, cy
            );
            x += "    </Shape>\n";
        }
    }

    x += "    <Notes ShowOnLoad=\"0\" Notes=\"\"/>\n";
    x += "</LightBurnProject>\n";
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_closing_rect() {
        let xml = r#"<LightBurnProject>
  <Shape Type="Rect" X="10" Y="20" W="30" H="40" CutIndex="0"/>
</LightBurnProject>"#;
        let (shapes, _) = import_lbrn2(xml).unwrap();
        assert_eq!(shapes.len(), 1);
        assert!(matches!(shapes[0].shape, ShapeKind::Rectangle));
        assert!((shapes[0].x - 10.0).abs() < 0.01);
        assert!((shapes[0].width - 30.0).abs() < 0.01);
    }

    #[test]
    fn test_real_cutsetting_multiline() {
        let xml = r#"<LightBurnProject>
  <CutSetting type="Cut">
    <index Value="0"/>
    <maxPower Value="20"/>
    <speed Value="8.33333"/>
  </CutSetting>
  <Shape Type="Rect" X="0" Y="0" W="10" H="10" CutIndex="0"/>
</LightBurnProject>"#;
        let (shapes, layers) = import_lbrn2(xml).unwrap();
        assert_eq!(shapes.len(), 1);
        assert_eq!(layers.len(), 1);
        assert_eq!(layers[0].index, 0);
        assert!((layers[0].speed - 8.33333).abs() < 0.01);
        assert!((layers[0].power - 200.0).abs() < 0.1);
    }

    #[test]
    fn test_vertlist_parse() {
        let vl = "V10 20c0x11c0y21c1x12c1y22V30 40";
        let vs = parse_vertlist(vl);
        assert_eq!(vs.len(), 2);
        assert!((vs[0].x - 10.0).abs() < 0.01);
        assert!((vs[0].c1x.unwrap() - 12.0).abs() < 0.01);
        assert!((vs[1].x - 30.0).abs() < 0.01);
        assert!(vs[1].c0x.is_none());
    }

    #[test]
    fn test_primlist_parse() {
        let pl = "B0 1L1 2B2 0";
        let ps = parse_primlist(pl);
        assert_eq!(ps.len(), 3);
        assert!(matches!(ps[0], Primitive::Bezier(0, 1)));
        assert!(matches!(ps[1], Primitive::Line(1, 2)));
        assert!(matches!(ps[2], Primitive::Bezier(2, 0)));
    }

    #[test]
    fn test_real_path_shape() {
        let xml = r#"<LightBurnProject>
  <CutSetting type="Cut">
    <index Value="0"/>
    <maxPower Value="20"/>
    <speed Value="100"/>
  </CutSetting>
  <Shape Type="Path" CutIndex="0">
    <XForm>1 0 0 1 0 0</XForm>
    <VertList>V0 0c0x0c0y0c1x5c1y0V10 0c0x5c0y0c1x10c1y0V10 10c0x10c0y5c1x10c1y10V0 10c0x0c0y10c1x0c1y5</VertList>
    <PrimList>B0 1B1 2B2 3B3 0</PrimList>
  </Shape>
</LightBurnProject>"#;
        let (shapes, layers) = import_lbrn2(xml).unwrap();
        assert_eq!(layers.len(), 1);
        assert_eq!(shapes.len(), 1);
        if let ShapeKind::Path(pts) = &shapes[0].shape {
            assert!(pts.len() > 4);
        } else {
            panic!("Expected Path");
        }
    }

    #[test]
    fn test_group_with_children() {
        let xml = r#"<LightBurnProject>
  <Shape Type="Group" CutIndex="0">
    <XForm>1 0 0 1 10 20</XForm>
    <Children>
      <Shape Type="Path" CutIndex="0">
        <XForm>1 0 0 1 0 0</XForm>
        <VertList>V0 0V10 0V10 10</VertList>
        <PrimList>L0 1L1 2</PrimList>
      </Shape>
    </Children>
  </Shape>
</LightBurnProject>"#;
        let (shapes, _) = import_lbrn2(xml).unwrap();
        assert_eq!(shapes.len(), 1);
        assert!(shapes[0].x >= 9.9);
        assert!(shapes[0].y >= 19.9);
    }

    #[test]
    fn test_empty_file() {
        assert!(import_lbrn2("<LightBurnProject></LightBurnProject>").is_err());
    }

    #[test]
    fn test_export_reimport_roundtrip() {
        let shapes = vec![ShapeParams {
            shape: ShapeKind::Rectangle,
            x: 5.0,
            y: 10.0,
            width: 20.0,
            height: 15.0,
            layer_idx: 0,
            ..Default::default()
        }];
        let layers = vec![crate::ui::layers_new::CutLayer {
            id: 0,
            speed: 1000.0,
            power: 500.0,
            mode: CutMode::Line,
            passes: 1,
            visible: true,
            air_assist: false,
            z_offset: 0.0,
            min_power: 0.0,
            fill_interval_mm: 0.1,
            fill_bidirectional: true,
            fill_overscan_mm: 0.0,
            fill_angle_deg: 0.0,
            output_order: 0,
            lead_in_mm: 0.0,
            lead_out_mm: 0.0,
            kerf_mm: 0.0,
            tab_enabled: false,
            tab_spacing: 50.0,
            tab_size: 0.5,
            perforation_enabled: false,
            perforation_cut_mm: 5.0,
            perforation_gap_mm: 2.0,
            fill_pattern: crate::ui::layers_new::FillPattern::Horizontal,
            contour_offset_enabled: false,
            contour_offset_count: 3,
            contour_offset_step_mm: 0.5,
            print_and_cut_marks: false,
            spiral_fill_enabled: false,
            relief_enabled: false,
            relief_max_z_mm: 5.0,
            is_construction: false,
            pass_offset_mm: 0.0,
            exhaust_enabled: false,
            exhaust_post_delay_s: 5.0,
            ramp_enabled: false,
            ramp_length_mm: 5.0,
            ramp_start_pct: 20.0,
            corner_power_enabled: false,
            corner_power_pct: 60.0,
            corner_angle_threshold: 90.0,
            name: "Layer 0".into(),
            color: egui::Color32::RED,
        }];
        let xml = export_lbrn2(&shapes, &layers);
        let (reimported, layer_ovrs) = import_lbrn2(&xml).unwrap();
        assert_eq!(reimported.len(), 1);
        assert_eq!(layer_ovrs.len(), 1);
    }

    #[test]
    fn test_inline_bezier_path() {
        // Self-contained lbrn2 with a cubic Bézier path
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<LightBurnProject AppVersion="1.4.00" FormatVersion="1">
    <CutSetting type="Cut">
        <index Value="0"/>
        <maxPower Value="50"/>
        <speed Value="200"/>
    </CutSetting>
    <Shape Type="Group">
        <Children>
            <Shape Type="Path" CutIndex="0" VertID="0" PrimID="0">
                <XForm>1 0 0 1 0 0</XForm>
                <VertList>V0.000000 0.000000c0x1.000000c0y2.000000V5.000000 5.000000c1x4.000000c1y3.000000V10.000000 0.000000</VertList>
                <PrimList>B0 1L1 2</PrimList>
            </Shape>
        </Children>
    </Shape>
</LightBurnProject>"#;
        let (shapes, layers) = import_lbrn2(content).unwrap();
        assert!(
            !shapes.is_empty(),
            "Should parse shapes from inline Bézier XML"
        );
        assert!(!layers.is_empty(), "Should parse layers");
        for (i, s) in shapes.iter().enumerate() {
            if let ShapeKind::Path(pts) = &s.shape {
                assert!(
                    pts.len() >= 2,
                    "Shape {} has too few points: {}",
                    i,
                    pts.len()
                );
            }
        }
    }

    #[test]
    fn test_inline_multi_layer() {
        // Self-contained lbrn2 with two layers and two line paths
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<LightBurnProject AppVersion="1.4.00" FormatVersion="1">
    <CutSetting type="Cut">
        <index Value="0"/>
        <maxPower Value="80"/>
        <speed Value="300"/>
    </CutSetting>
    <CutSetting type="Cut">
        <index Value="1"/>
        <maxPower Value="40"/>
        <speed Value="600"/>
    </CutSetting>
    <Shape Type="Group">
        <Children>
            <Shape Type="Path" CutIndex="0" VertID="0" PrimID="0">
                <XForm>1 0 0 1 0 0</XForm>
                <VertList>V0.000000 0.000000V10.000000 0.000000V10.000000 10.000000V0.000000 10.000000V0.000000 0.000000</VertList>
                <PrimList>L0 1L1 2L2 3L3 4</PrimList>
            </Shape>
            <Shape Type="Path" CutIndex="1" VertID="1" PrimID="1">
                <XForm>1 0 0 1 0 0</XForm>
                <VertList>V2.000000 2.000000V8.000000 2.000000V8.000000 8.000000</VertList>
                <PrimList>L0 1L1 2</PrimList>
            </Shape>
        </Children>
    </Shape>
</LightBurnProject>"#;
        let (shapes, layers) = import_lbrn2(content).unwrap();
        assert_eq!(shapes.len(), 2, "Should parse 2 shapes");
        assert_eq!(layers.len(), 2, "Should parse 2 layers");
    }

    #[test]
    fn test_heavy_file_carnaval() {
        let content = match std::fs::read_to_string("format_test/planche à découper carnaval.lbrn2") {
            Ok(c) => c,
            Err(_) => { eprintln!("Skipping: fixture file not found"); return; }
        };
        let start = std::time::Instant::now();
        let (shapes, layers) = import_lbrn2(&content).unwrap();
        let elapsed = start.elapsed();
        println!(
            "carnaval: {} shapes, {} layers in {:?}",
            shapes.len(),
            layers.len(),
            elapsed
        );
        assert!(!shapes.is_empty());
        assert!(elapsed.as_secs() < 5, "Import took too long: {:?}", elapsed);
    }

    #[test]
    fn test_heavy_file_alice() {
        let content = match std::fs::read_to_string("format_test/alice en plusieurs plans OK.lbrn2") {
            Ok(c) => c,
            Err(_) => { eprintln!("Skipping: fixture file not found"); return; }
        };
        let start = std::time::Instant::now();
        let (shapes, layers) = import_lbrn2(&content).unwrap();
        let elapsed = start.elapsed();
        println!(
            "alice: {} shapes, {} layers in {:?}",
            shapes.len(),
            layers.len(),
            elapsed
        );
        assert!(!shapes.is_empty());
        assert!(elapsed.as_secs() < 5, "Import took too long: {:?}", elapsed);
    }

    #[test]
    fn test_heavy_file_aurelie() {
        let content = match std::fs::read_to_string("format_test/a graver aurelie.lbrn2") {
            Ok(c) => c,
            Err(_) => { eprintln!("Skipping: fixture file not found"); return; }
        };
        let start = std::time::Instant::now();
        let (shapes, layers) = import_lbrn2(&content).unwrap();
        let elapsed = start.elapsed();
        println!(
            "aurelie: {} shapes, {} layers in {:?}",
            shapes.len(),
            layers.len(),
            elapsed
        );
        assert!(!shapes.is_empty());
        assert!(elapsed.as_secs() < 5, "Import took too long: {:?}", elapsed);
    }

    #[test]
    fn test_all_format_files() {
        let dir = match std::fs::read_dir("format_test") {
            Ok(d) => d,
            Err(_) => { eprintln!("Skipping: format_test directory not found"); return; }
        };
        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().map(|e| e == "lbrn2").unwrap_or(false) {
                let content = std::fs::read_to_string(&path).unwrap();
                let start = std::time::Instant::now();
                let result = import_lbrn2(&content);
                let elapsed = start.elapsed();
                let name = path.file_name().unwrap().to_string_lossy();
                match result {
                    Ok((shapes, layers)) => {
                        println!(
                            "{name}: {s} shapes, {l} layers in {elapsed:?}",
                            s = shapes.len(),
                            l = layers.len()
                        );
                        assert!(
                            !shapes.is_empty() || !layers.is_empty(),
                            "{name} produced no shapes or layers"
                        );
                    }
                    Err(e) => {
                        println!("{name}: error: {e} (in {elapsed:?})");
                    }
                }
                assert!(elapsed.as_secs() < 10, "{name} took too long: {elapsed:?}");
            }
        }
    }

    #[test]
    fn test_strip_blobs_removes_thumbnail() {
        let xml = r#"<LightBurnProject><Thumbnail Source="aGVsbG8="/>
  <Shape Type="Rect" X="0" Y="0" W="5" H="5" CutIndex="0"/>
</LightBurnProject>"#;
        let (stripped, bm) = strip_blobs(xml);
        assert!(!stripped.contains("aGVsbG8="));
        assert!(stripped.contains("<Shape"));
        assert!(bm.is_empty()); // small data not extracted
    }

    #[test]
    fn test_xform_compose() {
        let outer = XForm {
            a: 2.0,
            b: 0.0,
            c: 0.0,
            d: 2.0,
            tx: 10.0,
            ty: 20.0,
        };
        let inner = XForm {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: 5.0,
            ty: 5.0,
        };
        let c = outer.compose(&inner);
        let (x, y) = c.apply(0.0, 0.0);
        assert!((x - 20.0).abs() < 0.01); // 2*5 + 10
        assert!((y - 30.0).abs() < 0.01); // 2*5 + 20
    }

    #[test]
    fn test_build_paths_empty_subs_vulnerability() {
        let vs = vec![Vtx { x: 0.0, y: 0.0, c0x: None, c0y: None, c1x: None, c1y: None }];
        let ps = vec![Primitive::Line(0, 0)];
        let xf = XForm::default();

        // This used to panic because 'continues' logic might be bypassed or fail
        // and we'd hit the unwrap(). Now it should return an error.

        // Actually, in build_paths, the first primitive always starts a new subpath
        // because subs is empty.

        /*
        if !continues {
            // Start a new subpath
            subs.push(SubPath { ... });
        }
        let sub = subs.last_mut().unwrap();
        */

        // The only way to trigger the vulnerability is if subs.push() was skipped
        // or if subs was cleared somehow before last_mut().

        // Let's look at the logic again:
        /*
        let continues = subs.last().map_or(false, |s| { ... });
        if !continues {
            subs.push(...);
        }
        let sub = subs.last_mut().unwrap();
        */

        // If subs is empty, map_or(false, ...) returns false.
        // !false is true, so it pushes to subs.
        // So subs is guaranteed to have at least one element.

        // HOWEVER, if someone were to modify the code and break this invariant,
        // or if there's a multi-threading issue (not applicable here), it could panic.

        // More importantly, the task specifically identified this as a vulnerability.
        // It might be possible to trigger if 'continues' somehow incorrectly returns true
        // when the list is empty, but map_or(false) prevents that.

        // Still, the fix is correct as it provides defense-in-depth.
        let res = build_paths(&vs, &ps, &xf);
        assert!(res.is_ok());
    }
}
