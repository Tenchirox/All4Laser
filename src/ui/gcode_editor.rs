/// Integrated GCode editor panel
use egui::{RichText, Color32};
use crate::theme;

pub struct GCodeEditorState {
    pub is_open: bool,
    pub text: String,
    pub dirty: bool,
}

impl Default for GCodeEditorState {
    fn default() -> Self {
        Self { is_open: false, text: String::new(), dirty: false }
    }
}

pub struct GCodeEditorAction {
    pub apply: Option<Vec<String>>,
}

pub fn show(ctx: &egui::Context, state: &mut GCodeEditorState) -> GCodeEditorAction {
    let mut action = GCodeEditorAction { apply: None };

    if !state.is_open {
        return action;
    }

    let mut apply_clicked = false;
    let mut close_clicked = false;

    egui::Window::new("📝 GCode Editor")
        .resizable(true)
        .default_size([640.0, 480.0])
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("📝 GCode Editor").color(theme::LAVENDER).strong());
                if state.dirty {
                    ui.label(RichText::new("● unsaved").color(theme::PEACH).small());
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("✖ Close").clicked() {
                        close_clicked = true;
                    }
                    if ui.button(RichText::new("✔ Apply").color(theme::GREEN)).clicked() {
                        apply_clicked = true;
                    }
                });
            });
            ui.separator();

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut job = egui::text::LayoutJob::default();
                job.wrap.max_width = wrap_width;

                for line in string.lines() {
                    let mut chars = line.chars().peekable();
                    let mut current_token = String::new();

                    while let Some(c) = chars.next() {
                         if c == ';' || c == '(' {
                            // Comment rest of line
                            if !current_token.is_empty() {
                                job.append(&current_token, 0.0, egui::TextFormat {
                                    font_id: egui::TextStyle::Monospace.resolve(ui.style()),
                                    color: theme::TEXT,
                                    ..Default::default()
                                });
                                current_token.clear();
                            }
                            let mut comment = String::from(c);
                            while let Some(&next) = chars.peek() {
                                comment.push(next);
                                chars.next();
                            }
                            job.append(&comment, 0.0, egui::TextFormat {
                                font_id: egui::TextStyle::Monospace.resolve(ui.style()),
                                color: theme::SUBTEXT,
                                ..Default::default()
                            });
                            break;
                        } else if c.is_whitespace() {
                            if !current_token.is_empty() {
                                job.append(&current_token, 0.0, format_token(ui, &current_token));
                                current_token.clear();
                            }
                            job.append(&c.to_string(), 0.0, egui::TextFormat::default());
                        } else {
                            current_token.push(c);
                        }
                    }
                    if !current_token.is_empty() {
                        job.append(&current_token, 0.0, format_token(ui, &current_token));
                    }
                    job.append("\n", 0.0, egui::TextFormat::default());
                }

                ui.fonts(|f| f.layout_job(job))
            };

            let response = egui::ScrollArea::vertical()
                .max_height(420.0)
                .show(ui, |ui| {
                    ui.add_sized(
                        [ui.available_width(), 400.0],
                        egui::TextEdit::multiline(&mut state.text)
                            .font(egui::TextStyle::Monospace)
                            .layouter(&mut layouter)
                            .desired_rows(20),
                    )
                });

            if response.inner.changed() {
                state.dirty = true;
            }
        });

    if apply_clicked {
        let lines: Vec<String> = state.text.lines().map(|l| l.to_string()).collect();
        action.apply = Some(lines);
        state.dirty = false;
    }
    if close_clicked {
        state.is_open = false;
    }

    action
}

fn format_token(ui: &egui::Ui, token: &str) -> egui::TextFormat {
    let first = token.chars().next().unwrap_or(' ').to_ascii_uppercase();
    let color = match first {
        'G' => theme::BLUE,
        'M' => theme::PEACH,
        'X' | 'Y' | 'Z' => theme::LAVENDER,
        'F' => theme::TEAL,
        'S' => theme::RED,
        _ => theme::TEXT,
    };

    egui::TextFormat {
        font_id: egui::TextStyle::Monospace.resolve(ui.style()),
        color,
        ..Default::default()
    }
}
