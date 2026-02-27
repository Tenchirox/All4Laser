use egui::{Ui, RichText};
use crate::config::recent_files::RecentFiles;
use crate::theme;

use crate::i18n::{self, tr, Language};

pub struct ToolbarAction {
    pub connect_toggle: bool,
    pub open_file: bool,
    pub open_recent: Option<String>,
    pub save_file: bool,
    pub save_project: bool,
    pub open_project: bool,
    pub run_program: bool,
    pub frame_bbox: bool,
    pub dry_run: bool,
    pub abort_program: bool,
    pub hold: bool,
    pub resume: bool,
    pub home: bool,
    pub unlock: bool,
    pub set_zero: bool,
    pub reset: bool,
    pub set_theme: Option<theme::UiTheme>,
    pub set_layout: Option<theme::UiLayout>,
    pub set_language: Option<Language>,
    pub toggle_light_mode: bool,
    pub open_settings: bool,
    pub open_power_speed_test: bool,
    pub open_gcode_editor: bool,
    pub open_shortcuts: bool,
    pub open_tiling: bool,
    pub open_test_fire: bool,
}

impl Default for ToolbarAction {
    fn default() -> Self {
        Self {
            connect_toggle: false,
            open_file: false,
            open_recent: None,
            save_file: false,
            save_project: false,
            open_project: false,
            run_program: false,
            frame_bbox: false,
            dry_run: false,
            abort_program: false,
            hold: false,
            resume: false,
            home: false,
            unlock: false,
            set_zero: false,
            reset: false,
            set_theme: None,
            set_layout: None,
            set_language: None,
            toggle_light_mode: false,
            open_settings: false,
            open_power_speed_test: false,
            open_gcode_editor: false,
            open_shortcuts: false,
            open_tiling: false,
            open_test_fire: false,
        }
    }
}

pub fn show(
    ui: &mut Ui,
    connected: bool,
    running: bool,
    light_mode: bool,
    framing_active: bool,
    recent: &RecentFiles,
    has_file: bool,
) -> ToolbarAction {
    let mut action = ToolbarAction::default();

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;

        // Connect / Disconnect
        let conn_label = if connected { tr("Disconnect") } else { tr("Connect") };
        let conn_text = if connected { format!("⏏ {}", conn_label) } else { format!("🔌 {}", conn_label) };
        let conn_color = if connected { theme::RED } else { theme::GREEN };
        if ui.button(RichText::new(conn_text).color(conn_color).size(13.0)).clicked() {
            action.connect_toggle = true;
        }

        ui.separator();

        // File group
        if ui.button(RichText::new(format!("📂 {}", tr("Open"))).size(13.0)).clicked() {
            action.open_file = true;
        }
        // Recent files dropdown
        egui::menu::menu_button(ui, "▾", |ui| {
            ui.set_min_width(280.0);
            if recent.paths.is_empty() {
                ui.label("No recent files");
            } else {
                for path in &recent.paths {
                    let display = std::path::Path::new(path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(path);
                    if ui.selectable_label(false, display).clicked() {
                        action.open_recent = Some(path.clone());
                        ui.close_menu();
                    }
                }
            }
        });

        if ui.button(RichText::new(format!("💾 {}", tr("Save"))).size(13.0)).clicked() {
            action.save_file = true;
        }

        // Project menu
        egui::menu::menu_button(ui, "📁 Project", |ui| {
            if ui.button("📂 Open Project (.a4l)").clicked() {
                action.open_project = true;
                ui.close_menu();
            }
            if ui.add_enabled(has_file, egui::Button::new("💾 Save Project (.a4l)")).clicked() {
                action.save_project = true;
                ui.close_menu();
            }
        });

        ui.separator();

        // Run / Abort toggle
        if running {
            if ui.button(RichText::new(format!("⛔ {}", tr("Stop"))).color(theme::RED).size(13.0)).clicked() {
                action.abort_program = true;
            }
        } else {
            let run_btn = ui.add_enabled(
                connected,
                egui::Button::new(RichText::new(format!("▶ {}", tr("Run"))).color(theme::GREEN).size(13.0)),
            );
            if run_btn.clicked() { action.run_program = true; }

            let frame_lbl = if framing_active { format!("⏹ {}", tr("Stop")) } else { "⛶ Frame".to_string() };
            let frame_col = if framing_active { theme::RED } else { theme::SUBTEXT };
            if ui.add_enabled(connected, egui::Button::new(RichText::new(frame_lbl).color(frame_col).size(13.0))).clicked() {
                action.frame_bbox = true;
            }

            // Dry Run
             if ui.add_enabled(connected && has_file, egui::Button::new(RichText::new("🛡 Dry Run").color(theme::BLUE).size(13.0))).on_hover_text("Run job with Laser OFF (M5)").clicked() {
                action.dry_run = true;
            }
        }

        if ui.add_enabled(connected, egui::Button::new(RichText::new(format!("⏸ {}", tr("Hold"))).size(13.0))).clicked() { action.hold = true; }
        if ui.add_enabled(connected, egui::Button::new(RichText::new(format!("▶ {}", tr("Resume"))).size(13.0))).clicked() { action.resume = true; }

        ui.separator();

        if ui.add_enabled(connected, egui::Button::new(RichText::new(format!("🏠 {}", tr("Home"))).size(13.0))).clicked() { action.home = true; }
        if ui.add_enabled(connected, egui::Button::new(RichText::new(format!("🔓 {}", tr("Unlock"))).size(13.0))).clicked() { action.unlock = true; }
        if ui.add_enabled(connected, egui::Button::new(RichText::new("⊙ Zero").size(13.0))).clicked() { action.set_zero = true; }

        ui.separator();

        if ui.add_enabled(connected, egui::Button::new(RichText::new(format!("↻ {}", tr("Reset"))).color(theme::PEACH).size(13.0))).clicked() { action.reset = true; }

        // View menu
        egui::menu::menu_button(ui, "👁 View", |ui| {
            ui.label(RichText::new("Theme:").strong());
            if ui.selectable_label(false, "Catppuccin (Modern)").clicked() {
                action.set_theme = Some(theme::UiTheme::Modern);
                ui.close_menu();
            }
            if ui.selectable_label(false, "LightBurn-ish (Industrial)").clicked() {
                action.set_theme = Some(theme::UiTheme::Industrial);
                ui.close_menu();
            }
            
            ui.separator();
            ui.label(RichText::new("Layout:").strong());
            if ui.selectable_label(false, "Modern Layout").clicked() {
                action.set_layout = Some(theme::UiLayout::Modern);
                ui.close_menu();
            }
            if ui.selectable_label(false, "Classic (LightBurn Style)").clicked() {
                action.set_layout = Some(theme::UiLayout::Classic);
                ui.close_menu();
            }

            ui.separator();
            ui.label(RichText::new("Language:").strong());
            let current_lang = i18n::get_language();
            let langs = [
                Language::English, Language::French, Language::Japanese,
                Language::German, Language::Italian, Language::Arabic,
                Language::Spanish, Language::Portuguese
            ];
            for lang in langs {
                if ui.selectable_label(current_lang == lang, lang.name()).clicked() {
                    action.set_language = Some(lang);
                    ui.close_menu();
                }
            }
        });

        // Tools menu
        egui::menu::menu_button(ui, "🔧 Tools", |ui| {
            if ui.button("⊞ Power/Speed Test").clicked() {
                action.open_power_speed_test = true;
                ui.close_menu();
            }
            if ui.button("🔥 Test Fire").clicked() {
                action.open_test_fire = true;
                ui.close_menu();
            }
            ui.separator();
            if ui.add_enabled(has_file, egui::Button::new("📝 GCode Editor")).clicked() {
                action.open_gcode_editor = true;
                ui.close_menu();
            }
            if ui.add_enabled(has_file, egui::Button::new("⊟ Tiling")).clicked() {
                action.open_tiling = true;
                ui.close_menu();
            }
            if ui.button("⌨ Shortcuts").clicked() {
                action.open_shortcuts = true;
                ui.close_menu();
            }
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let label = if light_mode { "🌙" } else { "☀️" };
            if ui.button(RichText::new(label).size(13.0)).clicked() { 
                action.toggle_light_mode = true; 
            }
            ui.add_space(8.0);
            if ui.button(RichText::new(format!("⚙ {}", tr("Settings"))).size(13.0)).clicked() {
                action.open_settings = true;
            }
        });
    });

    action
}
