#![allow(dead_code)]

use crate::config::machine_profile::MachineProfile;
use crate::ui::camera::CameraCalibration;
use serde::{Deserialize, Serialize};

pub fn validate_safe_filename(name: &str) -> Result<(), String> {
    let path = std::path::Path::new(name);
    if path.file_name().and_then(|s| s.to_str()) != Some(name)
        || name.contains("..")
        || name.contains('/')
        || name.contains('\\')
    {
        return Err("Invalid filename: path traversal detected".to_string());
    }
    Ok(())
}

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

fn default_camera_opacity() -> f32 {
    0.5
}

fn sanitize_filename(name: &str) -> String {
    name.replace(' ', "_")
        .replace('/', "_")
        .replace('\\', "_")
        .replace('.', "_")
        .replace(':', "_")
        .to_lowercase()
}

fn is_safe_filename(name: &str) -> bool {
    !name.contains('/') && !name.contains('\\') && !name.contains("..") && !name.contains(':')
}

/// Job template (F106) — stores layer configurations for reuse
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobTemplate {
    pub name: String,
    pub layers: Vec<crate::ui::layers_new::CutLayer>,
    pub description: String,
}

/// Post-processor configuration (F42)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostProcessor {
    pub name: String,
    pub header: Vec<String>,
    pub footer: Vec<String>,
    pub laser_on: String,  // e.g. "M3" or "M4"
    pub laser_off: String, // e.g. "M5"
    pub air_on: String,    // e.g. "M8"
    pub air_off: String,   // e.g. "M9"
    pub comment_style: CommentStyle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CommentStyle {
    Semicolon,   // ; comment
    Parentheses, // (comment)
}

impl Default for PostProcessor {
    fn default() -> Self {
        Self {
            name: "GRBL Default".into(),
            header: vec!["G90".into(), "G21".into(), "M5".into()],
            footer: vec!["M5".into(), "G0 X0 Y0".into()],
            laser_on: "M3".into(),
            laser_off: "M5".into(),
            air_on: "M8".into(),
            air_off: "M9".into(),
            comment_style: CommentStyle::Semicolon,
        }
    }
}

impl PostProcessor {
    fn postprocessors_dir() -> std::path::PathBuf {
        std::env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("postprocessors")
    }

    pub fn save(&self) -> Result<(), String> {
        if self.name.contains('/') || self.name.contains('\\') || self.name.contains("..") {
            return Err("Invalid post-processor name".into());
        }
        let dir = Self::postprocessors_dir();
        let _ = std::fs::create_dir_all(&dir);
        let filename = sanitize_filename(&self.name) + ".json";
        let path = dir.join(filename);
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn load(name: &str) -> Result<PostProcessor, String> {
        if !is_safe_filename(name) {
            return Err("Invalid post-processor name".into());
        }
        let dir = Self::postprocessors_dir();
        let path = dir.join(format!("{}.json", name.replace(&['/', '\\', '.', ':', ' '][..], "_").to_lowercase()));
        let json = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }

    pub fn list() -> Vec<String> {
        let dir = Self::postprocessors_dir();
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

    pub fn builtin_presets() -> Vec<PostProcessor> {
        vec![
            PostProcessor::default(),
            PostProcessor {
                name: "Marlin".into(),
                header: vec!["G90".into(), "G21".into(), "M5".into()],
                footer: vec!["M5".into(), "G28 X Y".into()],
                laser_on: "M3".into(),
                laser_off: "M5".into(),
                air_on: "M8".into(),
                air_off: "M9".into(),
                comment_style: CommentStyle::Semicolon,
            },
            PostProcessor {
                name: "Smoothie".into(),
                header: vec!["G90".into(), "G21".into(), "M5".into()],
                footer: vec!["M5".into(), "G0 X0 Y0".into(), "M2".into()],
                laser_on: "M3".into(),
                laser_off: "M5".into(),
                air_on: "M8".into(),
                air_off: "M9".into(),
                comment_style: CommentStyle::Semicolon,
            },
            PostProcessor {
                name: "FluidNC".into(),
                header: vec!["G90".into(), "G21".into(), "M5".into()],
                footer: vec!["M5".into(), "G0 X0 Y0 F3000".into()],
                laser_on: "M4".into(),
                laser_off: "M5".into(),
                air_on: "M7".into(),
                air_off: "M9".into(),
                comment_style: CommentStyle::Parentheses,
            },
        ]
    }
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
        if template.name.contains('/')
            || template.name.contains('\\')
            || template.name.contains("..")
        {
            return Err("Invalid template name".into());
        }
        let dir = Self::templates_dir();
        let _ = std::fs::create_dir_all(&dir);
        let filename = sanitize_filename(&template.name) + ".json";
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
        if !is_safe_filename(name) {
            return Err("Invalid template name".into());
        }
        let dir = Self::templates_dir();
        let path = dir.join(format!("{}.json", name.replace(&['/', '\\', '.', ':', ' '][..], "_").to_lowercase()));
        let json = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }

    pub fn delete(name: &str) -> Result<(), String> {
        validate_safe_filename(name)?;
        if !is_safe_filename(name) {
            return Err("Invalid template name".into());
        }
        let dir = Self::templates_dir();
        let path = dir.join(format!("{}.json", name.replace(&['/', '\\', '.', ':', ' '][..], "_").to_lowercase()));
        std::fs::remove_file(path).map_err(|e| e.to_string())
    }
}

/// Parametric shape generator (F25)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ParametricKind {
    Box,
    LidBox,
    Gear,
    Star,
    Polygon,
    Spiral,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParametricParams {
    pub kind: ParametricKind,
    pub width_mm: f32,
    pub height_mm: f32,
    pub depth_mm: f32,
    pub thickness_mm: f32,
    pub teeth_count: u32, // for gears
    pub points: u32,      // for stars/polygons
    pub kerf_mm: f32,
}

impl Default for ParametricParams {
    fn default() -> Self {
        Self {
            kind: ParametricKind::Box,
            width_mm: 50.0,
            height_mm: 30.0,
            depth_mm: 20.0,
            thickness_mm: 3.0,
            teeth_count: 12,
            points: 5,
            kerf_mm: 0.1,
        }
    }
}

/// Variable Text / CSV data merge (F21)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VariableTextConfig {
    pub csv_path: Option<String>,
    pub columns: Vec<String>,
    pub current_row: usize,
    pub auto_increment: bool,
    pub prefix: String,
    pub suffix: String,
    pub start_number: u32,
}

impl Default for VariableTextConfig {
    fn default() -> Self {
        Self {
            csv_path: None,
            columns: Vec::new(),
            current_row: 0,
            auto_increment: true,
            prefix: String::new(),
            suffix: String::new(),
            start_number: 1,
        }
    }
}

impl VariableTextConfig {
    pub fn load_csv(&mut self, path: &str) -> Result<Vec<Vec<String>>, String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut rows = Vec::new();
        for (i, line) in content.lines().enumerate() {
            let fields: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
            if i == 0 {
                self.columns = fields.clone();
            }
            rows.push(fields);
        }
        self.csv_path = Some(path.to_string());
        Ok(rows)
    }

    pub fn serial_text(&self, row_idx: usize) -> String {
        format!(
            "{}{}{}",
            self.prefix,
            self.start_number + row_idx as u32,
            self.suffix
        )
    }
}

/// Jig/fixture template (F76)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JigTemplate {
    pub name: String,
    pub width_mm: f32,
    pub height_mm: f32,
    pub holes: Vec<(f32, f32, f32)>, // (x, y, diameter_mm)
    pub alignment_pins: Vec<(f32, f32)>,
    pub description: String,
}

impl JigTemplate {
    fn jigs_dir() -> std::path::PathBuf {
        std::env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("jigs")
    }

    pub fn save(&self) -> Result<(), String> {
        if self.name.contains('/') || self.name.contains('\\') || self.name.contains("..") {
            return Err("Invalid jig template name".into());
        }
        let dir = Self::jigs_dir();
        let _ = std::fs::create_dir_all(&dir);
        let filename = sanitize_filename(&self.name) + ".json";
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(dir.join(filename), json).map_err(|e| e.to_string())
    }

    pub fn load(name: &str) -> Result<JigTemplate, String> {
        if !is_safe_filename(name) {
            return Err("Invalid jig template name".into());
        }
        let path = Self::jigs_dir().join(format!("{name}.json"));
        let json = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }

    pub fn delete(name: &str) -> Result<(), String> {
        if name.contains('/') || name.contains('\\') || name.contains("..") {
            return Err("Invalid name".to_string());
        }
        let path = Self::jigs_dir().join(format!("{name}.json"));
        std::fs::remove_file(path).map_err(|e| e.to_string())
    }

    pub fn list() -> Vec<String> {
        let dir = Self::jigs_dir();
        std::fs::read_dir(dir)
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|e| {
                e.path()
                    .file_stem()
                    .map(|n| n.to_string_lossy().to_string())
            })
            .collect()
    }
}

/// Project revision entry (F110)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectRevision {
    pub revision: u32,
    pub timestamp: u64,
    pub description: String,
}

/// Project version history (F110)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProjectHistory {
    pub revisions: Vec<ProjectRevision>,
}

impl ProjectHistory {
    fn history_path(project_path: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(project_path).with_extension("history.json")
    }

    pub fn save(&self, project_path: &str) -> Result<(), String> {
        let path = Self::history_path(project_path);
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn load(project_path: &str) -> Self {
        let path = Self::history_path(project_path);
        std::fs::read_to_string(path)
            .ok()
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default()
    }

    pub fn add_revision(&mut self, description: &str) {
        let rev = self.revisions.len() as u32 + 1;
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.revisions.push(ProjectRevision {
            revision: rev,
            timestamp: ts,
            description: description.to_string(),
        });
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
    fn test_template_name_sanitization_prevents_path_traversal() {
        let dangerous_name = "../../../etc/passwd";

        let template = JobTemplate {
            name: dangerous_name.to_string(),
            description: "".to_string(),
            layers: vec![],
        };
        let _ = JobTemplate::save(&template);
        let _ = JobTemplate::load(dangerous_name);

        let pp = PostProcessor {
            name: dangerous_name.to_string(),
            header: vec![],
            footer: vec![],
            laser_on: "".to_string(),
            laser_off: "".to_string(),
            air_on: "".to_string(),
            air_off: "".to_string(),
            comment_style: CommentStyle::Semicolon,
        };
        let _ = pp.save();
        let _ = PostProcessor::load(dangerous_name);

        let jig = JigTemplate {
            name: dangerous_name.to_string(),
            width_mm: 10.0,
            height_mm: 10.0,
            holes: vec![],
            alignment_pins: vec![],
            description: "".to_string(),
        };
        let _ = jig.save();
        let _ = JigTemplate::load(dangerous_name);
    }

    #[test]
    fn legacy_project_without_camera_fields_still_loads() {
        let legacy = r#"{"version":1,"gcode_path":null,"gcode_content":null,"offset_x":10.0,"offset_y":-2.0,"rotation_deg":0.0,"machine_profile":null}"#;
        let parsed: ProjectFile =
            serde_json::from_str(legacy).expect("legacy project should deserialize");
        assert!(!parsed.camera_enabled);
        assert_eq!(parsed.camera_opacity, 0.5);
        assert_eq!(parsed.camera_device_index, 0);
        assert!(!parsed.camera_live_streaming);
        assert!(parsed.material_selected_preset.is_none());
    }

    #[test]
    fn test_validate_safe_filename() {
        assert!(validate_safe_filename("valid_name").is_ok());
        assert!(validate_safe_filename("valid_name_2").is_ok());
        assert!(validate_safe_filename("template").is_ok());

        assert!(validate_safe_filename("../test").is_err());
        assert!(validate_safe_filename("test/test").is_err());
        assert!(validate_safe_filename("/etc/passwd").is_err());
        assert!(validate_safe_filename("C:\\Windows\\System32").is_err());
        assert!(validate_safe_filename("..").is_err());
        assert!(validate_safe_filename("a/b").is_err());
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
        assert_eq!(
            back.material_selected_preset.as_deref(),
            Some("Acrylic 3mm")
        );
        assert!((back.camera_calibration.offset_x - 4.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Safe Name"), "safe_name");
        assert_eq!(sanitize_filename("test/dir\\file.name:yes"), "test_dir_file_name_yes");
        assert_eq!(sanitize_filename("../../../etc/passwd"), "_________etc_passwd");
    }

    #[test]
    fn test_is_safe_filename() {
        assert!(is_safe_filename("safe_name"));
        assert!(is_safe_filename("Safe Name"));
        assert!(!is_safe_filename("test/dir"));
        assert!(!is_safe_filename("test\\dir"));
        assert!(!is_safe_filename("../test"));
        assert!(!is_safe_filename("C:\\test"));
        assert!(!is_safe_filename("test:name"));
    }
}
