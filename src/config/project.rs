use serde::{Serialize, Deserialize};
use crate::config::machine_profile::MachineProfile;

/// An All4Laser project file (.a4l) – persists everything needed to restore a session
#[derive(Serialize, Deserialize, Default)]
pub struct ProjectFile {
    pub version: u32,
    pub gcode_path: Option<String>,
    pub gcode_content: Option<String>,
    pub offset_x: f32,
    pub offset_y: f32,
    pub machine_profile: Option<MachineProfile>,
}

impl ProjectFile {
    pub fn save(path: &str, project: &ProjectFile) -> Result<(), String> {
        let json = serde_json::to_string_pretty(project).map_err(|e| e.to_string())?;
        std::fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn load(path: &str) -> Result<ProjectFile, String> {
        let data = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&data).map_err(|e| e.to_string())
    }
}
