use crate::app_types::WizardState;
use crate::config::machine_profile::MachineProfile;
use crate::controller::ControllerKind;
use crate::laser::driver::{
    LaserDriverProfile, available_driver_profiles, effective_driver_profile,
};
/// Startup wizard UI (F43) — extracted from app.rs for maintainability
use crate::i18n::{tr, Language};

pub struct WizardContext<'a> {
    pub wizard: &'a mut WizardState,
    pub language: &'a mut Language,
    pub machine_profile: &'a mut MachineProfile,
}

pub struct WizardResult {
    pub finished: bool,
    pub controller_changed: bool,
}

pub fn show_wizard(ctx: &egui::Context, wctx: &mut WizardContext) -> WizardResult {
    let mut result = WizardResult {
        finished: false,
        controller_changed: false,
    };

    if !wctx.wizard.show {
        return result;
    }

    egui::Window::new(format!("🚀 {}", tr("Startup Wizard")))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            match wctx.wizard.step {
                0 => {
                    ui.label(
                        egui::RichText::new(format!("Step 1/3 — {}", tr("Language")))
                            .size(16.0)
                            .strong(),
                    );
                    ui.add_space(8.0);
                    for lang in &[
                        Language::English,
                        Language::French,
                        Language::German,
                        Language::Spanish,
                        Language::Italian,
                        Language::Portuguese,
                        Language::Japanese,
                        Language::Arabic,
                        Language::Chinese,
                        Language::Russian,
                        Language::Turkish,
                        Language::Korean,
                        Language::Polish,
                    ] {
                        if ui
                            .selectable_label(*wctx.language == *lang, lang.name())
                            .clicked()
                        {
                            *wctx.language = *lang;
                            crate::i18n::set_language(*wctx.language);
                        }
                    }
                    ui.add_space(8.0);
                    if ui.button(format!("{} →", tr("Next"))).clicked() {
                        wctx.wizard.step = 1;
                    }
                }
                1 => {
                    ui.label(
                        egui::RichText::new(format!("Step 2/3 — {}", tr("Settings")))
                            .size(16.0)
                            .strong(),
                    );
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", tr("Name")));
                        ui.text_edit_singleline(&mut wctx.machine_profile.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("{} (mm):", tr("Width")));
                        ui.add(
                            egui::DragValue::new(&mut wctx.machine_profile.workspace_x_mm)
                                .speed(10.0),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("{} (mm):", tr("Height")));
                        ui.add(
                            egui::DragValue::new(&mut wctx.machine_profile.workspace_y_mm)
                                .speed(10.0),
                        );
                    });
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button(format!("← {}", tr("Back"))).clicked() {
                            wctx.wizard.step = 0;
                        }
                        if ui.button(format!("{} →", tr("Next"))).clicked() {
                            wctx.wizard.step = 2;
                        }
                    });
                }
                _ => {
                    ui.label(
                        egui::RichText::new(format!("Step 3/3 — {}", tr("Controller")))
                            .size(16.0)
                            .strong(),
                    );
                    ui.add_space(8.0);
                    let prev_kind = wctx.machine_profile.controller_kind;
                    ui.selectable_value(
                        &mut wctx.machine_profile.controller_kind,
                        ControllerKind::Grbl,
                        "GRBL",
                    );
                    ui.selectable_value(
                        &mut wctx.machine_profile.controller_kind,
                        ControllerKind::Marlin,
                        "Marlin",
                    );
                    ui.selectable_value(
                        &mut wctx.machine_profile.controller_kind,
                        ControllerKind::Ruida,
                        "Ruida (beta)",
                    );
                    ui.selectable_value(
                        &mut wctx.machine_profile.controller_kind,
                        ControllerKind::Trocen,
                        "Trocen (beta)",
                    );
                    if wctx.machine_profile.controller_kind != prev_kind {
                        wctx.machine_profile.laser_driver_profile = LaserDriverProfile::Auto;
                        result.controller_changed = true;
                    }
                    ui.add_space(8.0);
                    ui.label(tr("Laser Driver"));
                    egui::ComboBox::from_id_salt("wizard_laser_driver_profile")
                        .selected_text(wctx.machine_profile.laser_driver_profile.label())
                        .show_ui(ui, |ui| {
                            for profile in
                                available_driver_profiles(wctx.machine_profile.controller_kind)
                            {
                                ui.selectable_value(
                                    &mut wctx.machine_profile.laser_driver_profile,
                                    profile,
                                    profile.label(),
                                );
                            }
                        });
                    ui.label(
                        egui::RichText::new(
                            wctx.machine_profile.laser_driver_profile.description(),
                        )
                        .small(),
                    );
                    if wctx.machine_profile.laser_driver_profile == LaserDriverProfile::Auto {
                        let resolved_profile = effective_driver_profile(
                            wctx.machine_profile.controller_kind,
                            wctx.machine_profile.laser_driver_profile,
                        );
                        ui.label(
                            egui::RichText::new(format!(
                                "Resolved now: {}",
                                resolved_profile.label()
                            ))
                            .small(),
                        );
                    }
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button(format!("← {}", tr("Back"))).clicked() {
                            wctx.wizard.step = 1;
                        }
                        if ui.button(format!("✅ {}", tr("Finish"))).clicked() {
                            result.finished = true;
                        }
                    });
                }
            }
            ui.add_space(4.0);
            if ui.small_button(tr("Skip wizard")).clicked() {
                result.finished = true;
            }
        });

    if result.finished {
        wctx.wizard.show = false;
    }

    result
}
