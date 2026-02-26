/// Keyboard shortcuts help panel
use egui::{Ui, RichText, Key, Modifiers};
use crate::theme;

pub struct ShortcutsState {
    pub is_open: bool,
}

impl Default for ShortcutsState {
    fn default() -> Self {
        Self { is_open: false }
    }
}

const SHORTCUTS: &[(&str, &str)] = &[
    ("Space", "Run / Pause program"),
    ("Escape", "Abort program"),
    ("F", "Toggle framing mode"),
    ("H", "Home machine"),
    ("Z", "Set zero (G92 X0Y0Z0)"),
    ("Arrow keys", "Jog X/Y by step"),
    ("Page Up / Down", "Jog Z by step"),
    ("Ctrl+O", "Open file"),
    ("Ctrl+S", "Save GCode"),
    ("Ctrl+Z", "Undo drawing"),
    ("?", "Show this help panel"),
    ("T", "Toggle theme (light/dark)"),
    ("+/-", "Zoom in/out"),
    ("Numpad 0", "Fit view to job"),
];

pub fn show(ui: &mut Ui, state: &mut ShortcutsState) {
    if !state.is_open {
        return;
    }

    let mut close = false;

    egui::Window::new("⌨ Keyboard Shortcuts")
        .resizable(false)
        .collapsible(false)
        .default_size([380.0, 400.0])
        .show(ui.ctx(), |ui| {
            egui::Grid::new("shortcuts_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label(RichText::new("Key").color(theme::LAVENDER).strong());
                    ui.label(RichText::new("Action").color(theme::LAVENDER).strong());
                    ui.end_row();

                    for (key, desc) in SHORTCUTS {
                        ui.label(RichText::new(*key).monospace().color(theme::BLUE));
                        ui.label(*desc);
                        ui.end_row();
                    }
                });
            ui.separator();
            if ui.button("Close").clicked() {
                close = true;
            }
        });

    if close {
        state.is_open = false;
    }
}

/// Returns true if a shortcut was handled (for use in keyboard handler)
pub fn check_open_shortcut(ctx: &egui::Context) -> bool {
    ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Questionmark))
}
