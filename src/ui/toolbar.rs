use crate::config::recent_files::RecentFiles;
use crate::controller::ControllerCapabilities;
use crate::i18n::{self, Language, tr};
use crate::theme;
use egui::{RichText, Ui};

/// C.6: Rich button atoms helper - combines icon and text for egui 0.34+ Button::new(atoms)
/// 
/// Usage: `ui.button(RichButton::atoms("📂", tr("Open")))` for rich icon+text buttons
pub struct RichButton<'a> {
    icon: &'a str,
    text: &'a str,
}

impl<'a> RichButton<'a> {
    pub fn new(icon: &'a str, text: &'a str) -> Self {
        Self { icon, text }
    }

    /// Returns the combined RichText for use with egui::Button::new()
    pub fn atoms(&self, compact: bool) -> RichText {
        let sz = if compact { 12.0 } else { 13.0 };
        if compact {
            RichText::new(self.icon).size(sz)
        } else {
            RichText::new(format!("{} {}", self.icon, self.text)).size(sz)
        }
    }

    /// Create a RichText directly with custom formatting
    pub fn atoms_colored(&self, compact: bool, color: egui::Color32) -> RichText {
        self.atoms(compact).color(color)
    }
}

pub struct ToolbarAction {
    pub connect_toggle: bool,
    pub open_file: bool,
    pub open_recent: Option<String>,
    pub save_file: bool,
    pub save_project: bool,
    pub open_project: bool,
    pub new_clear_project: bool,
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
    pub toggle_beginner_mode: bool,
    pub open_settings: bool,
    pub open_power_speed_test: bool,
    pub open_gcode_editor: bool,
    pub open_shortcuts: bool,
    pub open_tiling: bool,
    pub open_nesting: bool,
    pub open_job_queue: bool,
    pub open_test_fire: bool,
    pub export_lbrn2: bool,
    pub export_svg: bool,
    pub export_job_report: bool,
    pub save_job_template: bool,
    pub load_job_template: bool,

    pub zoom_in: bool,
    pub zoom_out: bool,
    pub undo: bool,
    pub redo: bool,
    pub open_about: bool,
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
            new_clear_project: false,
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
            toggle_beginner_mode: false,
            open_settings: false,
            open_power_speed_test: false,
            open_gcode_editor: false,
            open_shortcuts: false,
            open_tiling: false,
            open_nesting: false,
            open_job_queue: false,
            open_test_fire: false,
            export_lbrn2: false,
            export_svg: false,
            export_job_report: false,
            save_job_template: false,
            load_job_template: false,

            zoom_in: false,
            zoom_out: false,
            undo: false,
            redo: false,
            open_about: false,
        }
    }
}

impl ToolbarAction {
    pub fn merge(&mut self, other: Self) {
        self.connect_toggle |= other.connect_toggle;
        self.open_file |= other.open_file;
        if other.open_recent.is_some() {
            self.open_recent = other.open_recent;
        }
        self.save_file |= other.save_file;
        self.save_project |= other.save_project;
        self.open_project |= other.open_project;
        self.new_clear_project |= other.new_clear_project;
        self.run_program |= other.run_program;
        self.frame_bbox |= other.frame_bbox;
        self.dry_run |= other.dry_run;
        self.abort_program |= other.abort_program;
        self.hold |= other.hold;
        self.resume |= other.resume;
        self.home |= other.home;
        self.unlock |= other.unlock;
        self.set_zero |= other.set_zero;
        self.reset |= other.reset;
        if other.set_theme.is_some() {
            self.set_theme = other.set_theme;
        }
        if other.set_layout.is_some() {
            self.set_layout = other.set_layout;
        }
        if other.set_language.is_some() {
            self.set_language = other.set_language;
        }
        self.toggle_light_mode |= other.toggle_light_mode;
        self.toggle_beginner_mode |= other.toggle_beginner_mode;
        self.open_settings |= other.open_settings;
        self.open_power_speed_test |= other.open_power_speed_test;
        self.open_gcode_editor |= other.open_gcode_editor;
        self.open_shortcuts |= other.open_shortcuts;
        self.open_tiling |= other.open_tiling;
        self.open_nesting |= other.open_nesting;
        self.open_job_queue |= other.open_job_queue;
        self.open_test_fire |= other.open_test_fire;
        self.export_lbrn2 |= other.export_lbrn2;
        self.export_svg |= other.export_svg;
        self.export_job_report |= other.export_job_report;
        self.save_job_template |= other.save_job_template;
        self.load_job_template |= other.load_job_template;
        self.zoom_in |= other.zoom_in;
        self.zoom_out |= other.zoom_out;
        self.undo |= other.undo;
        self.redo |= other.redo;
        self.open_about |= other.open_about;
    }
}

pub fn show(
    ui: &mut Ui,
    connected: bool,
    running: bool,
    light_mode: bool,
    beginner_mode: bool,
    framing_active: bool,
    recent: &RecentFiles,
    has_file: bool,
    has_shapes: bool,
    caps: ControllerCapabilities,
    current_theme: theme::UiTheme,
    current_layout: theme::UiLayout,
) -> ToolbarAction {
    let mut action = ToolbarAction::default();

    // Adaptive sizing: use icon-only labels when toolbar is narrow
    let avail = ui.available_width();
    let compact = avail < 900.0;
    let sz = if compact { 12.0 } else { 13.0 };

    // Helper: produce "icon text" or just "icon" depending on width
    let label = |icon: &str, text: &str| -> String {
        if compact { icon.to_string() } else { format!("{icon} {text}") }
    };

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = if compact { 2.0 } else { 4.0 };

        // Connect / Disconnect
        let (conn_icon, conn_txt) = if connected { ("⏏", tr("Disconnect")) } else { ("🔌", tr("Connect")) };
        let conn_color = if connected { theme::RED } else { theme::GREEN };
        let conn_tip = if connected { tr("Disconnect") } else { tr("Connect") };
        if ui
            .button(RichText::new(label(conn_icon, &conn_txt)).color(conn_color).size(sz))
            .on_hover_text(format!("{} (Ctrl+K)", conn_tip))
            .clicked()
        {
            action.connect_toggle = true;
        }

        ui.separator();

        // Recent files dropdown
        ui.menu_button( &label("▾", &tr("Recent")), |ui| {
            ui.set_min_width(280.0);
            if recent.paths.is_empty() {
                ui.label(tr("No recent files"));
            } else {
                for path in &recent.paths {
                    let display = std::path::Path::new(path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(path);
                    if ui.selectable_label(false, display).clicked() {
                        action.open_recent = Some(path.clone());
                        ui.close();
                    }
                }
            }
        })
        .response
        .on_hover_text(tr("Recent Files"));
        
        // Open button
        if ui
            .button(RichText::new(label("📂", &tr("Open"))).size(sz))
            .on_hover_text(format!("{} (Ctrl+O)", tr("Open")))
            .clicked()
        {
            action.open_file = true;
        }

        if ui
            .button(RichText::new(label("💾", &tr("Save"))).size(sz))
            .on_hover_text(format!("{} (Ctrl+S)", tr("Save")))
            .clicked()
        {
            action.save_file = true;
        }

        // Project menu
        ui.menu_button( label("📁", &tr("Project")), |ui| {
            if ui.button(format!("� {}", tr("New Project"))).clicked() {
                action.new_clear_project = true;
                ui.close();
            }
            if ui.button(format!("� {} (.a4l)", tr("Open Project"))).clicked() {
                action.open_project = true;
                ui.close();
            }
            if ui
                .add_enabled(has_file, egui::Button::new(format!("💾 {} (.a4l)", tr("Save Project"))))
                .clicked()
            {
                action.save_project = true;
                ui.close();
            }
            ui.separator();
            if ui
                .add_enabled(has_shapes, egui::Button::new("📤 Export .lbrn2"))
                .clicked()
            {
                action.export_lbrn2 = true;
                ui.close();
            }
            if ui
                .add_enabled(has_shapes, egui::Button::new("📤 Export .svg"))
                .clicked()
            {
                action.export_svg = true;
                ui.close();
            }
            if ui
                .add_enabled(has_file, egui::Button::new(format!("📊 {}", tr("Export Job Report"))))
                .clicked()
            {
                action.export_job_report = true;
                ui.close();
            }
        });

        ui.separator();

        // Run / Abort toggle
        if running {
            // Pulsing animation for running status
            let pulse = (ui.ctx().animate_value_with_time(
                egui::Id::new("running_pulse"),
                1.0,
                0.5,
            ) * std::f32::consts::PI).sin() * 0.5 + 0.5;
            let pulse_color = egui::Color32::from_rgb(
                (theme::RED.r() as f32 * (0.5 + pulse * 0.5)) as u8,
                (theme::RED.g() as f32 * (0.5 + pulse * 0.5)) as u8,
                (theme::RED.b() as f32 * (0.5 + pulse * 0.5)) as u8,
            );
            
            ui.horizontal(|ui| {
                if ui
                    .button(
                        RichText::new(label("⛔", &tr("Stop")))
                            .color(pulse_color)
                            .size(sz),
                    )
                    .on_hover_text(tr("Stop"))
                    .clicked()
                {
                    action.abort_program = true;
                }
                // Running indicator
                ui.label(
                    RichText::new("●")
                        .color(pulse_color)
                        .size(16.0)
                )
                .on_hover_text(tr("Job is running"));
            });
        } else {
            let run_btn = ui.add_enabled(
                connected,
                egui::Button::new(
                    RichText::new(label("▶", &tr("Run")))
                        .color(theme::GREEN)
                        .size(sz),
                ),
            );
            if run_btn.on_hover_text(format!("{} (Space)", tr("Run"))).clicked() {
                action.run_program = true;
            }

            let frame_lbl = if framing_active {
                label("⏹", &tr("Stop"))
            } else {
                label("⛶", &tr("Frame"))
            };
            let frame_col = if framing_active {
                theme::RED
            } else {
                theme::SUBTEXT
            };
            if ui
                .add_enabled(
                    connected,
                    egui::Button::new(RichText::new(frame_lbl).color(frame_col).size(sz)),
                )
                .on_hover_text(format!("{} (F)", tr("Frame")))
                .clicked()
            {
                action.frame_bbox = true;
            }

            // Dry Run
            if ui
                .add_enabled(
                    connected && has_file,
                    egui::Button::new(RichText::new(label("🛡", &tr("Dry Run"))).color(theme::BLUE).size(sz)),
                )
                .on_hover_text(tr("Dry Run"))
                .clicked()
            {
                action.dry_run = true;
            }
        }

        if ui
            .add_enabled(
                connected && caps.supports_hold_resume,
                egui::Button::new(RichText::new(label("⏸", &tr("Hold"))).size(sz)),
            )
            .on_hover_text(tr("Hold"))
            .clicked()
        {
            action.hold = true;
        }
        if ui
            .add_enabled(
                connected && caps.supports_hold_resume,
                egui::Button::new(RichText::new(label("⏯", &tr("Resume"))).size(sz)),
            )
            .on_hover_text(tr("Resume"))
            .clicked()
        {
            action.resume = true;
        }

        ui.separator();

        if ui
            .add_enabled(
                connected && caps.supports_home,
                egui::Button::new(RichText::new(label("🏠", &tr("Home"))).size(sz)),
            )
            .on_hover_text(format!("{} (H)", tr("Home")))
            .clicked()
        {
            action.home = true;
        }
        if ui
            .add_enabled(
                connected && caps.supports_unlock,
                egui::Button::new(RichText::new(label("🔓", &tr("Unlock"))).size(sz)),
            )
            .on_hover_text(format!("{} (Ctrl+U)", tr("Unlock")))
            .clicked()
        {
            action.unlock = true;
        }
        if ui
            .add_enabled(
                connected,
                egui::Button::new(RichText::new(label("⊙", &tr("Zero"))).size(sz)),
            )
            .on_hover_text(format!("{} (Z)", tr("Set Zero")))
            .clicked()
        {
            action.set_zero = true;
        }

        ui.separator();

        if ui
            .add_enabled(
                connected && caps.supports_reset,
                egui::Button::new(
                    RichText::new(label("↻", &tr("Reset")))
                        .color(theme::PEACH)
                        .size(sz),
                ),
            )
            .on_hover_text(tr("Reset"))
            .clicked()
        {
            action.reset = true;
        }

        // View menu
        ui.menu_button( label("👁", &tr("View")), |ui| {
            ui.label(RichText::new(format!("{}:", tr("Theme"))).strong());
            if ui
                .selectable_label(current_theme == theme::UiTheme::Modern, tr("Modern (recommended)"))
                .clicked()
            {
                action.set_theme = Some(theme::UiTheme::Modern);
                ui.close();
            }
            if ui
                .selectable_label(current_theme == theme::UiTheme::Industrial, tr("Industrial (advanced)"))
                .clicked()
            {
                action.set_theme = Some(theme::UiTheme::Industrial);
                ui.close();
            }

            ui.separator();
            ui.label(RichText::new(format!("{}:", tr("Layout"))).strong());
            if ui
                .selectable_label(current_layout == theme::UiLayout::Modern, tr("Modern layout (simple)"))
                .clicked()
            {
                action.set_layout = Some(theme::UiLayout::Modern);
                ui.close();
            }
            if ui
                .selectable_label(current_layout == theme::UiLayout::Classic, tr("Classic layout (expert)"))
                .clicked()
            {
                action.set_layout = Some(theme::UiLayout::Classic);
                ui.close();
            }

            ui.separator();
            let beginner_label = if beginner_mode {
                format!("✅ {}", tr("Beginner Mode"))
            } else {
                tr("Beginner Mode")
            };
            if ui.selectable_label(beginner_mode, beginner_label).clicked() {
                action.toggle_beginner_mode = true;
                ui.close();
            }

            ui.separator();
            ui.label(RichText::new(format!("{}:", tr("Language"))).strong());
            let current_lang = i18n::get_language();
            let langs = [
                Language::English,
                Language::French,
                Language::Japanese,
                Language::German,
                Language::Italian,
                Language::Arabic,
                Language::Spanish,
                Language::Portuguese,
            ];
            for lang in langs {
                if ui
                    .selectable_label(current_lang == lang, lang.name())
                    .clicked()
                {
                    action.set_language = Some(lang);
                    ui.close();
                }
            }
        });

        // Tools menu
        ui.menu_button( label("🔧", &tr("Tools")), |ui| {
            if ui.button(format!("⊞ {}", tr("Power/Speed Test"))).clicked() {
                action.open_power_speed_test = true;
                ui.close();
            }
            if ui.button(format!("🔥 {}", tr("Test Fire"))).clicked() {
                action.open_test_fire = true;
                ui.close();
            }
            ui.separator();
            if ui
                .add_enabled(has_file, egui::Button::new(format!("📝 {}", tr("GCode Editor"))))
                .clicked()
            {
                action.open_gcode_editor = true;
                ui.close();
            }
            if ui
                .add_enabled(has_file, egui::Button::new(format!("⊟ {}", tr("Tiling"))))
                .clicked()
            {
                action.open_tiling = true;
                ui.close();
            }
            if ui
                .add_enabled(has_shapes, egui::Button::new(format!("🧩 {}", tr("Auto Nesting"))))
                .clicked()
            {
                action.open_nesting = true;
                ui.close();
            }
            if ui
                .add_enabled(has_file, egui::Button::new(format!("📚 {}", tr("Job Queue"))))
                .clicked()
            {
                action.open_job_queue = true;
                ui.close();
            }
            if ui.button(format!("⌨ {}", tr("Shortcuts"))).clicked() {
                action.open_shortcuts = true;
                ui.close();
            }
        });

        ui.separator();

        // Settings & theme toggle — in normal flow, no right_to_left
        if ui
            .add_enabled(
                caps.supports_grbl_settings,
                egui::Button::new(RichText::new(label("⚙", &tr("Settings"))).size(sz)),
            )
            .on_hover_text(tr("Settings"))
            .clicked()
        {
            action.open_settings = true;
        }

        let theme_icon = if light_mode { "🌙" } else { "☀" };
        let theme_tip = if light_mode { tr("Dark UI") } else { tr("Light UI") };
        if ui
            .button(RichText::new(label(theme_icon, &theme_tip)).size(sz))
            .on_hover_text(if light_mode { tr("Dark UI") } else { tr("Light UI") })
            .clicked()
        {
            action.toggle_light_mode = true;
        }
    });

    action
}

pub fn show_menu_bar(
    ui: &mut Ui,
    recent: &RecentFiles,
    has_file: bool,
    has_shapes: bool,
    beginner_mode: bool,
    light_mode: bool,
    caps: ControllerCapabilities,
    current_theme: theme::UiTheme,
    current_layout: theme::UiLayout,
) -> ToolbarAction {
    let mut action = ToolbarAction::default();

    egui::MenuBar::new().ui(ui, |ui| {
        // File / Fichier
        ui.menu_button(format!("📂 {}", tr("File")), |ui| {
            if ui.button(format!("📄 {}", tr("New Project"))).clicked() {
                action.new_clear_project = true;
                ui.close();
            }
            if ui.button(format!("📂 {}", tr("Open"))).clicked() {
                action.open_file = true;
                ui.close();
            }
            ui.menu_button(format!("▾ {}", tr("Recent Files")), |ui| {
                if recent.paths.is_empty() {
                    ui.label(tr("No recent files"));
                } else {
                    for path in &recent.paths {
                        let display = std::path::Path::new(path)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or(path);
                        if ui.selectable_label(false, display).clicked() {
                            action.open_recent = Some(path.clone());
                            ui.close();
                        }
                    }
                }
            });
            if ui.button(format!("💾 {}", tr("Save"))).clicked() {
                action.save_file = true;
                ui.close();
            }
            ui.separator();
            if ui.button(format!("📂 {} (.a4l)", tr("Open Project"))).clicked() {
                action.open_project = true;
                ui.close();
            }
            if ui
                .add_enabled(has_file, egui::Button::new(format!("💾 {} (.a4l)", tr("Save Project"))))
                .clicked()
            {
                action.save_project = true;
                ui.close();
            }
            ui.separator();
            if ui
                .add_enabled(has_shapes, egui::Button::new("📤 Export .lbrn2"))
                .clicked()
            {
                action.export_lbrn2 = true;
                ui.close();
            }
            if ui
                .add_enabled(has_shapes, egui::Button::new("📤 Export .svg"))
                .clicked()
            {
                action.export_svg = true;
                ui.close();
            }
            if ui
                .add_enabled(has_file, egui::Button::new(format!("📊 {}", tr("Export Job Report"))))
                .clicked()
            {
                action.export_job_report = true;
                ui.close();
            }
        });

        // Edit / Édition
        ui.menu_button(format!("✏ {}", tr("Edit")), |ui| {
            if ui.button(format!("↶ {}", tr("Undo"))).clicked() {
                action.undo = true;
                ui.close();
            }
            if ui.button(format!("↷ {}", tr("Redo"))).clicked() {
                action.redo = true;
                ui.close();
            }
            ui.separator();
            if ui.button(format!("🔍 {}", tr("Zoom In"))).clicked() {
                action.zoom_in = true;
                ui.close();
            }
            if ui.button(format!("🔎 {}", tr("Zoom Out"))).clicked() {
                action.zoom_out = true;
                ui.close();
            }
        });

        // View / Affichage
        ui.menu_button(format!("👁 {}", tr("View")), |ui| {
            ui.label(RichText::new(format!("{}:", tr("Theme"))).strong());
            if ui
                .selectable_label(current_theme == theme::UiTheme::Modern, tr("Modern (recommended)"))
                .clicked()
            {
                action.set_theme = Some(theme::UiTheme::Modern);
                ui.close();
            }
            if ui
                .selectable_label(current_theme == theme::UiTheme::Industrial, tr("Industrial (advanced)"))
                .clicked()
            {
                action.set_theme = Some(theme::UiTheme::Industrial);
                ui.close();
            }

            ui.separator();
            ui.label(RichText::new(format!("{}:", tr("Layout"))).strong());
            if ui
                .selectable_label(current_layout == theme::UiLayout::Modern, tr("Modern layout (simple)"))
                .clicked()
            {
                action.set_layout = Some(theme::UiLayout::Modern);
                ui.close();
            }
            if ui
                .selectable_label(current_layout == theme::UiLayout::Classic, tr("Classic layout (expert)"))
                .clicked()
            {
                action.set_layout = Some(theme::UiLayout::Classic);
                ui.close();
            }

            ui.separator();
            let beginner_label = if beginner_mode {
                format!("✅ {}", tr("Beginner Mode"))
            } else {
                tr("Beginner Mode")
            };
            if ui.selectable_label(beginner_mode, beginner_label).clicked() {
                action.toggle_beginner_mode = true;
                ui.close();
            }

            ui.separator();
            ui.label(RichText::new(format!("{}:", tr("Language"))).strong());
            let current_lang = i18n::get_language();
            let langs = [
                Language::English,
                Language::French,
                Language::Japanese,
                Language::German,
                Language::Italian,
                Language::Arabic,
                Language::Spanish,
                Language::Portuguese,
            ];
            for lang in langs {
                if ui
                    .selectable_label(current_lang == lang, lang.name())
                    .clicked()
                {
                    action.set_language = Some(lang);
                    ui.close();
                }
            }

            ui.separator();
            let theme_toggle_label = if light_mode {
                format!("🌙 {}", tr("Dark UI"))
            } else {
                format!("☀ {}", tr("Light UI"))
            };
            if ui.button(theme_toggle_label).clicked() {
                action.toggle_light_mode = true;
                ui.close();
            }
        });

        // Tools / Outils
        ui.menu_button(format!("🔧 {}", tr("Tools")), |ui| {
            if ui.button(format!("⊞ {}", tr("Power/Speed Test"))).clicked() {
                action.open_power_speed_test = true;
                ui.close();
            }
            if ui.button(format!("🔥 {}", tr("Test Fire"))).clicked() {
                action.open_test_fire = true;
                ui.close();
            }
            ui.separator();
            if ui
                .add_enabled(has_file, egui::Button::new(format!("📝 {}", tr("GCode Editor"))))
                .clicked()
            {
                action.open_gcode_editor = true;
                ui.close();
            }
            if ui
                .add_enabled(has_file, egui::Button::new(format!("⊟ {}", tr("Tiling"))))
                .clicked()
            {
                action.open_tiling = true;
                ui.close();
            }
            if ui
                .add_enabled(has_shapes, egui::Button::new(format!("🧩 {}", tr("Auto Nesting"))))
                .clicked()
            {
                action.open_nesting = true;
                ui.close();
            }
            if ui
                .add_enabled(has_file, egui::Button::new(format!("📚 {}", tr("Job Queue"))))
                .clicked()
            {
                action.open_job_queue = true;
                ui.close();
            }
            if ui.button(format!("⌨ {}", tr("Shortcuts"))).clicked() {
                action.open_shortcuts = true;
                ui.close();
            }
            ui.separator();
            if ui.button(format!("💾 {}", tr("Save Layer Template"))).on_hover_text(tr("Save Layer Template")).clicked() {
                action.save_job_template = true;
                ui.close();
            }
            if ui.button(format!("📂 {}", tr("Load Layer Template"))).on_hover_text(tr("Load Layer Template")).clicked() {
                action.load_job_template = true;
                ui.close();
            }
            ui.separator();
            if ui
                .add_enabled(
                    caps.supports_grbl_settings,
                    egui::Button::new(format!("⚙ {}", tr("Settings"))),
                )
                .clicked()
            {
                action.open_settings = true;
                ui.close();
            }
        });

        // About / À propos
        ui.menu_button(format!("ℹ {}", tr("Help")), |ui| {
            if ui.button(format!("ℹ {}", tr("About"))).clicked() {
                action.open_about = true;
                ui.close();
            }
        });
    });

    action
}
