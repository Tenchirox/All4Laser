use egui::RichText;
use std::sync::Arc;

use crate::theme;

#[derive(Clone, Debug)]
pub struct QueuedJob {
    pub id: u64,
    pub name: String,
    pub lines: Arc<Vec<String>>,
    pub attempts: u32,
}

#[derive(Clone, Debug)]
pub struct JobHistoryEntry {
    pub id: u64,
    pub name: String,
    pub lines: Arc<Vec<String>>,
    pub attempts: u32,
    pub status: String,
}

#[derive(Debug)]
pub struct JobQueueState {
    pub is_open: bool,
    pub auto_run_next: bool,
    pub queue: Vec<QueuedJob>,
    pub history: Vec<JobHistoryEntry>,
    next_id: u64,
}

impl Default for JobQueueState {
    fn default() -> Self {
        Self {
            is_open: false,
            auto_run_next: true,
            queue: Vec::new(),
            history: Vec::new(),
            next_id: 1,
        }
    }
}

impl JobQueueState {
    pub fn enqueue_job(&mut self, name: String, lines: Arc<Vec<String>>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.queue.push(QueuedJob {
            id,
            name,
            lines,
            attempts: 1,
        });
        id
    }

    pub fn pop_next_job(&mut self) -> Option<QueuedJob> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }

    pub fn move_up(&mut self, idx: usize) {
        if idx > 0 && idx < self.queue.len() {
            self.queue.swap(idx - 1, idx);
        }
    }

    pub fn move_down(&mut self, idx: usize) {
        if idx + 1 < self.queue.len() {
            self.queue.swap(idx, idx + 1);
        }
    }

    pub fn remove(&mut self, idx: usize) {
        if idx < self.queue.len() {
            self.queue.remove(idx);
        }
    }

    pub fn record_completion(&mut self, job: QueuedJob) {
        self.history.push(JobHistoryEntry {
            id: job.id,
            name: job.name,
            lines: job.lines,
            attempts: job.attempts,
            status: "Completed".to_string(),
        });
    }

    pub fn record_failure(&mut self, job: QueuedJob, reason: String) {
        self.history.push(JobHistoryEntry {
            id: job.id,
            name: job.name,
            lines: job.lines,
            attempts: job.attempts,
            status: format!("Failed: {reason}"),
        });
    }

    /// Job history statistics summary (F52)
    pub fn stats_summary(&self) -> (usize, usize, usize) {
        let total = self.history.len();
        let completed = self
            .history
            .iter()
            .filter(|h| h.status == "Completed")
            .count();
        let failed = total - completed;
        (total, completed, failed)
    }

    /// Save job history to file (F52)
    pub fn save_history(&self) {
        let path = Self::history_path();
        let entries: Vec<String> = self
            .history
            .iter()
            .map(|h| format!("{}|{}|{}|{}", h.id, h.name, h.attempts, h.status))
            .collect();
        let _ = std::fs::write(path, entries.join("\n"));
    }

    /// Load job history from file (F52)
    pub fn load_history(&mut self) {
        let path = Self::history_path();
        if let Ok(content) = std::fs::read_to_string(path) {
            for line in content.lines() {
                let parts: Vec<&str> = line.splitn(4, '|').collect();
                if parts.len() == 4 {
                    self.history.push(JobHistoryEntry {
                        id: parts[0].parse().unwrap_or(0),
                        name: parts[1].to_string(),
                        lines: Arc::new(Vec::new()),
                        attempts: parts[2].parse().unwrap_or(1),
                        status: parts[3].to_string(),
                    });
                }
            }
        }
    }

    fn history_path() -> std::path::PathBuf {
        std::env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("job_history.txt")
    }

    /// Batch enqueue multiple files from a directory (F109)
    pub fn batch_enqueue_from_paths(&mut self, paths: &[std::path::PathBuf]) -> Vec<u64> {
        let mut ids = Vec::new();
        for path in paths {
            if let Ok(content) = std::fs::read_to_string(path) {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "batch_file".into());
                let lines: Arc<Vec<String>> = Arc::new(content.lines().map(String::from).collect());
                let id = self.enqueue_job(name, lines);
                ids.push(id);
            }
        }
        ids
    }

    fn _dummy_record_failure_end(&self) {
        // marker to avoid duplicate match
    }

    pub fn record_aborted(&mut self, job: QueuedJob) {
        self.history.push(JobHistoryEntry {
            id: job.id,
            name: job.name,
            lines: job.lines,
            attempts: job.attempts,
            status: "Aborted".to_string(),
        });
    }

    pub fn retry_last_failed(&mut self) -> Option<u64> {
        let last_failed = self
            .history
            .iter()
            .rev()
            .find(|entry| entry.status.starts_with("Failed") || entry.status == "Aborted")?
            .clone();

        let id = self.next_id;
        self.next_id += 1;
        self.queue.push(QueuedJob {
            id,
            name: format!("{} (retry)", last_failed.name),
            lines: last_failed.lines,
            attempts: last_failed.attempts.saturating_add(1),
        });
        Some(id)
    }
}

#[derive(Default)]
pub struct JobQueueAction {
    pub enqueue_current: bool,
    pub start_next: bool,
    pub retry_last_failed: bool,
}

pub fn show(
    ctx: &egui::Context,
    state: &mut JobQueueState,
    has_loaded_program: bool,
    running: bool,
    active_job_name: Option<&str>,
) -> JobQueueAction {
    let mut action = JobQueueAction::default();

    if !state.is_open {
        return action;
    }

    let mut close_clicked = false;

    egui::Window::new("📚 Job Queue")
        .resizable(true)
        .collapsible(false)
        .default_width(520.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(
                        has_loaded_program,
                        egui::Button::new("➕ Queue Current Job"),
                    )
                    .clicked()
                {
                    action.enqueue_current = true;
                }
                if ui
                    .add_enabled(
                        !running && !state.queue.is_empty(),
                        egui::Button::new("▶ Start Next"),
                    )
                    .clicked()
                {
                    action.start_next = true;
                }
                if ui
                    .add_enabled(!running, egui::Button::new("🔁 Retry Last Failed"))
                    .clicked()
                {
                    action.retry_last_failed = true;
                }
                if ui.button("Close").clicked() {
                    close_clicked = true;
                }
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut state.auto_run_next, "Auto-run next queued job");
                ui.label(
                    RichText::new(format!(
                        "Pending: {} | History: {}",
                        state.queue.len(),
                        state.history.len()
                    ))
                    .small()
                    .color(theme::SUBTEXT),
                );
            });

            if let Some(name) = active_job_name {
                ui.label(
                    RichText::new(format!("Running: {name}"))
                        .color(theme::GREEN)
                        .strong(),
                );
            }

            ui.add_space(8.0);
            ui.label(RichText::new("Pending Queue").strong());
            egui::ScrollArea::vertical()
                .max_height(180.0)
                .show(ui, |ui| {
                    if state.queue.is_empty() {
                        ui.label(
                            RichText::new("No queued jobs.")
                                .small()
                                .color(theme::SUBTEXT),
                        );
                    } else {
                        let mut move_up_idx = None;
                        let mut move_down_idx = None;
                        let mut remove_idx = None;

                        for (idx, job) in state.queue.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(format!(
                                        "#{} {} ({} lines, try {})",
                                        job.id,
                                        job.name,
                                        job.lines.len(),
                                        job.attempts
                                    ))
                                    .small(),
                                );
                                if ui.button("↑").clicked() {
                                    move_up_idx = Some(idx);
                                }
                                if ui.button("↓").clicked() {
                                    move_down_idx = Some(idx);
                                }
                                if ui.button("✕").clicked() {
                                    remove_idx = Some(idx);
                                }
                            });
                        }

                        if let Some(idx) = move_up_idx {
                            state.move_up(idx);
                        }
                        if let Some(idx) = move_down_idx {
                            state.move_down(idx);
                        }
                        if let Some(idx) = remove_idx {
                            state.remove(idx);
                        }
                    }
                });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Execution History").strong());
                if ui.button("🧹 Clear").clicked() {
                    state.history.clear();
                }
            });

            egui::ScrollArea::vertical()
                .max_height(180.0)
                .show(ui, |ui| {
                    if state.history.is_empty() {
                        ui.label(
                            RichText::new("No history yet.")
                                .small()
                                .color(theme::SUBTEXT),
                        );
                    } else {
                        for entry in state.history.iter().rev() {
                            let color = if entry.status == "Completed" {
                                theme::GREEN
                            } else if entry.status == "Aborted" {
                                theme::PEACH
                            } else {
                                theme::RED
                            };
                            ui.label(
                                RichText::new(format!(
                                    "#{} {} — {} (try {})",
                                    entry.id, entry.name, entry.status, entry.attempts
                                ))
                                .small()
                                .color(color),
                            );
                        }
                    }
                });
        });

    if close_clicked {
        state.is_open = false;
    }

    action
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_reordering_and_pop_works() {
        let mut q = JobQueueState::default();
        q.enqueue_job("A".into(), std::sync::Arc::new(vec!["G0 X0".into()]));
        q.enqueue_job("B".into(), std::sync::Arc::new(vec!["G0 X1".into()]));
        q.enqueue_job("C".into(), std::sync::Arc::new(vec!["G0 X2".into()]));

        q.move_up(2);
        assert_eq!(q.queue[1].name, "C");

        q.move_down(0);
        assert_eq!(q.queue[0].name, "C");

        let first = q.pop_next_job().expect("first job should exist");
        assert_eq!(first.name, "C");
        assert_eq!(q.queue.len(), 2);
    }

    #[test]
    fn retry_last_failed_requeues_job() {
        let mut q = JobQueueState::default();
        let job_id = q.enqueue_job("Test".into(), std::sync::Arc::new(vec!["G1 X10".into()]));
        let job = q.pop_next_job().unwrap();
        assert_eq!(job.id, job_id);

        q.record_failure(job, "ALARM:2".into());
        let retry_id = q
            .retry_last_failed()
            .expect("retry should enqueue failed job");

        assert!(retry_id > job_id);
        assert_eq!(q.queue.len(), 1);
        assert!(q.queue[0].name.contains("retry"));
        assert_eq!(q.queue[0].attempts, 2);
    }
}
