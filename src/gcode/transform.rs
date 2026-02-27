pub fn apply_rotary(
    cmd: &str,
    diameter_mm: f32,
    axis: char
) -> String {
    if diameter_mm <= 0.1 { return cmd.to_string(); }

    // Parse G-code line briefly
    // This is a naive implementation; a full parser is better but for transform this might suffice
    // assuming 'G0 X.. Y..' format.

    // If command is not a move, return
    if !cmd.starts_with('G') { return cmd.to_string(); }

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
