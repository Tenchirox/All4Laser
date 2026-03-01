use std::sync::Arc;

use crate::grbl::{parser, protocol};
use crate::grbl::types::{GPoint, GrblResponse, GrblState, JogDirection, MacStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ControllerKind {
    Grbl,
    Ruida,
    Trocen,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grbl_backend_has_full_capabilities() {
        let backend = create_backend(ControllerKind::Grbl);
        let caps = backend.capabilities();
        assert!(caps.supports_status_poll);
        assert!(caps.supports_hold_resume);
        assert!(caps.supports_reset);
        assert!(caps.supports_feed_override);
        assert!(caps.supports_rapid_override);
        assert!(caps.supports_spindle_override);
        assert!(caps.supports_jog);
        assert!(caps.supports_home);
        assert!(caps.supports_unlock);
        assert!(caps.supports_grbl_settings);
    }

    #[test]
    fn line_backend_disables_grbl_specific_features() {
        let backend = create_backend(ControllerKind::Ruida);
        let caps = backend.capabilities();
        assert!(caps.supports_status_poll);
        assert!(caps.supports_hold_resume);
        assert!(caps.supports_reset);
        assert!(caps.supports_jog);
        assert!(!caps.supports_feed_override);
        assert!(!caps.supports_rapid_override);
        assert!(!caps.supports_spindle_override);
        assert!(!caps.supports_home);
        assert!(!caps.supports_unlock);
        assert!(!caps.supports_grbl_settings);
    }

    #[test]
    fn line_backend_maps_realtime_lines() {
        let backend = create_backend(ControllerKind::Trocen);
        assert_eq!(
            backend.realtime_line(RealtimeCommand::StatusReport),
            Some("status")
        );
        assert_eq!(
            backend.realtime_line(RealtimeCommand::FeedHold),
            Some("pause")
        );
        assert_eq!(
            backend.realtime_line(RealtimeCommand::CycleStart),
            Some("resume")
        );
        assert_eq!(backend.realtime_line(RealtimeCommand::Reset), Some("stop"));
        assert_eq!(backend.realtime_line(RealtimeCommand::FeedOverridePlus10), None);
    }

    #[test]
    fn line_backend_parses_status_and_coords() {
        let backend = create_backend(ControllerKind::Ruida);
        let parsed = backend.parse_response("RUN X12.5 Y-4.0 Z1.25");
        match parsed {
            ControllerResponse::Grbl(GrblResponse::Status(state)) => {
                assert_eq!(state.status, MacStatus::Run);
                assert!((state.mpos.x - 12.5).abs() < 1e-3);
                assert!((state.mpos.y + 4.0).abs() < 1e-3);
                assert!((state.mpos.z - 1.25).abs() < 1e-3);
            }
            other => panic!("Unexpected parse result: {other:?}"),
        }
    }

    #[test]
    fn line_backend_parses_ok_and_error() {
        let backend = create_backend(ControllerKind::Ruida);

        match backend.parse_response("ok") {
            ControllerResponse::Grbl(GrblResponse::Ok) => {}
            other => panic!("Unexpected ok parse result: {other:?}"),
        }

        match backend.parse_response("error:23") {
            ControllerResponse::Grbl(GrblResponse::Error(23)) => {}
            other => panic!("Unexpected error parse result: {other:?}"),
        }
    }
}

impl Default for ControllerKind {
    fn default() -> Self {
        Self::Grbl
    }
}

impl ControllerKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Grbl => "GRBL",
            Self::Ruida => "Ruida (beta)",
            Self::Trocen => "Trocen (beta)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealtimeCommand {
    StatusReport,
    CycleStart,
    FeedHold,
    Reset,
    FeedOverrideReset,
    FeedOverridePlus10,
    FeedOverrideMinus10,
    FeedOverridePlus1,
    FeedOverrideMinus1,
    RapidOverride100,
    RapidOverride50,
    RapidOverride25,
    SpindleOverrideReset,
    SpindleOverridePlus10,
    SpindleOverrideMinus10,
    SpindleOverridePlus1,
    SpindleOverrideMinus1,
}

#[derive(Debug, Clone, Copy)]
pub struct ControllerCapabilities {
    pub supports_status_poll: bool,
    pub supports_hold_resume: bool,
    pub supports_reset: bool,
    pub supports_feed_override: bool,
    pub supports_rapid_override: bool,
    pub supports_spindle_override: bool,
    pub supports_jog: bool,
    pub supports_home: bool,
    pub supports_unlock: bool,
    pub supports_grbl_settings: bool,
}

#[derive(Debug, Clone)]
pub enum ControllerResponse {
    Grbl(GrblResponse),
    Message,
}

pub trait ControllerBackend: Send + Sync {
    fn capabilities(&self) -> ControllerCapabilities;
    fn parse_response(&self, line: &str) -> ControllerResponse;
    fn realtime_byte(&self, command: RealtimeCommand) -> Option<u8>;
    fn realtime_line(&self, command: RealtimeCommand) -> Option<&'static str>;
    fn jog_command(&self, dir: JogDirection, step: f32, speed: f32) -> Option<String>;
}

#[derive(Default)]
struct GrblBackend;

impl ControllerBackend for GrblBackend {
    fn capabilities(&self) -> ControllerCapabilities {
        ControllerCapabilities {
            supports_status_poll: true,
            supports_hold_resume: true,
            supports_reset: true,
            supports_feed_override: true,
            supports_rapid_override: true,
            supports_spindle_override: true,
            supports_jog: true,
            supports_home: true,
            supports_unlock: true,
            supports_grbl_settings: true,
        }
    }

    fn parse_response(&self, line: &str) -> ControllerResponse {
        ControllerResponse::Grbl(parser::parse_response(line))
    }

    fn realtime_byte(&self, command: RealtimeCommand) -> Option<u8> {
        let byte = match command {
            RealtimeCommand::StatusReport => protocol::CMD_STATUS_REPORT,
            RealtimeCommand::CycleStart => protocol::CMD_CYCLE_START,
            RealtimeCommand::FeedHold => protocol::CMD_FEED_HOLD,
            RealtimeCommand::Reset => protocol::CMD_RESET,
            RealtimeCommand::FeedOverrideReset => protocol::FEED_OV_RESET,
            RealtimeCommand::FeedOverridePlus10 => protocol::FEED_OV_PLUS_10,
            RealtimeCommand::FeedOverrideMinus10 => protocol::FEED_OV_MINUS_10,
            RealtimeCommand::FeedOverridePlus1 => protocol::FEED_OV_PLUS_1,
            RealtimeCommand::FeedOverrideMinus1 => protocol::FEED_OV_MINUS_1,
            RealtimeCommand::RapidOverride100 => protocol::RAPID_OV_100,
            RealtimeCommand::RapidOverride50 => protocol::RAPID_OV_50,
            RealtimeCommand::RapidOverride25 => protocol::RAPID_OV_25,
            RealtimeCommand::SpindleOverrideReset => protocol::SPINDLE_OV_RESET,
            RealtimeCommand::SpindleOverridePlus10 => protocol::SPINDLE_OV_PLUS_10,
            RealtimeCommand::SpindleOverrideMinus10 => protocol::SPINDLE_OV_MINUS_10,
            RealtimeCommand::SpindleOverridePlus1 => protocol::SPINDLE_OV_PLUS_1,
            RealtimeCommand::SpindleOverrideMinus1 => protocol::SPINDLE_OV_MINUS_1,
        };
        Some(byte)
    }

    fn realtime_line(&self, _command: RealtimeCommand) -> Option<&'static str> {
        None
    }

    fn jog_command(&self, dir: JogDirection, step: f32, speed: f32) -> Option<String> {
        Some(protocol::jog_command(dir, step, speed))
    }
}

struct LineProtocolBackend {
    kind: ControllerKind,
}

impl ControllerBackend for LineProtocolBackend {
    fn capabilities(&self) -> ControllerCapabilities {
        let _ = self.kind;
        ControllerCapabilities {
            supports_status_poll: true,
            supports_hold_resume: true,
            supports_reset: true,
            supports_feed_override: false,
            supports_rapid_override: false,
            supports_spindle_override: false,
            supports_jog: true,
            supports_home: false,
            supports_unlock: false,
            supports_grbl_settings: false,
        }
    }

    fn parse_response(&self, line: &str) -> ControllerResponse {
        if let Some(response) = parse_line_protocol_response(line) {
            return ControllerResponse::Grbl(response);
        }
        ControllerResponse::Message
    }

    fn realtime_byte(&self, _command: RealtimeCommand) -> Option<u8> {
        None
    }

    fn realtime_line(&self, command: RealtimeCommand) -> Option<&'static str> {
        match self.kind {
            ControllerKind::Ruida | ControllerKind::Trocen => match command {
                // Best-effort text protocol fallback for line-oriented serial bridges.
                RealtimeCommand::StatusReport => Some("status"),
                RealtimeCommand::CycleStart => Some("resume"),
                RealtimeCommand::FeedHold => Some("pause"),
                RealtimeCommand::Reset => Some("stop"),
                _ => None,
            },
            ControllerKind::Grbl => None,
        }
    }

    fn jog_command(&self, dir: JogDirection, step: f32, speed: f32) -> Option<String> {
        match self.kind {
            ControllerKind::Ruida | ControllerKind::Trocen => {
                let cmd = match dir {
                    JogDirection::Home => format!("G90 G0 X0 Y0 F{speed}"),
                    JogDirection::Zup => format!("G91 G0 Z{step:.1} F{speed}"),
                    JogDirection::Zdown => format!("G91 G0 Z-{step:.1} F{speed}"),
                    _ => {
                        let (dx, dy) = direction_delta(dir);
                        format!("G91 G0 X{:.1} Y{:.1} F{speed}", dx * step, dy * step)
                    }
                };
                Some(cmd)
            }
            ControllerKind::Grbl => None,
        }
    }
}

pub fn create_backend(kind: ControllerKind) -> Arc<dyn ControllerBackend> {
    match kind {
        ControllerKind::Grbl => Arc::new(GrblBackend),
        ControllerKind::Ruida => Arc::new(LineProtocolBackend {
            kind: ControllerKind::Ruida,
        }),
        ControllerKind::Trocen => Arc::new(LineProtocolBackend {
            kind: ControllerKind::Trocen,
        }),
    }
}

fn parse_line_protocol_response(line: &str) -> Option<GrblResponse> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    let lower = trimmed.to_ascii_lowercase();

    if lower == "ok" || lower == "ready" || lower == "done" {
        return Some(GrblResponse::Ok);
    }

    if let Some(err) = lower.strip_prefix("error") {
        return Some(GrblResponse::Error(extract_int(err).unwrap_or(-1)));
    }

    if let Some(alarm) = lower.strip_prefix("alarm") {
        return Some(GrblResponse::Alarm(extract_int(alarm).unwrap_or(-1)));
    }

    if let Some(status) = infer_status(&lower) {
        let mut state = GrblState::default();
        state.status = status;

        let x = extract_axis_value(trimmed, 'X');
        let y = extract_axis_value(trimmed, 'Y');
        let z = extract_axis_value(trimmed, 'Z');
        if x.is_some() || y.is_some() || z.is_some() {
            state.mpos = GPoint::new(x.unwrap_or(0.0), y.unwrap_or(0.0), z.unwrap_or(0.0));
            state.wpos = state.mpos;
        }

        return Some(GrblResponse::Status(state));
    }

    None
}

fn infer_status(line_lower: &str) -> Option<MacStatus> {
    if line_lower.contains("idle") || line_lower.contains("ready") {
        return Some(MacStatus::Idle);
    }
    if line_lower.contains("run")
        || line_lower.contains("busy")
        || line_lower.contains("working")
        || line_lower.contains("cutting")
        || line_lower.contains("engraving")
    {
        return Some(MacStatus::Run);
    }
    if line_lower.contains("hold") || line_lower.contains("pause") {
        return Some(MacStatus::Hold);
    }
    if line_lower.contains("jog") {
        return Some(MacStatus::Jog);
    }
    if line_lower.contains("alarm") {
        return Some(MacStatus::Alarm);
    }
    if line_lower.contains("door") {
        return Some(MacStatus::Door);
    }
    if line_lower.contains("home") {
        return Some(MacStatus::Home);
    }
    if line_lower.contains("sleep") {
        return Some(MacStatus::Sleep);
    }
    None
}

fn extract_axis_value(line: &str, axis: char) -> Option<f32> {
    let bytes = line.as_bytes();
    let axis_lower = axis.to_ascii_lowercase() as u8;
    let axis_upper = axis.to_ascii_uppercase() as u8;

    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == axis_lower || b == axis_upper {
            i += 1;
            while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b':' || bytes[i] == b'=') {
                i += 1;
            }
            let start = i;
            while i < bytes.len()
                && (bytes[i].is_ascii_digit()
                    || bytes[i] == b'.'
                    || bytes[i] == b'-'
                    || bytes[i] == b'+')
            {
                i += 1;
            }
            if start < i {
                if let Ok(v) = line[start..i].parse::<f32>() {
                    return Some(v);
                }
            }
        } else {
            i += 1;
        }
    }
    None
}

fn extract_int(fragment: &str) -> Option<i32> {
    let mut out = String::new();
    for c in fragment.chars() {
        if c.is_ascii_digit() || (out.is_empty() && c == '-') {
            out.push(c);
        } else if !out.is_empty() {
            break;
        }
    }
    if out.is_empty() {
        None
    } else {
        out.parse::<i32>().ok()
    }
}
