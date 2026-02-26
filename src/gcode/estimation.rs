use std::time::Duration;
use crate::gcode::types::{GCodeLine, ModalState};

#[derive(Debug, Clone, Default)]
pub struct EstimationResult {
    pub total_travel_mm: f32,
    pub total_burn_mm: f32,
    pub estimated_seconds: f32,
}

impl EstimationResult {
    pub fn duration(&self) -> Duration {
        Duration::from_secs_f32(self.estimated_seconds)
    }
}

pub fn estimate(lines: &[GCodeLine]) -> EstimationResult {
    let mut result = EstimationResult::default();
    let mut state = ModalState::default();

    for line in lines {
        // Track modal state
        if let Some(g) = line.g_code {
            if matches!(g, 0 | 1 | 2 | 3) {
                state.current_g = g;
            }
            if g == 90 { state.absolute = true; }
            if g == 91 { state.absolute = false; }
        }
        if let Some(f) = line.f { state.f = f; }
        if let Some(s) = line.s { state.s = s; }
        if let Some(m) = line.m_code {
            if matches!(m, 3 | 4) { state.laser_on = true; }
            if m == 5 { state.laser_on = false; }
        }

        // Calculate move
        let has_xyz = line.x.is_some() || line.y.is_some() || line.z.is_some();
        if has_xyz && matches!(state.current_g, 0 | 1 | 2 | 3) {
            let nx = line.x.unwrap_or(state.x);
            let ny = line.y.unwrap_or(state.y);
            
            let dist = if state.absolute {
                ((nx - state.x).powi(2) + (ny - state.y).powi(2)).sqrt()
            } else {
                (nx.powi(2) + ny.powi(2)).sqrt()
            };

            if dist > 0.0 {
                let is_burn = state.laser_on && state.current_g > 0;
                if is_burn {
                    result.total_burn_mm += dist;
                } else {
                    result.total_travel_mm += dist;
                }

                // Time est: dist / feed_rate (mm/min) * 60
                // For G0, we assume a "rapid" rate if state.f is 0, let's guess 3000
                let feed = if state.current_g == 0 {
                    if state.f > 0.0 { state.f } else { 3000.0 }
                } else {
                    state.f
                };

                if feed > 0.0 {
                    result.estimated_seconds += (dist / feed) * 60.0;
                }
            }

            if state.absolute {
                state.x = nx;
                state.y = ny;
            } else {
                state.x += nx;
                state.y += ny;
            }
        }
    }

    result
}
