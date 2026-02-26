use egui::{Ui, RichText, ScrollArea, TextEdit};
use crate::theme;

pub struct ConsoleAction {
    pub send_command: Option<String>,
}

pub fn show(ui: &mut Ui, log: &[String], input: &mut String) -> ConsoleAction {
    let mut action = ConsoleAction { send_command: None };

    ui.group(|ui| {
        ui.label(RichText::new("Console").color(theme::LAVENDER).strong().size(14.0));
        ui.add_space(4.0);

        let available = ui.available_height() - 30.0;
        let log_height = available.max(60.0);

        ScrollArea::vertical()
            .max_height(log_height)
            .stick_to_bottom(true)
            .id_salt("console_scroll")
            .show(ui, |ui| {
                for line in log {
                    let color = if line.contains("error") || line.contains("ALARM") {
                        theme::RED
                    } else if line.contains("ok") {
                        theme::GREEN
                    } else if line.starts_with(">") {
                        theme::YELLOW
                    } else {
                        theme::SUBTEXT
                    };
                    ui.label(RichText::new(line).color(color).monospace().size(11.0));
                }
            });

        ui.horizontal(|ui| {
            let input_response = ui.add(
                TextEdit::singleline(input)
                    .desired_width(180.0) // Stable width to avoid feedback loops
                    .hint_text("GCode command…")
                    .font(egui::TextStyle::Monospace),
            );
            if ui.button("Send").clicked()
                || (input_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
            {
                if !input.trim().is_empty() {
                    action.send_command = Some(input.trim().to_string());
                    input.clear();
                }
                input_response.request_focus();
            }
        });
    });

    action
}
