use crate::i18n::tr;
use crate::theme;
use egui::{RichText, ScrollArea, TextEdit, Ui};

pub struct ConsoleAction {
    pub send_command: Option<String>,
    pub clear_log: bool,
    pub export_log: bool,
}

#[derive(Default)]
pub struct ConsoleState {
    pub input: String,
    pub history: Vec<String>,
    pub history_cursor: Option<usize>,
    pub filter_text: String,
    pub filter_error_only: bool,
}

const COMMON_COMMANDS: &[&str] = &["G0", "G1", "G28", "M3", "M5", "M8", "M9", "$H", "$$", "$X", "$C"];

fn classify_line_color(line: &str) -> egui::Color32 {
    let trimmed = line.trim_start();
    if trimmed.starts_with("error:") || trimmed.starts_with("ALARM") {
        theme::RED
    } else if trimmed == "ok" || trimmed.starts_with("ok ") {
        theme::GREEN
    } else if trimmed.starts_with(">") || trimmed.starts_with("[CMD]") {
        theme::YELLOW
    } else if trimmed.starts_with("[MSG:") || trimmed.starts_with("Grbl ") {
        theme::TEAL
    } else if trimmed.starts_with("Preflight") || trimmed.starts_with("[PRE]") {
        theme::LAVENDER
    } else {
        theme::SUBTEXT
    }
}

pub fn show(ui: &mut Ui, log: &[String], console_state: &mut ConsoleState) -> ConsoleAction {
    let mut action = ConsoleAction { send_command: None, clear_log: false, export_log: false };

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(crate::i18n::tr("Console"))
                    .color(theme::LAVENDER)
                    .strong()
                    .size(14.0),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.small_button("💾").on_hover_text(tr("Export log")).clicked() {
                    action.export_log = true;
                }
                if ui.small_button("🗑").on_hover_text(crate::i18n::tr("Clear")).clicked() {
                    action.clear_log = true;
                }
                ui.label(
                    RichText::new(format!("{} lines", log.len()))
                        .small()
                        .color(theme::SUBTEXT),
                );
            });
        });
        ui.add_space(4.0);

        // Filter controls
        ui.horizontal(|ui| {
            ui.checkbox(&mut console_state.filter_error_only, tr("Errors only"));
            ui.add(
                TextEdit::singleline(&mut console_state.filter_text)
                    .desired_width(120.0)
                    .hint_text(tr("Filter...")),
            );
        });
        ui.add_space(2.0);

        let available = ui.available_height() - 30.0;
        let log_height = available.max(60.0);

        ScrollArea::vertical()
            .max_height(log_height)
            .stick_to_bottom(true)
            .id_salt("console_scroll")
            .show(ui, |ui| {
                for line in log {
                    // Apply filters
                    if console_state.filter_error_only && !line.trim_start().starts_with("error:") {
                        continue;
                    }
                    if !console_state.filter_text.is_empty() 
                        && !line.to_lowercase().contains(&console_state.filter_text.to_lowercase()) {
                        continue;
                    }
                    let color = classify_line_color(line);
                    ui.label(RichText::new(line).color(color).monospace().size(11.0));
                }
            });

        ui.horizontal(|ui| {
            let input_width = (ui.available_width() - 50.0).max(100.0);
            let input_response = ui.add(
                TextEdit::singleline(&mut console_state.input)
                    .desired_width(input_width)
                    .hint_text(tr("GCode command…"))
                    .font(egui::TextStyle::Monospace),
            );

            // Auto-complete popup
            if input_response.has_focus() && !console_state.input.is_empty() {
                let matches: Vec<&str> = COMMON_COMMANDS
                    .iter()
                    .filter(|&&cmd| cmd.to_lowercase().starts_with(&console_state.input.to_lowercase()))
                    .copied()
                    .collect();
                if !matches.is_empty() && console_state.input.len() < 4 {
                    egui::popup_below_widget(ui, ui.id().with("autocomplete"), &input_response, |ui| {
                        ui.set_min_width(80.0);
                        for cmd in matches.iter().take(5) {
                            if ui.selectable_label(false, *cmd).clicked() {
                                console_state.input = cmd.to_string();
                            }
                        }
                    });
                }
            }

            // Command history navigation
            if input_response.has_focus() {
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    if !console_state.history.is_empty() {
                        let cursor = console_state.history_cursor
                            .map(|c| c.saturating_sub(1))
                            .unwrap_or(console_state.history.len() - 1);
                        console_state.history_cursor = Some(cursor);
                        console_state.input = console_state.history[cursor].clone();
                    }
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    if let Some(cursor) = console_state.history_cursor {
                        if cursor + 1 < console_state.history.len() {
                            let next = cursor + 1;
                            console_state.history_cursor = Some(next);
                            console_state.input = console_state.history[next].clone();
                        } else {
                            console_state.history_cursor = None;
                            console_state.input.clear();
                        }
                    }
                }
            }

            if ui.button(tr("Send")).clicked()
                || (input_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
            {
                if !console_state.input.trim().is_empty() {
                    let cmd = console_state.input.trim().to_string();
                    console_state.history.push(cmd.clone());
                    console_state.history_cursor = None;
                    action.send_command = Some(cmd);
                    console_state.input.clear();
                }
                input_response.request_focus();
            }
        });
    });

    action
}
