use egui::{Ui, RichText};
use serde::{Deserialize, Serialize};
use crate::theme;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaterialPreset {
    pub name: String,
    pub thickness_mm: f32,
    pub speed: f32,
    pub power: f32,
    pub cut_speed: f32,
    pub cut_power: f32,
}

impl Default for MaterialPreset {
    fn default() -> Self {
        Self {
            name: "New Material".into(),
            thickness_mm: 3.0,
            speed: 1000.0,
            power: 800.0,
            cut_speed: 300.0,
            cut_power: 1000.0,
        }
    }
}

pub struct MaterialsState {
    pub presets: Vec<MaterialPreset>,
    pub selected: usize,
    pub editing: bool,
    pub edit_preset: MaterialPreset,
}

impl Default for MaterialsState {
    fn default() -> Self {
        let mut s = Self {
            presets: Vec::new(),
            selected: 0,
            editing: false,
            edit_preset: MaterialPreset::default(),
        };
        s.load();
        if s.presets.is_empty() {
            s.presets = vec![
                MaterialPreset { name: "Plywood 3mm".into(), thickness_mm: 3.0, speed: 800.0, power: 750.0, cut_speed: 200.0, cut_power: 1000.0 },
                MaterialPreset { name: "Plywood 6mm".into(), thickness_mm: 6.0, speed: 600.0, power: 900.0, cut_speed: 100.0, cut_power: 1000.0 },
                MaterialPreset { name: "Anodized Aluminum".into(), thickness_mm: 1.5, speed: 1500.0, power: 600.0, cut_speed: 500.0, cut_power: 800.0 },
                MaterialPreset { name: "Acrylic 3mm".into(), thickness_mm: 3.0, speed: 500.0, power: 850.0, cut_speed: 150.0, cut_power: 950.0 },
                MaterialPreset { name: "Leather".into(), thickness_mm: 2.0, speed: 1200.0, power: 500.0, cut_speed: 300.0, cut_power: 700.0 },
            ];
            s.save();
        }
        s
    }
}

impl MaterialsState {
    fn json_path() -> std::path::PathBuf {
        std::env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("materials.json")
    }

    pub fn load(&mut self) {
        if let Ok(data) = std::fs::read_to_string(Self::json_path()) {
            if let Ok(presets) = serde_json::from_str::<Vec<MaterialPreset>>(&data) {
                self.presets = presets;
            }
        }
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.presets) {
            let _ = std::fs::write(Self::json_path(), json);
        }
    }
}

pub struct MaterialApplyAction {
    pub apply_speed: Option<f32>,
    pub apply_power: Option<f32>,
    pub apply_cut_speed: Option<f32>,
    pub apply_cut_power: Option<f32>,
}

impl Default for MaterialApplyAction {
    fn default() -> Self {
        Self { apply_speed: None, apply_power: None, apply_cut_speed: None, apply_cut_power: None }
    }
}

pub fn show(ui: &mut Ui, state: &mut MaterialsState) -> MaterialApplyAction {
    let mut action = MaterialApplyAction::default();

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("📦 Material Presets").color(theme::LAVENDER).strong());
        });
        ui.add_space(4.0);

        // Dropdown for presets
        let current_name = state.presets.get(state.selected).map(|p| p.name.clone()).unwrap_or_default();
        egui::ComboBox::from_id_source("material_combo")
            .selected_text(&current_name)
            .width(180.0)
            .show_ui(ui, |ui| {
                for (i, preset) in state.presets.iter().enumerate() {
                    ui.selectable_value(&mut state.selected, i, &preset.name);
                }
            });

        ui.add_space(4.0);

        if let Some(preset) = state.presets.get(state.selected) {
            let preset = preset.clone();
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("{}mm | Speed:{} | Power:{}", preset.thickness_mm, preset.speed, preset.power)).small());
            });
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if ui.button(RichText::new("✔ Apply").color(theme::GREEN)).clicked() {
                    action.apply_speed = Some(preset.speed);
                    action.apply_power = Some(preset.power);
                    action.apply_cut_speed = Some(preset.cut_speed);
                    action.apply_cut_power = Some(preset.cut_power);
                }
                if ui.button("✎ Edit").clicked() {
                    state.editing = true;
                    state.edit_preset = preset.clone();
                }
                if ui.button("+").clicked() {
                    state.editing = true;
                    state.edit_preset = MaterialPreset::default();
                    state.selected = state.presets.len();
                    state.presets.push(MaterialPreset::default());
                }
                let is_default = state.presets.len() <= 1;
                if !is_default && ui.button(RichText::new("🗑").color(theme::RED)).clicked() {
                    state.presets.remove(state.selected);
                    state.selected = state.selected.saturating_sub(1);
                    state.save();
                }
            });
        }

        if state.editing {
            ui.separator();

            // Capture info from edit_preset using local copies to avoid simultaneous borrows
            let mut save_clicked = false;
            let mut cancel_clicked = false;

            // Temporarily pull values out to allow independent UI borrows
            let mut ep = state.edit_preset.clone();
            
            ui.horizontal(|ui| {
                ui.label("Name:"); ui.text_edit_singleline(&mut ep.name);
            });
            ui.horizontal(|ui| {
                ui.label("Thickness (mm):");
                ui.add(egui::DragValue::new(&mut ep.thickness_mm).speed(0.5).suffix(" mm"));
            });
            ui.horizontal(|ui| {
                ui.label("Engrave Speed:"); ui.add(egui::DragValue::new(&mut ep.speed).speed(10.0));
                ui.label("Power:"); ui.add(egui::DragValue::new(&mut ep.power).speed(5.0));
            });
            ui.horizontal(|ui| {
                ui.label("Cut Speed:"); ui.add(egui::DragValue::new(&mut ep.cut_speed).speed(10.0));
                ui.label("Power:"); ui.add(egui::DragValue::new(&mut ep.cut_power).speed(5.0));
            });
            ui.horizontal(|ui| {
                if ui.button(RichText::new("💾 Save").color(theme::GREEN)).clicked() {
                    save_clicked = true;
                }
                if ui.button("Cancel").clicked() {
                    cancel_clicked = true;
                }
            });

            // Write back changes
            state.edit_preset = ep;

            if save_clicked {
                let sel = state.selected;
                let new_preset = state.edit_preset.clone();
                if let Some(p) = state.presets.get_mut(sel) {
                    *p = new_preset;
                }
                state.editing = false;
                state.save();
            }
            if cancel_clicked {
                state.editing = false;
            }
        }
    });

    action
}
