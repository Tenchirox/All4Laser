pub fn apply_rotary(cmd: &str, diameter_mm: f32, axis: char) -> String {
    if diameter_mm <= 0.1 {
        return cmd.to_string();
    }

    // Parse G-code line briefly
    // This is a naive implementation; a full parser is better but for transform this might suffice
    // assuming 'G0 X.. Y..' format.

    // If command is not a move, return
    if !cmd.starts_with('G') {
        return cmd.to_string();
    }

    // We need to find Y value (or whatever axis is being rotated) and convert it.
    // Scale factor: degrees per mm = 360 / (PI * D)
    let scale = 360.0 / (std::f32::consts::PI * diameter_mm);

    let mut parts: Vec<String> = Vec::new();
    for token in cmd.split_whitespace() {
        if token.starts_with('Y') && axis == 'Y' {
            if let Ok(val) = token[1..].parse::<f32>() {
                parts.push(format!("Y{:.3}", val * scale)); // Convert mm to degrees?
            // Wait, if it's a roller (Y axis), usually it stays in mm if steps/mm is set for roller diameter.
            // But if it's a chuck/A-axis, we need degrees.
            // Let's assume 'Y' axis replacement means we send degrees to Y because Y steps/mm is set to 1 step/degree?
            // Or maybe we output to A?

            // Let's implement: "Map Y mm to A degrees" if axis == 'A'.
            // If axis == 'Y', we assume we are re-mapping Y-mm to Y-degrees (if the machine expects degrees).
            } else {
                parts.push(token.to_string());
            }
        } else if token.starts_with('Y') && axis == 'A' {
            if let Ok(val) = token[1..].parse::<f32>() {
                parts.push(format!("A{:.3}", val * scale));
            } else {
                parts.push(token.to_string());
            }
        } else {
            parts.push(token.to_string());
        }
    }
    parts.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_rotary_diameter_too_small() {
        assert_eq!(apply_rotary("G0 Y10", 0.1, 'Y'), "G0 Y10");
        assert_eq!(apply_rotary("G0 Y10", 0.05, 'Y'), "G0 Y10");
        assert_eq!(apply_rotary("G0 Y10", 0.0, 'Y'), "G0 Y10");
        assert_eq!(apply_rotary("G0 Y10", -1.0, 'Y'), "G0 Y10");
    }

    #[test]
    fn test_apply_rotary_non_gcode() {
        assert_eq!(apply_rotary("M3 S1000", 50.0, 'Y'), "M3 S1000");
        assert_eq!(apply_rotary("M5", 50.0, 'Y'), "M5");
        assert_eq!(apply_rotary("", 50.0, 'Y'), "");
        assert_eq!(apply_rotary("  ", 50.0, 'Y'), "  ");
        assert_eq!(apply_rotary(";", 50.0, 'Y'), ";");
    }

    #[test]
    fn test_apply_rotary_no_y_axis() {
        assert_eq!(apply_rotary("G0 X10 Z5", 50.0, 'Y'), "G0 X10 Z5");
        assert_eq!(apply_rotary("G1 X20.5 F3000", 50.0, 'A'), "G1 X20.5 F3000");
    }

    #[test]
    fn test_apply_rotary_y_axis_to_y_degrees() {
        let scale = 360.0 / (std::f32::consts::PI * 50.0);
        let expected_y = format!("Y{:.3}", 10.0 * scale);

        let cmd = "G0 X10 Y10";
        let result = apply_rotary(cmd, 50.0, 'Y');
        assert_eq!(result, format!("G0 X10 {}", expected_y));
    }

    #[test]
    fn test_apply_rotary_y_axis_to_a_degrees() {
        let scale = 360.0 / (std::f32::consts::PI * 50.0);
        let expected_a = format!("A{:.3}", 10.0 * scale);

        let cmd = "G1 X15 Y10 F1000";
        let result = apply_rotary(cmd, 50.0, 'A');
        assert_eq!(result, format!("G1 X15 {} F1000", expected_a));
    }

    #[test]
    fn test_apply_rotary_unparseable_y() {
        assert_eq!(apply_rotary("G0 X10 Yabc", 50.0, 'Y'), "G0 X10 Yabc");
        assert_eq!(apply_rotary("G1 Y F1000", 50.0, 'A'), "G1 Y F1000");
    }

    #[test]
    fn test_apply_rotary_different_axis_target() {
        assert_eq!(apply_rotary("G0 X10 Y10", 50.0, 'Z'), "G0 X10 Y10");
        assert_eq!(apply_rotary("G0 X10 Y10", 50.0, 'X'), "G0 X10 Y10");
    }

    #[test]
    fn test_apply_rotary_multiple_y() {
        let scale = 360.0 / (std::f32::consts::PI * 50.0);
        let expected_y1 = format!("Y{:.3}", 10.0 * scale);
        let expected_y2 = format!("Y{:.3}", 20.0 * scale);

        let cmd = "G0 Y10 Y20";
        let result = apply_rotary(cmd, 50.0, 'Y');
        assert_eq!(result, format!("G0 {} {}", expected_y1, expected_y2));
    }
}
