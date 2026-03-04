use serde::{Deserialize, Serialize};
use crate::theme::{UiTheme, UiLayout};
use crate::i18n::Language;
use crate::app::RightPanelTab;
use crate::ui::camera::CameraCalibration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisplayUnit {
    Millimeters,
    Inches,
}

impl Default for DisplayUnit {
    fn default() -> Self { DisplayUnit::Millimeters }
}

impl DisplayUnit {
    pub fn label(self) -> &'static str {
        match self {
            DisplayUnit::Millimeters => "mm",
            DisplayUnit::Inches => "in",
        }
    }

    /// Convert mm to display unit
    pub fn from_mm(self, mm: f32) -> f32 {
        match self {
            DisplayUnit::Millimeters => mm,
            DisplayUnit::Inches => mm / 25.4,
        }
    }

    /// Convert display unit back to mm
    pub fn to_mm(self, val: f32) -> f32 {
        match self {
            DisplayUnit::Millimeters => val,
            DisplayUnit::Inches => val * 25.4,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(default = "default_theme")]
    pub theme: UiTheme,
    #[serde(default = "default_layout")]
    pub layout: UiLayout,
    #[serde(default)]
    pub language: Language,
    #[serde(default = "default_light_mode")]
    pub light_mode: bool,
    #[serde(default = "default_beginner_mode")]
    pub beginner_mode: bool,
    #[serde(default = "default_active_tab")]
    pub active_tab: RightPanelTab,
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
    pub display_unit: DisplayUnit,
    #[serde(default)]
    pub first_run_done: bool,
    // Cost estimation (F17)
    #[serde(default = "default_cost_per_hour")]
    pub cost_per_hour: f32,
    #[serde(default)]
    pub cost_per_m2: f32,
    #[serde(default)]
    pub cost_currency: String,
    // Custom shortcuts (F89)
    #[serde(default = "default_shortcuts")]
    pub custom_shortcuts: std::collections::HashMap<String, String>,
    // Darkroom mode (F70)
    #[serde(default)]
    pub darkroom_mode: bool,
    // Accessibility (F56)
    #[serde(default)]
    pub colorblind_mode: ColorblindMode,
    #[serde(default)]
    pub high_contrast: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ColorblindMode {
    None,
    Protanopia,
    Deuteranopia,
    Tritanopia,
}

impl Default for ColorblindMode {
    fn default() -> Self { ColorblindMode::None }
}

fn default_theme() -> UiTheme { UiTheme::Modern }
fn default_layout() -> UiLayout { UiLayout::Modern }
fn default_light_mode() -> bool { true }
fn default_beginner_mode() -> bool { true }
fn default_active_tab() -> RightPanelTab { RightPanelTab::Art }
fn default_camera_opacity() -> f32 { 0.5 }
fn default_cost_per_hour() -> f32 { 15.0 }
fn default_shortcuts() -> std::collections::HashMap<String, String> {
    let mut m = std::collections::HashMap::new();
    m.insert("run".into(), "Ctrl+R".into());
    m.insert("stop".into(), "Escape".into());
    m.insert("save".into(), "Ctrl+S".into());
    m.insert("open".into(), "Ctrl+O".into());
    m.insert("undo".into(), "Ctrl+Z".into());
    m.insert("redo".into(), "Ctrl+Y".into());
    m.insert("delete".into(), "Delete".into());
    m.insert("copy".into(), "Ctrl+C".into());
    m.insert("paste".into(), "Ctrl+V".into());
    m.insert("select_all".into(), "Ctrl+A".into());
    m
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            layout: default_layout(),
            language: Language::French,
            light_mode: default_light_mode(),
            beginner_mode: true,
            active_tab: default_active_tab(),
            camera_enabled: false,
            camera_opacity: default_camera_opacity(),
            camera_calibration: CameraCalibration::default(),
            camera_snapshot_path: None,
            camera_device_index: 0,
            camera_live_streaming: false,
            material_selected_preset: None,
            display_unit: DisplayUnit::Millimeters,
            first_run_done: false,
            cost_per_hour: default_cost_per_hour(),
            cost_per_m2: 0.0,
            cost_currency: "€".into(),
            custom_shortcuts: default_shortcuts(),
            darkroom_mode: false,
            colorblind_mode: ColorblindMode::None,
            high_contrast: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_settings_without_camera_fields_still_load() {
        let legacy = r#"{"theme":"Modern","layout":"Modern","language":"English","light_mode":true,"beginner_mode":true,"active_tab":"Art"}"#;
        let parsed: AppSettings = serde_json::from_str(legacy).expect("legacy settings should deserialize");
        assert!(!parsed.camera_enabled);
        assert_eq!(parsed.camera_opacity, 0.5);
        assert_eq!(parsed.camera_device_index, 0);
        assert!(!parsed.camera_live_streaming);
        assert!(parsed.material_selected_preset.is_none());
    }

    #[test]
    fn settings_camera_fields_roundtrip() {
        let mut s = AppSettings::default();
        s.camera_enabled = true;
        s.camera_opacity = 0.72;
        s.camera_calibration.offset_x = 12.0;
        s.camera_calibration.offset_y = -5.0;
        s.camera_calibration.scale = 1.25;
        s.camera_calibration.rotation = 7.5;
        s.camera_snapshot_path = Some("/tmp/cam.png".to_string());
        s.camera_device_index = 3;
        s.camera_live_streaming = true;
        s.material_selected_preset = Some("Plywood 3mm".to_string());

        let json = serde_json::to_string(&s).unwrap();
        let back: AppSettings = serde_json::from_str(&json).unwrap();
        assert!(back.camera_enabled);
        assert_eq!(back.camera_device_index, 3);
        assert!(back.camera_live_streaming);
        assert_eq!(back.camera_snapshot_path.as_deref(), Some("/tmp/cam.png"));
        assert_eq!(back.material_selected_preset.as_deref(), Some("Plywood 3mm"));
        assert!((back.camera_calibration.scale - 1.25).abs() < f32::EPSILON);
    }
}

impl AppSettings {
    fn json_path() -> std::path::PathBuf {
        std::env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("settings.json")
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
