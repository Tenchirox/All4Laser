use crate::config::settings::DisplayUnit;
use crate::controller::ControllerCapabilities;
use crate::grbl::types::{GrblState, MacStatus};
use crate::i18n::tr;
use crate::theme;
use egui::{RichText, Ui};
use std::time::Duration;

pub struct StatusBarAction {
    pub feed_up: bool,
    pub feed_down: bool,
    pub feed_reset: bool,
    pub rapid_up: bool,
    pub rapid_down: bool,
    pub rapid_reset: bool,
    pub spindle_up: bool,
    pub spindle_down: bool,
    pub spindle_reset: bool,
    pub toggle_unit: bool,
}

impl Default for StatusBarAction {
    fn default() -> Self {
        Self {
            feed_up: false,
            feed_down: false,
            feed_reset: false,
            rapid_up: false,
            rapid_down: false,
            rapid_reset: false,
            spindle_up: false,
            spindle_down: false,
            spindle_reset: false,
            toggle_unit: false,
        }
    }
}

pub fn show(
    ui: &mut Ui,
    state: &GrblState,
    file_info: Option<(&str, usize, Duration)>,
    progress: Option<(usize, usize)>,
    pass_info: Option<(u32, u32)>,
    zoom: f32,
    caps: ControllerCapabilities,
    display_unit: DisplayUnit,
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
        ui.label(badge);

        ui.separator();

        // Override controls
        let feed_color = if state.override_feed != 100 { theme::YELLOW } else { theme::TEXT };
        ui.label(RichText::new(format!("{}:{}%", tr("Speed"), state.override_feed)).color(feed_color).monospace().size(sz))
            .on_hover_text(tr("Movement speed override"));
        if ui
            .add_enabled(caps.supports_feed_override && state.override_feed != 100, egui::Button::new("↺").small())
            .on_hover_text(tr("Speed reset 100%"))
            .clicked()
        {
            action.feed_reset = true;
        }
        if ui
            .add_enabled(caps.supports_feed_override, egui::Button::new("+").small())
            .on_hover_text(tr("Speed +10%"))
            .clicked()
        {
            action.feed_up = true;
        }
        if ui
            .add_enabled(caps.supports_feed_override, egui::Button::new("-").small())
            .on_hover_text(tr("Speed -10%"))
            .clicked()
        {
            action.feed_down = true;
        }

        ui.separator();

        let rapid_color = if state.override_rapid != 100 { theme::YELLOW } else { theme::TEXT };
        ui.label(RichText::new(format!("{}:{}%", tr("Rapid"), state.override_rapid)).color(rapid_color).monospace().size(sz))
            .on_hover_text(tr("Rapid movement override"));
        if ui
            .add_enabled(caps.supports_rapid_override && state.override_rapid != 100, egui::Button::new("↺").small())
            .on_hover_text(tr("Rapid reset 100%"))
            .clicked()
        {
            action.rapid_reset = true;
        }
        if ui
            .add_enabled(caps.supports_rapid_override, egui::Button::new("+").small())
            .on_hover_text(tr("Rapid 100%"))
            .clicked()
        {
            action.rapid_up = true;
        }
        if ui
            .add_enabled(caps.supports_rapid_override, egui::Button::new("-").small())
            .on_hover_text(tr("Rapid 25%"))
            .clicked()
        {
            action.rapid_down = true;
        }

        ui.separator();

        let spindle_color = if state.override_spindle != 100 { theme::YELLOW } else { theme::TEXT };
        ui.label(RichText::new(format!("{}:{}%", tr("Power"), state.override_spindle)).color(spindle_color).monospace().size(sz))
            .on_hover_text(tr("Laser power override"));
        if ui
            .add_enabled(
                caps.supports_spindle_override && state.override_spindle != 100,
                egui::Button::new("↺").small(),
            )
            .on_hover_text(tr("Power reset 100%"))
            .clicked()
        {
            action.spindle_reset = true;
        }
        if ui
            .add_enabled(
                caps.supports_spindle_override,
                egui::Button::new("+").small(),
            )
            .on_hover_text(tr("Power +10%"))
            .clicked()
        {
            action.spindle_up = true;
        }
        if ui
            .add_enabled(
                caps.supports_spindle_override,
                egui::Button::new("-").small(),
            )
            .on_hover_text(tr("Power -10%"))
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

        ui.separator();
        ui.label(
            RichText::new(format!("{}: {:.0}%", tr("Zoom"), zoom * 100.0))
                .monospace()
                .size(sz)
                .color(theme::SUBTEXT),
        )
        .on_hover_text(tr("Current preview zoom level"));

        // Progress bar + text
        if let Some((current, total)) = progress {
            let pct = if total > 0 {
                (current as f32 / total as f32) * 100.0
            } else {
                0.0
            };
            ui.separator();
            let bar_width = if compact { 50.0 } else { 80.0 };
            let bar = egui::ProgressBar::new(pct / 100.0)
                .desired_width(bar_width)
                .text(format!("{pct:.0}%"));
            ui.add(bar);
            if !compact {
                ui.label(
                    RichText::new(format!("{current}/{total}"))
                        .color(theme::YELLOW)
                        .monospace()
                        .size(sz),
                );
            }
            if let Some((current_pass, total_passes)) = pass_info {
                if total_passes > 1 {
                    ui.label(
                        RichText::new(format!("{} {}/{}", tr("Pass"), current_pass, total_passes))
                            .color(theme::LAVENDER)
                            .monospace()
                            .size(sz),
                    );
                }
            }
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
                )
                .on_hover_text(tr("Estimated job cost based on machine time and power consumption"));
            }
        }
    });

    action
}

fn status_badge(status: MacStatus) -> (&'static str, egui::Color32) {
    match status {
        MacStatus::Disconnected => ("DISCONNECT", theme::SURFACE2),
        MacStatus::Connecting => ("CONNECT", theme::YELLOW),
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
