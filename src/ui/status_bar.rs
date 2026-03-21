use crate::config::settings::{DisplayUnit, SpeedUnit};
use crate::controller::ControllerCapabilities;
use crate::grbl::types::{GrblState, MacStatus};
use crate::i18n::tr;
use crate::theme;
use egui::{RichText, Ui};
use std::time::Duration;

pub struct StatusBarAction {
    pub feed_up: bool,
    pub feed_down: bool,
    pub rapid_up: bool,
    pub rapid_down: bool,
    pub spindle_up: bool,
    pub spindle_down: bool,
    pub toggle_unit: bool,
    pub toggle_speed_unit: bool,
}

impl Default for StatusBarAction {
    fn default() -> Self {
        Self {
            feed_up: false,
            feed_down: false,
            rapid_up: false,
            rapid_down: false,
            spindle_up: false,
            spindle_down: false,
            toggle_unit: false,
            toggle_speed_unit: false,
        }
    }
}

pub fn show(
    ui: &mut Ui,
    state: &GrblState,
    file_info: Option<(&str, usize, Duration)>,
    progress: Option<(usize, usize)>,
    caps: ControllerCapabilities,
    display_unit: DisplayUnit,
    speed_unit: SpeedUnit,
    cost_estimate: Option<(f32, &str)>,
) -> StatusBarAction {
    let mut action = StatusBarAction::default();

    let avail = ui.available_width();
    let compact = avail < 700.0;
    let sz = if compact { 10.0 } else { 11.0 };

    ui.horizontal_wrapped(|ui| {
        // Status badge
        let (badge_text, badge_color) = status_badge(state.status);
        let badge_str = if compact {
            badge_text[..3.min(badge_text.len())].to_string()
        } else {
            format!(" {badge_text} ")
        };
        let badge = RichText::new(badge_str)
            .color(theme::CRUST)
            .background_color(badge_color)
            .strong()
            .size(sz);
        let badge_resp = ui.label(badge);
        if compact {
            badge_resp.on_hover_text(badge_text);
        }

        ui.separator();

        // Override controls
        let sep = if compact { "" } else { ":" };

        ui.label(RichText::new(format!("F{sep}{}%", state.override_feed)).color(theme::TEXT).monospace().size(sz))
            .on_hover_text(tr("Feed override"));
        if ui
            .add_enabled(caps.supports_feed_override, egui::Button::new("▲").small())
            .on_hover_text(tr("Increase feed override"))
            .clicked()
        {
            action.feed_up = true;
        }
        if ui
            .add_enabled(caps.supports_feed_override, egui::Button::new("▼").small())
            .on_hover_text(tr("Decrease feed override"))
            .clicked()
        {
            action.feed_down = true;
        }

        ui.separator();

        ui.label(RichText::new(format!("R{sep}{}%", state.override_rapid)).color(theme::TEXT).monospace().size(sz))
            .on_hover_text(tr("Rapid override 100%"));
        if ui
            .add_enabled(caps.supports_rapid_override, egui::Button::new("▲").small())
            .on_hover_text(tr("Rapid override 100%"))
            .clicked()
        {
            action.rapid_up = true;
        }
        if ui
            .add_enabled(caps.supports_rapid_override, egui::Button::new("▼").small())
            .on_hover_text(tr("Rapid override 25%"))
            .clicked()
        {
            action.rapid_down = true;
        }

        ui.separator();

        ui.label(RichText::new(format!("S{sep}{}%", state.override_spindle)).color(theme::TEXT).monospace().size(sz))
            .on_hover_text(tr("Spindle:"));
        if ui
            .add_enabled(
                caps.supports_spindle_override,
                egui::Button::new("▲").small(),
            )
            .on_hover_text(tr("Increase laser power override"))
            .clicked()
        {
            action.spindle_up = true;
        }
        if ui
            .add_enabled(
                caps.supports_spindle_override,
                egui::Button::new("▼").small(),
            )
            .on_hover_text(tr("Decrease laser power override"))
            .clicked()
        {
            action.spindle_down = true;
        }

        ui.separator();

        // Unit toggle (F96)
        let unit_label = display_unit.label();
        if ui
            .small_button(unit_label)
            .on_hover_text(tr("Toggle mm / inches"))
            .clicked()
        {
            action.toggle_unit = true;
        }

        // Speed unit toggle
        if ui
            .small_button(speed_unit.label())
            .on_hover_text(tr("Toggle mm/min / mm/s"))
            .clicked()
        {
            action.toggle_speed_unit = true;
        }

        // File info
        if let Some((current, total)) = progress {
            let pct = if total > 0 {
                (current as f32 / total as f32) * 100.0
            } else {
                0.0
            };
            ui.label(
                RichText::new(format!("{current}/{total} ({pct:.0}%)"))
                    .color(theme::YELLOW)
                    .monospace()
                    .size(sz),
            );
        }
        if let Some((filename, lines, est)) = file_info {
            if !compact {
                let time_str = format_duration(est);
                ui.label(
                    RichText::new(format!("{filename} | {lines} lines | ~{time_str}"))
                        .color(theme::SUBTEXT)
                        .size(sz),
                );
            }
        }
        if let Some((cost, currency)) = cost_estimate {
            if cost > 0.0 {
                ui.label(
                    RichText::new(format!("~{cost:.2}{currency}"))
                        .color(theme::GREEN)
                        .size(sz),
                );
            }
        }
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
