use super::types::*;

/// Parse a GRBL status report: `<Idle|MPos:0.000,0.000,0.000|FS:0,0|Ov:100,100,100>`
pub fn parse_status(line: &str) -> Option<GrblState> {
    let line = line.trim();
    if !line.starts_with('<') || !line.ends_with('>') {
        return None;
    }
    let inner = &line[1..line.len() - 1];
    let parts: Vec<&str> = inner.split('|').collect();
    if parts.is_empty() {
        return None;
    }

    let mut state = GrblState::default();
    state.status = MacStatus::from_str(parts[0]);

    let mut has_mpos = false;
    let mut has_wpos = false;

    for part in &parts[1..] {
        if let Some(val) = part.strip_prefix("MPos:") {
            if let Some(p) = parse_point(val) {
                state.mpos = p;
                has_mpos = true;
            }
        } else if let Some(val) = part.strip_prefix("WPos:") {
            if let Some(p) = parse_point(val) {
                state.wpos = p;
                has_wpos = true;
            }
        } else if let Some(val) = part.strip_prefix("WCO:") {
            if let Some(p) = parse_point(val) {
                state.wco = p;
            }
        } else if let Some(val) = part.strip_prefix("FS:") {
            let nums: Vec<&str> = val.split(',').collect();
            if nums.len() >= 2 {
                state.feed_rate = nums[0].parse().unwrap_or(0.0);
                state.spindle_speed = nums[1].parse().unwrap_or(0.0);
            }
        } else if let Some(val) = part.strip_prefix("F:") {
            state.feed_rate = val.parse().unwrap_or(0.0);
        } else if let Some(val) = part.strip_prefix("Ov:") {
            let nums: Vec<&str> = val.split(',').collect();
            if nums.len() >= 3 {
                state.override_feed = nums[0].parse().unwrap_or(100);
                state.override_rapid = nums[1].parse().unwrap_or(100);
                state.override_spindle = nums[2].parse().unwrap_or(100);
            }
        } else if let Some(val) = part.strip_prefix("Bf:") {
            let nums: Vec<&str> = val.split(',').collect();
            if nums.len() >= 2 {
                state.buffer_plan = nums[0].parse().unwrap_or(0);
                state.buffer_rx = nums[1].parse().unwrap_or(0);
            }
        }
    }

    // Derive WPos from MPos and WCO if needed
    if has_mpos && !has_wpos {
        state.wpos = state.mpos - state.wco;
    }
    if has_wpos && !has_mpos {
        state.mpos = GPoint::new(
            state.wpos.x + state.wco.x,
            state.wpos.y + state.wco.y,
            state.wpos.z + state.wco.z,
        );
    }

    Some(state)
}

/// Parse a GRBL response line
pub fn parse_response(line: &str) -> GrblResponse {
    let line = line.trim();

    if line == "ok" {
        return GrblResponse::Ok;
    }
    if let Some(code) = line.strip_prefix("error:") {
        return GrblResponse::Error(code.parse().unwrap_or(-1));
    }
    if let Some(code) = line.strip_prefix("ALARM:") {
        return GrblResponse::Alarm(code.parse().unwrap_or(-1));
    }
    if line.starts_with('<') {
        if let Some(state) = parse_status(line) {
            return GrblResponse::Status(state);
        }
    }
    if let Some(ver) = line.strip_prefix("Grbl ") {
        return GrblResponse::GrblVersion(ver.to_string());
    }
    if line.starts_with('$') && line.contains('=') {
        let parts: Vec<&str> = line[1..].splitn(2, '=').collect();
        if parts.len() == 2 {
            if let Ok(id) = parts[0].parse::<i32>() {
                return GrblResponse::Setting(id, parts[1].to_string());
            }
        }
    }

    GrblResponse::Message(line.to_string())
}

fn parse_point(s: &str) -> Option<GPoint> {
    let nums: Vec<&str> = s.split(',').collect();
    if nums.len() >= 3 {
        Some(GPoint::new(
            nums[0].parse().ok()?,
            nums[1].parse().ok()?,
            nums[2].parse().ok()?,
        ))
    } else {
        None
    }
}
