pub struct GCodeBuilder {
    pub lines: Vec<String>,
    current_x: Option<f32>,
    current_y: Option<f32>,
    current_speed: Option<f32>,
    current_power: Option<f32>,
    laser_on: bool,
}

impl Default for GCodeBuilder {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            current_x: None,
            current_y: None,
            current_speed: None,
            current_power: None,
            laser_on: false,
        }
    }
}

impl GCodeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn comment(&mut self, text: &str) {
        self.lines.push(format!("; {}", text));
    }

    pub fn raw(&mut self, cmd: &str) {
        self.lines.push(cmd.to_string());
    }

    /// Reset state (e.g. at start of job)
    pub fn reset_state(&mut self) {
        self.current_x = None;
        self.current_y = None;
        self.current_speed = None;
        self.current_power = None;
        self.laser_on = false;
    }

    /// Rapid move (G0)
    pub fn rapid(&mut self, x: f32, y: f32) {
        if self.laser_on {
            self.laser_off();
        }

        if self.is_at(x, y) {
            return;
        }

        self.lines.push(format!("G0 X{:.3} Y{:.3}", x, y));
        self.current_x = Some(x);
        self.current_y = Some(y);
    }

    /// Linear cut/move (G1)
    pub fn linear(&mut self, x: f32, y: f32, speed: f32, power: f32) {
        // Ensure laser state
        if !self.laser_on || self.current_power != Some(power) {
            self.laser_on(power);
        }

        // Check if we are already there (zero length move)
        if self.is_at(x, y) {
            return;
        }

        // Optimize speed parameter: only send F if changed
        let speed_cmd = if self.current_speed != Some(speed) {
            self.current_speed = Some(speed);
            format!(" F{:.0}", speed)
        } else {
            String::new()
        };

        self.lines.push(format!("G1 X{:.3} Y{:.3}{}", x, y, speed_cmd));
        self.current_x = Some(x);
        self.current_y = Some(y);
    }

    pub fn laser_off(&mut self) {
        if self.laser_on {
            self.lines.push("M5".to_string());
            self.laser_on = false;
        }
    }

    fn laser_on(&mut self, power: f32) {
        // Always send M3 if we were off, or if power changed (M3 Sxxx updates power dynamically in GRBL laser mode)
        if !self.laser_on || self.current_power != Some(power) {
            self.lines.push(format!("M3 S{:.0}", power));
            self.current_power = Some(power);
            self.laser_on = true;
        }
    }

    fn is_at(&self, x: f32, y: f32) -> bool {
        if let (Some(cx), Some(cy)) = (self.current_x, self.current_y) {
            (cx - x).abs() < 0.001 && (cy - y).abs() < 0.001
        } else {
            false
        }
    }

    pub fn finish(mut self) -> Vec<String> {
        self.laser_off();
        self.lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_state() {
        let mut b = GCodeBuilder::new();
        b.rapid(10.0, 10.0);
        b.linear(20.0, 10.0, 1000.0, 500.0); // Should add F1000, M3 S500
        b.linear(20.0, 20.0, 1000.0, 500.0); // Should NOT add F or S
        b.linear(10.0, 20.0, 500.0, 500.0);  // Should add F500
        b.linear(10.0, 10.0, 500.0, 100.0);  // Should add S100
        b.rapid(0.0, 0.0);                   // Should add M5

        let lines = b.finish();

        assert!(lines.contains(&"G0 X10.000 Y10.000".to_string()));
        assert!(lines.contains(&"M3 S500".to_string()));
        assert!(lines.contains(&"G1 X20.000 Y10.000 F1000".to_string()));
        assert!(lines.contains(&"G1 X20.000 Y20.000".to_string())); // No F/S
        assert!(lines.contains(&"G1 X10.000 Y20.000 F500".to_string()));
        assert!(lines.contains(&"M3 S100".to_string()));
        assert!(lines.contains(&"M5".to_string()));
    }
}
