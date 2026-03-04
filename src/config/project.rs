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
    #[serde(default)]
    pub material_selected_preset: Option<String>,
    #[serde(default)]
    pub checkpoint_line: Option<usize>,
    #[serde(default)]
    pub project_notes: String,
}

fn default_camera_opacity() -> f32 { 0.5 }

/// Job template (F106) — stores layer configurations for reuse
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobTemplate {
    pub name: String,
    pub layers: Vec<crate::ui::layers_new::CutLayer>,
    pub description: String,
}

impl JobTemplate {
    fn templates_dir() -> std::path::PathBuf {
        std::env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("templates")
    }

    pub fn save(template: &JobTemplate) -> Result<(), String> {
        let dir = Self::templates_dir();
        let _ = std::fs::create_dir_all(&dir);
        let filename = template.name.replace(' ', "_").to_lowercase() + ".json";
        let path = dir.join(filename);
        let json = serde_json::to_string_pretty(template).map_err(|e| e.to_string())?;
        std::fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn list_templates() -> Vec<String> {
        let dir = Self::templates_dir();
        let mut names = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.path().file_stem() {
                    names.push(name.to_string_lossy().to_string());
                }
            }
        }
        names
    }

    pub fn load(name: &str) -> Result<JobTemplate, String> {
        let dir = Self::templates_dir();
        let path = dir.join(format!("{name}.json"));
        let json = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }

    pub fn delete(name: &str) -> Result<(), String> {
        let dir = Self::templates_dir();
        let path = dir.join(format!("{name}.json"));
        std::fs::remove_file(path).map_err(|e| e.to_string())
    }
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

    /// Path to the auto-save recovery file (F71)
    pub fn recovery_path() -> std::path::PathBuf {
        std::env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("autosave.a4l.recovery")
    }

    /// Save recovery snapshot (called periodically)
    pub fn save_recovery(project: &ProjectFile) {
        let path = Self::recovery_path();
        if let Ok(json) = serde_json::to_string(project) {
            let _ = std::fs::write(path, json);
        }
    }

    /// Try loading a recovery file (called on startup)
    pub fn load_recovery() -> Option<ProjectFile> {
        let path = Self::recovery_path();
        if !path.exists() {
            return None;
        }
        let data = std::fs::read_to_string(&path).ok()?;
        serde_json::from_str(&data).ok()
    }

    /// Delete the recovery file (called after successful load or explicit save)
    pub fn clear_recovery() {
        let path = Self::recovery_path();
        let _ = std::fs::remove_file(path);
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
        assert!(parsed.material_selected_preset.is_none());
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
        p.material_selected_preset = Some("Acrylic 3mm".to_string());

        let json = serde_json::to_string(&p).unwrap();
        let back: ProjectFile = serde_json::from_str(&json).unwrap();
        assert!(back.camera_enabled);
        assert_eq!(back.camera_device_index, 2);
        assert!(back.camera_live_streaming);
        assert_eq!(back.camera_snapshot_path.as_deref(), Some("snapshot.png"));
        assert_eq!(back.material_selected_preset.as_deref(), Some("Acrylic 3mm"));
        assert!((back.camera_calibration.offset_x - 4.5).abs() < f32::EPSILON);
    }
}
