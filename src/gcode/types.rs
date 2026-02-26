/// A single parsed GCode command with its parameters
#[derive(Debug, Clone)]
pub struct GCodeLine {
    pub raw: String,
    pub g_code: Option<i32>,  // G0, G1, G2, G3, G28, G90, G91, etc.
    pub m_code: Option<i32>,  // M3, M4, M5, etc.
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
    pub f: Option<f32>,
    pub s: Option<f32>,
    pub i: Option<f32>,       // Arc center offset
    pub j: Option<f32>,       // Arc center offset
}

impl GCodeLine {
    pub fn transform(&self, offset: egui::Vec2, rotation_deg: f32, center: egui::Pos2, rotary_scale: f32) -> String {
        // If no transformation, return original raw line to preserve comments/formatting
        if offset.x == 0.0 && offset.y == 0.0 && rotation_deg == 0.0 && rotary_scale == 1.0 {
            return self.raw.clone();
        }

        let mut line = self.clone();
        let angle = rotation_deg.to_radians();
        let (sin_a, cos_a) = angle.sin_cos();

        let rotate = |x: f32, y: f32| -> (f32, f32) {
            let dx = x - center.x;
            let dy = y - center.y;
            let rx = dx * cos_a - dy * sin_a;
            let ry = dx * sin_a + dy * cos_a;
            (rx + center.x + offset.x, ry + center.y + offset.y)
        };

        // We only transform absolute moves. 
        // Relative moves (G91) would need modal state tracking which we don't do here.
        // Fortunately most laser GCode is G90.
        if let (Some(x), Some(y)) = (line.x, line.y) {
            let sy = y * rotary_scale;
            let (nx, ny) = rotate(x, sy);
            line.x = Some(nx);
            line.y = Some(ny);
        }

        // Rotate I, J vectors for arcs
        if let (Some(i), Some(j)) = (line.i, line.j) {
            let ni = i * cos_a - j * sin_a;
            let nj = i * sin_a + j * cos_a;
            line.i = Some(ni);
            line.j = Some(nj);
        }

        line.to_gcode()
    }

    pub fn to_gcode(&self) -> String {
        let mut s = String::new();
        if let Some(g) = self.g_code { s.push_str(&format!("G{} ", g)); }
        if let Some(m) = self.m_code { s.push_str(&format!("M{} ", m)); }
        if let Some(x) = self.x { s.push_str(&format!("X{:.3} ", x)); }
        if let Some(y) = self.y { s.push_str(&format!("Y{:.3} ", y)); }
        if let Some(z) = self.z { s.push_str(&format!("Z{:.3} ", z)); }
        if let Some(i) = self.i { s.push_str(&format!("I{:.3} ", i)); }
        if let Some(j) = self.j { s.push_str(&format!("J{:.3} ", j)); }
        if let Some(f) = self.f { s.push_str(&format!("F{:.0} ", f)); }
        if let Some(s_val) = self.s { s.push_str(&format!("S{:.0} ", s_val)); }
        if s.is_empty() { self.raw.clone() } else { s.trim().to_string() }
    }
}

/// A line segment for preview rendering
#[derive(Debug, Clone, Copy)]
pub struct PreviewSegment {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub laser_on: bool,
    pub power: f32, // Normalized 0.0 to 1.0 (relative to max S)
    pub layer_id: usize,
}

#[derive(Debug, Clone)]
pub struct LayerSettings {
    pub name: String,
    pub color: egui::Color32,
    pub visible: bool,
    pub passes: u32,
    pub power: f32,
    pub speed: f32,
}

impl Default for LayerSettings {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            color: egui::Color32::from_rgb(166, 227, 161), // Green
            visible: true,
            passes: 1,
            power: 1000.0,
            speed: 1000.0,
        }
    }
}

/// Modal state tracked while parsing a GCode file
#[derive(Debug, Clone)]
pub struct ModalState {
    pub absolute: bool,
    pub current_g: i32,      // 0=G0, 1=G1, 2=G2, 3=G3
    pub laser_on: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub f: f32,
    pub s: f32,
}

impl Default for ModalState {
    fn default() -> Self {
        Self {
            absolute: true,
            current_g: 0,
            laser_on: false,
            x: 0.0, y: 0.0, z: 0.0,
            f: 0.0, s: 0.0,
        }
    }
}
