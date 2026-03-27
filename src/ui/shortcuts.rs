#![allow(dead_code)]

use crate::i18n::tr;
use crate::theme;
/// Keyboard shortcuts help panel
use egui::{Key, Modifiers, RichText, Ui};

pub struct ShortcutsState {
    pub is_open: bool,
    pub filter: String,
}

impl Default for ShortcutsState {
    fn default() -> Self {
        Self { is_open: false, filter: String::new() }
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
    ("Ctrl+Z", "Undo shape edits"),
    ("Ctrl+Shift+Z / Ctrl+Y", "Redo shape edits"),
    ("Ctrl+C", "Copy selected shape(s)"),
    ("Ctrl+X", "Cut selected shape(s)"),
    ("Ctrl+V", "Paste shape(s)"),
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

    egui::Window::new(format!("⌨ {}", tr("Keyboard Shortcuts")))
        .resizable(true)
        .collapsible(false)
        .default_size([380.0, 400.0])
        .show(ui.ctx(), |ui| {
            // Filter input
            ui.horizontal(|ui| {
                ui.label(format!("{}:", tr("Search")));
                ui.text_edit_singleline(&mut state.filter);
                if ui.button(tr("Clear")).clicked() {
                    state.filter.clear();
                }
            });
            ui.separator();
            
            egui::Grid::new("shortcuts_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label(RichText::new(tr("Key")).color(theme::LAVENDER).strong());
                    ui.label(RichText::new(tr("Action")).color(theme::LAVENDER).strong());
                    ui.end_row();

                    let filter_lower = state.filter.to_lowercase();
                    for (key, desc) in SHORTCUTS {
                        // Apply filter
                        if !filter_lower.is_empty() 
                            && !key.to_lowercase().contains(&filter_lower)
                            && !desc.to_lowercase().contains(&filter_lower) {
                            continue;
                        }
                        ui.label(RichText::new(*key).monospace().color(theme::BLUE));
                        ui.label(*desc);
                        ui.end_row();
                    }
                });
            ui.separator();
            if ui.button(tr("Close")).clicked() {
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
