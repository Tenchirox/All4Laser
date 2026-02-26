/// Integrated GCode editor panel
use egui::RichText;
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

            let response = egui::ScrollArea::vertical()
                .max_height(420.0)
                .show(ui, |ui| {
                    ui.add_sized(
                        [ui.available_width(), 400.0],
                        egui::TextEdit::multiline(&mut state.text)
                            .font(egui::TextStyle::Monospace)
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
