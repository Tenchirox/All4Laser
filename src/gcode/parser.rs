use super::types::GCodeLine;

/// Parse a single GCode line into a structured command
pub fn parse_line(raw: &str) -> GCodeLine {
    let raw = raw.trim();
    // Strip comments
    let code = if let Some(idx) = raw.find(';') {
        &raw[..idx]
    } else if let Some(idx) = raw.find('(') {
        &raw[..idx]
    } else {
        raw
    };
    let code = code.trim().to_uppercase();

    let mut line = GCodeLine {
        raw: raw.to_string(),
        g_code: None,
        m_code: None,
        x: None, y: None, z: None,
        f: None, s: None,
        i: None, j: None,
    };

    let mut chars = code.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            'G' => line.g_code = Some(extract_int(&mut chars)),
            'M' => line.m_code = Some(extract_int(&mut chars)),
            'X' => line.x = Some(extract_float(&mut chars)),
            'Y' => line.y = Some(extract_float(&mut chars)),
            'Z' => line.z = Some(extract_float(&mut chars)),
            'F' => line.f = Some(extract_float(&mut chars)),
            'S' => line.s = Some(extract_float(&mut chars)),
            'I' => line.i = Some(extract_float(&mut chars)),
            'J' => line.j = Some(extract_float(&mut chars)),
            _ => {}
        }
    }

    line
}

fn extract_int(chars: &mut std::iter::Peekable<std::str::Chars>) -> i32 {
    let mut s = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '-' || c == '+' {
            s.push(c);
            chars.next();
        } else {
            break;
        }
    }
    s.parse().unwrap_or(0)
}

fn extract_float(chars: &mut std::iter::Peekable<std::str::Chars>) -> f32 {
    let mut s = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '.' || c == '-' || c == '+' {
            s.push(c);
            chars.next();
        } else {
            break;
        }
    }
    s.parse().unwrap_or(0.0)
}
