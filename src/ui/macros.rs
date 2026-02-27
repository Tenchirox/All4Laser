use std::fs;
use serde::{Serialize, Deserialize};
use egui::{Ui, RichText};
use crate::theme;

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
            state.items.push(MacroDef { label: "Probe Z".into(), gcode: "G38.2 Z-20 F100\nG92 Z0\nG0 Z5".into() });
            state.items.push(MacroDef { label: "Laser Test".into(), gcode: "M3 S10\nG4 P2\nM5".into() });
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
    pub execute_macro: Option<String>,
}

pub fn show(ui: &mut Ui, state: &mut MacrosState, connected: bool) -> MacrosAction {
    let mut action = MacrosAction { execute_macro: None };

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Macros").color(theme::LAVENDER).strong());
            if ui.button("+").clicked() {
                state.items.push(MacroDef { label: "New Macro".into(), gcode: "".into() });
                state.editing_idx = Some(state.items.len() - 1);
                state.edit_label = "New Macro".into();
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
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut state.edit_label);
                    });
                    ui.label("GCode (multiline):");
                    ui.text_edit_multiline(&mut state.edit_gcode);
                    ui.horizontal(|ui| {
                        if ui.button(RichText::new("Save").color(theme::GREEN)).clicked() {
                            state.items[i].label = state.edit_label.clone();
                            state.items[i].gcode = state.edit_gcode.clone();
                            state.editing_idx = None;
                            state.save();
                        }
                        if ui.button("Cancel").clicked() {
                            state.editing_idx = None;
                        }
                        if ui.button(RichText::new("Delete").color(theme::RED)).clicked() {
                            delete_idx = Some(i);
                            state.editing_idx = None;
                        }
                    });
                });
            } else {
                let mac_label = state.items[i].label.clone();
                let mac_gcode = state.items[i].gcode.clone();
                ui.horizontal(|ui| {
                    if ui.add_enabled(connected, egui::Button::new(&mac_label)).clicked() {
                        action.execute_macro = Some(mac_gcode.clone());
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
