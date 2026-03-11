#![allow(dead_code)]
use crate::ui::drawing::{ShapeKind, ShapeParams};
use crate::ui::layers_new::{CutLayer, CutMode};
use std::collections::HashMap;

#[derive(Clone,Debug)]
struct XForm{a:f32,b:f32,c:f32,d:f32,tx:f32,ty:f32}
impl Default for XForm{
    fn default()->Self{Self{a:1.0,b:0.0,c:0.0,d:1.0,tx:0.0,ty:0.0}}
}
impl XForm{
    fn apply(&self,x:f32,y:f32)->(f32,f32){
        (self.a*x+self.c*y+self.tx, self.b*x+self.d*y+self.ty)
    }
    fn compose(&self,i:&XForm)->XForm{
        XForm{a:self.a*i.a+self.c*i.b, b:self.b*i.a+self.d*i.b,
              c:self.a*i.c+self.c*i.d, d:self.b*i.c+self.d*i.d,
              tx:self.a*i.tx+self.c*i.ty+self.tx,
              ty:self.b*i.tx+self.d*i.ty+self.ty}
    }
}
fn parse_xform(t:&str)->XForm{
    let n:Vec<f32>=t.split_whitespace().filter_map(|s|s.parse().ok()).collect();
    if n.len()>=6{XForm{a:n[0],b:n[1],c:n[2],d:n[3],tx:n[4],ty:n[5]}}else{XForm::default()}
}
#[derive(Clone,Debug,Default)]
struct Vtx{x:f32,y:f32,c0x:Option<f32>,c0y:Option<f32>,c1x:Option<f32>,c1y:Option<f32>}
fn parse_vertlist(t:&str)->Vec<Vtx>{
    t.split('V').filter(|c|!c.trim().is_empty()).filter_map(|c|pv(c.trim())).collect()
}
fn pv(c:&str)->Option<Vtx>{
    let me=c.find('c').unwrap_or(c.len());
    let mut co=c[..me].split_whitespace();
    let x=co.next()?.parse().ok()?; let y=co.next()?.parse().ok()?;
    let r=&c[me..];
    Some(Vtx{x,y,c0x:ecp(r,"c0x"),c0y:ecp(r,"c0y"),c1x:ecp(r,"c1x"),c1y:ecp(r,"c1y")})
}
fn ecp(t:&str,p:&str)->Option<f32>{
    let s=t.find(p)?+p.len();
    let e=t[s..].find('c').unwrap_or(t.len()-s);
    t[s..s+e].trim().parse().ok()
}
#[derive(Clone,Debug)]
enum Pm{L(usize,usize),B(usize,usize)}
fn parse_primlist(t:&str)->Vec<Pm>{
    let mut o=Vec::new(); let b=t.trim().as_bytes(); let mut i=0;
    while i<b.len(){
        let c=b[i] as char;
        if c!='B'&&c!='L'{i+=1;continue;}
        i+=1; let s1=i;
        while i<b.len()&&b[i].is_ascii_digit(){i+=1;}
        let i1:usize=t[s1..i].parse().unwrap_or(0);
        while i<b.len()&&b[i]==b' '{i+=1;}
        let s2=i;
        while i<b.len()&&b[i].is_ascii_digit(){i+=1;}
        let i2:usize=t[s2..i].parse().unwrap_or(0);
        o.push(if c=='B'{Pm::B(i1,i2)}else{Pm::L(i1,i2)});
    }
    o
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
        let vx = ea(tag, "vx").and_then(|s| s.parse::<f32>().ok());
        let vy = ea(tag, "vy").and_then(|s| s.parse::<f32>().ok());
        if let (Some(x), Some(y)) = (vx, vy) {
            vs.push(Vtx {
                x, y,
                c0x: ea(tag, "c0x").and_then(|s| s.parse().ok()),
                c0y: ea(tag, "c0y").and_then(|s| s.parse().ok()),
                c1x: ea(tag, "c1x").and_then(|s| s.parse().ok()),
                c1y: ea(tag, "c1y").and_then(|s| s.parse().ok()),
            });
        }
        pos = e;
    }
    vs
}

/// Parse <P T="B" p0="0" p1="1"/> elements (.lbrn v1 format)
fn parse_p_elements(inner: &str) -> Vec<Pm> {
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
        let t = ea(tag, "T").unwrap_or_default();
        let p0: usize = ea(tag, "p0").and_then(|s| s.parse().ok()).unwrap_or(0);
        let p1: usize = ea(tag, "p1").and_then(|s| s.parse().ok()).unwrap_or(0);
        match t.as_str() {
            "B" => ps.push(Pm::B(p0, p1)),
            "L" => ps.push(Pm::L(p0, p1)),
            _ => {}
        }
        pos = e;
    }
    ps
}

fn fbz(p0:(f32,f32),c0:(f32,f32),c1:(f32,f32),p1:(f32,f32),n:usize)->Vec<(f32,f32)>{
    (0..=n).map(|i|{let t=i as f32/n as f32;let m=1.0-t;
        let m2=m*m;let m3=m2*m;let t2=t*t;let t3=t2*t;
        (m3*p0.0+3.0*m2*t*c0.0+3.0*m*t2*c1.0+t3*p1.0,
         m3*p0.1+3.0*m2*t*c0.1+3.0*m*t2*c1.1+t3*p1.1)
    }).collect()
}
fn build_path(vs:&[Vtx],ps:&[Pm],xf:&XForm)->Vec<(f32,f32)>{
    if vs.is_empty()||ps.is_empty(){return vec![];}
    let mut pts:Vec<(f32,f32)>=Vec::new();
    for p in ps{match p{
        Pm::L(i0,i1)=>{
            let v0=&vs[(*i0).min(vs.len()-1)];let v1=&vs[(*i1).min(vs.len()-1)];
            let p0=xf.apply(v0.x,v0.y);let p1=xf.apply(v1.x,v1.y);
            if pts.is_empty()||pts.last()!=Some(&p0){pts.push(p0);}
            pts.push(p1);
        }
        Pm::B(i0,i1)=>{
            let v0=&vs[(*i0).min(vs.len()-1)];let v1=&vs[(*i1).min(vs.len()-1)];
            let p0=xf.apply(v0.x,v0.y);let p1=xf.apply(v1.x,v1.y);
            // c0 = outgoing control point from start vertex, c1 = incoming to end vertex
            let cp0=xf.apply(v0.c0x.unwrap_or(v0.x),v0.c0y.unwrap_or(v0.y));
            let cp1=xf.apply(v1.c1x.unwrap_or(v1.x),v1.c1y.unwrap_or(v1.y));
            let bz=fbz(p0,cp0,cp1,p1,32);
            let s=if!pts.is_empty()&&pts.last()==bz.first(){1}else{0};
            pts.extend_from_slice(&bz[s..]);
        }
    }}
    // Close path if last primitive connects back to first vertex
    if pts.len()>=3{
        if let(Some(first),Some(last))=(pts.first(),pts.last()){
            if(first.0-last.0).abs()>0.001||(first.1-last.1).abs()>0.001{
                // Check if the path *should* be closed (last prim ends at first prim's start vertex)
                if let(Some(first_p),Some(last_p))=(ps.first(),ps.last()){
                    let fi=match first_p{Pm::L(i,_)|Pm::B(i,_)=>*i};
                    let li=match last_p{Pm::L(_,i)|Pm::B(_,i)=>*i};
                    if fi==li{
                        pts.push(*first);
                    }
                }
            }
        }
    }
    pts
}
fn p2s(pts:&[(f32,f32)],li:usize)->Option<ShapeParams>{
    if pts.len()<2{return None;}
    let mx=pts.iter().map(|p|p.0).fold(f32::MAX,f32::min);
    let my=pts.iter().map(|p|p.1).fold(f32::MAX,f32::min);
    let r:Vec<(f32,f32)>=pts.iter().map(|&(x,y)|(x-mx,y-my)).collect();
    Some(ShapeParams{shape:ShapeKind::Path(r),x:mx,y:my,layer_idx:li,..Default::default()})
}
fn ea(l:&str,a:&str)->Option<String>{
    let p=format!("{}=\"",a);let s=l.find(&p)?+p.len();
    let e=l[s..].find('"')?;Some(l[s..s+e].to_string())
}
fn cvf(b:&str,tag:&str)->Option<f32>{
    let p=format!("<{} ",tag);let i=b.find(&p)?;ea(&b[i..],"Value")?.parse().ok()
}
fn cvu(b:&str,tag:&str)->Option<usize>{
    let p=format!("<{} ",tag);let i=b.find(&p)?;ea(&b[i..],"Value")?.parse().ok()
}
fn tc<'a>(b:&'a str,tag:&str)->Option<&'a str>{
    let o=format!("<{}>",tag);let c=format!("</{}>",tag);
    let s=b.find(&o)?+o.len();let e=b[s..].find(&c)?+s;Some(&b[s..e])
}

#[derive(Clone, Debug)]
pub struct LbrnLayerOverride {
    pub index: usize,
    pub speed: f32,
    pub power: f32,
    pub mode: CutMode,
    pub passes: u32,
}

fn pm(s: &str) -> CutMode {
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
        let co = match fc(&content[is..], "<Shape ", "</Shape>") {
            Some(o) => o,
            None => { pos = is; continue; }
        };
        let inner = &content[is..is + co];
        let bend = is + co + "</Shape>".len();
        if let Some(pid) = ea(otag, "PrimID") {
            if let Some(pt) = tc(inner, "PrimList") {
                map.entry(pid).or_insert_with(|| pt.to_string());
            }
        }
        // Recurse into Children
        if let Some(ch) = tc(inner, "Children") {
            let child_map = collect_shared_primlists(ch);
            for (k, v) in child_map {
                map.entry(k).or_insert(v);
            }
        }
        pos = bend;
    }
    map
}

/// Parse a .lbrn2 XML file and extract shapes + layer overrides
pub fn import_lbrn2(content: &str) -> Result<(Vec<ShapeParams>, Vec<LbrnLayerOverride>), String> {
    let mut shapes = Vec::new();
    let mut lo = Vec::new();
    pcs(content, &mut lo);
    let shared_prims = collect_shared_primlists(content);
    psh(content, &XForm::default(), &shared_prims, &mut shapes);
    if shapes.is_empty() {
        psc(content, &mut shapes);
    }
    if shapes.is_empty() && lo.is_empty() {
        return Err("No shapes or layers found in LightBurn file. \
            The file may be empty or in an unsupported format."
            .into());
    }
    Ok((shapes, lo))
}

fn pcs(c: &str, out: &mut Vec<LbrnLayerOverride>) {
    let mut pos = 0;
    loop {
        let st = match c[pos..].find("<CutSetting") {
            Some(p) => pos + p,
            None => break,
        };
        if let Some(eo) = c[st..].find("</CutSetting>") {
            let end = st + eo + "</CutSetting>".len();
            let blk = &c[st..end];
            let ms = ea(blk, "type").unwrap_or_default();
            out.push(LbrnLayerOverride {
                index: cvu(blk, "index").unwrap_or(0),
                speed: cvf(blk, "speed").unwrap_or(1000.0),
                power: cvf(blk, "maxPower").unwrap_or(50.0) * 10.0,
                mode: pm(&ms),
                passes: 1,
            });
            pos = end;
        } else if let Some(eo) = c[st..].find("/>") {
            let end = st + eo + 2;
            let ln = &c[st..end];
            out.push(LbrnLayerOverride {
                index: ea(ln, "index").and_then(|s| s.parse().ok()).unwrap_or(0),
                speed: ea(ln, "speed").and_then(|s| s.parse().ok()).unwrap_or(1000.0),
                power: ea(ln, "maxPower")
                    .and_then(|s| s.parse::<f32>().ok())
                    .unwrap_or(50.0)
                    * 10.0,
                mode: pm(&ea(ln, "type").unwrap_or_default()),
                passes: 1,
            });
            pos = end;
        } else {
            break;
        }
    }
}

fn fc(c: &str, otag: &str, ctag: &str) -> Option<usize> {
    let mut d = 1i32;
    let mut i = 0;
    while i < c.len() {
        if c[i..].starts_with(ctag) {
            d -= 1;
            if d == 0 {
                return Some(i);
            }
            i += ctag.len();
        } else if c[i..].starts_with(otag) {
            let a = i + otag.len();
            if let Some(gt) = c[a..].find('>') {
                let te = a + gt;
                if te > 0 && &c[te - 1..te] != "/" {
                    d += 1;
                }
                i = te + 1;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    None
}

fn psh(content: &str, pxf: &XForm, shared_prims: &HashMap<String, String>, out: &mut Vec<ShapeParams>) {
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
        let st = ea(otag, "Type").unwrap_or_default();
        let ci = ea(otag, "CutIndex")
            .or_else(|| ea(otag, "Layer"))
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);

        if otag.ends_with("/>") {
            pil(otag, ci, pxf, out);
            pos = te + 1;
            continue;
        }

        let is = te + 1;
        let co = match fc(&content[is..], "<Shape ", "</Shape>") {
            Some(o) => o,
            None => {
                pos = is;
                continue;
            }
        };
        let inner = &content[is..is + co];
        let bend = is + co + "</Shape>".len();

        let lxf = tc(inner, "XForm").map(parse_xform).unwrap_or_default();
        let cxf = pxf.compose(&lxf);

        match st.as_str() {
            "Group" => {
                if let Some(ch) = tc(inner, "Children") {
                    psh(ch, &cxf, shared_prims, out);
                }
            }
            _ => {
                let mut found = false;
                // Try .lbrn2 packed format first
                if let Some(vt) = tc(inner, "VertList") {
                    let vs = parse_vertlist(vt);
                    // Try local PrimList first, then shared by PrimID
                    let pt_str = tc(inner, "PrimList").map(|s| s.to_string())
                        .or_else(|| ea(otag, "PrimID").and_then(|pid| shared_prims.get(&pid).cloned()));
                    if let Some(pt) = pt_str {
                        let ps = parse_primlist(&pt);
                        let pts = build_path(&vs, &ps, &cxf);
                        if let Some(s) = p2s(&pts, ci) {
                            out.push(s);
                            found = true;
                        }
                    }
                }
                // Try .lbrn v1 element format (<V .../> and <P .../>)
                if !found {
                    let vs = parse_v_elements(inner);
                    let ps = parse_p_elements(inner);
                    if !vs.is_empty() && !ps.is_empty() {
                        let pts = build_path(&vs, &ps, &cxf);
                        if let Some(s) = p2s(&pts, ci) {
                            out.push(s);
                            found = true;
                        }
                    }
                }
                // Fallback: try parsing as inline shape with the composed transform
                if !found {
                    pil(otag, ci, &cxf, out);
                }
            }
        }
        pos = bend;
    }
}

fn pil(tag: &str, li: usize, xf: &XForm, out: &mut Vec<ShapeParams>) {
    let st = ea(tag, "Type").unwrap_or_default();
    match st.as_str() {
        "Rect" => {
            if let (Some(x), Some(y)) = (
                ea(tag, "X").and_then(|s| s.parse::<f32>().ok()),
                ea(tag, "Y").and_then(|s| s.parse::<f32>().ok()),
            ) {
                let w: f32 = ea(tag, "W").and_then(|s| s.parse().ok()).unwrap_or(10.0);
                let h: f32 = ea(tag, "H").and_then(|s| s.parse().ok()).unwrap_or(10.0);
                // If transform is non-identity, emit as Path to preserve rotation/skew
                if !xf_is_identity(xf) {
                    let corners = vec![
                        xf.apply(x, y),
                        xf.apply(x + w, y),
                        xf.apply(x + w, y + h),
                        xf.apply(x, y + h),
                        xf.apply(x, y),
                    ];
                    if let Some(s) = p2s(&corners, li) {
                        out.push(s);
                    }
                } else {
                    out.push(ShapeParams {
                        shape: ShapeKind::Rectangle,
                        x, y, width: w, height: h, layer_idx: li,
                        ..Default::default()
                    });
                }
            }
        }
        "Ellipse" => {
            if let (Some(cx), Some(cy)) = (
                ea(tag, "CX").and_then(|s| s.parse::<f32>().ok()),
                ea(tag, "CY").and_then(|s| s.parse::<f32>().ok()),
            ) {
                let rx: f32 = ea(tag, "Rx").and_then(|s| s.parse().ok()).unwrap_or(10.0);
                let ry: f32 = ea(tag, "Ry").and_then(|s| s.parse().ok()).unwrap_or(rx);
                // Always emit ellipse as Path for accuracy (handles Ry != Rx, transforms)
                let steps = 64;
                let mut pts = Vec::with_capacity(steps + 1);
                for i in 0..=steps {
                    let angle = 2.0 * std::f32::consts::PI * i as f32 / steps as f32;
                    let px = cx + rx * angle.cos();
                    let py = cy + ry * angle.sin();
                    pts.push(xf.apply(px, py));
                }
                if let Some(s) = p2s(&pts, li) {
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

fn psc(content: &str, out: &mut Vec<ShapeParams>) {
    let id_xf = XForm::default();
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("<Shape ") && t.ends_with("/>") {
            let li = ea(t, "CutIndex")
                .or_else(|| ea(t, "Layer"))
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            pil(t, li, &id_xf, out);
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

    // Only emit CutSettings for layers referenced by shapes
    for &i in &used {
        if let Some(l) = layers.get(i) {
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
                    (x0, y0), (x0 + s.width, y0),
                    (x0 + s.width, y0 + s.height), (x0, y0 + s.height),
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
                    x += &format!(
                        "V{px:.6} {py:.6}c0x{c0x:.6}c0y{c0y:.6}c1x{c1x:.6}c1y{c1y:.6}"
                    );
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
            ShapeKind::Path(pts) if pts.len() >= 2 => {
                x += &format!(
                    "            <Shape Type=\"Path\" CutIndex=\"{}\" VertID=\"{}\" PrimID=\"{}\">\n",
                    s.layer_idx, vid, pid
                );
                x += "                <XForm>1 0 0 1 0 0</XForm>\n";
                x += "                <VertList>";
                for p in pts {
                    let (wx, wy) = s.world_pos(p.0, p.1);
                    x += &format!("V{wx:.6} {wy:.6}");
                }
                x += "</VertList>\n";
                x += "                <PrimList>";
                let n = pts.len();
                let closed = n >= 3
                    && (pts[0].0 - pts[n - 1].0).abs() < 0.01
                    && (pts[0].1 - pts[n - 1].1).abs() < 0.01;
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
                x += "            </Shape>\n";
                vid += 1;
                pid += 1;
            }
            _ => {}
        }
    }

    x += "        </Children>\n";
    x += "    </Shape>\n";
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
        assert!(matches!(ps[0], Pm::B(0, 1)));
        assert!(matches!(ps[1], Pm::L(1, 2)));
        assert!(matches!(ps[2], Pm::B(2, 0)));
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
        let shapes = vec![
            ShapeParams {
                shape: ShapeKind::Rectangle,
                x: 5.0, y: 10.0, width: 20.0, height: 15.0,
                layer_idx: 0, ..Default::default()
            },
        ];
        let layers = vec![crate::ui::layers_new::CutLayer {
            id: 0, speed: 1000.0, power: 500.0,
            mode: CutMode::Line, passes: 1, visible: true,
            air_assist: false, z_offset: 0.0, min_power: 0.0,
            fill_interval_mm: 0.1, fill_bidirectional: true,
            fill_overscan_mm: 0.0, fill_angle_deg: 0.0,
            output_order: 0, lead_in_mm: 0.0, lead_out_mm: 0.0,
            kerf_mm: 0.0, tab_enabled: false, tab_spacing: 50.0,
            tab_size: 0.5, perforation_enabled: false,
            perforation_cut_mm: 5.0, perforation_gap_mm: 2.0,
            fill_pattern: crate::ui::layers_new::FillPattern::Horizontal,
            contour_offset_enabled: false, contour_offset_count: 3,
            contour_offset_step_mm: 0.5, print_and_cut_marks: false,
            spiral_fill_enabled: false, relief_enabled: false,
            relief_max_z_mm: 5.0, is_construction: false,
            pass_offset_mm: 0.0, exhaust_enabled: false,
            exhaust_post_delay_s: 5.0, ramp_enabled: false,
            ramp_length_mm: 5.0, ramp_start_pct: 20.0,
            corner_power_enabled: false, corner_power_pct: 60.0,
            corner_angle_threshold: 90.0, name: "Layer 0".into(),
            color: egui::Color32::RED,
        }];
        let xml = export_lbrn2(&shapes, &layers);
        let (reimported, layer_ovrs) = import_lbrn2(&xml).unwrap();
        assert_eq!(reimported.len(), 1);
        assert_eq!(layer_ovrs.len(), 1);
    }

    #[test]
    fn test_real_file_1() {
        let content = std::fs::read_to_string("format_test/1 lexvoto.lbrn2").unwrap();
        let (shapes, layers) = import_lbrn2(&content).unwrap();
        assert!(!shapes.is_empty(), "Should parse shapes from real file 1");
        assert!(!layers.is_empty(), "Should parse layers from real file 1");
        println!("File 1: {} shapes, {} layers", shapes.len(), layers.len());
        // Verify shapes have valid coordinates
        for (i, s) in shapes.iter().enumerate() {
            if let ShapeKind::Path(pts) = &s.shape {
                assert!(pts.len() >= 2, "Shape {} has too few points: {}", i, pts.len());
            }
        }
    }

    #[test]
    fn test_real_file_2() {
        let content = std::fs::read_to_string("format_test/2 lexvoto.lbrn2").unwrap();
        let (shapes, layers) = import_lbrn2(&content).unwrap();
        assert!(!shapes.is_empty(), "Should parse shapes from real file 2");
        assert!(!layers.is_empty(), "Should parse layers from real file 2");
        println!("File 2: {} shapes, {} layers", shapes.len(), layers.len());
    }

    #[test]
    fn test_xform_compose() {
        let outer = XForm { a: 2.0, b: 0.0, c: 0.0, d: 2.0, tx: 10.0, ty: 20.0 };
        let inner = XForm { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: 5.0, ty: 5.0 };
        let c = outer.compose(&inner);
        let (x, y) = c.apply(0.0, 0.0);
        assert!((x - 20.0).abs() < 0.01); // 2*5 + 10
        assert!((y - 30.0).abs() < 0.01); // 2*5 + 20
    }
}
