use crate::i18n::tr;
use crate::theme;
/// Integrated GCode editor panel
use egui::RichText;

pub struct GCodeEditorState {
    pub is_open: bool,
    pub text: String,
    pub dirty: bool,
    pub confirm_close: bool,
    pub search_query: String,
    pub search_case_sensitive: bool,
}

impl Default for GCodeEditorState {
    fn default() -> Self {
        Self {
            is_open: false,
            text: String::new(),
            dirty: false,
            confirm_close: false,
            search_query: String::new(),
            search_case_sensitive: false,
        }
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

    // Confirmation dialog for unsaved changes
    if state.confirm_close {
        egui::Window::new(format!("⚠ {}", tr("Unsaved Changes")))
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(tr("You have unsaved changes. Discard them?"));
                ui.horizontal(|ui| {
                    if ui.button(format!("💾 {}", tr("Save & Close"))).clicked() {
                        apply_clicked = true;
                        state.confirm_close = false;
                        close_clicked = true;
                    }
                    if ui.button(format!("✘ {}", tr("Discard"))).clicked() {
                        state.confirm_close = false;
                        close_clicked = true;
                    }
                    if ui.button(tr("Cancel")).clicked() {
                        state.confirm_close = false;
                    }
                });
            });
    }

    egui::Window::new("📝 GCode Editor")
        .resizable(true)
        .default_size([640.0, 480.0])
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if state.dirty {
                    ui.label(RichText::new("● unsaved").color(theme::PEACH).small());
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(format!("✖ {}", tr("Close"))).clicked() {
                        if state.dirty {
                            state.confirm_close = true;
                        } else {
                            close_clicked = true;
                        }
                    }
                    if ui
                        .button(RichText::new("✔ Apply").color(theme::GREEN))
                        .clicked()
                    {
                        apply_clicked = true;
                    }
                });
            });
            ui.separator();
            
            // Search bar
            ui.horizontal(|ui| {
                ui.label(tr("Search:"));
                let search_response = ui.add(
                    egui::TextEdit::singleline(&mut state.search_query)
                        .hint_text(tr("Ctrl+F to search"))
                        .desired_width(200.0)
                );
                ui.checkbox(&mut state.search_case_sensitive, tr("Case Sensitive"));
                
                // Handle Ctrl+F focus
                if ui.input(|i| i.key_pressed(egui::Key::F) && i.modifiers.ctrl) {
                    ui.memory_mut(|mem| mem.request_focus(search_response.id));
                }
                
                // Highlight search results
                if !state.search_query.is_empty() {
                    let match_count = count_matches(&state.text, &state.search_query, state.search_case_sensitive);
                    if match_count == 0 {
                        ui.label(RichText::new("No matches").color(theme::SUBTEXT).small());
                    } else {
                        ui.label(RichText::new(format!("{} matches", match_count)).color(theme::GREEN).small());
                    }
                }
            });
            
            ui.separator();

            let mut layouter = |ui: &egui::Ui, string: &dyn egui::TextBuffer, wrap_width: f32| {
                let string = string.as_str();
                let mut job = egui::text::LayoutJob::default();
                job.wrap.max_width = wrap_width;

                for line in string.lines() {
                    let mut chars = line.chars().peekable();
                    let mut current_token = String::new();

                    while let Some(c) = chars.next() {
                        if c == ';' || c == '(' {
                            // Comment rest of line
                            if !current_token.is_empty() {
                                job.append(
                                    &current_token,
                                    0.0,
                                    egui::TextFormat {
                                        font_id: egui::TextStyle::Monospace.resolve(ui.style()),
                                        color: theme::TEXT,
                                        ..Default::default()
                                    },
                                );
                                current_token.clear();
                            }
                            let mut comment = String::from(c);
                            while let Some(&next) = chars.peek() {
                                comment.push(next);
                                chars.next();
                            }
                            job.append(
                                &comment,
                                0.0,
                                egui::TextFormat {
                                    font_id: egui::TextStyle::Monospace.resolve(ui.style()),
                                    color: theme::SUBTEXT,
                                    ..Default::default()
                                },
                            );
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

                ui.ctx().fonts_mut(|f| f.layout_job(job))
            };

            // Line numbers + editor layout
            let font_id = egui::TextStyle::Monospace.resolve(ui.style());
            let line_height = ui.ctx().fonts_mut(|f| f.row_height(&font_id));
            let text = &state.text;
            let line_count = text.lines().count().max(1);
            let gutter_width = format!("{}", line_count).len() as f32 * 8.0 + 16.0;

            ui.horizontal(|ui| {
                // Gutter with line numbers
                ui.vertical(|ui| {
                    ui.set_width(gutter_width);
                    for i in 1..=line_count {
                        ui.label(
                            RichText::new(format!("{}", i))
                                .font(font_id.clone())
                                .color(theme::SUBTEXT)
                                .small(),
                        );
                    }
                });

                let response = egui::ScrollArea::vertical()
                    .max_height(ui.available_height() - 100.0) // Extra space for search bar
                    .show(ui, |ui| {
                        ui.add_sized(
                            [ui.available_width(), (line_count as f32 * line_height).max(400.0)],
                            egui::TextEdit::multiline(&mut state.text)
                                .font(egui::TextStyle::Monospace)
                                .layouter(&mut layouter)
                                .desired_rows(line_count.max(20)),
                        )
                    });

                if response.inner.changed() {
                    state.dirty = true;
                }
            });
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

fn count_matches(text: &str, query: &str, case_sensitive: bool) -> usize {
    if query.is_empty() {
        return 0;
    }

    let text_owned;
    let query_owned;
    let search_text = if case_sensitive {
        text
    } else {
        text_owned = text.to_lowercase();
        &text_owned
    };
    let search_query = if case_sensitive {
        query
    } else {
        query_owned = query.to_lowercase();
        &query_owned
    };

    let mut count = 0;
    for line in search_text.lines() {
        let mut start = 0;
        while let Some(pos) = line[start..].find(search_query) {
            count += 1;
            start += pos + 1;
        }
    }

    count
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
