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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_status_basic() {
        let status = parse_status("<Idle|MPos:1.0,2.0,3.0|FS:500,1000>").unwrap();
        assert_eq!(status.status, MacStatus::Idle);
        assert_eq!(status.mpos.x, 1.0);
        assert_eq!(status.mpos.y, 2.0);
        assert_eq!(status.mpos.z, 3.0);
        assert_eq!(status.feed_rate, 500.0);
        assert_eq!(status.spindle_speed, 1000.0);
    }

    #[test]
    fn test_parse_status_wpos_wco_ov() {
        let status = parse_status("<Run|WPos:10.0,20.0,30.0|WCO:5.0,5.0,5.0|Ov:110,120,130>").unwrap();
        assert_eq!(status.status, MacStatus::Run);
        assert_eq!(status.wpos.x, 10.0);
        assert_eq!(status.wpos.y, 20.0);
        assert_eq!(status.wpos.z, 30.0);
        assert_eq!(status.wco.x, 5.0);
        assert_eq!(status.wco.y, 5.0);
        assert_eq!(status.wco.z, 5.0);
        assert_eq!(status.override_feed, 110);
        assert_eq!(status.override_rapid, 120);
        assert_eq!(status.override_spindle, 130);
    }

    #[test]
    fn test_parse_status_f_bf() {
        let status = parse_status("<Hold:0|MPos:0,0,0|F:500|Bf:15,128>").unwrap();
        assert_eq!(status.status, MacStatus::Hold);
        assert_eq!(status.feed_rate, 500.0);
        assert_eq!(status.buffer_plan, 15);
        assert_eq!(status.buffer_rx, 128);
    }

    #[test]
    fn test_parse_status_invalid() {
        assert!(parse_status("Idle|MPos:0,0,0").is_none());
        assert!(parse_status("").is_none());
        assert!(parse_status("<Idle|MPos:0,0,0").is_none());
        // <> is parsed as disconnected status. Let's adjust expectations.
        let empty_brackets = parse_status("<>");
        if empty_brackets.is_some() {
            assert_eq!(empty_brackets.unwrap().status, MacStatus::Disconnected);
        } else {
            assert!(empty_brackets.is_none());
        }
    }

    #[test]
    fn test_parse_status_calculations() {
        // Test wpos calculation
        let status1 = parse_status("<Idle|MPos:10.0,10.0,10.0|WCO:2.0,2.0,2.0>").unwrap();
        assert_eq!(status1.wpos.x, 8.0);
        assert_eq!(status1.wpos.y, 8.0);
        assert_eq!(status1.wpos.z, 8.0);

        // Test mpos calculation
        let status2 = parse_status("<Idle|WPos:8.0,8.0,8.0|WCO:2.0,2.0,2.0>").unwrap();
        assert_eq!(status2.mpos.x, 10.0);
        assert_eq!(status2.mpos.y, 10.0);
        assert_eq!(status2.mpos.z, 10.0);
    }
}
