use egui::{Ui, RichText};
use crate::controller::ControllerCapabilities;
use crate::grbl::types::{GrblState, MacStatus};
use crate::theme;
use std::time::Duration;

pub struct StatusBarAction {
    pub feed_up: bool,
    pub feed_down: bool,
    pub rapid_up: bool,
    pub rapid_down: bool,
    pub spindle_up: bool,
    pub spindle_down: bool,
}

impl Default for StatusBarAction {
    fn default() -> Self {
        Self {
            feed_up: false, feed_down: false,
            rapid_up: false, rapid_down: false,
            spindle_up: false, spindle_down: false,
        }
    }
}

pub fn show(
    ui: &mut Ui,
    state: &GrblState,
    file_info: Option<(&str, usize, Duration)>,
    progress: Option<(usize, usize)>,
    caps: ControllerCapabilities,
) -> StatusBarAction {
    let mut action = StatusBarAction::default();

    ui.horizontal(|ui| {
        // Status badge
        let (badge_text, badge_color) = status_badge(state.status);
        let badge = RichText::new(format!(" {badge_text} "))
            .color(theme::CRUST)
            .background_color(badge_color)
            .strong()
            .size(12.0);
        ui.label(badge);

        ui.separator();

        // Override controls
        ui.label(RichText::new("Feed:").color(theme::SUBTEXT).size(11.0));
        ui.label(RichText::new(format!("{}%", state.override_feed)).color(theme::TEXT).monospace().size(11.0));
        if ui.add_enabled(caps.supports_feed_override, egui::Button::new("▲").small()).clicked() {
            action.feed_up = true;
        }
        if ui.add_enabled(caps.supports_feed_override, egui::Button::new("▼").small()).clicked() {
            action.feed_down = true;
        }

        ui.separator();

        ui.label(RichText::new("Rapid:").color(theme::SUBTEXT).size(11.0));
        ui.label(RichText::new(format!("{}%", state.override_rapid)).color(theme::TEXT).monospace().size(11.0));
        if ui.add_enabled(caps.supports_rapid_override, egui::Button::new("▲").small()).clicked() {
            action.rapid_up = true;
        }
        if ui.add_enabled(caps.supports_rapid_override, egui::Button::new("▼").small()).clicked() {
            action.rapid_down = true;
        }

        ui.separator();

        ui.label(RichText::new("Spindle:").color(theme::SUBTEXT).size(11.0));
        ui.label(RichText::new(format!("{}%", state.override_spindle)).color(theme::TEXT).monospace().size(11.0));
        if ui.add_enabled(caps.supports_spindle_override, egui::Button::new("▲").small()).clicked() {
            action.spindle_up = true;
        }
        if ui.add_enabled(caps.supports_spindle_override, egui::Button::new("▼").small()).clicked() {
            action.spindle_down = true;
        }

        // File info on the right
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if let Some((filename, lines, est)) = file_info {
                let time_str = format_duration(est);
                ui.label(RichText::new(format!("{filename} | {lines} lines | ~{time_str}")).color(theme::SUBTEXT).size(11.0));
            }
            if let Some((current, total)) = progress {
                let pct = if total > 0 { (current as f32 / total as f32) * 100.0 } else { 0.0 };
                ui.label(RichText::new(format!("{current}/{total} ({pct:.0}%)")).color(theme::YELLOW).monospace().size(11.0));
            }
        });
    });

    action
}

fn status_badge(status: MacStatus) -> (&'static str, egui::Color32) {
    match status {
        MacStatus::Disconnected => ("DISCONNECTED", theme::SURFACE2),
        MacStatus::Connecting => ("CONNECTING", theme::YELLOW),
        MacStatus::Idle => ("IDLE", theme::GREEN),
        MacStatus::Run => ("RUN", theme::BLUE),
        MacStatus::Hold => ("HOLD", theme::YELLOW),
        MacStatus::Jog => ("JOG", theme::LAVENDER),
        MacStatus::Alarm => ("ALARM", theme::RED),
        MacStatus::Door => ("DOOR", theme::PEACH),
        MacStatus::Check => ("CHECK", theme::MAUVE),
        MacStatus::Home => ("HOME", theme::BLUE),
        MacStatus::Sleep => ("SLEEP", theme::OVERLAY0),
    }
}

fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();
    if secs < 60 {
        format!("{secs}s")
    } else if secs < 3600 {
        format!("{}m {:02}s", secs / 60, secs % 60)
    } else {
        format!("{}h {:02}m", secs / 3600, (secs % 3600) / 60)
    }
}
