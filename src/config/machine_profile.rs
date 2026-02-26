use serde::{Deserialize, Serialize};

/// Machine profile saved to disk (port, baud, workspace, kinematics)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MachineProfile {
    pub name: String,
    pub workspace_x_mm: f32,
    pub workspace_y_mm: f32,
    pub max_rate_x: f32,
    pub max_rate_y: f32,
    pub accel_x: f32,
    pub accel_y: f32,
    pub return_to_origin: bool,
    pub air_assist: bool,
    pub rotary_enabled: bool,
    pub rotary_diameter_mm: f32,
}

impl Default for MachineProfile {
    fn default() -> Self {
        Self {
            name: "Default Machine".into(),
            workspace_x_mm: 400.0,
            workspace_y_mm: 400.0,
            max_rate_x: 3000.0,
            max_rate_y: 3000.0,
            accel_x: 200.0,
            accel_y: 200.0,
            return_to_origin: true,
            air_assist: false,
            rotary_enabled: false,
            rotary_diameter_mm: 50.0,
        }
    }
}

impl MachineProfile {
    fn json_path() -> std::path::PathBuf {
        std::env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("machine_profile.json")
    }

    pub fn load() -> Self {
        std::fs::read_to_string(Self::json_path())
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(Self::json_path(), json);
        }
    }
}
