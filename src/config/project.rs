use serde::{Serialize, Deserialize};
use crate::config::machine_profile::MachineProfile;
use crate::ui::camera::CameraCalibration;

/// An All4Laser project file (.a4l) – persists everything needed to restore a session
#[derive(Serialize, Deserialize, Default)]
pub struct ProjectFile {
    #[serde(default)]
    pub version: u32,
    #[serde(default)]
    pub gcode_path: Option<String>,
    #[serde(default)]
    pub gcode_content: Option<String>,
    #[serde(default)]
    pub offset_x: f32,
    #[serde(default)]
    pub offset_y: f32,
    #[serde(default)]
    pub rotation_deg: f32,
    #[serde(default)]
    pub machine_profile: Option<MachineProfile>,
    #[serde(default)]
    pub camera_enabled: bool,
    #[serde(default = "default_camera_opacity")]
    pub camera_opacity: f32,
    #[serde(default)]
    pub camera_calibration: CameraCalibration,
    #[serde(default)]
    pub camera_snapshot_path: Option<String>,
    #[serde(default)]
    pub camera_device_index: i32,
    #[serde(default)]
    pub camera_live_streaming: bool,
}

fn default_camera_opacity() -> f32 { 0.5 }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_project_without_camera_fields_still_loads() {
        let legacy = r#"{"version":1,"gcode_path":null,"gcode_content":null,"offset_x":10.0,"offset_y":-2.0,"rotation_deg":0.0,"machine_profile":null}"#;
        let parsed: ProjectFile = serde_json::from_str(legacy).expect("legacy project should deserialize");
        assert!(!parsed.camera_enabled);
        assert_eq!(parsed.camera_opacity, 0.5);
        assert_eq!(parsed.camera_device_index, 0);
        assert!(!parsed.camera_live_streaming);
    }

    #[test]
    fn project_camera_fields_roundtrip() {
        let mut p = ProjectFile::default();
        p.camera_enabled = true;
        p.camera_opacity = 0.61;
        p.camera_calibration.offset_x = 4.5;
        p.camera_calibration.offset_y = -3.2;
        p.camera_calibration.scale = 1.08;
        p.camera_calibration.rotation = -2.0;
        p.camera_snapshot_path = Some("snapshot.png".to_string());
        p.camera_device_index = 2;
        p.camera_live_streaming = true;

        let json = serde_json::to_string(&p).unwrap();
        let back: ProjectFile = serde_json::from_str(&json).unwrap();
        assert!(back.camera_enabled);
        assert_eq!(back.camera_device_index, 2);
        assert!(back.camera_live_streaming);
        assert_eq!(back.camera_snapshot_path.as_deref(), Some("snapshot.png"));
        assert!((back.camera_calibration.offset_x - 4.5).abs() < f32::EPSILON);
    }
}
