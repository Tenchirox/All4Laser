use serde::{Deserialize, Serialize};
use crate::theme::{UiTheme, UiLayout};
use crate::i18n::Language;
use crate::app::RightPanelTab;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: UiTheme,
    pub layout: UiLayout,
    pub language: Language,
    pub light_mode: bool,
    pub active_tab: RightPanelTab,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: UiTheme::Modern,
            layout: UiLayout::Modern,
            language: Language::French,
            light_mode: false,
            active_tab: RightPanelTab::Art,
        }
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
