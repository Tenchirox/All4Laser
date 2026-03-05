/// Persistent event log (F67)
/// Records all app events to event_log.txt for debugging and auditing.

pub struct EventLog {
    pub entries: Vec<String>,
}

impl Default for EventLog {
    fn default() -> Self {
        Self { entries: Self::load_from_disk() }
    }
}

impl EventLog {
    fn file_path() -> std::path::PathBuf {
        std::env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("event_log.txt")
    }

    fn load_from_disk() -> Vec<String> {
        let path = Self::file_path();
        std::fs::read_to_string(path)
            .unwrap_or_default()
            .lines()
            .map(String::from)
            .collect()
    }

    pub fn push(&mut self, entry: String) {
        self.entries.push(entry);
        if self.entries.len() > 2000 {
            self.entries.drain(0..500);
        }
    }

    pub fn save(&self) {
        let path = Self::file_path();
        let content = self.entries.join("\n");
        let _ = std::fs::write(path, content);
    }
}
