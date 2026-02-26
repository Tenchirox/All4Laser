use std::fs;
use std::time::Duration;

use super::parser;
use super::types::*;

/// A loaded GCode file with parsed commands and preview data
#[derive(Debug, Clone)]
pub struct GCodeFile {
    pub filename: String,
    pub lines: Vec<GCodeLine>,
    pub segments: Vec<PreviewSegment>,
    pub estimated_time: Duration,
    pub layers: Vec<LayerSettings>,
}

impl GCodeFile {
    /// Load and parse a GCode file
    pub fn load(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| format!("Cannot read {path}: {e}"))?;
        let filename = std::path::Path::new(path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string());

        Self::from_content(&filename, &content)
    }

    /// Create GCodeFile from raw string content
    pub fn from_content(filename: &str, content: &str) -> Result<Self, String> {
        let lines: Vec<GCodeLine> = content.lines().map(parser::parse_line).collect();
        let (segments, layers, estimated_time) = build_preview(&lines);

        Ok(Self {
            filename: filename.to_string(),
            lines,
            segments,
            estimated_time,
            layers,
        })
    }

    /// Create GCodeFile from a list of lines (already raw GCode)
    pub fn from_lines(filename: &str, raw_lines: &[String]) -> Self {
        let lines: Vec<GCodeLine> = raw_lines.iter().map(|l| parser::parse_line(l)).collect();
        let (segments, layers, estimated_time) = build_preview(&lines);

        Self {
            filename: filename.to_string(),
            lines,
            segments,
            estimated_time,
            layers,
        }
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Returns the bounding box of the GCode (min_x, min_y, max_x, max_y)
    pub fn bounds(&self) -> Option<(f32, f32, f32, f32)> {
        if self.segments.is_empty() {
            return None;
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for seg in &self.segments {
            min_x = min_x.min(seg.x1).min(seg.x2);
            min_y = min_y.min(seg.y1).min(seg.y2);
            max_x = max_x.max(seg.x1).max(seg.x2);
            max_y = max_y.max(seg.y1).max(seg.y2);
        }

        Some((min_x, min_y, max_x, max_y))
    }
}

/// Machine kinematic parameters from GRBL settings
#[derive(Clone, Copy, Debug)]
pub struct KinematicParams {
    /// Max X feed rate mm/min ($110)
    pub max_rate_x: f32,
    /// Max Y feed rate mm/min ($111)
    pub max_rate_y: f32,
    /// X acceleration mm/s² ($120)
    pub accel_x: f32,
    /// Y acceleration mm/s² ($121)
    pub accel_y: f32,
}

impl Default for KinematicParams {
    fn default() -> Self {
        Self {
            max_rate_x: 3000.0,
            max_rate_y: 3000.0,
            accel_x: 200.0, // conservative
            accel_y: 200.0,
        }
    }
}

/// Estimate time for a single move using a trapezoidal velocity profile (mm/min, mm/s²)
pub fn move_time_trapezoid(distance_mm: f32, feed_rate_mmpm: f32, accel_mm_s2: f32) -> f64 {
    if distance_mm <= 0.0 || feed_rate_mmpm <= 0.0 {
        return 0.0;
    }

    let v_max = (feed_rate_mmpm / 60.0) as f64; // convert to mm/s
    let a = accel_mm_s2 as f64;
    let d = distance_mm as f64;

    // Distance needed to accelerate from 0 to v_max (and decelerate back)
    let d_ramp = v_max * v_max / a; // = 2 * (0.5 * v_max² / a)

    if d >= d_ramp {
        // Full trapezoidal: accelerate → cruise → decelerate
        let t_ramp = 2.0 * v_max / a;
        let d_cruise = d - d_ramp;
        let t_cruise = d_cruise / v_max;
        t_ramp + t_cruise
    } else {
        // Triangular: never reaches v_max
        // d = 0.5*a*t_half^2  x2 → t_total = 2 * sqrt(d/a)
        2.0 * (d / a).sqrt()
    }
}

/// Build preview segments and estimate execution time from parsed GCode
fn build_preview(lines: &[GCodeLine]) -> (Vec<PreviewSegment>, Vec<LayerSettings>, Duration) {
    let kin = KinematicParams::default();
    let mut segments = Vec::new();
    let mut layers = vec![LayerSettings::default()];
    let mut current_layer_idx = 0;
    let mut state = ModalState::default();
    let mut total_time_secs: f64 = 0.0;

    for line in lines {
        // Track G90/G91
        if let Some(g) = line.g_code {
            match g {
                90 => state.absolute = true,
                91 => state.absolute = false,
                0 | 1 | 2 | 3 => state.current_g = g,
                _ => {}
            }
        }

        // Try to detect layer changes in comments (LaserGRBL/LightBurn style)
        if line.raw.contains(";LAYER:") {
            let name = line.raw.split(":").nth(1).unwrap_or("Layer").trim().to_string();
            // Check if already exists
            if let Some(idx) = layers.iter().position(|l| l.name == name) {
                current_layer_idx = idx;
            } else {
                current_layer_idx = layers.len();
                layers.push(LayerSettings {
                    name,
                    color: egui::Color32::from_rgb(100 + ((current_layer_idx * 40) % 155) as u8, 200, 255),
                    ..Default::default()
                });
            }
        }

        // Track M codes (laser on/off)
        if let Some(m) = line.m_code {
            match m {
                3 | 4 => state.laser_on = true,
                5 => state.laser_on = false,
                _ => {}
            }
        }

        // Track S parameter
        if let Some(s) = line.s {
            state.s = s;
        }

        // Track F parameter
        if let Some(f) = line.f {
            state.f = f;
        }

        // Process movement
        let is_move = line.g_code.is_some() && matches!(line.g_code, Some(0) | Some(1))
            || (line.g_code.is_none() && (line.x.is_some() || line.y.is_some()));

        if is_move || matches!(line.g_code, Some(0) | Some(1)) {
            let new_x = if let Some(x) = line.x {
                if state.absolute { x } else { state.x + x }
            } else {
                state.x
            };
            let new_y = if let Some(y) = line.y {
                if state.absolute { y } else { state.y + y }
            } else {
                state.y
            };
            let new_z = if let Some(z) = line.z {
                if state.absolute { z } else { state.z + z }
            } else {
                state.z
            };

            if new_x != state.x || new_y != state.y {
                let is_laser = state.laser_on && state.current_g != 0 && state.s > 0.0;
                let power = if is_laser { (state.s / 1000.0).min(1.0) } else { 0.0 };
                segments.push(PreviewSegment {
                    x1: state.x,
                    y1: state.y,
                    x2: new_x,
                    y2: new_y,
                    laser_on: is_laser,
                    power,
                    layer_id: current_layer_idx,
                });

                let dx = new_x - state.x;
                let dy = new_y - state.y;
                let dist = (dx * dx + dy * dy).sqrt();

                if state.f > 0.0 {
                    // Use component-weighted acceleration (vector diagonal accel)
                    let angle = dy.atan2(dx).abs();
                    let a_eff = kin.accel_x * angle.cos() + kin.accel_y * angle.sin();
                    let a_eff = a_eff.max(10.0); // guard against div/0

                    // Cap feed to machine max rate
                    let f_capped = state.f
                        .min(kin.max_rate_x * angle.cos() + kin.max_rate_y * angle.sin());

                    total_time_secs += move_time_trapezoid(dist, f_capped, a_eff);
                }
            }

            state.x = new_x;
            state.y = new_y;
            state.z = new_z;
        }
    }

    let estimated = Duration::from_secs_f64(total_time_secs);
    (segments, layers, estimated)
}
