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
    ("Space", tr("Run / Pause program")),
    ("Escape", tr("Abort program")),
    ("F", tr("Toggle framing mode")),
    ("H", tr("Home machine")),
    ("Z", tr("Set zero (G92 X0Y0Z0)")),
    ("Arrow keys", tr("Jog X/Y by step")),
    ("Page Up / Down", tr("Jog Z by step")),
    ("Ctrl+O", tr("Open file")),
    ("Ctrl+S", tr("Save GCode")),
    ("Ctrl+Z", tr("Undo shape edits")),
    ("Ctrl+Shift+Z / Ctrl+Y", tr("Redo shape edits")),
    ("Ctrl+C", tr("Copy selected shape(s)")),
    ("Ctrl+X", tr("Cut selected shape(s)")),
    ("Ctrl+V", tr("Paste shape(s)")),
    ("?", tr("Show this help panel")),
    ("T", tr("Toggle theme (light/dark)")),
    ("+/-", tr("Zoom in/out")),
    ("Numpad 0", tr("Fit view to job")),
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
