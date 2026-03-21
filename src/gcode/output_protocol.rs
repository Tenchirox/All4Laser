#![allow(dead_code)]

/// Abstraction over laser output protocols (GRBL, Marlin, EGV/K40).
///
/// The `GCodeBuilder` emits high-level operations (rapid, linear, laser_on/off).
/// An `OutputProtocol` translates those into protocol-specific command strings.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolKind {
    Grbl,
    Marlin,
    Egv,
}

impl Default for ProtocolKind {
    fn default() -> Self {
        ProtocolKind::Grbl
    }
}

impl ProtocolKind {
    pub const ALL: [ProtocolKind; 3] = [ProtocolKind::Grbl, ProtocolKind::Marlin, ProtocolKind::Egv];

    pub fn label(self) -> &'static str {
        match self {
            ProtocolKind::Grbl => "GRBL",
            ProtocolKind::Marlin => "Marlin",
            ProtocolKind::Egv => "EGV (K40)",
        }
    }
}

/// Trait implemented by each protocol to translate high-level laser commands.
pub trait OutputProtocol: Send + Sync {
    /// Preamble commands (units, mode, etc.)
    fn preamble(&self) -> Vec<String>;

    /// Rapid move (laser off, fast travel)
    fn rapid(&self, x: f32, y: f32) -> Vec<String>;

    /// Linear cut move
    fn linear(&self, x: f32, y: f32, speed_cmd: &str) -> Vec<String>;

    /// Turn laser on at given power
    fn laser_on(&self, power: f32) -> Vec<String>;

    /// Turn laser off
    fn laser_off(&self) -> Vec<String>;

    /// Set feed rate
    fn set_speed(&self, speed: f32) -> String;

    /// Air assist on
    fn air_assist_on(&self) -> String;

    /// Air assist off
    fn air_assist_off(&self) -> String;

    /// Exhaust on
    fn exhaust_on(&self) -> String;

    /// Exhaust off
    fn exhaust_off(&self) -> String;

    /// Dwell / pause
    fn dwell(&self, seconds: f32) -> String;

    /// Z move
    fn z_move(&self, z: f32) -> String;

    /// End-of-job (home, laser off, etc.)
    fn footer(&self) -> Vec<String>;

    /// Comment prefix
    fn comment(&self, text: &str) -> String;

    /// Protocol kind
    fn kind(&self) -> ProtocolKind;

    /// Whether this protocol supports standard GCode (GRBL/Marlin do, EGV doesn't)
    fn is_gcode_based(&self) -> bool {
        true
    }
}

// ─── GRBL ────────────────────────────────────────────────────────────────────

pub struct GrblProtocol;

impl OutputProtocol for GrblProtocol {
    fn preamble(&self) -> Vec<String> {
        vec!["G90".into(), "G21".into()]
    }

    fn rapid(&self, x: f32, y: f32) -> Vec<String> {
        vec![format!("G0 X{:.3} Y{:.3}", x, y)]
    }

    fn linear(&self, x: f32, y: f32, speed_cmd: &str) -> Vec<String> {
        vec![format!("G1 X{:.3} Y{:.3}{}", x, y, speed_cmd)]
    }

    fn laser_on(&self, power: f32) -> Vec<String> {
        vec![format!("M3 S{:.0}", power)]
    }

    fn laser_off(&self) -> Vec<String> {
        vec!["M5".into()]
    }

    fn set_speed(&self, speed: f32) -> String {
        format!(" F{:.0}", speed)
    }

    fn air_assist_on(&self) -> String {
        "M8".into()
    }

    fn air_assist_off(&self) -> String {
        "M9".into()
    }

    fn exhaust_on(&self) -> String {
        "M7".into()
    }

    fn exhaust_off(&self) -> String {
        "M9".into()
    }

    fn dwell(&self, seconds: f32) -> String {
        format!("G4 P{:.1}", seconds)
    }

    fn z_move(&self, z: f32) -> String {
        format!("G0 Z{:.2}", z)
    }

    fn footer(&self) -> Vec<String> {
        vec!["M5".into(), "G0 X0.000 Y0.000".into()]
    }

    fn comment(&self, text: &str) -> String {
        format!("; {}", text)
    }

    fn kind(&self) -> ProtocolKind {
        ProtocolKind::Grbl
    }
}

// ─── Marlin ──────────────────────────────────────────────────────────────────

pub struct MarlinProtocol;

impl OutputProtocol for MarlinProtocol {
    fn preamble(&self) -> Vec<String> {
        vec![
            "G21".into(),   // mm
            "G90".into(),   // absolute positioning
            "M05".into(),   // laser off
        ]
    }

    fn rapid(&self, x: f32, y: f32) -> Vec<String> {
        // Marlin: ensure laser off before travel
        vec![format!("G0 X{:.3} Y{:.3}", x, y)]
    }

    fn linear(&self, x: f32, y: f32, speed_cmd: &str) -> Vec<String> {
        vec![format!("G1 X{:.3} Y{:.3}{}", x, y, speed_cmd)]
    }

    fn laser_on(&self, power: f32) -> Vec<String> {
        // Marlin uses M03 Sxxx for laser power
        vec![format!("M03 S{:.0}", power)]
    }

    fn laser_off(&self) -> Vec<String> {
        vec!["M05".into()]
    }

    fn set_speed(&self, speed: f32) -> String {
        format!(" F{:.0}", speed)
    }

    fn air_assist_on(&self) -> String {
        "M7".into() // Marlin mist coolant = air assist
    }

    fn air_assist_off(&self) -> String {
        "M9".into()
    }

    fn exhaust_on(&self) -> String {
        "M106".into() // Fan on (generic)
    }

    fn exhaust_off(&self) -> String {
        "M107".into() // Fan off
    }

    fn dwell(&self, seconds: f32) -> String {
        // Marlin G4 uses milliseconds with P or seconds with S
        format!("G4 S{:.1}", seconds)
    }

    fn z_move(&self, z: f32) -> String {
        format!("G0 Z{:.2}", z)
    }

    fn footer(&self) -> Vec<String> {
        vec![
            "M05".into(),
            "G0 X0 Y0".into(),
            "M09".into(), // air assist off
        ]
    }

    fn comment(&self, text: &str) -> String {
        format!("; {}", text)
    }

    fn kind(&self) -> ProtocolKind {
        ProtocolKind::Marlin
    }
}

// ─── EGV (K40 / Lihuiyu / LHYMICRO-GL) ─────────────────────────────────────

/// EGV protocol for K40 Chinese laser cutters.
/// This is a compact binary-like protocol, NOT standard GCode.
/// We emit the text-encoded form that can be sent via USB to the M2 Nano board.
pub struct EgvProtocol {
    /// Maximum Y height in mm (needed for coordinate flip)
    pub max_height: f32,
}

impl Default for EgvProtocol {
    fn default() -> Self {
        Self { max_height: 300.0 }
    }
}

impl EgvProtocol {
    /// Encode a distance as EGV nano-distance string
    fn nano_distance(v: u32) -> String {
        if v == 0 {
            return "0".into();
        }
        let mut result = String::new();
        let mut remaining = v;
        // Leading 'z' characters for multiples of 256
        while remaining > 255 {
            result.push('z');
            remaining -= 255;
        }
        if remaining > 52 {
            result.push_str(&format!("{:03}", remaining));
        } else if remaining > 0 {
            // Distance lookup table (a-z = 1-26, A-Z = 27-52)
            let ch = if remaining <= 26 {
                (b'a' + remaining as u8 - 1) as char
            } else {
                (b'A' + remaining as u8 - 27) as char
            };
            result.push(ch);
        }
        result
    }
}

impl OutputProtocol for EgvProtocol {
    fn preamble(&self) -> Vec<String> {
        // EGV preamble: Initialize
        vec!["IPP".into()]
    }

    fn rapid(&self, x: f32, y: f32) -> Vec<String> {
        // EGV: rapid = move with laser off (U prefix)
        let xi = (x * 1000.0 / 25.4) as u32; // convert mm to mils (K40 uses mils internally)
        let yi = ((self.max_height - y) * 1000.0 / 25.4) as u32;
        vec![format!("U{}_{}", Self::nano_distance(xi), Self::nano_distance(yi))]
    }

    fn linear(&self, x: f32, y: f32, _speed_cmd: &str) -> Vec<String> {
        let xi = (x * 1000.0 / 25.4) as u32;
        let yi = ((self.max_height - y) * 1000.0 / 25.4) as u32;
        vec![format!("D{}_{}", Self::nano_distance(xi), Self::nano_distance(yi))]
    }

    fn laser_on(&self, _power: f32) -> Vec<String> {
        vec!["D".into()] // EGV laser on
    }

    fn laser_off(&self) -> Vec<String> {
        vec!["U".into()] // EGV laser off
    }

    fn set_speed(&self, speed: f32) -> String {
        // EGV speed is set via CV command (speed in mm/s for board)
        let speed_val = (speed / 60.0) as u32; // mm/min -> mm/s
        format!("CV{}", speed_val.max(1))
    }

    fn air_assist_on(&self) -> String {
        String::new() // K40 doesn't have GCode air assist
    }

    fn air_assist_off(&self) -> String {
        String::new()
    }

    fn exhaust_on(&self) -> String {
        String::new()
    }

    fn exhaust_off(&self) -> String {
        String::new()
    }

    fn dwell(&self, _seconds: f32) -> String {
        String::new() // Not supported in EGV
    }

    fn z_move(&self, _z: f32) -> String {
        String::new() // K40 doesn't have Z
    }

    fn footer(&self) -> Vec<String> {
        vec!["FNSE-".into()] // EGV end sequence
    }

    fn comment(&self, _text: &str) -> String {
        String::new() // EGV doesn't support comments
    }

    fn kind(&self) -> ProtocolKind {
        ProtocolKind::Egv
    }

    fn is_gcode_based(&self) -> bool {
        false
    }
}

// ─── Factory ─────────────────────────────────────────────────────────────────

pub fn create_protocol(kind: ProtocolKind) -> Box<dyn OutputProtocol> {
    match kind {
        ProtocolKind::Grbl => Box::new(GrblProtocol),
        ProtocolKind::Marlin => Box::new(MarlinProtocol),
        ProtocolKind::Egv => Box::new(EgvProtocol::default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grbl_rapid_format() {
        let p = GrblProtocol;
        let cmds = p.rapid(10.5, 20.3);
        assert_eq!(cmds, vec!["G0 X10.500 Y20.300"]);
    }

    #[test]
    fn grbl_laser_on_off() {
        let p = GrblProtocol;
        assert_eq!(p.laser_on(500.0), vec!["M3 S500"]);
        assert_eq!(p.laser_off(), vec!["M5"]);
    }

    #[test]
    fn marlin_preamble_includes_m05() {
        let p = MarlinProtocol;
        let pre = p.preamble();
        assert!(pre.contains(&"M05".to_string()));
    }

    #[test]
    fn marlin_laser_uses_m03() {
        let p = MarlinProtocol;
        let cmds = p.laser_on(800.0);
        assert_eq!(cmds, vec!["M03 S800"]);
    }

    #[test]
    fn egv_is_not_gcode_based() {
        let p = EgvProtocol::default();
        assert!(!p.is_gcode_based());
    }

    #[test]
    fn egv_nano_distance_small() {
        assert_eq!(EgvProtocol::nano_distance(1), "a");
        assert_eq!(EgvProtocol::nano_distance(26), "z");
        assert_eq!(EgvProtocol::nano_distance(27), "A");
        assert_eq!(EgvProtocol::nano_distance(52), "Z");
    }

    #[test]
    fn egv_nano_distance_large() {
        assert_eq!(EgvProtocol::nano_distance(100), "100");
        assert_eq!(EgvProtocol::nano_distance(255), "255");
        assert_eq!(EgvProtocol::nano_distance(256), "za");
    }

    #[test]
    fn factory_returns_correct_kind() {
        let p = create_protocol(ProtocolKind::Grbl);
        assert_eq!(p.kind(), ProtocolKind::Grbl);
        let p = create_protocol(ProtocolKind::Marlin);
        assert_eq!(p.kind(), ProtocolKind::Marlin);
        let p = create_protocol(ProtocolKind::Egv);
        assert_eq!(p.kind(), ProtocolKind::Egv);
    }

    #[test]
    fn all_protocols_have_labels() {
        for kind in ProtocolKind::ALL {
            assert!(!kind.label().is_empty());
        }
    }
}
