use crate::i18n::tr;
use crate::theme;
use egui::{RichText, Ui};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Serialize, Deserialize)]
pub struct MacroDef {
    pub label: String,
    pub gcode: String,
}

#[derive(Clone)]
pub struct MacrosState {
    pub items: Vec<MacroDef>,
    pub editing_idx: Option<usize>,
    pub edit_label: String,
    pub edit_gcode: String,
}

impl Default for MacrosState {
    fn default() -> Self {
        let mut state = Self {
            items: Vec::new(),
            editing_idx: None,
            edit_label: String::new(),
            edit_gcode: String::new(),
        };
        state.load();

        if state.items.is_empty() {
            state.items.push(MacroDef {
                label: "Probe Z".into(),
                gcode: "G38.2 Z-20 F100\nG92 Z0\nG0 Z5".into(),
            });
            state.items.push(MacroDef {
                label: "Laser Test".into(),
                gcode: "M3 S10\nG4 P2\nM5".into(),
            });
        }
        state
    }
}

impl MacrosState {
    const FILE_PATH: &'static str = "macros.json";

    pub fn load(&mut self) {
        if let Ok(data) = fs::read_to_string(Self::FILE_PATH) {
            if let Ok(items) = serde_json::from_str(&data) {
                self.items = items;
            }
        }
    }

    pub fn save(&self) {
        if let Ok(data) = serde_json::to_string_pretty(&self.items) {
            let _ = fs::write(Self::FILE_PATH, data);
        }
    }
}

pub struct MacrosAction {
    pub execute_macro: Option<MacroDef>,
}

pub fn show(ui: &mut Ui, state: &mut MacrosState, connected: bool) -> MacrosAction {
    let mut action = MacrosAction {
        execute_macro: None,
    };

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new(tr("Macros")).color(theme::LAVENDER).strong());
            if ui.button("+").clicked() {
                state.items.push(MacroDef {
                    label: tr("New Macro"),
                    gcode: "".into(),
                });
                state.editing_idx = Some(state.items.len() - 1);
                state.edit_label = tr("New Macro");
                state.edit_gcode = "".into();
                state.save();
            }
        });

        ui.add_space(4.0);

        let mut delete_idx = None;

        let count = state.items.len();
        for i in 0..count {
            if state.editing_idx == Some(i) {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(tr("Name:"));
                        ui.text_edit_singleline(&mut state.edit_label);
                    });
                    ui.label(tr("GCode (multiline):"));
                    ui.text_edit_multiline(&mut state.edit_gcode);
                    let has_executable = state.edit_gcode.lines().map(str::trim).any(|line| {
                        !line.is_empty() && !line.starts_with(';') && !line.starts_with('#')
                    });
                    if !has_executable {
                        ui.label(
                            RichText::new(tr("Add at least one executable G-code line."))
                                .small()
                                .color(theme::SUBTEXT),
                        );
                    }
                    ui.horizontal(|ui| {
                        let save_enabled = !state.edit_label.trim().is_empty() && has_executable;
                        if ui
                            .add_enabled(
                                save_enabled,
                                egui::Button::new(RichText::new(tr("Save")).color(theme::GREEN)),
                            )
                            .clicked()
                        {
                            state.items[i].label = state.edit_label.trim().to_string();
                            state.items[i].gcode = state.edit_gcode.clone();
                            state.editing_idx = None;
                            state.save();
                        }
                        if ui.button(tr("Cancel")).clicked() {
                            state.editing_idx = None;
                        }
                        if ui
                            .button(RichText::new(tr("Delete")).color(theme::RED))
                            .clicked()
                        {
                            delete_idx = Some(i);
                            state.editing_idx = None;
                        }
                    });
                });
            } else {
                let mac_label = state.items[i].label.clone();
                let mac_gcode = state.items[i].gcode.clone();
                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(connected, egui::Button::new(&mac_label))
                        .clicked()
                    {
                        action.execute_macro = Some(MacroDef {
                            label: mac_label.clone(),
                            gcode: mac_gcode.clone(),
                        });
                    }
                    if ui.button("✎").clicked() {
                        state.editing_idx = Some(i);
                        state.edit_label = mac_label;
                        state.edit_gcode = mac_gcode;
                    }
                });
            }
        }

        if let Some(i) = delete_idx {
            state.items.remove(i);
            state.save();
        }
    });

    action
}
