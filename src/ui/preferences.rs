use crate::i18n::tr;
use crate::theme;
use crate::config::settings::{AppSettings, DisplayUnit, ColorblindMode};
use egui::{Context, RichText, ScrollArea, Ui, Window};

#[derive(Default, Clone)]
pub struct PreferencesState {
    pub is_open: bool,
    pub selected_tab: PreferencesTab,
    pub show_about: bool,
    // Appearance settings
    pub pending_theme: Option<theme::UiTheme>,
    pub pending_layout: Option<theme::UiLayout>,
    pub pending_light_mode: Option<bool>,
    // Language settings
    pub pending_language: Option<crate::i18n::Language>,
    // General settings
    pub pending_display_units: Option<DisplayUnit>,
    pub pending_beginner_mode: Option<bool>,
    pub pending_max_undo_steps: Option<usize>,
    // Accessibility settings
    pub pending_colorblind_mode: Option<ColorblindMode>,
    pub pending_high_contrast: Option<bool>,
    // Advanced settings
    pub pending_darkroom_mode: Option<bool>,
    pub pending_touch_mode: Option<bool>,
    pub pending_watch_folder_enabled: Option<bool>,
    // Machine settings
    pub pending_workspace_width: Option<f32>,
    pub pending_workspace_height: Option<f32>,
    pub pending_auto_optimization: Option<bool>,
    pub pending_auto_update: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum PreferencesTab {
    Appearance,
    Language,
    General,
    Machine,
    Accessibility,
    Advanced,
}

impl Default for PreferencesTab {
    fn default() -> Self {
        Self::Appearance
    }
}


/// Returns true if preferences were applied (so caller can sync app fields).
pub fn show(ctx: &Context, state: &mut PreferencesState, app_settings: &mut AppSettings) -> bool {
    if !state.is_open {
        return false;
    }
    let mut applied = false;

    let mut window_open = true;
    let mut should_close = false;
    let mut new_tab = state.selected_tab;
    
    Window::new(tr("Preferences"))
        .open(&mut window_open)
        .resizable(true)
        .default_width(600.0)
        .default_height(500.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                for tab in [
                    PreferencesTab::Appearance,
                    PreferencesTab::Language,
                    PreferencesTab::General,
                    PreferencesTab::Accessibility,
                    PreferencesTab::Advanced,
                    PreferencesTab::Machine,
                ] {
                    let selected = new_tab == tab;
                    let text = match tab {
                        PreferencesTab::Appearance => tr("Appearance"),
                        PreferencesTab::Language => tr("Language"),
                        PreferencesTab::General => tr("General"),
                        PreferencesTab::Accessibility => tr("Accessibility"),
                        PreferencesTab::Advanced => tr("Advanced"),
                        PreferencesTab::Machine => tr("Machine"),
                    };
                    
                    if ui.selectable_label(selected, text).clicked() {
                        new_tab = tab;
                    }
                }
            });
            
            ui.add_space(16.0);

            ScrollArea::vertical().show(ui, |ui| {
                match new_tab {
                    PreferencesTab::Appearance => show_appearance_tab(ui, state, app_settings),
                    PreferencesTab::Language => show_language_tab(ui, state, app_settings),
                    PreferencesTab::General => show_general_tab(ui, state, app_settings),
                    PreferencesTab::Accessibility => show_accessibility_tab(ui, state, app_settings),
                    PreferencesTab::Advanced => show_advanced_tab(ui, state, app_settings),
                    PreferencesTab::Machine => show_machine_tab(ui, state, app_settings),
                }
            });

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(8.0);
            
            ui.horizontal(|ui| {
                if ui.button(tr("Apply")).clicked() {
                    apply_preferences(state, app_settings);
                    applied = true;
                }
                if ui.button(tr("Cancel")).clicked() {
                    should_close = true;
                    reset_pending_state(state);
                }
                if ui.button(tr("OK")).clicked() {
                    apply_preferences(state, app_settings);
                    applied = true;
                    should_close = true;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(tr("About")).clicked() {
                        state.show_about = true;
                    }
                });
            });
        });

    if !window_open || should_close {
        state.is_open = false;
    }
    state.selected_tab = new_tab;
    applied
}

fn show_appearance_tab(ui: &mut Ui, state: &mut PreferencesState, app_settings: &AppSettings) {
    ui.heading(RichText::new(tr("Appearance Settings")).color(theme::LAVENDER).strong());
    ui.add_space(8.0);

    ui.group(|ui| {
        ui.label(tr("Theme"));
        ui.add_space(4.0);
        
        let current_theme = state.pending_theme.unwrap_or(app_settings.theme);
        
        ui.horizontal(|ui| {
            ui.label(tr("Theme:"));
            egui::ComboBox::from_id_salt("pref_theme")
                .selected_text(match current_theme {
                    theme::UiTheme::Modern => tr("Modern"),
                    theme::UiTheme::Industrial | theme::UiTheme::Pro => tr("Industrial"),
                })
                .show_ui(ui, |ui| {
                    if ui.selectable_label(current_theme == theme::UiTheme::Modern, tr("Modern")).clicked() {
                        state.pending_theme = Some(theme::UiTheme::Modern);
                    }
                    if ui.selectable_label(current_theme == theme::UiTheme::Industrial || current_theme == theme::UiTheme::Pro, tr("Industrial")).clicked() {
                        state.pending_theme = Some(theme::UiTheme::Industrial);
                    }
                });
        });
    });

    ui.add_space(8.0);

    ui.group(|ui| {
        ui.label(tr("Layout"));
        ui.add_space(4.0);
        
        let current_layout = state.pending_layout.unwrap_or(app_settings.layout);
        
        ui.horizontal(|ui| {
            ui.label(tr("Layout:"));
            egui::ComboBox::from_id_salt("pref_layout")
                .selected_text(match current_layout {
                    theme::UiLayout::Modern => tr("Modern"),
                    theme::UiLayout::Classic | theme::UiLayout::Pro => tr("Classic"),
                })
                .show_ui(ui, |ui| {
                    if ui.selectable_label(current_layout == theme::UiLayout::Modern, tr("Modern")).clicked() {
                        state.pending_layout = Some(theme::UiLayout::Modern);
                    }
                    if ui.selectable_label(current_layout == theme::UiLayout::Classic || current_layout == theme::UiLayout::Pro, tr("Classic")).clicked() {
                        state.pending_layout = Some(theme::UiLayout::Classic);
                    }
                });
        });
    });

    ui.add_space(8.0);

    ui.group(|ui| {
        ui.checkbox(
            &mut state.pending_light_mode.get_or_insert(app_settings.light_mode),
            tr("Light Mode"),
        );
    });
}

fn show_language_tab(ui: &mut Ui, state: &mut PreferencesState, app_settings: &AppSettings) {
    ui.heading(RichText::new(tr("Language Settings")).color(theme::LAVENDER).strong());
    ui.add_space(8.0);

    ui.group(|ui| {
        ui.label(tr("Interface Language"));
        ui.add_space(4.0);
        
        let current_language = state.pending_language.unwrap_or(app_settings.language);
        
        ui.horizontal(|ui| {
            ui.label(tr("Language:"));
            egui::ComboBox::from_id_salt("pref_language")
                .selected_text(match current_language {
                    crate::i18n::Language::English => "English",
                    crate::i18n::Language::French => "Français",
                    crate::i18n::Language::German => "Deutsch",
                    crate::i18n::Language::Spanish => "Español",
                    crate::i18n::Language::Chinese => "中文",
                    crate::i18n::Language::Japanese => "日本語",
                    crate::i18n::Language::Italian => "Italiano",
                    crate::i18n::Language::Arabic => "العربية",
                    crate::i18n::Language::Portuguese => "Português",
                    crate::i18n::Language::Russian => "Русский",
                    crate::i18n::Language::Turkish => "Türkçe",
                    crate::i18n::Language::Korean => "한국어",
                    crate::i18n::Language::Polish => "Polski",
                })
                .show_ui(ui, |ui| {
                    use crate::i18n::Language::*;
                    let langs = [
                        (English, "English"), (French, "Français"), (German, "Deutsch"),
                        (Spanish, "Español"), (Chinese, "中文"), (Japanese, "日本語"),
                        (Italian, "Italiano"), (Arabic, "العربية"), (Portuguese, "Português"),
                        (Russian, "Русский"), (Turkish, "Türkçe"), (Korean, "한국어"),
                        (Polish, "Polski"),
                    ];
                    for (lang, name) in langs {
                        if ui.selectable_label(current_language == lang, name).clicked() {
                            state.pending_language = Some(lang);
                        }
                    }
                });
        });
    });
}

fn show_general_tab(ui: &mut Ui, state: &mut PreferencesState, app_settings: &AppSettings) {
    ui.heading(RichText::new(tr("General Settings")).color(theme::LAVENDER).strong());
    ui.add_space(8.0);

    ui.group(|ui| {
        ui.label(tr("Display"));
        ui.add_space(4.0);
        
        let current_units = state.pending_display_units.unwrap_or(app_settings.display_unit);
        
        ui.horizontal(|ui| {
            ui.label(tr("Display Units:"));
            egui::ComboBox::from_id_salt("pref_units")
                .selected_text(match current_units {
                    DisplayUnit::Millimeters => tr("Millimeters"),
                    DisplayUnit::Inches => tr("Inches"),
                })
                .show_ui(ui, |ui| {
                    if ui.selectable_label(current_units == DisplayUnit::Millimeters, tr("Millimeters")).clicked() {
                        state.pending_display_units = Some(DisplayUnit::Millimeters);
                    }
                    if ui.selectable_label(current_units == DisplayUnit::Inches, tr("Inches")).clicked() {
                        state.pending_display_units = Some(DisplayUnit::Inches);
                    }
                });
        });
    });

    ui.add_space(8.0);

    ui.group(|ui| {
        ui.label(tr("User Interface"));
        ui.add_space(4.0);
        
        ui.checkbox(
            &mut state.pending_beginner_mode.get_or_insert(app_settings.beginner_mode),
            tr("Beginner Mode"),
        );
        
        ui.add_space(4.0);
        
        ui.horizontal(|ui| {
            ui.label(tr("Max Undo Steps:"));
            let steps = state.pending_max_undo_steps.get_or_insert(app_settings.max_undo_steps);
            if ui.add(egui::DragValue::new(&mut *steps).range(1..=100)).changed() {
                state.pending_max_undo_steps = Some(*steps);
            }
        });
    });
}

fn show_accessibility_tab(ui: &mut Ui, state: &mut PreferencesState, app_settings: &AppSettings) {
    ui.heading(RichText::new(tr("Accessibility Settings")).color(theme::LAVENDER).strong());
    ui.add_space(8.0);

    ui.group(|ui| {
        ui.label(tr("Visual"));
        ui.add_space(4.0);
        
        let current_colorblind_mode = state.pending_colorblind_mode.unwrap_or(app_settings.colorblind_mode);
        
        ui.horizontal(|ui| {
            ui.label(tr("Colorblind Mode:"));
            egui::ComboBox::from_id_salt("pref_colorblind")
                .selected_text(match current_colorblind_mode {
                    ColorblindMode::None => tr("None"),
                    ColorblindMode::Protanopia => tr("Protanopia"),
                    ColorblindMode::Deuteranopia => tr("Deuteranopia"),
                    ColorblindMode::Tritanopia => tr("Tritanopia"),
                })
                .show_ui(ui, |ui| {
                    if ui.selectable_label(current_colorblind_mode == ColorblindMode::None, tr("None")).clicked() {
                        state.pending_colorblind_mode = Some(ColorblindMode::None);
                    }
                    if ui.selectable_label(current_colorblind_mode == ColorblindMode::Protanopia, tr("Protanopia")).clicked() {
                        state.pending_colorblind_mode = Some(ColorblindMode::Protanopia);
                    }
                    if ui.selectable_label(current_colorblind_mode == ColorblindMode::Deuteranopia, tr("Deuteranopia")).clicked() {
                        state.pending_colorblind_mode = Some(ColorblindMode::Deuteranopia);
                    }
                    if ui.selectable_label(current_colorblind_mode == ColorblindMode::Tritanopia, tr("Tritanopia")).clicked() {
                        state.pending_colorblind_mode = Some(ColorblindMode::Tritanopia);
                    }
                });
        });
        
        ui.add_space(4.0);
        
        ui.checkbox(
            &mut state.pending_high_contrast.get_or_insert(app_settings.high_contrast),
            tr("High Contrast"),
        );
    });
}

fn show_advanced_tab(ui: &mut Ui, state: &mut PreferencesState, app_settings: &AppSettings) {
    ui.heading(RichText::new(tr("Advanced Settings")).color(theme::LAVENDER).strong());
    ui.add_space(8.0);

    ui.group(|ui| {
        ui.label(tr("Display Modes"));
        ui.add_space(4.0);
        
        ui.checkbox(
            &mut state.pending_darkroom_mode.get_or_insert(app_settings.darkroom_mode),
            tr("Darkroom Mode"),
        );
        
        ui.add_space(4.0);
        
        ui.checkbox(
            &mut state.pending_touch_mode.get_or_insert(app_settings.touch_mode),
            tr("Touch Mode"),
        );
    });

    ui.add_space(8.0);

    ui.group(|ui| {
        ui.label(tr("File Management"));
        ui.add_space(4.0);
        
        ui.checkbox(
            &mut state.pending_watch_folder_enabled.get_or_insert(app_settings.watch_folder_enabled),
            tr("Watch Folder for Auto-import"),
        );
    });
}

fn show_machine_tab(ui: &mut Ui, state: &mut PreferencesState, app_settings: &AppSettings) {
    ui.heading(RichText::new(tr("Machine Settings")).color(theme::LAVENDER).strong());
    ui.add_space(8.0);

    ui.group(|ui| {
        ui.label(tr("Workspace Size"));
        ui.add_space(4.0);
        
        ui.horizontal(|ui| {
            ui.label(tr("Width (mm):"));
            let width = state.pending_workspace_width.get_or_insert(app_settings.workspace_width_mm);
            if ui.add(egui::DragValue::new(&mut *width).range(50.0..=2000.0).speed(1.0).suffix(" mm")).changed() {
                state.pending_workspace_width = Some(*width);
            }
        });
        
        ui.add_space(4.0);
        
        ui.horizontal(|ui| {
            ui.label(tr("Height (mm):"));
            let height = state.pending_workspace_height.get_or_insert(app_settings.workspace_height_mm);
            if ui.add(egui::DragValue::new(&mut *height).range(50.0..=2000.0).speed(1.0).suffix(" mm")).changed() {
                state.pending_workspace_height = Some(*height);
            }
        });
        
        ui.add_space(4.0);
        ui.label(RichText::new(tr("Note: These changes will be applied to your machine profile")).size(12.0).color(theme::SUBTEXT));
    });

    ui.add_space(8.0);

    ui.group(|ui| {
        ui.label(tr("Optimization"));
        ui.add_space(4.0);
        
        ui.checkbox(
            &mut state.pending_auto_optimization.get_or_insert(app_settings.auto_optimization),
            tr("Enable Automatic Optimization"),
        );
        ui.label(RichText::new(tr("Automatically optimize tool paths and nesting for better efficiency")).size(12.0).color(theme::SUBTEXT));
    });

    ui.add_space(8.0);

    ui.group(|ui| {
        ui.label(tr("Updates"));
        ui.add_space(4.0);
        
        ui.checkbox(
            &mut state.pending_auto_update.get_or_insert(app_settings.auto_update),
            tr("Enable Automatic Updates"),
        );
        ui.label(RichText::new(tr("Automatically check and install updates in the background")).size(12.0).color(theme::SUBTEXT));
    });
}

fn apply_preferences(state: &mut PreferencesState, app_settings: &mut AppSettings) {
    if let Some(theme) = state.pending_theme.take() {
        app_settings.theme = theme;
    }
    if let Some(layout) = state.pending_layout.take() {
        app_settings.layout = layout;
    }
    if let Some(light_mode) = state.pending_light_mode.take() {
        app_settings.light_mode = light_mode;
    }
    if let Some(language) = state.pending_language.take() {
        app_settings.language = language;
    }
    if let Some(units) = state.pending_display_units.take() {
        app_settings.display_unit = units;
    }
    if let Some(beginner_mode) = state.pending_beginner_mode.take() {
        app_settings.beginner_mode = beginner_mode;
    }
    if let Some(max_undo_steps) = state.pending_max_undo_steps.take() {
        app_settings.max_undo_steps = max_undo_steps;
    }
    if let Some(colorblind_mode) = state.pending_colorblind_mode.take() {
        app_settings.colorblind_mode = colorblind_mode;
    }
    if let Some(high_contrast) = state.pending_high_contrast.take() {
        app_settings.high_contrast = high_contrast;
    }
    if let Some(darkroom_mode) = state.pending_darkroom_mode.take() {
        app_settings.darkroom_mode = darkroom_mode;
    }
    if let Some(touch_mode) = state.pending_touch_mode.take() {
        app_settings.touch_mode = touch_mode;
    }
    if let Some(watch_folder_enabled) = state.pending_watch_folder_enabled.take() {
        app_settings.watch_folder_enabled = watch_folder_enabled;
    }
    if let Some(workspace_width) = state.pending_workspace_width.take() {
        app_settings.workspace_width_mm = workspace_width;
    }
    if let Some(workspace_height) = state.pending_workspace_height.take() {
        app_settings.workspace_height_mm = workspace_height;
    }
    if let Some(auto_optimization) = state.pending_auto_optimization.take() {
        app_settings.auto_optimization = auto_optimization;
    }
    if let Some(auto_update) = state.pending_auto_update.take() {
        app_settings.auto_update = auto_update;
    }
    
    reset_pending_state(state);
}

fn reset_pending_state(state: &mut PreferencesState) {
    state.pending_theme = None;
    state.pending_layout = None;
    state.pending_light_mode = None;
    state.pending_language = None;
    state.pending_display_units = None;
    state.pending_beginner_mode = None;
    state.pending_max_undo_steps = None;
    state.pending_colorblind_mode = None;
    state.pending_high_contrast = None;
    state.pending_darkroom_mode = None;
    state.pending_touch_mode = None;
    state.pending_watch_folder_enabled = None;
    state.pending_workspace_width = None;
    state.pending_workspace_height = None;
    state.pending_auto_optimization = None;
    state.pending_auto_update = None;
}
