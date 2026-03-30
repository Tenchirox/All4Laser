use crate::i18n::tr;
use crate::theme;
use crate::ui::layers_new::CutMode;
use egui::{RichText, Ui};
use serde::{Deserialize, Serialize};

fn default_machine_profile() -> String {
    String::new()
}
fn default_recommended_passes() -> u32 {
    1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialOperation {
    Engrave,
    Cut,
    Hybrid,
}

impl Default for MaterialOperation {
    fn default() -> Self {
        Self::Cut
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaterialPreset {
    pub name: String,
    pub thickness_mm: f32,
    pub speed: f32,
    pub power: f32,
    pub cut_speed: f32,
    pub cut_power: f32,
    #[serde(default = "default_machine_profile")]
    pub machine_profile: String,
    #[serde(default)]
    pub operation: MaterialOperation,
    #[serde(default = "default_recommended_passes")]
    pub recommended_passes: u32,
    #[serde(default)]
    pub is_favorite: bool,
}

impl Default for MaterialPreset {
    fn default() -> Self {
        Self {
            name: tr("New Material").into(),
            thickness_mm: 3.0,
            speed: 1000.0,
            power: 800.0,
            cut_speed: 300.0,
            cut_power: 1000.0,
            machine_profile: String::new(),
            operation: MaterialOperation::Cut,
            recommended_passes: 1,
            is_favorite: false,
        }
    }
}

impl MaterialPreset {
    pub fn as_layer_update(&self) -> LayerMaterialUpdate {
        let (speed, power, mode) = match self.operation {
            MaterialOperation::Engrave => (self.speed, self.power, CutMode::Fill),
            MaterialOperation::Cut => (self.cut_speed, self.cut_power, CutMode::Line),
            MaterialOperation::Hybrid => (self.cut_speed, self.cut_power, CutMode::FillAndLine),
        };

        LayerMaterialUpdate {
            preset_name: self.name.clone(),
            speed,
            power,
            passes: self.recommended_passes.max(1),
            mode,
        }
    }
}

fn default_presets() -> Vec<MaterialPreset> {
    vec![
        MaterialPreset {
            name: "Plywood 3mm".into(),
            thickness_mm: 3.0,
            speed: 800.0,
            power: 75.0,
            cut_speed: 200.0,
            cut_power: 100.0,
            operation: MaterialOperation::Cut,
            recommended_passes: 1,
            ..MaterialPreset::default()
        },
        MaterialPreset {
            name: "Plywood 6mm".into(),
            thickness_mm: 6.0,
            speed: 600.0,
            power: 90.0,
            cut_speed: 100.0,
            cut_power: 100.0,
            operation: MaterialOperation::Cut,
            recommended_passes: 2,
            ..MaterialPreset::default()
        },
        MaterialPreset {
            name: "Anodized Aluminum".into(),
            thickness_mm: 1.5,
            speed: 1500.0,
            power: 60.0,
            cut_speed: 500.0,
            cut_power: 80.0,
            operation: MaterialOperation::Engrave,
            recommended_passes: 1,
            ..MaterialPreset::default()
        },
        MaterialPreset {
            name: "Acrylic 3mm".into(),
            thickness_mm: 3.0,
            speed: 500.0,
            power: 85.0,
            cut_speed: 150.0,
            cut_power: 95.0,
            operation: MaterialOperation::Cut,
            recommended_passes: 1,
            ..MaterialPreset::default()
        },
        MaterialPreset {
            name: "Leather".into(),
            thickness_mm: 2.0,
            speed: 1200.0,
            power: 50.0,
            cut_speed: 300.0,
            cut_power: 70.0,
            operation: MaterialOperation::Engrave,
            recommended_passes: 1,
            ..MaterialPreset::default()
        },
    ]
}

#[derive(Clone, Debug)]
pub struct ActiveLayerSummary {
    pub name: String,
    pub speed: f32,
    pub power: f32,
    pub passes: u32,
    pub mode: CutMode,
}

#[derive(Clone, Debug, Default)]
pub struct MaterialsUiContext {
    pub machine_profile_name: Option<String>,
    pub active_layer: Option<ActiveLayerSummary>,
}

#[derive(Clone, Debug)]
pub struct LayerMaterialUpdate {
    pub preset_name: String,
    pub speed: f32,
    pub power: f32,
    pub passes: u32,
    pub mode: CutMode,
}

fn operation_for_mode(mode: CutMode) -> MaterialOperation {
    match mode {
        CutMode::Line => MaterialOperation::Cut,
        CutMode::Fill => MaterialOperation::Engrave,
        CutMode::FillAndLine | CutMode::Offset => MaterialOperation::Hybrid,
    }
}

fn recommendation_score(preset: &MaterialPreset, context: &MaterialsUiContext) -> f32 {
    let mut score = 0.0;

    if let Some(machine_name) = context.machine_profile_name.as_deref() {
        if preset.machine_profile.trim().is_empty() {
            score += 12.0;
        } else if preset.machine_profile.eq_ignore_ascii_case(machine_name) {
            score += 42.0;
        } else {
            score -= 16.0;
        }
    }

    if let Some(layer) = &context.active_layer {
        score += 20.0 - ((preset.cut_speed - layer.speed).abs() / 200.0);
        score += 20.0 - ((preset.cut_power - layer.power).abs() / 40.0);
        score += 8.0 - ((preset.recommended_passes as f32 - layer.passes as f32).abs() * 3.0);

        if preset.operation == operation_for_mode(layer.mode) {
            score += 18.0;
        }
    }

    score
}

pub fn recommended_preset_index(
    state: &MaterialsState,
    context: &MaterialsUiContext,
) -> Option<usize> {
    state
        .presets
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| {
            recommendation_score(a, context)
                .partial_cmp(&recommendation_score(b, context))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(idx, _)| idx)
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
            s.presets = default_presets();
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
        let presets_clone = self.presets.clone();
        let path = Self::json_path();
        std::thread::spawn(move || {
            if let Ok(json) = serde_json::to_string_pretty(&presets_clone) {
                let _ = std::fs::write(path, json);
            }
        });
    }

    pub fn selected_preset_name(&self) -> Option<&str> {
        self.presets.get(self.selected).map(|p| p.name.as_str())
    }

    pub fn select_preset_by_name(&mut self, name: &str) -> bool {
        if let Some(idx) = self
            .presets
            .iter()
            .position(|p| p.name.eq_ignore_ascii_case(name))
        {
            self.selected = idx;
            return true;
        }
        false
    }
}

pub struct MaterialApplyAction {
    pub apply_speed: Option<f32>,
    pub apply_power: Option<f32>,
    pub apply_cut_speed: Option<f32>,
    pub apply_cut_power: Option<f32>,
    pub apply_to_active_layer: Option<LayerMaterialUpdate>,
}

impl Default for MaterialApplyAction {
    fn default() -> Self {
        Self {
            apply_speed: None,
            apply_power: None,
            apply_cut_speed: None,
            apply_cut_power: None,
            apply_to_active_layer: None,
        }
    }
}

pub fn show(ui: &mut Ui, state: &mut MaterialsState) -> MaterialApplyAction {
    show_with_context(ui, state, &MaterialsUiContext::default())
}

pub fn show_with_context(
    ui: &mut Ui,
    state: &mut MaterialsState,
    context: &MaterialsUiContext,
) -> MaterialApplyAction {
    let mut action = MaterialApplyAction::default();

    if state.selected >= state.presets.len() && !state.presets.is_empty() {
        state.selected = state.presets.len() - 1;
    }

    let recommendation_idx = recommended_preset_index(state, context);

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!("📦 {}", tr("Material Presets")))
                    .color(theme::LAVENDER)
                    .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(format!("📤 {}", tr("Export"))).clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("JSON", &["json"])
                        .save_file()
                    {
                        if let Ok(json) = serde_json::to_string_pretty(&state.presets) {
                            let _ = std::fs::write(path, json);
                        }
                    }
                }
                if ui.button(format!("📥 {}", tr("Import"))).clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("JSON", &["json"])
                        .pick_file()
                    {
                        if let Ok(data) = std::fs::read_to_string(path) {
                            if let Ok(new_presets) =
                                serde_json::from_str::<Vec<MaterialPreset>>(&data)
                            {
                                state.presets.extend(new_presets);
                                state.save();
                            }
                        }
                    }
                }
            });
        });
        ui.add_space(4.0);

        if let Some(layer) = &context.active_layer {
            ui.label(
                RichText::new(format!(
                    "{}: {} | {} {} | S{} | {} {}",
                    tr("Layer"),
                    layer.name,
                    layer.speed.round(),
                    tr("mm/min"),
                    layer.power.round(),
                    layer.passes,
                    tr("pass(es)")
                ))
                .small(),
            );
        }

        if let Some(idx) = recommendation_idx {
            if let Some(preset) = state.presets.get(idx) {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("⭐ {}: {}", tr("Recommended"), preset.name))
                            .color(theme::GREEN)
                            .strong(),
                    );
                    if context.active_layer.is_some() && ui.button(tr("Apply Recommended")).clicked() {
                        action.apply_speed = Some(preset.speed);
                        action.apply_power = Some(preset.power);
                        action.apply_cut_speed = Some(preset.cut_speed);
                        action.apply_cut_power = Some(preset.cut_power);
                        action.apply_to_active_layer = Some(preset.as_layer_update());
                        state.selected = idx;
                    }
                });
                ui.add_space(2.0);
            }
        }

        // Dropdown for presets
        let current_name = state
            .presets
            .get(state.selected)
            .map(|p| p.name.clone())
            .unwrap_or_default();
        egui::ComboBox::from_id_salt("material_combo")
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
                let machine_scope = if preset.machine_profile.trim().is_empty() {
                    tr("All machines").to_string()
                } else {
                    format!("{}: {}", tr("Machine"), preset.machine_profile)
                };
                ui.label(
                    RichText::new(format!(
                        "{}mm | {} {} / {}% | {} {} / {}% | {} {} | {}",
                        preset.thickness_mm,
                        tr("Engrave"),
                        preset.speed,
                        preset.power,
                        tr("Cut"),
                        preset.cut_speed,
                        preset.cut_power,
                        preset.recommended_passes,
                        tr("Passes"),
                        machine_scope
                    ))
                    .small(),
                );
            });
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui
                    .button(RichText::new(format!("✔ {}", tr("Apply"))).color(theme::GREEN))
                    .clicked()
                {
                    action.apply_speed = Some(preset.speed);
                    action.apply_power = Some(preset.power);
                    action.apply_cut_speed = Some(preset.cut_speed);
                    action.apply_cut_power = Some(preset.cut_power);
                }
                if context.active_layer.is_some() && ui.button(format!("🎯 {}", tr("Apply to Active Layer"))).clicked()
                {
                    action.apply_to_active_layer = Some(preset.as_layer_update());
                    action.apply_speed = Some(preset.speed);
                    action.apply_power = Some(preset.power);
                    action.apply_cut_speed = Some(preset.cut_speed);
                    action.apply_cut_power = Some(preset.cut_power);
                }
                if ui.button(format!("✎ {}", tr("Edit"))).clicked() {
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
                ui.label(format!("{}:", tr("Name")));
                ui.text_edit_singleline(&mut ep.name);
            });
            ui.horizontal(|ui| {
                ui.label(format!("{} (mm):", tr("Thickness")));
                ui.add(
                    egui::DragValue::new(&mut ep.thickness_mm)
                        .speed(0.5)
                        .suffix(" mm"),
                );
            });
            ui.horizontal(|ui| {
                ui.label(format!("{} {}:", tr("Engrave"), tr("Speed")));
                ui.add(egui::DragValue::new(&mut ep.speed).speed(10.0));
                ui.label(format!("{}:", tr("Power")));
                ui.add(egui::DragValue::new(&mut ep.power).speed(1.0).range(0.0..=100.0).suffix("%"));
            });
            ui.horizontal(|ui| {
                ui.label(format!("{} {}:", tr("Cut"), tr("Speed")));
                ui.add(egui::DragValue::new(&mut ep.cut_speed).speed(10.0));
                ui.label(format!("{}:", tr("Power")));
                ui.add(egui::DragValue::new(&mut ep.cut_power).speed(1.0).range(0.0..=100.0).suffix("%"));
            });
            ui.horizontal(|ui| {
                ui.label(format!("{}:", tr("Recommended Passes")));
                ui.add(egui::DragValue::new(&mut ep.recommended_passes).range(1..=20));
            });
            ui.horizontal(|ui| {
                ui.label(format!("{}:", tr("Machine Profile")));
                ui.text_edit_singleline(&mut ep.machine_profile);
            });
            ui.horizontal(|ui| {
                ui.label(format!("{}:", tr("Operation")));
                egui::ComboBox::from_id_salt("material_operation_combo")
                    .selected_text(match ep.operation {
                        MaterialOperation::Engrave => tr("Engrave"),
                        MaterialOperation::Cut => tr("Cut"),
                        MaterialOperation::Hybrid => tr("Hybrid"),
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut ep.operation,
                            MaterialOperation::Engrave,
                            tr("Engrave"),
                        );
                        ui.selectable_value(&mut ep.operation, MaterialOperation::Cut, tr("Cut"));
                        ui.selectable_value(&mut ep.operation, MaterialOperation::Hybrid, tr("Hybrid"));
                    });
            });
            ui.horizontal(|ui| {
                if ui
                    .button(RichText::new(format!("💾 {}", tr("Save"))).color(theme::GREEN))
                    .clicked()
                {
                    save_clicked = true;
                }
                if ui.button(tr("Cancel")).clicked() {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_state() -> MaterialsState {
        MaterialsState {
            presets: vec![
                MaterialPreset {
                    name: "Generic".into(),
                    machine_profile: String::new(),
                    operation: MaterialOperation::Cut,
                    cut_speed: 300.0,
                    cut_power: 800.0,
                    recommended_passes: 1,
                    ..MaterialPreset::default()
                },
                MaterialPreset {
                    name: "Machine A Cut".into(),
                    machine_profile: "Machine A".into(),
                    operation: MaterialOperation::Cut,
                    cut_speed: 280.0,
                    cut_power: 790.0,
                    recommended_passes: 1,
                    ..MaterialPreset::default()
                },
                MaterialPreset {
                    name: "Machine B Engrave".into(),
                    machine_profile: "Machine B".into(),
                    operation: MaterialOperation::Engrave,
                    speed: 1800.0,
                    power: 220.0,
                    recommended_passes: 1,
                    ..MaterialPreset::default()
                },
            ],
            selected: 0,
            editing: false,
            edit_preset: MaterialPreset::default(),
        }
    }

    #[test]
    fn recommendation_prefers_machine_and_mode_match() {
        let state = sample_state();
        let ctx = MaterialsUiContext {
            machine_profile_name: Some("Machine A".into()),
            active_layer: Some(ActiveLayerSummary {
                name: "C01".into(),
                speed: 300.0,
                power: 800.0,
                passes: 1,
                mode: CutMode::Line,
            }),
        };

        let idx = recommended_preset_index(&state, &ctx).expect("a preset should be recommended");
        assert_eq!(state.presets[idx].name, "Machine A Cut");
    }

    #[test]
    fn as_layer_update_maps_operation() {
        let preset = MaterialPreset {
            name: "Hybrid mat".into(),
            operation: MaterialOperation::Hybrid,
            cut_speed: 220.0,
            cut_power: 700.0,
            recommended_passes: 3,
            ..MaterialPreset::default()
        };

        let update = preset.as_layer_update();
        assert_eq!(update.mode, CutMode::FillAndLine);
        assert_eq!(update.speed, 220.0);
        assert_eq!(update.power, 700.0);
        assert_eq!(update.passes, 3);
    }
}
