use crate::i18n::tr;
use crate::theme;
use crate::config::settings::{AppSettings, DisplayUnit, ColorblindMode};
use egui::{Context, RichText, ScrollArea, Ui, Window};

#[derive(Default, Clone)]
pub struct PreferencesState {
    pub is_open: bool,
    pub selected_tab: PreferencesTab,
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
        let mut theme_index = match current_theme {
            theme::UiTheme::Modern => 0,
            theme::UiTheme::Industrial => 1,
            theme::UiTheme::Pro => 1, // Deprecated: map to Industrial
        };
        
        ui.horizontal(|ui| {
            ui.label(tr("Theme:"));
            if egui::ComboBox::from_label("")
                .selected_text(match current_theme {
                    theme::UiTheme::Modern => tr("Modern"),
                    theme::UiTheme::Industrial => tr("Industrial"),
                    theme::UiTheme::Pro => tr("Industrial"), // Deprecated: map to Industrial
                })
                .show_ui(ui, |ui| {
                    if ui.selectable_label(false, tr("Modern")).clicked() {
                        theme_index = 0;
                    }
                    if ui.selectable_label(false, tr("Industrial")).clicked() {
                        theme_index = 1;
                    }
                })
                .response
                .changed()
            {
                state.pending_theme = match theme_index {
                    0 => Some(theme::UiTheme::Modern),
                    1 => Some(theme::UiTheme::Industrial),
                    _ => None,
                };
            }
        });
    });

    ui.add_space(8.0);

    ui.group(|ui| {
        ui.label(tr("Layout"));
        ui.add_space(4.0);
        
        let current_layout = state.pending_layout.unwrap_or(app_settings.layout);
        let mut layout_index = match current_layout {
            theme::UiLayout::Modern => 0,
            theme::UiLayout::Classic => 1,
            theme::UiLayout::Pro => 1, // Deprecated: map to Classic
        };
        
        ui.horizontal(|ui| {
            ui.label(tr("Layout:"));
            if egui::ComboBox::from_label("")
                .selected_text(match current_layout {
                    theme::UiLayout::Modern => tr("Modern"),
                    theme::UiLayout::Classic => tr("Classic"),
                    theme::UiLayout::Pro => tr("Classic"), // Deprecated: map to Classic
                })
                .show_ui(ui, |ui| {
                    if ui.selectable_label(false, tr("Modern")).clicked() {
                        layout_index = 0;
                    }
                    if ui.selectable_label(false, tr("Classic")).clicked() {
                        layout_index = 1;
                    }
                })
                .response
                .changed()
            {
                state.pending_layout = match layout_index {
                    0 => Some(theme::UiLayout::Modern),
                    1 => Some(theme::UiLayout::Classic),
                    _ => None,
                };
            }
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
        let mut language_index = match current_language {
            crate::i18n::Language::English => 0,
            crate::i18n::Language::French => 1,
            crate::i18n::Language::German => 2,
            crate::i18n::Language::Spanish => 3,
            crate::i18n::Language::Chinese => 4,
            crate::i18n::Language::Japanese => 5,
            crate::i18n::Language::Italian => 6,
            crate::i18n::Language::Arabic => 7,
            crate::i18n::Language::Portuguese => 8,
            crate::i18n::Language::Russian => 9,
            crate::i18n::Language::Turkish => 10,
            crate::i18n::Language::Korean => 11,
            crate::i18n::Language::Polish => 12,
        };
        
        ui.horizontal(|ui| {
            ui.label(tr("Language:"));
            if egui::ComboBox::from_label("")
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
                    if ui.selectable_label(false, "English").clicked() {
                        language_index = 0;
                    }
                    if ui.selectable_label(false, "Français").clicked() {
                        language_index = 1;
                    }
                    if ui.selectable_label(false, "Deutsch").clicked() {
                        language_index = 2;
                    }
                    if ui.selectable_label(false, "Español").clicked() {
                        language_index = 3;
                    }
                    if ui.selectable_label(false, "中文").clicked() {
                        language_index = 4;
                    }
                    if ui.selectable_label(false, "日本語").clicked() {
                        language_index = 5;
                    }
                    if ui.selectable_label(false, "Italiano").clicked() {
                        language_index = 6;
                    }
                    if ui.selectable_label(false, "العربية").clicked() {
                        language_index = 7;
                    }
                    if ui.selectable_label(false, "Português").clicked() {
                        language_index = 8;
                    }
                    if ui.selectable_label(false, "Русский").clicked() {
                        language_index = 9;
                    }
                    if ui.selectable_label(false, "Türkçe").clicked() {
                        language_index = 10;
                    }
                    if ui.selectable_label(false, "한국어").clicked() {
                        language_index = 11;
                    }
                    if ui.selectable_label(false, "Polski").clicked() {
                        language_index = 12;
                    }
                })
                .response
                .changed()
            {
                state.pending_language = match language_index {
                    0 => Some(crate::i18n::Language::English),
                    1 => Some(crate::i18n::Language::French),
                    2 => Some(crate::i18n::Language::German),
                    3 => Some(crate::i18n::Language::Spanish),
                    4 => Some(crate::i18n::Language::Chinese),
                    5 => Some(crate::i18n::Language::Japanese),
                    6 => Some(crate::i18n::Language::Italian),
                    7 => Some(crate::i18n::Language::Arabic),
                    8 => Some(crate::i18n::Language::Portuguese),
                    9 => Some(crate::i18n::Language::Russian),
                    10 => Some(crate::i18n::Language::Turkish),
                    11 => Some(crate::i18n::Language::Korean),
                    12 => Some(crate::i18n::Language::Polish),
                    _ => None,
                };
            }
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
            if egui::ComboBox::from_label("")
                .selected_text(match current_units {
                    DisplayUnit::Millimeters => tr("Millimeters"),
                    DisplayUnit::Inches => tr("Inches"),
                })
                .show_ui(ui, |ui| {
                    if ui.selectable_label(false, tr("Millimeters")).clicked() {
                        state.pending_display_units = Some(DisplayUnit::Millimeters);
                    }
                    if ui.selectable_label(false, tr("Inches")).clicked() {
                        state.pending_display_units = Some(DisplayUnit::Inches);
                    }
                })
                .response
                .changed()
            {
                // Handled in the combo box
            }
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
        let mut mode_index = match current_colorblind_mode {
            ColorblindMode::None => 0,
            ColorblindMode::Protanopia => 1,
            ColorblindMode::Deuteranopia => 2,
            ColorblindMode::Tritanopia => 3,
        };
        
        ui.horizontal(|ui| {
            ui.label(tr("Colorblind Mode:"));
            if egui::ComboBox::from_label("")
                .selected_text(match current_colorblind_mode {
                    ColorblindMode::None => tr("None"),
                    ColorblindMode::Protanopia => tr("Protanopia"),
                    ColorblindMode::Deuteranopia => tr("Deuteranopia"),
                    ColorblindMode::Tritanopia => tr("Tritanopia"),
                })
                .show_ui(ui, |ui| {
                    if ui.selectable_label(false, tr("None")).clicked() {
                        mode_index = 0;
                    }
                    if ui.selectable_label(false, tr("Protanopia")).clicked() {
                        mode_index = 1;
                    }
                    if ui.selectable_label(false, tr("Deuteranopia")).clicked() {
                        mode_index = 2;
                    }
                    if ui.selectable_label(false, tr("Tritanopia")).clicked() {
                        mode_index = 3;
                    }
                })
                .response
                .changed()
            {
                state.pending_colorblind_mode = match mode_index {
                    0 => Some(ColorblindMode::None),
                    1 => Some(ColorblindMode::Protanopia),
                    2 => Some(ColorblindMode::Deuteranopia),
                    3 => Some(ColorblindMode::Tritanopia),
                    _ => None,
                };
            }
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
