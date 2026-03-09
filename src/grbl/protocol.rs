#![allow(dead_code)]

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
        JogDirection::Home => vec!["G90".to_string(), format!("G1X0Y0F{speed}")],
        _ => {
            let (dx, dy) = direction_delta(dir);
            let dz = match dir {
                JogDirection::Zup => step,
                JogDirection::Zdown => -step,
                _ => 0.0,
            };
            let mut cmd = "G1".to_string();
            if dx != 0.0 {
                cmd += &format!("X{:.1}", dx * step);
            }
            if dy != 0.0 {
                cmd += &format!("Y{:.1}", dy * step);
            }
            if dz != 0.0 {
                cmd += &format!("Z{dz:.1}");
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grbl::types::JogDirection;

    #[test]
    fn test_direction_delta() {
        assert_eq!(direction_delta(JogDirection::N), (0.0, 1.0));
        assert_eq!(direction_delta(JogDirection::S), (0.0, -1.0));
        assert_eq!(direction_delta(JogDirection::E), (1.0, 0.0));
        assert_eq!(direction_delta(JogDirection::W), (-1.0, 0.0));
        assert_eq!(direction_delta(JogDirection::NE), (1.0, 1.0));
        assert_eq!(direction_delta(JogDirection::NW), (-1.0, 1.0));
        assert_eq!(direction_delta(JogDirection::SE), (1.0, -1.0));
        assert_eq!(direction_delta(JogDirection::SW), (-1.0, -1.0));
        assert_eq!(direction_delta(JogDirection::Zup), (0.0, 0.0));
        assert_eq!(direction_delta(JogDirection::Zdown), (0.0, 0.0));
        assert_eq!(direction_delta(JogDirection::Home), (0.0, 0.0));
    }

    #[test]
    fn test_jog_command() {
        assert_eq!(jog_command(JogDirection::Home, 10.0, 1000.0), "$J=G90X0Y0F1000");
        assert_eq!(jog_command(JogDirection::Zup, 5.0, 500.0), "$J=G91Z5.0F500");
        assert_eq!(jog_command(JogDirection::Zdown, 2.5, 300.0), "$J=G91Z-2.5F300");

        assert_eq!(jog_command(JogDirection::N, 10.0, 1000.0), "$J=G91X0.0Y10.0F1000");
        assert_eq!(jog_command(JogDirection::S, 5.0, 800.0), "$J=G91X0.0Y-5.0F800");
        assert_eq!(jog_command(JogDirection::E, 2.0, 200.0), "$J=G91X2.0Y0.0F200");
        assert_eq!(jog_command(JogDirection::W, 1.5, 100.0), "$J=G91X-1.5Y0.0F100");

        assert_eq!(jog_command(JogDirection::NE, 10.0, 1000.0), "$J=G91X10.0Y10.0F1000");
        assert_eq!(jog_command(JogDirection::SW, 5.0, 500.0), "$J=G91X-5.0Y-5.0F500");
    }

    #[test]
    fn test_jog_command_legacy() {
        assert_eq!(
            jog_command_legacy(JogDirection::Home, 10.0, 1000.0),
            vec!["G90".to_string(), "G1X0Y0F1000".to_string()]
        );
        assert_eq!(
            jog_command_legacy(JogDirection::Zup, 5.0, 500.0),
            vec!["G91".to_string(), "G1Z5.0F500".to_string(), "G90".to_string()]
        );
        assert_eq!(
            jog_command_legacy(JogDirection::Zdown, 2.5, 300.0),
            vec!["G91".to_string(), "G1Z-2.5F300".to_string(), "G90".to_string()]
        );

        assert_eq!(
            jog_command_legacy(JogDirection::N, 10.0, 1000.0),
            vec!["G91".to_string(), "G1Y10.0F1000".to_string(), "G90".to_string()]
        );
        assert_eq!(
            jog_command_legacy(JogDirection::S, 5.0, 800.0),
            vec!["G91".to_string(), "G1Y-5.0F800".to_string(), "G90".to_string()]
        );
        assert_eq!(
            jog_command_legacy(JogDirection::E, 2.0, 200.0),
            vec!["G91".to_string(), "G1X2.0F200".to_string(), "G90".to_string()]
        );
        assert_eq!(
            jog_command_legacy(JogDirection::W, 1.5, 100.0),
            vec!["G91".to_string(), "G1X-1.5F100".to_string(), "G90".to_string()]
        );

        assert_eq!(
            jog_command_legacy(JogDirection::NE, 10.0, 1000.0),
            vec!["G91".to_string(), "G1X10.0Y10.0F1000".to_string(), "G90".to_string()]
        );
        assert_eq!(
            jog_command_legacy(JogDirection::SW, 5.0, 500.0),
            vec!["G91".to_string(), "G1X-5.0Y-5.0F500".to_string(), "G90".to_string()]
        );
    }
}
