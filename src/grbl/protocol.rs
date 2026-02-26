use super::types::JogDirection;

/// Build a jog command for GRBL v1.1+ ($J= syntax)
pub fn jog_command(dir: JogDirection, step: f32, speed: f32) -> String {
    match dir {
        JogDirection::Home => format!("$J=G90X0Y0F{speed}"),
        JogDirection::Zup => format!("$J=G91Z{step:.1}F{speed}"),
        JogDirection::Zdown => format!("$J=G91Z-{step:.1}F{speed}"),
        _ => {
            let (dx, dy) = direction_delta(dir);
            format!("$J=G91X{:.1}Y{:.1}F{speed}", dx * step, dy * step)
        }
    }
}

/// Build a legacy jog command for GRBL v0.9
pub fn jog_command_legacy(dir: JogDirection, step: f32, speed: f32) -> Vec<String> {
    match dir {
        JogDirection::Home => vec![
            "G90".to_string(),
            format!("G1X0Y0F{speed}"),
        ],
        _ => {
            let (dx, dy) = direction_delta(dir);
            let dz = match dir {
                JogDirection::Zup => step,
                JogDirection::Zdown => -step,
                _ => 0.0,
            };
            let mut cmd = "G1".to_string();
            if dx != 0.0 { cmd += &format!("X{:.1}", dx * step); }
            if dy != 0.0 { cmd += &format!("Y{:.1}", dy * step); }
            if dz != 0.0 { cmd += &format!("Z{dz:.1}"); }
            cmd += &format!("F{speed}");

            vec!["G91".to_string(), cmd, "G90".to_string()]
        }
    }
}

fn direction_delta(dir: JogDirection) -> (f32, f32) {
    match dir {
        JogDirection::N => (0.0, 1.0),
        JogDirection::S => (0.0, -1.0),
        JogDirection::E => (1.0, 0.0),
        JogDirection::W => (-1.0, 0.0),
        JogDirection::NE => (1.0, 1.0),
        JogDirection::NW => (-1.0, 1.0),
        JogDirection::SE => (1.0, -1.0),
        JogDirection::SW => (-1.0, -1.0),
        _ => (0.0, 0.0),
    }
}

/// GRBL real-time override bytes
pub const FEED_OV_RESET: u8 = 0x90;
pub const FEED_OV_PLUS_10: u8 = 0x91;
pub const FEED_OV_MINUS_10: u8 = 0x92;
pub const FEED_OV_PLUS_1: u8 = 0x93;
pub const FEED_OV_MINUS_1: u8 = 0x94;
pub const RAPID_OV_100: u8 = 0x95;
pub const RAPID_OV_50: u8 = 0x96;
pub const RAPID_OV_25: u8 = 0x97;
pub const SPINDLE_OV_RESET: u8 = 0x99;
pub const SPINDLE_OV_PLUS_10: u8 = 0x9A;
pub const SPINDLE_OV_MINUS_10: u8 = 0x9B;
pub const SPINDLE_OV_PLUS_1: u8 = 0x9C;
pub const SPINDLE_OV_MINUS_1: u8 = 0x9D;
pub const CMD_STATUS_REPORT: u8 = b'?';
pub const CMD_CYCLE_START: u8 = b'~';
pub const CMD_FEED_HOLD: u8 = b'!';
pub const CMD_RESET: u8 = 0x18;
