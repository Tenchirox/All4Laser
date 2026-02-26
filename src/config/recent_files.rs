use serde::{Deserialize, Serialize};

const MAX_RECENT: usize = 10;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct RecentFiles {
    pub paths: Vec<String>,
}

impl RecentFiles {
    fn json_path() -> std::path::PathBuf {
        std::env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("recent_files.json")
    }

    pub fn load() -> Self {
        std::fs::read_to_string(Self::json_path())
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_default()
    }

    pub fn push(&mut self, path: &str) {
        self.paths.retain(|p| p != path);
        self.paths.insert(0, path.to_string());
        self.paths.truncate(MAX_RECENT);
        let _ = serde_json::to_string_pretty(self)
            .ok()
            .and_then(|json| std::fs::write(Self::json_path(), json).ok());
    }
}
