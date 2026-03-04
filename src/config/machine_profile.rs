use serde::{Deserialize, Serialize};

use crate::controller::ControllerKind;

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
    pub rotary_axis: char, // 'Y' (roller) or 'A' (chuck)
    pub rotary_steps_per_deg: f32,
    #[serde(default = "default_controller_kind")]
    pub controller_kind: ControllerKind,

    // Tube wear tracking (F97)
    #[serde(default)]
    pub tube_hours_total: f64,
    #[serde(default = "default_tube_life")]
    pub tube_life_hours: f64,

    // Maintenance tracking (F27)
    #[serde(default)]
    pub maintenance_jobs_since_lens_clean: u32,
    #[serde(default = "default_lens_clean_interval")]
    pub lens_clean_interval_jobs: u32,
    #[serde(default)]
    pub maintenance_jobs_since_belt_check: u32,
    #[serde(default = "default_belt_check_interval")]
    pub belt_check_interval_jobs: u32,
    #[serde(default)]
    pub total_jobs_completed: u32,
}

fn default_controller_kind() -> ControllerKind {
    ControllerKind::Grbl
}
fn default_tube_life() -> f64 { 2000.0 }
fn default_lens_clean_interval() -> u32 { 20 }
fn default_belt_check_interval() -> u32 { 100 }

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
            rotary_axis: 'Y',
            rotary_steps_per_deg: 1.0,
            controller_kind: default_controller_kind(),
            tube_hours_total: 0.0,
            tube_life_hours: default_tube_life(),
            maintenance_jobs_since_lens_clean: 0,
            lens_clean_interval_jobs: default_lens_clean_interval(),
            maintenance_jobs_since_belt_check: 0,
            belt_check_interval_jobs: default_belt_check_interval(),
            total_jobs_completed: 0,
        }
    }
}

impl MachineProfile {
    /// Record laser-on time from a completed job (F97)
    pub fn record_job_burn_time(&mut self, burn_seconds: f64) {
        self.tube_hours_total += burn_seconds / 3600.0;
    }

    /// Tube wear percentage (0..100+)
    pub fn tube_wear_pct(&self) -> f64 {
        if self.tube_life_hours <= 0.0 { return 0.0; }
        (self.tube_hours_total / self.tube_life_hours * 100.0).min(999.0)
    }

    /// Record a completed job for maintenance tracking (F27)
    pub fn record_job_completed(&mut self) {
        self.total_jobs_completed += 1;
        self.maintenance_jobs_since_lens_clean += 1;
        self.maintenance_jobs_since_belt_check += 1;
    }

    /// Reset lens cleaning counter (F27)
    pub fn reset_lens_clean(&mut self) {
        self.maintenance_jobs_since_lens_clean = 0;
    }

    /// Reset belt check counter (F27)
    pub fn reset_belt_check(&mut self) {
        self.maintenance_jobs_since_belt_check = 0;
    }

    /// Return maintenance alerts if any (F27)
    pub fn maintenance_alerts(&self) -> Vec<String> {
        let mut alerts = Vec::new();
        if self.lens_clean_interval_jobs > 0
            && self.maintenance_jobs_since_lens_clean >= self.lens_clean_interval_jobs
        {
            alerts.push(format!(
                "🔍 Lens/mirror cleaning due ({} jobs since last clean).",
                self.maintenance_jobs_since_lens_clean
            ));
        }
        if self.belt_check_interval_jobs > 0
            && self.maintenance_jobs_since_belt_check >= self.belt_check_interval_jobs
        {
            alerts.push(format!(
                "🔧 Belt/alignment check due ({} jobs since last check).",
                self.maintenance_jobs_since_belt_check
            ));
        }
        alerts
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

/// Persistent store for multiple machine profiles (F11)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MachineProfileStore {
    pub profiles: Vec<MachineProfile>,
    pub active_index: usize,
}

impl Default for MachineProfileStore {
    fn default() -> Self {
        Self {
            profiles: vec![MachineProfile::default()],
            active_index: 0,
        }
    }
}

impl MachineProfileStore {
    fn json_path() -> std::path::PathBuf {
        std::env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("machine_profiles.json")
    }

    pub fn load() -> Self {
        if let Ok(data) = std::fs::read_to_string(Self::json_path()) {
            if let Ok(store) = serde_json::from_str::<MachineProfileStore>(&data) {
                if !store.profiles.is_empty() {
                    return store;
                }
            }
        }
        // Migrate from legacy single-profile file
        let legacy = MachineProfile::load();
        let store = Self {
            profiles: vec![legacy],
            active_index: 0,
        };
        store.save();
        store
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(Self::json_path(), json);
        }
    }

    pub fn active(&self) -> &MachineProfile {
        self.profiles.get(self.active_index).unwrap_or_else(|| &self.profiles[0])
    }

    pub fn active_mut(&mut self) -> &mut MachineProfile {
        let idx = self.active_index.min(self.profiles.len().saturating_sub(1));
        &mut self.profiles[idx]
    }

    pub fn set_active(&mut self, index: usize) {
        if index < self.profiles.len() {
            self.active_index = index;
        }
    }

    pub fn add(&mut self, profile: MachineProfile) {
        self.profiles.push(profile);
        self.active_index = self.profiles.len() - 1;
    }

    pub fn duplicate_active(&mut self) {
        let mut copy = self.active().clone();
        copy.name = format!("{} (copy)", copy.name);
        self.add(copy);
    }

    pub fn remove(&mut self, index: usize) {
        if self.profiles.len() <= 1 {
            return; // Keep at least one profile
        }
        self.profiles.remove(index);
        if self.active_index >= self.profiles.len() {
            self.active_index = self.profiles.len() - 1;
        }
    }

    pub fn export_profile(&self, index: usize, path: &str) -> Result<(), String> {
        let profile = self.profiles.get(index).ok_or("Invalid profile index")?;
        let json = serde_json::to_string_pretty(profile).map_err(|e| e.to_string())?;
        std::fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn import_profile(&mut self, path: &str) -> Result<String, String> {
        let data = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let profile: MachineProfile = serde_json::from_str(&data).map_err(|e| e.to_string())?;
        let name = profile.name.clone();
        self.add(profile);
        Ok(name)
    }
}
