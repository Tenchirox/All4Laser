use std::collections::BTreeMap;
use egui::{Context, RichText, Grid, ScrollArea, Window};
use crate::theme;

#[derive(Default, Clone)]
pub struct SettingsDialogState {
    pub is_open: bool,
    pub settings: BTreeMap<i32, String>,
    pub pending_writes: Vec<(i32, String)>,
}

pub fn show(ctx: &Context, state: &mut SettingsDialogState) {
    if !state.is_open {
        return;
    }

    let mut open = state.is_open;
    Window::new("GRBL Settings")
        .open(&mut open)
        .resizable(true)
        .default_width(500.0)
        .default_height(400.0)
        .show(ctx, |ui| {
            ui.label(RichText::new("Machine Firmware Settings").color(theme::LAVENDER).strong());
            ui.add_space(8.0);
            
            ScrollArea::vertical().show(ui, |ui| {
                Grid::new("grbl_settings_grid")
                    .num_columns(3)
                    .spacing([16.0, 8.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label(RichText::new("ID").strong());
                        ui.label(RichText::new("Value").strong());
                        ui.label(RichText::new("Description").strong());
                        ui.end_row();

                        let mut updates = Vec::new();
                        for (&id, val) in &state.settings {
                            ui.label(format!("${id}"));
                            
                            let mut edit_val = val.clone();
                            if ui.text_edit_singleline(&mut edit_val).changed() {
                                updates.push((id, edit_val));
                            }
                            
                            ui.label(get_setting_description(id));
                            ui.end_row();
                        }
                        
                        for (id, val) in updates {
                            state.settings.insert(id, val);
                        }

                        if state.settings.is_empty() {
                            ui.label("Waiting for settings...");
                            ui.end_row();
                        }
                    });
            });

            ui.add_space(8.0);
            
            ui.horizontal(|ui| {
                if ui.button(RichText::new("Save to Board").color(theme::GREEN)).clicked() {
                    state.pending_writes.clear();
                    for (&id, val) in &state.settings {
                        state.pending_writes.push((id, val.clone()));
                    }
                }
                if ui.button("Refresh").clicked() {
                    state.settings.clear();
                    state.pending_writes.push((-1, "$$".to_string())); // Special marker
                }
            });
        });

    state.is_open = open;
}

fn get_setting_description(id: i32) -> &'static str {
    match id {
        0 => "Step pulse time, microseconds",
        1 => "Step idle delay, milliseconds",
        2 => "Step port invert, mask",
        3 => "Direction port invert, mask",
        4 => "Step enable invert, boolean",
        5 => "Limit pins invert, boolean",
        6 => "Probe pin invert, boolean",
        10 => "Status report options, mask",
        11 => "Junction deviation, millimeters",
        12 => "Arc tolerance, millimeters",
        13 => "Report inches, boolean",
        20 => "Soft limits enable, boolean",
        21 => "Hard limits enable, boolean",
        22 => "Homing cycle enable, boolean",
        23 => "Homing direction invert, mask",
        24 => "Homing locate feed rate, mm/min",
        25 => "Homing search seek rate, mm/min",
        26 => "Homing switch debounce delay, ms",
        27 => "Homing switch pull-off distance, mm",
        30 => "Maximum spindle speed, RPM",
        31 => "Minimum spindle speed, RPM",
        32 => "Laser-mode enable, boolean",
        100 => "X-axis steps per millimeter",
        101 => "Y-axis steps per millimeter",
        102 => "Z-axis steps per millimeter",
        110 => "X-axis maximum rate, mm/min",
        111 => "Y-axis maximum rate, mm/min",
        112 => "Z-axis maximum rate, mm/min",
        120 => "X-axis acceleration, mm/sec^2",
        121 => "Y-axis acceleration, mm/sec^2",
        122 => "Z-axis acceleration, mm/sec^2",
        130 => "X-axis maximum travel, millimeters",
        131 => "Y-axis maximum travel, millimeters",
        132 => "Z-axis maximum travel, millimeters",
        _ => "Unknown setting",
    }
}
