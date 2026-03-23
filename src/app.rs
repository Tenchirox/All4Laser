use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};

use egui::{CentralPanel, RichText, SidePanel, TopBottomPanel};

use crate::app_types::{
    AutosaveState, CameraLiveState, JobTransform, TestFireState, WizardState,
    shape_fill_warning,
};
use crate::config::machine_profile::{MachineProfile, MachineProfileStore};
use crate::config::recent_files::RecentFiles;
use crate::config::settings::AppSettings;
use crate::controller::{
    ControllerBackend, ControllerCapabilities, ControllerKind, ControllerResponse, RealtimeCommand,
};
use crate::gcode::file::GCodeFile;
use crate::grbl::types::*;
use crate::imaging;
use crate::preview::renderer::PreviewRenderer;
use crate::serial::connection::{self, SerialConnection, SerialMsg};
use crate::theme;
use crate::ui;
use crate::ui::drawing::{ShapeKind, ShapeParams};
use crate::ui::marker_detect::detect_cross_and_circle_markers;
use crate::ui::node_edit::{NodeEditSnapshot, redo_history_step, undo_history_step};
use crate::ui::offset::JoinStyle;
use crate::ui::preflight::{PreflightContext, PreflightReport, PreflightSeverity};

const MAX_LOG_LINES: usize = 500;

const STATUS_POLL_MS: u64 = 250;
const LEFT_PANEL_WIDTH: f32 = 280.0;

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum RightPanelTab {
    Cuts,
    Move,
    Console,
    Art,
    Laser,
    Job,
    Notes,
}

pub struct All4LaserApp {
    // GRBL state
    grbl_state: GrblState,

    // Serial / TCP
    connection: Option<SerialConnection>,
    ports: Vec<String>,
    selected_port: usize,
    baud_rates: Vec<u32>,
    selected_baud: usize,
    use_tcp: bool,
    tcp_host: String,
    tcp_port_str: String,

    // GCode
    loaded_file: Option<GCodeFile>,
    program_lines: std::sync::Arc<Vec<String>>,
    program_index: usize,
    running: bool,
    is_dry_run: bool, // new flag for dry run

    // Preview
    renderer: PreviewRenderer,
    needs_auto_fit: bool,

    // Console
    console_log: VecDeque<String>,
    console_input: String,

    // Jog
    jog_step: f32,
    jog_feed: f32,

    // Import Dialog
    import_state: Option<ui::image_dialog::ImageImportState>,

    // Settings Dialog
    settings_state: Option<ui::settings_dialog::SettingsDialogState>,

    // Macros
    macros_state: ui::macros::MacrosState,

    // Drawing Tools
    drawing_state: crate::ui::drawing::DrawingState,
    clipboard_shapes: Vec<ShapeParams>,
    clipboard_paste_serial: u32,
    node_undo_stack: VecDeque<NodeEditSnapshot>,
    node_redo_stack: VecDeque<NodeEditSnapshot>,
    node_move_undo_armed: bool,
    shape_transform_undo_armed: bool,
    node_smooth_strength: f32,
    node_corner_strength: f32,
    path_simplify_tolerance: f32,
    path_smooth_strength: f32,
    path_smooth_iterations: u32,

    // Power/Speed Test
    power_speed_test: ui::power_speed_test::PowerSpeedTestState,

    // Recent Files (MRU)
    recent_files: RecentFiles,

    // Machine Profile
    machine_profile: MachineProfile,
    profile_store: MachineProfileStore,
    controller_backend: Arc<dyn ControllerBackend>,

    // Job Transform
    job_transform: JobTransform,

    // End-of-job notification
    notify_job_done: bool,
    notify_sound_enabled: bool,

    // Error Notification
    last_error: Option<String>,

    // GCode Editor
    gcode_editor: ui::gcode_editor::GCodeEditorState,

    // Shortcuts panel
    shortcuts: ui::shortcuts::ShortcutsState,

    // Preflight
    preflight_state: ui::preflight::PreflightState,

    // Tiling
    tiling: ui::tiling::TilingState,
    nesting_state: ui::nesting::NestingState,
    job_queue_state: ui::job_queue::JobQueueState,
    active_queue_job: Option<ui::job_queue::QueuedJob>,

    // Material Library
    materials_state: ui::materials::MaterialsState,

    // Test Fire
    test_fire: TestFireState,

    // Feed / Spindle RT override (0-100%, 100 = no change)
    feed_override_pct: f32,
    spindle_override_pct: f32,

    // Theme
    ui_theme: theme::UiTheme,
    ui_layout: theme::UiLayout,
    light_mode: bool,
    beginner_mode: bool,

    // Focus Target
    is_focus_on: bool,
    framing_active: bool,
    framing_wait_idle: bool,

    // Estimation
    estimation: crate::gcode::estimation::EstimationResult,

    // Camera
    camera_state: ui::camera::CameraState,
    camera_live: CameraLiveState,
    circular_array_state: ui::circular_array::CircularArrayState,
    grid_array_state: ui::grid_array::GridArrayState,
    offset_state: ui::offset::OffsetState,
    boolean_ops_state: ui::boolean_ops::BooleanOpsState,

    // Professional Tier
    text_state: ui::text::TextToolState,
    generator_state: ui::generators::GeneratorState,

    // Layers (New System)
    layers: Vec<ui::layers_new::CutLayer>,
    active_layer_idx: usize,
    cut_settings_state: ui::cut_settings::CutSettingsState,

    // Language
    language: crate::i18n::Language,
    active_tab: RightPanelTab,

    // Display units (F96)
    display_unit: crate::config::settings::DisplayUnit,
    speed_unit: crate::config::settings::SpeedUnit,

    // Persistence
    settings: AppSettings,

    // Preflight checks
    preflight_report: Option<PreflightReport>,
    preflight_block_critical: bool,

    // Event log (F67)
    event_log: crate::config::event_log::EventLog,

    // Project notes (F90)
    project_notes: String,

    // Startup wizard (F43)
    wizard: WizardState,

    // Auto-save (F71)
    autosave: AutosaveState,

    // Timing
    last_poll: Instant,
    about_open: bool,

    // Update checker
    update_available: Option<String>,
    update_receiver: Option<crossbeam_channel::Receiver<String>>,
    update_progress_rx: Option<crossbeam_channel::Receiver<crate::updater::UpdateProgress>>,
    update_progress_state: Option<crate::updater::UpdateProgress>,

    // Background LightBurn import
    lbrn_import_receiver: Option<crossbeam_channel::Receiver<Result<(Vec<ShapeParams>, Vec<crate::gcode::lbrn_import::LbrnLayerOverride>, String), String>>>,
    lbrn_loading_msg: Option<String>,

    // Batch operations (multi-select)
    batch_move_x: f32,
    batch_move_y: f32,
    batch_target_layer: usize,
}

impl All4LaserApp {
    fn render_top_panel(&mut self, ctx: &egui::Context) {
        let is_connected = self.is_connected();
        let is_running = self.running;
        let is_light = self.light_mode;
        let caps = self.controller_capabilities();

        let mut menu_actions = ui::toolbar::ToolbarAction::default();
        if self.ui_theme == theme::UiTheme::Industrial || self.ui_theme == theme::UiTheme::Pro || self.ui_theme == theme::UiTheme::Rayforge {
            TopBottomPanel::top("menu_bar_panel").show(ctx, |ui| {
                menu_actions = ui::toolbar::show_menu_bar(
                    ui,
                    &self.recent_files,
                    self.loaded_file.is_some(),
                    !self.drawing_state.shapes.is_empty(),
                    self.beginner_mode,
                    self.light_mode,
                    caps,
                );
            });
        }

        TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(4.0);
            let has_file = self.loaded_file.is_some();
            let has_shapes = !self.drawing_state.shapes.is_empty();
            let mut actions = ui::toolbar::show(
                ui,
                is_connected,
                is_running,
                is_light,
                self.beginner_mode,
                self.framing_active,
                &self.recent_files,
                has_file,
                has_shapes,
                caps,
            );
            actions.merge(menu_actions);
            ui.add_space(4.0);

            self.handle_toolbar_actions(ctx, actions);
        });
    }

    fn render_bottom_panel(&mut self, ctx: &egui::Context) {
        let caps = self.controller_capabilities();
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            if self.running && !self.program_lines.is_empty() {
                let progress = self.program_index as f32 / self.program_lines.len() as f32;
                let bar = egui::ProgressBar::new(progress).text(format!(
                    "{}/{}",
                    self.program_index,
                    self.program_lines.len()
                ));
                ui.add(bar);
                ui.add_space(4.0);
            }

            let file_info = self
                .loaded_file
                .as_ref()
                .map(|f| (f.filename.as_str(), f.line_count(), f.estimated_time));
            let progress = if self.running {
                Some((self.program_index, self.program_lines.len()))
            } else {
                None
            };

            ui.add_space(4.0);
            let cost = if self.estimation.total_burn_mm > 0.0 {
                let time_hours = self
                    .loaded_file
                    .as_ref()
                    .map(|f| f.estimated_time.as_secs_f32() / 3600.0)
                    .unwrap_or(0.0);
                Some((
                    time_hours * self.settings.cost_per_hour,
                    self.settings.cost_currency.as_str(),
                ))
            } else {
                None
            };
            let sb_actions = ui::status_bar::show(
                ui,
                &self.grbl_state,
                file_info,
                progress,
                caps,
                self.display_unit,
                self.speed_unit,
                cost,
            );
            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            // Palette
            let pal_action = ui::cut_palette::show(ui, &self.layers, self.active_layer_idx);
            if let Some(idx) = pal_action.select_layer {
                self.assign_selected_shapes_to_layer(idx);
                self.active_layer_idx = idx;
                // Automatically set drawing tool to this layer
                self.drawing_state.current.layer_idx = idx;
            }
            if let Some(idx) = pal_action.open_settings {
                self.cut_settings_state.editing_layer_idx = Some(idx);
                self.cut_settings_state.is_open = true;
            }

            if sb_actions.feed_up {
                self.send_realtime_or_warn(RealtimeCommand::FeedOverridePlus10, "Feed override");
            }
            if sb_actions.feed_down {
                self.send_realtime_or_warn(RealtimeCommand::FeedOverrideMinus10, "Feed override");
            }
            if sb_actions.rapid_up {
                self.send_realtime_or_warn(RealtimeCommand::RapidOverride100, "Rapid override");
            }
            if sb_actions.rapid_down {
                self.send_realtime_or_warn(RealtimeCommand::RapidOverride25, "Rapid override");
            }
            if sb_actions.spindle_up {
                self.send_realtime_or_warn(
                    RealtimeCommand::SpindleOverridePlus10,
                    "Laser power override",
                );
            }
            if sb_actions.spindle_down {
                self.send_realtime_or_warn(
                    RealtimeCommand::SpindleOverrideMinus10,
                    "Laser power override",
                );
            }
            if sb_actions.toggle_unit {
                use crate::config::settings::DisplayUnit;
                self.display_unit = match self.display_unit {
                    DisplayUnit::Millimeters => DisplayUnit::Inches,
                    DisplayUnit::Inches => DisplayUnit::Millimeters,
                };
                self.sync_settings();
            }
            if sb_actions.toggle_speed_unit {
                self.speed_unit = self.speed_unit.toggle();
                self.sync_settings();
            }
        });
    }

    fn render_left_panel(&mut self, ctx: &egui::Context) {
        let connected = self.is_connected();
        let left_panel_width = match self.ui_layout {
            theme::UiLayout::Modern => LEFT_PANEL_WIDTH,
            theme::UiLayout::Pro => 300.0,
            theme::UiLayout::Classic => 220.0,
        };
        SidePanel::left("left_panel")
            .resizable(true)
            .default_width(left_panel_width)
            .width_range(120.0..=600.0)
            .show(ctx, |ui| {
                egui::ScrollArea::both()
                    .id_salt("left_scroll")
                    .auto_shrink([true, false])
                    .show(ui, |ui| {
                        if self.ui_layout == theme::UiLayout::Classic {
                            self.ui_left_tools_classic(ui);
                        } else {
                            self.ui_left_content(ui, connected);
                        }
                    });
            });
    }

    fn render_right_panel(&mut self, ctx: &egui::Context) {
        let connected = self.is_connected();
        let right_panel_width = match self.ui_layout {
            theme::UiLayout::Modern => 220.0,
            theme::UiLayout::Pro => 280.0,
            theme::UiLayout::Classic => 340.0,
        };
        SidePanel::right("right_panel")
            .resizable(true)
            .default_width(right_panel_width)
            .width_range(120.0..=600.0)
            .show(ctx, |ui| {
                if self.ui_layout == theme::UiLayout::Modern {
                    self.ui_right_content(ui);
                } else {
                    self.ui_right_tabs(ui, connected);
                }
            });

        if self.ui_layout == theme::UiLayout::Pro {
            egui::TopBottomPanel::bottom("bottom_console_panel")
                .resizable(true)
                .default_height(150.0)
                .show(ctx, |ui| {
                    self.ui_right_content(ui);
                });
        }
    }

    fn render_ui_layout(&mut self, ctx: &egui::Context) {
        self.render_top_panel(ctx);
        self.render_bottom_panel(ctx);
        self.render_left_panel(ctx);
        self.render_right_panel(ctx);
        self.update_preview(ctx);
    }

    fn process_modals(&mut self, ctx: &egui::Context) {
        // === Preflight Modal ===
        let preflight_action = ui::preflight::show(ctx, &mut self.preflight_state);
        if preflight_action.proceed {
            self.preflight_state.bypass = true;
            self.run_program_with_preflight();
        }

        // Tool windows dispatch
        self.update_tool_windows(ctx);

        // Import modal
        self.update_import_modal(ctx);

        // Modals dispatch (cut settings, GRBL settings)
        self.update_modals(ctx);
    }

    fn process_notifications(&mut self, ctx: &egui::Context) {
        // === Error Notification ===
        // We clone the error first to avoid borrowing `self` in the closure
        let error_to_show = self.last_error.clone();
        if let Some(err) = error_to_show {
            let mut open = true;
            egui::Window::new("❌ Error")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new(err).color(theme::RED));
                    ui.add_space(8.0);
                    if ui.button("OK").clicked() {
                        self.last_error = None;
                    }
                });
            if !open {
                self.last_error = None;
            }
        }

        // === Job Done Notification ===
        if self.notify_job_done {
            let mut dismiss = false;
            egui::Window::new(format!("✅ {}", crate::i18n::tr("Job Complete")))
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new(crate::i18n::tr("Program finished successfully!")).size(16.0));
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.notify_sound_enabled, format!("🔔 {}", crate::i18n::tr("Sound")));
                    });
                    ui.add_space(4.0);
                    if ui.button("OK").clicked() {
                        dismiss = true;
                    }
                });
            if dismiss {
                if self.notify_sound_enabled {
                    crate::ui::sound::play_notification_sound();
                }
                self.notify_job_done = false;
            }
        }

        if self.about_open {
            let mut open = true;
            let mut close_clicked = false;
            egui::Window::new(crate::i18n::tr("About All4Laser"))
                .open(&mut open)
                .collapsible(false)
                .resizable(true)
                .default_width(420.0)
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new("All4Laser").strong().size(22.0));
                    ui.label(format!("v{}", env!("CARGO_PKG_VERSION")));
                    ui.label(crate::i18n::tr("Advanced Laser Control Software"));
                    ui.add_space(6.0);
                    ui.label("Copyright (c) 2024-2026 Tenchirox — GPL-3.0");
                    ui.add_space(10.0);

                    ui.label(egui::RichText::new(crate::i18n::tr("Acknowledgments")).strong().size(15.0));
                    ui.add_space(4.0);

                    ui.label(egui::RichText::new(crate::i18n::tr("Inspiration")).strong());
                    ui.label("- LaserMagic (MadSquirrels) — Multi-protocol laser library (GPL-v3)");
                    ui.label("- LightBurn — UI/UX patterns, layer system");
                    ui.label("- LaserGRBL — Open-source GRBL laser control");
                    ui.add_space(6.0);

                    ui.label(egui::RichText::new(crate::i18n::tr("Libraries")).strong());
                    egui::ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
                        let libs = [
                            ("egui / eframe", "Immediate-mode GUI framework"),
                            ("image", "Image decoding & manipulation"),
                            ("usvg / tiny-skia", "SVG parsing & 2D rendering"),
                            ("serde / serde_json", "Serialization & persistence"),
                            ("serialport", "Serial port communication"),
                            ("geo", "Computational geometry"),
                            ("rusttype", "Font rasterization"),
                            ("font-kit", "Font discovery"),
                            ("crossbeam-channel", "Lock-free channels"),
                            ("ureq", "HTTP client (updates & API)"),
                            ("rfd", "Native file dialogs"),
                            ("lopdf", "PDF import"),
                            ("qrcode", "QR code generation"),
                            ("indexmap", "Ordered maps"),
                            ("webbrowser", "System browser integration"),
                            ("base64", "Binary encoding"),
                            ("log / env_logger", "Structured logging"),
                        ];
                        for (name, role) in libs {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(name).strong().monospace());
                                ui.label(format!("— {role}"));
                            });
                        }
                        #[cfg(target_os = "linux")]
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("v4l").strong().monospace());
                            ui.label("— Camera capture (Linux)");
                        });
                        #[cfg(target_os = "windows")]
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("nokhwa").strong().monospace());
                            ui.label("— Camera capture (Windows)");
                        });
                    });

                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(crate::i18n::tr("Thank you to all open-source contributors!"))
                            .italics()
                            .color(crate::theme::SUBTEXT),
                    );
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.link("GitHub").clicked() {
                            let _ = webbrowser::open("https://github.com/Tenchirox/All4Laser");
                        }
                        ui.label("·");
                        if ui.link("LaserMagic").clicked() {
                            let _ = webbrowser::open("https://gitlab.com/MadSquirrels/lasermagic/lasermagic");
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(crate::i18n::tr("OK")).clicked() {
                                close_clicked = true;
                            }
                        });
                    });
                });
            if !open || close_clicked {
                self.about_open = false;
            }
        }

        // === Update Progress Receiver ===
        if let Some(rx) = &self.update_progress_rx {
            while let Ok(progress) = rx.try_recv() {
                self.update_progress_state = Some(progress);
            }
        }

        // === Update Notification ===
        let mut close_update = false;
        let mut start_auto_update = false;
        let mut log_message = None;
        if let Some(new_version) = &self.update_available {
            let new_version_str = new_version.clone();
            let is_downloading = matches!(
                &self.update_progress_state,
                Some(crate::updater::UpdateProgress::Downloading { .. })
                    | Some(crate::updater::UpdateProgress::Installing)
            );
            egui::Window::new(format!("🎉 {}", crate::i18n::tr("Update Available!")))
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-10.0, -10.0))
                .show(ctx, |ui| {
                    ui.label(format!("{} ({}).", crate::i18n::tr("A new version is available"), new_version_str));
                    ui.add_space(4.0);

                    // Show progress if update is in progress
                    if let Some(state) = &self.update_progress_state {
                        match state {
                            crate::updater::UpdateProgress::Downloading { percent, bytes_done, bytes_total } => {
                                ui.horizontal(|ui| {
                                    ui.spinner();
                                    ui.label(format!(
                                        "{}: {:.0}% ({:.1} / {:.1} MB)",
                                        crate::i18n::tr("Downloading"),
                                        percent,
                                        *bytes_done as f64 / 1_048_576.0,
                                        *bytes_total as f64 / 1_048_576.0,
                                    ));
                                });
                                let bar = egui::ProgressBar::new(percent / 100.0).animate(true);
                                ui.add(bar);
                            }
                            crate::updater::UpdateProgress::Installing => {
                                ui.horizontal(|ui| {
                                    ui.spinner();
                                    ui.label(crate::i18n::tr("Installing..."));
                                });
                            }
                            crate::updater::UpdateProgress::Done(path) => {
                                ui.colored_label(
                                    crate::theme::GREEN,
                                    format!("✅ {} {}", crate::i18n::tr("Installed to"), path.display()),
                                );
                                ui.label(crate::i18n::tr("Restart the application to use the new version."));
                                if ui.button(crate::i18n::tr("OK")).clicked() {
                                    close_update = true;
                                }
                            }
                            crate::updater::UpdateProgress::Error(e) => {
                                ui.colored_label(
                                    crate::theme::RED,
                                    format!("❌ {}: {}", crate::i18n::tr("Update failed"), e),
                                );
                                if ui.button(crate::i18n::tr("Dismiss")).clicked() {
                                    close_update = true;
                                }
                            }
                            _ => {}
                        }
                    }

                    if !is_downloading && !matches!(&self.update_progress_state, Some(crate::updater::UpdateProgress::Done(_))) {
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            if ui.button(format!("⬇ {}", crate::i18n::tr("Auto-Update"))).clicked() {
                                start_auto_update = true;
                            }
                            if ui.button(crate::i18n::tr("Download")).clicked() {
                                if let Err(e) = webbrowser::open(
                                    "https://github.com/arkypita/All4Laser/releases/latest",
                                ) {
                                    log_message = Some(format!("Failed to open browser: {}", e));
                                }
                                close_update = true;
                            }
                            if ui.button(crate::i18n::tr("Dismiss")).clicked() {
                                close_update = true;
                            }
                        });
                    }
                });
        }

        if start_auto_update {
            let (tx, rx) = crossbeam_channel::unbounded();
            self.update_progress_rx = Some(rx);
            let api_url = "https://api.github.com/repos/arkypita/All4Laser/releases/latest".to_string();
            let current_version = env!("CARGO_PKG_VERSION").to_string();
            std::thread::spawn(move || {
                crate::updater::run_update_flow(&api_url, &current_version, tx);
            });
        }

        if let Some(msg) = log_message {
            self.log(msg);
        }

        if close_update {
            self.update_available = None;
            self.update_progress_state = None;
            self.update_progress_rx = None;
        }
    }

    fn process_session_lifecycle(&mut self, ctx: &egui::Context) {
        // Auto-save (F71)
        self.perform_autosave();

        // Recovery prompt (F71)
        if self.autosave.show_recovery_prompt {
            let mut restore = false;
            let mut discard = false;
            egui::Window::new(format!("🔄 {}", crate::i18n::tr("Session Recovery")))
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label(crate::i18n::tr("A previous session was interrupted. Restore it?"));
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button(format!("✅ {}", crate::i18n::tr("Restore"))).clicked() {
                            restore = true;
                        }
                        if ui.button(format!("🗑 {}", crate::i18n::tr("Discard"))).clicked() {
                            discard = true;
                        }
                    });
                });
            if restore {
                if let Some(recovery) = self.autosave.pending_recovery.take() {
                    self.apply_recovery(recovery);
                }
                self.autosave.show_recovery_prompt = false;
            }
            if discard {
                self.autosave.pending_recovery = None;
                self.autosave.show_recovery_prompt = false;
                crate::config::project::ProjectFile::clear_recovery();
            }
        }

        // Startup wizard (F43)
        {
            let mut wctx = ui::wizard::WizardContext {
                wizard: &mut self.wizard,
                language: &mut self.language,
                machine_profile: &mut self.machine_profile,
            };
            let wresult = ui::wizard::show_wizard(ctx, &mut wctx);
            if wresult.controller_changed {
                self.sync_controller_backend();
            }
            if wresult.finished {
                self.settings.first_run_done = true;
                if let Some(p) = self
                    .profile_store
                    .profiles
                    .get_mut(self.profile_store.active_index)
                {
                    *p = self.machine_profile.clone();
                }
                self.profile_store.save();
                self.sync_settings();
                self.log("Setup wizard completed.".into());
            }
        }

        // Request repaint for live updates
        ctx.request_repaint_after(Duration::from_millis(50));

        // Auto-fit after file load
        if self.needs_auto_fit {
            if let Some(file) = &self.loaded_file {
                let rect = ctx.available_rect();
                let offset = egui::vec2(self.job_transform.offset_x, self.job_transform.offset_y);
                self.renderer
                    .auto_fit(&file.segments, rect, offset, self.job_transform.rotation);
                self.needs_auto_fit = false;
            }
        }

        // Sync workspace size from machine profile to renderer
        self.renderer.workspace_size = egui::vec2(
            self.machine_profile.workspace_x_mm,
            self.machine_profile.workspace_y_mm,
        );
    }

    fn process_input(&mut self, ctx: &egui::Context) {
        // Handle keyboard shortcuts (only when no text input is focused)
        if !ctx.wants_keyboard_input() {
            self.handle_keyboard(ctx);
        }

        // Handle drag-and-drop
        self.handle_file_drop(ctx);
    }

    fn process_background_updates(&mut self, ctx: &egui::Context) {
        if let Some(rx) = &self.update_receiver {
            match rx.try_recv() {
                Ok(version) => {
                    self.update_available = Some(version);
                    self.update_receiver = None; // Stop listening
                }
                Err(crossbeam_channel::TryRecvError::Disconnected) => {
                    self.update_receiver = None; // Stop listening
                }
                Err(crossbeam_channel::TryRecvError::Empty) => {}
            }
        }

        // Poll background LightBurn import
        if let Some(rx) = &self.lbrn_import_receiver {
            match rx.try_recv() {
                Ok(Ok((shapes, layer_overrides, fname))) => {
                    self.lbrn_import_receiver = None;
                    self.lbrn_loading_msg = None;
                    self.drawing_state.shapes = shapes;
                    for ovr in layer_overrides {
                        if let Some(layer) = self.layers.iter_mut().find(|l| l.id == ovr.index) {
                            layer.speed = ovr.speed;
                            layer.power = ovr.power;
                            layer.mode = ovr.mode;
                            layer.passes = ovr.passes;
                        } else {
                            self.layers.push(crate::ui::layers_new::CutLayer {
                                id: ovr.index,
                                speed: ovr.speed,
                                power: ovr.power,
                                mode: ovr.mode,
                                passes: ovr.passes,
                                visible: true,
                                air_assist: false,
                                z_offset: 0.0,
                                min_power: 0.0,
                                fill_interval_mm: 0.1,
                                fill_bidirectional: true,
                                fill_overscan_mm: 0.0,
                                fill_overscan_speed_factor: 0.0,
                                fill_angle_deg: 0.0,
                                output_order: ovr.index as i32,
                                lead_in_mm: 0.0,
                                lead_out_mm: 0.0,
                                kerf_mm: 0.0,
                                tab_enabled: false,
                                tab_spacing: 50.0,
                                tab_size: 0.5,
                                tab_auto: false,
                                tab_count: 4,
                                perforation_enabled: false,
                                perforation_cut_mm: 5.0,
                                perforation_gap_mm: 2.0,
                                fill_pattern: crate::ui::layers_new::FillPattern::Horizontal,
                                contour_offset_enabled: false,
                                contour_offset_count: 3,
                                contour_offset_step_mm: 0.5,
                                print_and_cut_marks: false,
                                spiral_fill_enabled: false,
                                relief_enabled: false,
                                relief_max_z_mm: 5.0,
                                is_construction: false,
                                pass_offset_mm: 0.0,
                                exhaust_enabled: false,
                                exhaust_post_delay_s: 5.0,
                                ramp_enabled: false,
                                ramp_length_mm: 5.0,
                                ramp_start_pct: 20.0,
                                corner_power_enabled: false,
                                corner_power_pct: 60.0,
                                corner_angle_threshold: 90.0,
                                depth_enabled: false,
                                depth_total_mm: 3.0,
                                depth_step_down_mm: 1.0,
                                name: format!("Layer {}", ovr.index),
                                color: egui::Color32::RED,
                            });
                        }
                    }
                    // Skip heavy GCode generation here — shapes are already
                    // visible in the preview. GCode will be generated on-demand
                    // when the user starts a job or explicitly requests it.
                    // self.regenerate_drawing_gcode();
                    self.log(format!(
                        "LightBurn imported: {fname} ({} shapes)",
                        self.drawing_state.shapes.len()
                    ));
                }
                Ok(Err(e)) => {
                    self.lbrn_import_receiver = None;
                    self.lbrn_loading_msg = None;
                    self.show_error(e);
                }
                Err(crossbeam_channel::TryRecvError::Disconnected) => {
                    self.lbrn_import_receiver = None;
                    self.lbrn_loading_msg = None;
                }
                Err(crossbeam_channel::TryRecvError::Empty) => {
                    // Still loading — request repaint to keep polling
                    ctx.request_repaint();
                }
            }
        }

        self.poll_camera_stream(ctx);

        // Poll serial
        self.poll_serial();
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ports = connection::list_ports();
        let profile_store = MachineProfileStore::load();
        let machine_profile = profile_store.active().clone();
        let controller_backend = crate::controller::create_backend(machine_profile.controller_kind);

        let mut app = Self {
            grbl_state: GrblState::default(),
            connection: None,
            ports,
            selected_port: 0,
            baud_rates: ui::connection::default_baud_rates(),
            selected_baud: 4, // 115200
            use_tcp: false,
            tcp_host: String::new(),
            tcp_port_str: "23".to_string(),
            loaded_file: None,
            program_lines: std::sync::Arc::new(Vec::new()),
            program_index: 0,
            running: false,
            is_dry_run: false,
            renderer: PreviewRenderer::default(),
            needs_auto_fit: false,
            console_log: VecDeque::from(vec!["All4Laser ready.".to_string()]),
            console_input: String::new(),
            jog_step: 1.0,
            jog_feed: 1000.0,
            import_state: None,
            settings_state: None,
            macros_state: ui::macros::MacrosState::default(),
            drawing_state: crate::ui::drawing::DrawingState::default(),
            clipboard_shapes: Vec::new(),
            clipboard_paste_serial: 0,
            node_undo_stack: VecDeque::new(),
            node_redo_stack: VecDeque::new(),
            node_move_undo_armed: true,
            shape_transform_undo_armed: true,
            node_smooth_strength: 0.7,
            node_corner_strength: 0.7,
            path_simplify_tolerance: 0.2,
            path_smooth_strength: 0.35,
            path_smooth_iterations: 1,
            power_speed_test: ui::power_speed_test::PowerSpeedTestState::default(),
            recent_files: RecentFiles::load(),
            machine_profile,
            profile_store,
            controller_backend,
            job_transform: JobTransform::default(),
            notify_job_done: false,
            notify_sound_enabled: true,
            last_error: None,
            gcode_editor: ui::gcode_editor::GCodeEditorState::default(),
            shortcuts: ui::shortcuts::ShortcutsState::default(),
            preflight_state: ui::preflight::PreflightState::default(),
            tiling: ui::tiling::TilingState::default(),
            nesting_state: ui::nesting::NestingState::default(),
            job_queue_state: ui::job_queue::JobQueueState::default(),
            active_queue_job: None,
            materials_state: ui::materials::MaterialsState::default(),
            test_fire: TestFireState::default(),
            ui_theme: theme::UiTheme::Modern,
            ui_layout: theme::UiLayout::Modern,
            light_mode: false,
            beginner_mode: true,
            is_focus_on: false,
            framing_active: false,
            framing_wait_idle: false,
            last_poll: Instant::now(),
            feed_override_pct: 100.0,
            spindle_override_pct: 100.0,
            estimation: crate::gcode::estimation::EstimationResult::default(),
            camera_state: ui::camera::CameraState::default(),
            camera_live: CameraLiveState::default(),
            circular_array_state: ui::circular_array::CircularArrayState::default(),
            grid_array_state: ui::grid_array::GridArrayState::default(),
            offset_state: ui::offset::OffsetState::default(),
            boolean_ops_state: ui::boolean_ops::BooleanOpsState::default(),
            text_state: ui::text::TextToolState::default(),
            generator_state: ui::generators::GeneratorState::default(),
            layers: ui::layers_new::CutLayer::default_palette(),
            active_layer_idx: 0,
            cut_settings_state: ui::cut_settings::CutSettingsState::default(),
            language: crate::i18n::Language::English, // Will be overridden
            active_tab: RightPanelTab::Cuts,          // Will be overridden
            display_unit: crate::config::settings::DisplayUnit::Millimeters, // Will be overridden
            speed_unit: crate::config::settings::SpeedUnit::MmPerMin,    // Will be overridden
            settings: AppSettings::load(),
            preflight_report: None,
            preflight_block_critical: true,
            event_log: crate::config::event_log::EventLog::default(),
            project_notes: String::new(),
            wizard: WizardState::default(),
            autosave: AutosaveState::default(),
            about_open: false,
            update_available: None,
            update_receiver: None,
            update_progress_rx: None,
            update_progress_state: None,
            lbrn_import_receiver: None,
            lbrn_loading_msg: None,
            batch_move_x: 0.0,
            batch_move_y: 0.0,
            batch_target_layer: 0,
        };

        let update_url = "https://api.github.com/repos/arkypita/All4Laser/releases/latest";
        let (tx, rx) = crossbeam_channel::bounded(1);
        app.update_receiver = Some(rx);

        std::thread::spawn(move || {
            if let Ok(res) = ureq::get(update_url).call() {
                if let Ok(json) = res.into_json::<serde_json::Value>() {
                    if let Some(tag) = json["tag_name"].as_str() {
                        let current_version = format!("v{}", env!("CARGO_PKG_VERSION"));
                        if tag != current_version {
                            let _ = tx.send(tag.to_string());
                        }
                    }
                }
            }
        });

        // Apply loaded settings
        app.ui_theme = app.settings.theme;
        app.ui_layout = app.settings.layout;
        app.light_mode = app.settings.light_mode;
        app.beginner_mode = app.settings.beginner_mode;
        app.language = app.settings.language;
        app.active_tab = app.settings.active_tab;
        app.display_unit = app.settings.display_unit;
        app.speed_unit = app.settings.speed_unit;
        app.camera_state.enabled = app.settings.camera_enabled;
        app.camera_state.opacity = app.settings.camera_opacity;
        app.camera_state.calibration = app.settings.camera_calibration.clone();
        app.camera_state.snapshot_path = app.settings.camera_snapshot_path.clone();
        app.camera_state.device_index = app.settings.camera_device_index;
        app.camera_state.live_streaming = app.settings.camera_live_streaming;
        if let Some(preset_name) = app.settings.material_selected_preset.as_deref() {
            app.materials_state.select_preset_by_name(preset_name);
        }
        // Restore AI / LLM generator settings
        app.generator_state.ai_config = app.settings.ai_config.clone();
        app.generator_state.ai_use_llm = app.settings.ai_use_llm;
        app.generator_state.ai_layer_cut = app.settings.ai_layer_cut;
        app.generator_state.ai_layer_engrave = app.settings.ai_layer_engrave;
        app.generator_state.ai_layer_fine = app.settings.ai_layer_fine;
        if app.camera_state.live_streaming {
            app.start_live_camera();
        } else if let Some(path) = app.camera_state.snapshot_path.clone() {
            let _ = app.load_camera_snapshot_from_path(&cc.egui_ctx, &path);
        }

        // Check for recovery file (F71)
        if let Some(recovery) = crate::config::project::ProjectFile::load_recovery() {
            app.autosave.pending_recovery = Some(recovery);
            app.autosave.show_recovery_prompt = true;
        }

        // Startup wizard (F43)
        if !app.settings.first_run_done {
            app.wizard.show = true;
            app.wizard.step = 0;
        }

        crate::i18n::set_language(app.language);
        app.apply_theme(&cc.egui_ctx);
        app
    }

    fn build_recovery_snapshot(&self) -> crate::config::project::ProjectFile {
        crate::config::project::ProjectFile {
            version: 2,
            gcode_path: self.loaded_file.as_ref().map(|f| f.filename.clone()),
            gcode_content: if !self.program_lines.is_empty() {
                Some(self.program_lines.join("\n"))
            } else {
                None
            },
            offset_x: self.job_transform.offset_x,
            offset_y: self.job_transform.offset_y,
            rotation_deg: self.job_transform.rotation,
            machine_profile: Some(self.machine_profile.clone()),
            camera_enabled: self.camera_state.enabled,
            camera_opacity: self.camera_state.opacity,
            camera_calibration: self.camera_state.calibration.clone(),
            camera_snapshot_path: self.camera_state.snapshot_path.clone(),
            camera_device_index: self.camera_state.device_index,
            camera_live_streaming: self.camera_state.live_streaming,
            material_selected_preset: self
                .materials_state
                .selected_preset_name()
                .map(str::to_string),
            checkpoint_line: if self.running {
                Some(self.program_index)
            } else {
                None
            },
            project_notes: self.project_notes.clone(),
        }
    }

    fn apply_recovery(&mut self, recovery: crate::config::project::ProjectFile) {
        self.job_transform.offset_x = recovery.offset_x;
        self.job_transform.offset_y = recovery.offset_y;
        self.job_transform.rotation = recovery.rotation_deg;
        if let Some(content) = recovery.gcode_content {
            let lines: Vec<String> = content.lines().map(String::from).collect();
            let name = recovery.gcode_path.as_deref().unwrap_or("recovered");
            let file = crate::gcode::file::GCodeFile::from_lines(name, &lines);
            self.set_loaded_file(file, lines);
            // If we have a checkpoint, set the program index so user can resume (F36)
            if let Some(line) = recovery.checkpoint_line {
                self.program_index = line.min(self.program_lines.len());
                self.log(format!("Job checkpoint restored at line {}.", line));
            }
            self.needs_auto_fit = true;
        }
        crate::config::project::ProjectFile::clear_recovery();
        self.log("Session recovered from auto-save.".into());
    }

    fn perform_autosave(&mut self) {
        if self.autosave.last_save.elapsed() < Duration::from_secs(self.autosave.interval_secs) {
            return;
        }
        self.autosave.last_save = Instant::now();
        if self.program_lines.is_empty() && self.drawing_state.shapes.is_empty() {
            return;
        }
        let snapshot = self.build_recovery_snapshot();
        crate::config::project::ProjectFile::save_recovery(&snapshot);
    }

    fn sync_settings(&mut self) {
        self.settings.theme = self.ui_theme;
        self.settings.layout = self.ui_layout;
        self.settings.light_mode = self.light_mode;
        self.settings.beginner_mode = self.beginner_mode;
        self.settings.language = self.language;
        self.settings.active_tab = self.active_tab;
        self.settings.display_unit = self.display_unit;
        self.settings.speed_unit = self.speed_unit;
        self.settings.camera_enabled = self.camera_state.enabled;
        self.settings.camera_opacity = self.camera_state.opacity;
        self.settings.camera_calibration = self.camera_state.calibration.clone();
        self.settings.camera_snapshot_path = self.camera_state.snapshot_path.clone();
        self.settings.camera_device_index = self.camera_state.device_index;
        self.settings.camera_live_streaming = self.camera_state.live_streaming;
        self.settings.material_selected_preset = self
            .materials_state
            .selected_preset_name()
            .map(str::to_string);
        // Persist AI / LLM generator settings
        self.settings.ai_config = self.generator_state.ai_config.clone();
        self.settings.ai_use_llm = self.generator_state.ai_use_llm;
        self.settings.ai_layer_cut = self.generator_state.ai_layer_cut;
        self.settings.ai_layer_engrave = self.generator_state.ai_layer_engrave;
        self.settings.ai_layer_fine = self.generator_state.ai_layer_fine;
        self.settings.save();
        self.event_log.save();
    }

    fn materials_ui_context(&self) -> ui::materials::MaterialsUiContext {
        ui::materials::MaterialsUiContext {
            machine_profile_name: Some(self.machine_profile.name.clone()),
            active_layer: self.layers.get(self.active_layer_idx).map(|layer| {
                ui::materials::ActiveLayerSummary {
                    name: layer.name.clone(),
                    speed: layer.speed,
                    power: layer.power,
                    passes: layer.passes,
                    mode: layer.mode,
                }
            }),
            speed_unit: self.speed_unit,
        }
    }

    fn apply_material_action(&mut self, mat_action: ui::materials::MaterialApplyAction) {
        if let Some(update) = mat_action.apply_to_active_layer {
            let mut applied_layer_name: Option<String> = None;
            if let Some(layer) = self.layers.get_mut(self.active_layer_idx) {
                layer.speed = update.speed;
                layer.power = update.power;
                layer.passes = update.passes.max(1);
                layer.mode = update.mode;
                applied_layer_name = Some(layer.name.clone());
            }

            if let Some(layer_name) = applied_layer_name {
                self.log(format!(
                    "Applied material preset '{}' to layer {}.",
                    update.preset_name, layer_name
                ));
                if !self.drawing_state.shapes.is_empty() {
                    self.regenerate_drawing_gcode();
                }
            }
        }

        if let Some(s) = mat_action.apply_speed {
            self.jog_feed = s;
        }
        if let Some(p) = mat_action.apply_power {
            self.test_fire.power = p / 10.0;
        }
    }

    pub fn apply_theme(&self, ctx: &egui::Context) {
        theme::apply_theme(
            ctx,
            &theme::AppTheme {
                ui_theme: self.ui_theme,
                is_light: self.light_mode,
            },
        );
    }

    fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    fn sync_controller_backend(&mut self) {
        self.controller_backend =
            crate::controller::create_backend(self.machine_profile.controller_kind);
    }

    fn apply_controller_kind_change(&mut self, previous_kind: ControllerKind) {
        self.sync_controller_backend();
        if previous_kind != self.machine_profile.controller_kind && self.is_connected() {
            self.disconnect();
            self.log(
                "Controller backend changed. Reconnect to apply protocol changes.".to_string(),
            );
        }
    }

    fn send_realtime(&self, command: RealtimeCommand) -> bool {
        let Some(conn) = self.connection.as_ref() else {
            return false;
        };
        if let Some(byte) = self.controller_backend.realtime_byte(command) {
            conn.send_byte(byte);
            return true;
        }
        if let Some(line) = self.controller_backend.realtime_line(command) {
            conn.send(line);
            return true;
        }
        false
    }

    fn controller_capabilities(&self) -> ControllerCapabilities {
        self.controller_backend.capabilities()
    }

    fn send_realtime_or_warn(&mut self, command: RealtimeCommand, action_label: &str) -> bool {
        if self.send_realtime(command) {
            return true;
        }
        if self.is_connected() {
            self.log(format!(
                "{action_label} is not supported by {} backend.",
                self.machine_profile.controller_kind.label()
            ));
        }
        false
    }

    fn log(&mut self, msg: String) {
        let timestamp = chrono_lite();
        let entry = format!("[{timestamp}] {msg}");
        self.console_log.push_back(entry.clone());
        if self.console_log.len() > MAX_LOG_LINES {
            self.console_log.pop_front();
        }
        self.event_log.push(entry);
    }

    fn show_error(&mut self, msg: String) {
        self.log(format!("ERROR: {}", msg));
        self.last_error = Some(msg);
    }

    fn poll_serial(&mut self) {
        // Poll for status periodically
        if self.is_connected() && self.last_poll.elapsed() > Duration::from_millis(STATUS_POLL_MS) {
            self.send_realtime(RealtimeCommand::StatusReport);
            self.last_poll = Instant::now();
        }

        // Drain incoming messages into a local buffer to avoid borrow issues
        let msgs: Vec<SerialMsg> = if let Some(conn) = self.connection.as_ref() {
            let mut v = Vec::new();
            while let Ok(msg) = conn.rx.try_recv() {
                v.push(msg);
            }
            v
        } else {
            Vec::new()
        };

        for msg in msgs {
            match msg {
                SerialMsg::Parsed { raw, response } => {
                    self.log(raw);
                    match response {
                        ControllerResponse::Grbl(response) => match response {
                            GrblResponse::Status(state) => {
                                let old_status = self.grbl_state.status;
                                // Store machine position in renderer for crosshair display
                                self.renderer.machine_pos = egui::pos2(state.mpos.x, state.mpos.y);
                                self.grbl_state = state;

                                if self.framing_active {
                                    if self.grbl_state.status == MacStatus::Run {
                                        self.framing_wait_idle = true;
                                    } else if self.grbl_state.status == MacStatus::Idle
                                        && old_status == MacStatus::Run
                                        && self.framing_wait_idle
                                    {
                                        self.framing_wait_idle = false;
                                        self.send_frame_sequence();
                                    }
                                }
                            }
                            GrblResponse::Ok => {
                                if self.running && self.program_index < self.program_lines.len() {
                                    self.send_next_program_line();
                                } else if self.running
                                    && self.program_index >= self.program_lines.len()
                                {
                                    self.handle_program_completed();
                                }
                            }
                            GrblResponse::Error(code) => {
                                if self.running {
                                    self.handle_program_failed(format!("Controller error:{code}"));
                                }
                            }
                            GrblResponse::Alarm(code) => {
                                self.handle_program_failed(format!("ALARM:{code}"));
                            }
                            GrblResponse::GrblVersion(ver) => {
                                self.log(format!("Grbl {ver}"));
                                // Auto-detect firmware type (F47)
                                let ver_lower = ver.to_ascii_lowercase();
                                let detected = if ver_lower.contains("fluidnc")
                                    || ver_lower.contains("fluid")
                                {
                                    Some(("FluidNC (GRBL-compatible)", ControllerKind::Grbl))
                                } else if ver_lower.contains("grbl") {
                                    Some(("GRBL", ControllerKind::Grbl))
                                } else {
                                    None
                                };
                                if let Some((name, kind)) = detected {
                                    if self.machine_profile.controller_kind != kind {
                                        let prev = self.machine_profile.controller_kind;
                                        self.machine_profile.controller_kind = kind;
                                        self.apply_controller_kind_change(prev);
                                        self.log(format!("Auto-detected firmware: {name}"));
                                    }
                                }
                            }
                            GrblResponse::Setting(id, val) => {
                                // Auto-sync workspace size from $130/$131
                                if let Ok(v) = val.parse::<f32>() {
                                    match id {
                                        130 => {
                                            self.renderer.workspace_size.x = v;
                                            self.machine_profile.workspace_x_mm = v;
                                        }
                                        131 => {
                                            self.renderer.workspace_size.y = v;
                                            self.machine_profile.workspace_y_mm = v;
                                        }
                                        _ => {}
                                    }
                                }
                                if let Some(state) = &mut self.settings_state {
                                    if state.is_open {
                                        state.settings.insert(id, val);
                                    }
                                }
                            }
                            GrblResponse::Message(_) => {}
                        },
                        ControllerResponse::Message => {}
                    }
                }
                SerialMsg::Connected(port) => {
                    self.grbl_state.status = MacStatus::Idle;
                    self.log(format!("Connected to {port}"));
                    // Auto-detect firmware (F47): send $I for GRBL identification
                    if let Some(conn) = self.connection.as_ref() {
                        conn.send("$I");
                    }
                }
                SerialMsg::Disconnected(reason) => {
                    if self.running {
                        self.handle_program_failed(format!("Disconnected: {reason}"));
                    }
                    self.connection = None;
                    self.grbl_state = GrblState::default();
                    self.running = false;
                    self.is_dry_run = false;
                    self.framing_active = false;
                    self.log(format!("Disconnected: {reason}"));
                }
                SerialMsg::Error(err) => {
                    self.log(format!("Serial error: {err}"));
                }
            }
        }
    }

    fn send_next_program_line(&mut self) {
        while self.program_index < self.program_lines.len() {
            let line_idx = self.program_index;
            self.program_index += 1;

            let mut cmd = if let (Some(file), Some(center)) =
                (&self.loaded_file, self.job_transform.center)
            {
                if let Some(parsed) = file.lines.get(line_idx) {
                    // Standard transform (offset/rotate)
                    let transformed = parsed.transform(
                        egui::vec2(self.job_transform.offset_x, self.job_transform.offset_y),
                        self.job_transform.rotation,
                        center,
                        1.0,
                    );

                    // Apply Rotary transformation if enabled
                    if self.machine_profile.rotary_enabled {
                        crate::gcode::transform::apply_rotary(
                            &transformed,
                            self.machine_profile.rotary_diameter_mm,
                            self.machine_profile.rotary_axis,
                        )
                    } else {
                        transformed
                    }
                } else {
                    self.program_lines[line_idx].clone()
                }
            } else {
                self.program_lines[line_idx].clone()
            };

            // Dry Run: Replace M3/M4 with M5
            if self.is_dry_run {
                if cmd.contains("M3") || cmd.contains("M4") {
                    cmd = cmd.replace("M3", "M5").replace("M4", "M5");
                }
                // Strip S parameter if present to be safe, though M5 ignores it
                // Simple string replacement might be brittle, but sufficient for now.
            }

            let trimmed = cmd.trim().to_string();
            if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('(') {
                continue;
            }
            if let Some(conn) = self.connection.as_ref() {
                conn.send(&trimmed);
            }
            return;
        }
    }

    fn connect(&mut self) {
        let port = match self.ports.get(self.selected_port) {
            Some(p) => p.clone(),
            None => {
                self.show_error("No port selected".to_string());
                return;
            }
        };
        let baud = ui::connection::get_baud(&self.baud_rates, self.selected_baud);
        self.machine_profile.preferred_use_tcp = false;
        self.machine_profile.preferred_port = port.clone();
        self.machine_profile.preferred_baud = baud;
        self.machine_profile.preferred_output_protocol = self.settings.output_protocol;
        if let Some(active) = self
            .profile_store
            .profiles
            .get_mut(self.profile_store.active_index)
        {
            active.preferred_use_tcp = self.machine_profile.preferred_use_tcp;
            active.preferred_port = self.machine_profile.preferred_port.clone();
            active.preferred_baud = self.machine_profile.preferred_baud;
            active.preferred_output_protocol = self.machine_profile.preferred_output_protocol;
        }
        self.profile_store.save();
        self.log(format!("Connecting to {port} @ {baud}…"));
        self.grbl_state.status = MacStatus::Connecting;

        match SerialConnection::connect(&port, baud, self.controller_backend.clone()) {
            Ok(conn) => {
                self.connection = Some(conn);
            }
            Err(e) => {
                self.grbl_state.status = MacStatus::Disconnected;
                self.show_error(format!("Connection failed: {e}"));
            }
        }
    }

    fn connect_tcp(&mut self) {
        let host = self.tcp_host.trim().to_string();
        if host.is_empty() {
            self.show_error("TCP host is empty".to_string());
            return;
        }
        let port: u16 = self.tcp_port_str.trim().parse().unwrap_or(23);
        self.machine_profile.preferred_use_tcp = true;
        self.machine_profile.preferred_tcp_host = host.clone();
        self.machine_profile.preferred_tcp_port = port;
        self.machine_profile.preferred_output_protocol = self.settings.output_protocol;
        if let Some(active) = self
            .profile_store
            .profiles
            .get_mut(self.profile_store.active_index)
        {
            active.preferred_use_tcp = self.machine_profile.preferred_use_tcp;
            active.preferred_tcp_host = self.machine_profile.preferred_tcp_host.clone();
            active.preferred_tcp_port = self.machine_profile.preferred_tcp_port;
            active.preferred_output_protocol = self.machine_profile.preferred_output_protocol;
        }
        self.profile_store.save();
        self.log(format!("Connecting via TCP to {host}:{port}…"));
        self.grbl_state.status = MacStatus::Connecting;

        match SerialConnection::connect_tcp(&host, port, self.controller_backend.clone()) {
            Ok(conn) => {
                self.connection = Some(conn);
            }
            Err(e) => {
                self.grbl_state.status = MacStatus::Disconnected;
                self.show_error(format!("TCP connection failed: {e}"));
            }
        }
    }

    fn disconnect(&mut self) {
        if let Some(conn) = self.connection.take() {
            conn.disconnect();
        }
        self.grbl_state = GrblState::default();
        self.running = false;
        self.is_dry_run = false;
        self.framing_active = false;
        self.log("Disconnected".to_string());
    }

    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("GCode", &["nc", "gcode", "ngc", "gc"])
            .add_filter("SVG", &["svg"])
            .add_filter("Images", &["png", "jpg", "jpeg", "bmp"])
            .add_filter("LightBurn Project", &["lbrn", "lbrn2"])
            .add_filter("DXF", &["dxf"])
            .add_filter("Ruida RD", &["rd"])
            .add_filter("HPGL / PLT", &["plt", "hpgl"])
            .add_filter("PDF", &["pdf"])
            .add_filter("Adobe Illustrator", &["ai"])
            .add_filter("xTool Creative Space", &["xcs"])
            .add_filter("All files", &["*"])
            .pick_file()
        {
            let path_str = path.to_string_lossy().to_string();
            self.load_file_path(&path_str);
        }
    }

    fn load_file_path(&mut self, path: &str) {
        let path_obj = std::path::Path::new(path);
        let ext = path_obj
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        let filename = path_obj
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(path)
            .to_string();

        // Push to recent files immediately (before any failure)
        self.recent_files.push(path);

        match ext.as_str() {
            "svg" => {
                let data = match std::fs::read(path) {
                    Ok(d) => d,
                    Err(e) => {
                        self.show_error(format!("Error reading SVG: {e}"));
                        return;
                    }
                };
                let layers = imaging::svg::extract_layers(&data);
                let mut svg_params = imaging::svg::SvgParams::default();
                svg_params.layers = layers;
                let mut raster_params = imaging::raster::RasterParams::default();
                raster_params.spot_interp_enabled = true;
                raster_params.spot_size_min_mm = self.machine_profile.spot_size_min_mm;
                raster_params.spot_size_max_mm = self.machine_profile.spot_size_max_mm;
                self.import_state = Some(ui::image_dialog::ImageImportState {
                    import_type: ui::image_dialog::ImportType::Svg(data),
                    filename,
                    raster_params,
                    svg_params,
                    materials: ui::materials::MaterialsState::default(),
                    texture: None,
                    needs_texture_update: true,
                    vectorize: false,
                });
            }
            "png" | "jpg" | "jpeg" | "bmp" => {
                let img = match image::open(path) {
                    Ok(i) => i.to_rgba8(), // we need rgba for texture
                    Err(e) => {
                        self.show_error(format!("Error opening image: {e}"));
                        return;
                    }
                };

                let mut raster_params = imaging::raster::RasterParams::default();
                raster_params.spot_interp_enabled = true;
                raster_params.spot_size_min_mm = self.machine_profile.spot_size_min_mm;
                raster_params.spot_size_max_mm = self.machine_profile.spot_size_max_mm;
                self.import_state = Some(ui::image_dialog::ImageImportState {
                    import_type: ui::image_dialog::ImportType::Raster(
                        image::DynamicImage::ImageRgba8(img),
                    ),
                    filename,
                    raster_params,
                    svg_params: imaging::svg::SvgParams::default(),
                    materials: ui::materials::MaterialsState::default(),
                    texture: None, // Will be loaded in update()
                    needs_texture_update: true,
                    vectorize: false,
                });
            }
            "dxf" => {
                let data = match std::fs::read_to_string(path) {
                    Ok(d) => d,
                    Err(e) => {
                        self.show_error(format!("Error reading DXF: {e}"));
                        return;
                    }
                };
                let params = crate::imaging::dxf::DxfParams::default();
                match crate::imaging::dxf::dxf_to_gcode(&data, &params) {
                    Ok(lines) => {
                        let file = GCodeFile::from_lines(&filename, &lines);
                        self.set_loaded_file(file, lines);
                        self.log(format!("DXF imported: {filename}"));
                    }
                    Err(e) => self.show_error(format!("DXF import failed: {e}")),
                }
            }
            "lbrn" | "lbrn2" => {
                let path_owned = path.to_string();
                let fname = filename.clone();
                let (tx, rx) = crossbeam_channel::bounded(1);
                self.lbrn_import_receiver = Some(rx);
                self.lbrn_loading_msg = Some(format!("Importing {filename}…"));
                self.log(format!("Loading LightBurn file: {filename}…"));
                std::thread::spawn(move || {
                    let data = match std::fs::read_to_string(&path_owned) {
                        Ok(d) => d,
                        Err(e) => {
                            let _ = tx.send(Err(format!("Error reading LightBurn file: {e}")));
                            return;
                        }
                    };
                    match crate::gcode::lbrn_import::import_lbrn2(&data) {
                        Ok((shapes, layer_overrides)) => {
                            let _ = tx.send(Ok((shapes, layer_overrides, fname)));
                        }
                        Err(e) => {
                            let _ = tx.send(Err(format!("LightBurn import failed: {e}")));
                        }
                    }
                });
            }
            "pdf" | "ai" => {
                let data = match std::fs::read(path) {
                    Ok(d) => d,
                    Err(e) => {
                        self.show_error(format!("Error reading file: {e}"));
                        return;
                    }
                };
                self.import_shapes_or_error(
                    crate::imaging::pdf::parse_pdf(&data, 0),
                    "PDF/AI",
                    &filename,
                );
            }
            "xcs" => {
                self.import_text_shapes(path, &filename, "XCS", |data| {
                    crate::gcode::xcs_import::import_xcs(data)
                });
            }
            "rd" => {
                let data = match std::fs::read(path) {
                    Ok(d) => d,
                    Err(e) => {
                        self.show_error(format!("Error reading RD file: {e}"));
                        return;
                    }
                };
                self.import_shapes_or_error(
                    crate::imaging::ruida_rd::import_rd(&data),
                    "Ruida RD",
                    &filename,
                );
            }
            "plt" | "hpgl" => {
                self.import_text_shapes(path, &filename, "HPGL", |data| {
                    crate::imaging::hpgl::parse_hpgl(data, 0)
                });
            }
            _ => {
                // Default to GCode loading
                match GCodeFile::load(path) {
                    Ok(file) => {
                        let lines = file.lines.iter().map(|l| l.raw.clone()).collect();
                        self.set_loaded_file(file, lines);
                    }
                    Err(e) => {
                        self.show_error(format!("Failed to load file: {e}"));
                    }
                }
            }
        }
    }

    /// Helper: read a text file, parse it into shapes, and add to drawing.
    fn import_text_shapes(
        &mut self,
        path: &str,
        filename: &str,
        format_label: &str,
        parser: impl FnOnce(&str) -> Result<Vec<ShapeParams>, String>,
    ) {
        let data = match std::fs::read_to_string(path) {
            Ok(d) => d,
            Err(e) => {
                self.show_error(format!("Error reading {format_label} file: {e}"));
                return;
            }
        };
        self.import_shapes_or_error(parser(&data), format_label, filename);
    }

    /// Helper: given a parse result, extend shapes or show error.
    fn import_shapes_or_error(
        &mut self,
        result: Result<Vec<ShapeParams>, String>,
        format_label: &str,
        filename: &str,
    ) {
        match result {
            Ok(shapes) => {
                self.push_node_undo_snapshot();
                self.drawing_state.shapes.extend(shapes);
                self.regenerate_drawing_gcode();
                self.log(format!("{format_label} imported: {filename}"));
            }
            Err(e) => self.show_error(format!("{format_label} import failed: {e}")),
        }
    }

    fn set_loaded_file(&mut self, file: GCodeFile, lines: Vec<String>) {
        self.log(format!(
            "Loaded {} ({} lines)",
            file.filename,
            file.line_count()
        ));
        // Populate GCode editor text
        self.gcode_editor.text = lines.join("\n");
        self.gcode_editor.dirty = false;
        self.program_lines = std::sync::Arc::new(lines);
        self.program_index = 0;
        // Calculate job center for rotation
        if let Some((min_x, min_y, max_x, max_y)) = file.bounds() {
            self.job_transform.center = Some(egui::Pos2::new(
                (min_x + max_x) / 2.0,
                (min_y + max_y) / 2.0,
            ));
        } else {
            self.job_transform.center = Some(egui::Pos2::ZERO);
        }

        self.loaded_file = Some(file);
        if let Some(file) = self.loaded_file.as_ref() {
            self.estimation = crate::gcode::estimation::estimate(&file.lines);
        }
    }

    fn regenerate_drawing_gcode(&mut self) {
        let lines = crate::ui::drawing::generate_all_gcode_with_protocol(
            &self.drawing_state,
            &self.layers,
            self.settings.output_protocol,
        );
        let file = GCodeFile::from_lines("drawing", &lines);
        self.set_loaded_file(file, lines);
    }

    fn preview_used_layer_indices(&self) -> Vec<usize> {
        let mut used = Vec::new();

        for shape in &self.drawing_state.shapes {
            if shape.layer_idx < self.layers.len() {
                used.push(shape.layer_idx);
            }
        }

        if used.is_empty() {
            if let Some(file) = &self.loaded_file {
                for seg in &file.segments {
                    if seg.layer_id < self.layers.len() {
                        used.push(seg.layer_id);
                    }
                }
            }
        }

        used.sort_unstable();
        used.dedup();

        if used.is_empty() && self.active_layer_idx < self.layers.len() {
            used.push(self.active_layer_idx);
        }

        used
    }

    fn assign_selected_shapes_to_layer(&mut self, layer_idx: usize) -> usize {
        if layer_idx >= self.layers.len() {
            return 0;
        }

        let selection: Vec<usize> = self.renderer.selected_shape_idx.iter().copied().collect();
        if selection.is_empty() {
            return 0;
        }

        let mut changed = 0usize;
        for shape_idx in selection {
            if let Some(shape) = self.drawing_state.shapes.get_mut(shape_idx) {
                if shape.layer_idx != layer_idx {
                    shape.layer_idx = layer_idx;
                    changed += 1;
                }
            }
        }

        if changed > 0 {
            self.regenerate_drawing_gcode();
        }

        changed
    }

    fn selected_shape_indices(&self) -> Vec<usize> {
        self.renderer
            .selected_shape_idx
            .iter()
            .copied()
            .filter(|idx| *idx < self.drawing_state.shapes.len())
            .collect()
    }

    fn copy_selected_shapes_to_clipboard(&mut self) -> usize {
        let selection = self.selected_shape_indices();
        if selection.is_empty() {
            return 0;
        }

        let mut copied = Vec::new();
        for idx in selection {
            if let Some(shape) = self.drawing_state.shapes.get(idx) {
                copied.push(shape.clone());
            }
        }

        if copied.is_empty() {
            return 0;
        }

        self.clipboard_shapes = copied;
        self.clipboard_paste_serial = 0;
        self.clipboard_shapes.len()
    }

    fn delete_selected_shapes(&mut self) -> usize {
        let mut selection = self.selected_shape_indices();
        if selection.is_empty() {
            return 0;
        }

        selection.sort_by(|a, b| b.cmp(a));
        let removed = selection.len();
        for idx in selection {
            self.drawing_state.shapes.remove(idx);
        }

        self.renderer.selected_shape_idx.clear();
        self.renderer.selected_node = None;
        self.renderer.selected_nodes.clear();
        self.regenerate_drawing_gcode();
        removed
    }

    fn paste_shapes_from_clipboard(&mut self) -> usize {
        if self.clipboard_shapes.is_empty() {
            return 0;
        }

        self.clipboard_paste_serial = self.clipboard_paste_serial.saturating_add(1);
        let offset = 5.0 * self.clipboard_paste_serial as f32;

        self.renderer.selected_shape_idx.clear();
        self.renderer.selected_node = None;
        self.renderer.selected_nodes.clear();

        let mut pasted = 0usize;
        for mut shape in self.clipboard_shapes.clone() {
            shape.x += offset;
            shape.y += offset;
            self.drawing_state.shapes.push(shape);
            let new_idx = self.drawing_state.shapes.len().saturating_sub(1);
            self.renderer.selected_shape_idx.insert(new_idx);
            pasted += 1;
        }

        if let Some(last) = self.renderer.selected_shape_idx.iter().last().copied() {
            if let Some(shape) = self.drawing_state.shapes.get(last) {
                self.drawing_state.current = shape.clone();
            }
        }

        if pasted > 0 {
            self.regenerate_drawing_gcode();
        }
        pasted
    }

    fn capture_node_snapshot(&self) -> NodeEditSnapshot {
        NodeEditSnapshot {
            shapes: self.drawing_state.shapes.clone(),
            selected_shape_idx: self.renderer.selected_shape_idx.iter().copied().collect(),
            selected_node: self.renderer.selected_node,
            selected_nodes: self.renderer.selected_nodes.iter().copied().collect(),
        }
    }

    fn push_node_undo_snapshot(&mut self) {
        self.node_undo_stack.push_back(self.capture_node_snapshot());
        if self.node_undo_stack.len() > crate::ui::node_edit::MAX_NODE_HISTORY {
            self.node_undo_stack.pop_front();
        }
        self.node_redo_stack.clear();
    }

    fn restore_node_snapshot(&mut self, snap: NodeEditSnapshot) {
        self.drawing_state.shapes = snap.shapes;
        self.renderer.selected_shape_idx.clear();
        for idx in snap.selected_shape_idx {
            self.renderer.selected_shape_idx.insert(idx);
        }
        self.renderer.selected_node = snap.selected_node;
        self.renderer.selected_nodes.clear();
        for key in snap.selected_nodes {
            self.renderer.selected_nodes.insert(key);
        }
        self.regenerate_drawing_gcode();
    }

    fn undo_node_edit(&mut self) -> bool {
        let current = self.capture_node_snapshot();
        let Some(prev) = undo_history_step(
            &mut self.node_undo_stack,
            &mut self.node_redo_stack,
            current,
        ) else {
            return false;
        };
        self.restore_node_snapshot(prev);
        true
    }

    fn redo_node_edit(&mut self) -> bool {
        let current = self.capture_node_snapshot();
        let Some(next) = redo_history_step(
            &mut self.node_undo_stack,
            &mut self.node_redo_stack,
            current,
        ) else {
            return false;
        };
        self.restore_node_snapshot(next);
        true
    }

    fn load_camera_snapshot_from_path(
        &mut self,
        ctx: &egui::Context,
        path: &str,
    ) -> Result<(), String> {
        match image::open(path) {
            Ok(img) => {
                let rgba = img.to_rgba8();
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [rgba.width() as usize, rgba.height() as usize],
                    rgba.as_flat_samples().as_slice(),
                );
                let texture = ctx.load_texture(
                    format!("camera_snapshot_{path}"),
                    color_image,
                    Default::default(),
                );
                self.camera_state.texture = Some(texture);
                self.camera_state.snapshot_path = Some(path.to_string());
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }

    fn load_camera_snapshot(&mut self, ctx: &egui::Context) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Image", &["png", "jpg", "jpeg", "bmp", "webp"])
            .pick_file()
        else {
            return;
        };

        self.stop_live_camera();
        let path_str = path.to_string_lossy().to_string();
        match self.load_camera_snapshot_from_path(ctx, &path_str) {
            Ok(()) => {
                self.camera_state.enabled = true;
                self.log(format!("Camera snapshot loaded: {}", path.display()));
            }
            Err(e) => self.show_error(format!("Failed to open camera snapshot: {e}")),
        }
    }

    fn start_live_camera(&mut self) {
        self.stop_live_camera();

        let device_index = self.camera_state.device_index.max(0) as u32;
        match crate::camera_stream::CameraStream::start(device_index) {
            Ok(stream) => {
                self.camera_live.stream = Some(stream);
                self.camera_state.live_streaming = true;
                self.camera_state.snapshot_path = None;
                self.camera_state.texture = None;
                self.camera_state.enabled = true;
                self.log(format!("Live camera started (device {device_index})."));
            }
            Err(e) => {
                self.camera_state.live_streaming = false;
                self.show_error(format!("Unable to start live camera: {e}"));
            }
        }
    }

    fn stop_live_camera(&mut self) {
        if let Some(mut stream) = self.camera_live.stream.take() {
            stream.stop();
        }
        self.camera_state.live_streaming = false;
    }

    fn update_camera_texture_from_frame(
        &mut self,
        ctx: &egui::Context,
        frame: crate::camera_stream::CameraFrame,
    ) {
        self.camera_live.last_frame_width = frame.width;
        self.camera_live.last_frame_height = frame.height;
        self.camera_live.last_frame_rgba = frame.rgba.clone();
        let color_image =
            egui::ColorImage::from_rgba_unmultiplied([frame.width, frame.height], &frame.rgba);
        if let Some(texture) = self.camera_state.texture.as_mut() {
            texture.set(color_image, Default::default());
        } else {
            self.camera_state.texture =
                Some(ctx.load_texture("camera_live_stream", color_image, Default::default()));
        }
        self.camera_state.latest_rgba = Some((frame.width, frame.height, frame.rgba.clone()));
    }

    fn poll_camera_stream(&mut self, ctx: &egui::Context) {
        let mut latest_frame = None;
        let mut latest_error = None;

        if let Some(stream) = self.camera_live.stream.as_ref() {
            latest_frame = stream.try_recv_latest_frame();
            latest_error = stream.try_recv_error();
        }

        if let Some(err) = latest_error {
            self.stop_live_camera();
            self.show_error(err);
        }

        if let Some(frame) = latest_frame {
            self.update_camera_texture_from_frame(ctx, frame);
            self.camera_state.enabled = true;
            ctx.request_repaint();
        }
    }

    fn start_camera_calibration_wizard(&mut self) {
        if self.camera_state.texture.is_none() {
            self.show_error("Start live camera (or load image) before calibration wizard.".into());
            return;
        }
        self.camera_live.point_align_picks.clear();
        self.camera_state.point_align_active = false;
        self.camera_state.point_align_pick_count = 0;

        self.camera_live.calibration_picks.clear();
        self.camera_state.calibration_wizard_active = true;
        self.camera_state.calibration_pick_count = 0;
        self.log("Calibration wizard started: pick origin, +X, +Y on camera overlay.".into());
    }

    fn stop_camera_calibration_wizard(&mut self) {
        self.camera_live.calibration_picks.clear();
        self.camera_state.calibration_wizard_active = false;
        self.camera_state.calibration_pick_count = 0;
    }

    fn start_camera_point_align(&mut self) {
        if self.camera_state.texture.is_none() {
            self.show_error("Start live camera (or load image) before point align.".into());
            return;
        }
        if self.loaded_file.as_ref().and_then(|f| f.bounds()).is_none() {
            self.show_error("Load a job before 2-point align.".into());
            return;
        }
        self.camera_live.calibration_picks.clear();
        self.camera_state.calibration_wizard_active = false;
        self.camera_state.calibration_pick_count = 0;

        self.camera_live.point_align_picks.clear();
        self.camera_state.point_align_active = true;
        self.camera_state.point_align_pick_count = 0;
        self.log(
            "2-point align started: pick bottom-left then bottom-right target on camera overlay."
                .into(),
        );
    }

    fn stop_camera_point_align(&mut self) {
        self.camera_live.point_align_picks.clear();
        self.camera_state.point_align_active = false;
        self.camera_state.point_align_pick_count = 0;
    }

    fn apply_calibration_from_picks(&mut self) -> bool {
        if self.camera_live.calibration_picks.len() < 3 {
            return false;
        }

        let wsx = self.renderer.workspace_size.x.max(1.0);
        let wsy = self.renderer.workspace_size.y.max(1.0);
        let p0 = self.camera_live.calibration_picks[0];
        let px = self.camera_live.calibration_picks[1];
        let py = self.camera_live.calibration_picks[2];

        let vx = px - p0;
        let vy = py - p0;
        let len_x = (vx.x * vx.x + vx.y * vx.y).sqrt();
        let len_y = (vy.x * vy.x + vy.y * vy.y).sqrt();
        if len_x <= 1e-3 || len_y <= 1e-3 {
            self.show_error("Calibration picks are too close. Retry calibration wizard.".into());
            return false;
        }

        let scale_x = len_x / wsx;
        let scale_y = len_y / wsy;
        let scale = ((scale_x + scale_y) * 0.5).max(0.01);

        let rot_x = vx.y.atan2(vx.x).to_degrees();
        let rot_y = vy.y.atan2(vy.x).to_degrees() - 90.0;
        let (sx, cx) = rot_x.to_radians().sin_cos();
        let (sy, cy) = rot_y.to_radians().sin_cos();
        let rotation = (sx + sy).atan2(cx + cy).to_degrees();

        let w = wsx * scale;
        let h = wsy * scale;
        let (sin_a, cos_a) = rotation.to_radians().sin_cos();
        let rx = (-w * 0.5) * cos_a - (-h * 0.5) * sin_a;
        let ry = (-w * 0.5) * sin_a + (-h * 0.5) * cos_a;
        let delta_x = w * 0.5 + rx;
        let delta_y = h * 0.5 + ry;

        self.camera_state.calibration.scale = scale;
        self.camera_state.calibration.rotation = rotation;
        self.camera_state.calibration.offset_x = p0.x - delta_x;
        self.camera_state.calibration.offset_y = p0.y - delta_y;

        self.log(format!(
            "Camera calibration updated (scale {:.4}, rot {:.2}°).",
            scale, rotation
        ));
        true
    }

    fn apply_point_align_from_picks(&mut self) -> bool {
        if self.camera_live.point_align_picks.len() < 2 {
            return false;
        }
        let Some(file) = self.loaded_file.as_ref() else {
            self.show_error("Load a job before point align.".into());
            return false;
        };
        let Some((min_x, min_y, max_x, max_y)) = file.bounds() else {
            self.show_error("Loaded job has no valid bounds for point align.".into());
            return false;
        };

        let src0 = egui::pos2(min_x, min_y);
        let mut src1 = egui::pos2(max_x, min_y);
        if (src1.x - src0.x).abs() <= 1e-3 {
            src1 = egui::pos2(min_x, max_y);
        }
        if (src1.x - src0.x).abs() <= 1e-3 && (src1.y - src0.y).abs() <= 1e-3 {
            self.show_error("Job is degenerate for point align.".into());
            return false;
        }

        let dst0 = self.camera_live.point_align_picks[0];
        let dst1 = self.camera_live.point_align_picks[1];

        let src_angle = (src1.y - src0.y).atan2(src1.x - src0.x);
        let dst_angle = (dst1.y - dst0.y).atan2(dst1.x - dst0.x);
        let rot = (dst_angle - src_angle).to_degrees();

        let center = egui::pos2((min_x + max_x) * 0.5, (min_y + max_y) * 0.5);
        let (sin_a, cos_a) = rot.to_radians().sin_cos();
        let dx = src0.x - center.x;
        let dy = src0.y - center.y;
        let r0 = egui::pos2(
            center.x + dx * cos_a - dy * sin_a,
            center.y + dx * sin_a + dy * cos_a,
        );

        self.job_transform.rotation = rot;
        self.job_transform.offset_x = dst0.x - r0.x;
        self.job_transform.offset_y = dst0.y - r0.y;
        self.needs_auto_fit = true;

        self.log(format!(
            "2-point align applied (offset X={:.2} Y={:.2}, rot={:.2}°).",
            self.job_transform.offset_x, self.job_transform.offset_y, self.job_transform.rotation
        ));
        true
    }

    fn auto_detect_camera_mark(&mut self) {
        if let Some((width, height, rgba)) = self.camera_state.latest_rgba.clone() {
            if let Some(img) = image::RgbaImage::from_raw(width as u32, height as u32, rgba) {
                if let Some((cx, cy)) = crate::imaging::camera_vision::find_alignment_mark(&img) {
                    let calib = &self.camera_state.calibration;
                    let scale = calib.scale.max(0.01);
                    let base_w = self.machine_profile.workspace_x_mm.max(1.0);
                    let base_h = self.machine_profile.workspace_y_mm.max(1.0);
                    let w = base_w * scale;
                    let h = base_h * scale;

                    // Map from [0..1] normalized image coordinate
                    let nx = cx / (width as f32);
                    let ny = cy / (height as f32);

                    let (sin_a, cos_a) = calib.rotation.to_radians().sin_cos();
                    let cx_rot = calib.offset_x + w * 0.5;
                    let cy_rot = calib.offset_y + h * 0.5;

                    let px = calib.offset_x + nx * w;
                    let py = calib.offset_y + ny * h;

                    let dx = px - cx_rot;
                    let dy = py - cy_rot;
                    let wx = cx_rot + dx * cos_a - dy * sin_a;
                    let wy = cy_rot + dx * sin_a + dy * cos_a;

                    self.log(format!("Auto-detected fiducial mark at ({wx:.2}, {wy:.2}) mm."));
                    self.handle_camera_pick_point(egui::Pos2::new(wx, wy));
                } else {
                    self.show_error("No alignment fiducial (dark circle/cross) detected in the current frame.".into());
                }
            } else {
                self.show_error("Failed to parse camera frame.".into());
            }
        } else {
            self.show_error("No live camera frame available to auto-detect.".into());
        }
    }

    fn handle_camera_pick_point(&mut self, point: egui::Pos2) {
        if self.camera_state.calibration_wizard_active {
            self.camera_live.calibration_picks.push(point);
            self.camera_state.calibration_pick_count = self.camera_live.calibration_picks.len();
            if self.camera_live.calibration_picks.len() >= 3 {
                let applied = self.apply_calibration_from_picks();
                self.stop_camera_calibration_wizard();
                if applied {
                    self.sync_settings();
                }
            }
            return;
        }

        if self.camera_state.point_align_active {
            self.camera_live.point_align_picks.push(point);
            self.camera_state.point_align_pick_count = self.camera_live.point_align_picks.len();
            if self.camera_live.point_align_picks.len() >= 2 {
                self.apply_point_align_from_picks();
                self.stop_camera_point_align();
            }
        }
    }

    fn camera_overlay_center_world(&self) -> egui::Pos2 {
        let scale = self.camera_state.calibration.scale.max(0.01);
        egui::Pos2::new(
            self.camera_state.calibration.offset_x + (self.renderer.workspace_size.x * scale) * 0.5,
            self.camera_state.calibration.offset_y + (self.renderer.workspace_size.y * scale) * 0.5,
        )
    }

    fn camera_pixel_to_world(
        &self,
        px: f32,
        py: f32,
        frame_w: usize,
        frame_h: usize,
    ) -> egui::Pos2 {
        let w = frame_w.max(1) as f32;
        let h = frame_h.max(1) as f32;
        let nx = px / w;
        let ny = py / h;

        let wsx = self.renderer.workspace_size.x.max(1.0);
        let wsy = self.renderer.workspace_size.y.max(1.0);
        let scale = self.camera_state.calibration.scale.max(0.01);
        let base_w = wsx * scale;
        let base_h = wsy * scale;

        let raw_x = self.camera_state.calibration.offset_x + nx * base_w;
        let raw_y = self.camera_state.calibration.offset_y + ny * base_h;

        let cx = self.camera_state.calibration.offset_x + base_w * 0.5;
        let cy = self.camera_state.calibration.offset_y + base_h * 0.5;
        let angle = self.camera_state.calibration.rotation.to_radians();
        let (sin_a, cos_a) = angle.sin_cos();
        let dx = raw_x - cx;
        let dy = raw_y - cy;
        egui::Pos2::new(cx + dx * cos_a - dy * sin_a, cy + dx * sin_a + dy * cos_a)
    }

    fn auto_detect_camera_markers(&mut self) {
        let mut rgba: Vec<u8> = Vec::new();
        let mut width = 0usize;
        let mut height = 0usize;
        let mut source = "none";

        if !self.camera_live.last_frame_rgba.is_empty()
            && self.camera_live.last_frame_width > 0
            && self.camera_live.last_frame_height > 0
        {
            rgba = self.camera_live.last_frame_rgba.clone();
            width = self.camera_live.last_frame_width;
            height = self.camera_live.last_frame_height;
            source = "live";
        } else if let Some(path) = self.camera_state.snapshot_path.as_deref() {
            match image::open(path) {
                Ok(img) => {
                    let data = img.to_rgba8();
                    width = data.width() as usize;
                    height = data.height() as usize;
                    rgba = data.into_raw();
                    source = "snapshot";
                }
                Err(e) => {
                    self.camera_state.detection_status =
                        format!("Marker detection failed: cannot read image ({e}).");
                    self.camera_state.detected_cross_world = None;
                    self.camera_state.detected_circle_world = None;
                    self.show_error(self.camera_state.detection_status.clone());
                    return;
                }
            }
        }

        if rgba.is_empty() || width == 0 || height == 0 {
            self.camera_state.detection_status =
                "Marker detection failed: no camera frame available (start live camera or load image)."
                    .to_string();
            self.camera_state.detected_cross_world = None;
            self.camera_state.detected_circle_world = None;
            self.show_error(self.camera_state.detection_status.clone());
            return;
        }

        if let Some((cross_px, circle_px)) = detect_cross_and_circle_markers(&rgba, width, height) {
            let cross_world = self.camera_pixel_to_world(cross_px.0, cross_px.1, width, height);
            let circle_world = self.camera_pixel_to_world(circle_px.0, circle_px.1, width, height);
            self.camera_state.detected_cross_world = Some(cross_world);
            self.camera_state.detected_circle_world = Some(circle_world);
            self.camera_state.auto_detection_success_count = self
                .camera_state
                .auto_detection_success_count
                .saturating_add(1);
            self.camera_state.detection_status = format!(
                "Auto-detect success ({source}): markers mapped to workspace/project coordinates."
            );
            self.log(format!(
                "Auto markers detected: cross ({:.1}, {:.1}) mm, circle ({:.1}, {:.1}) mm.",
                cross_world.x, cross_world.y, circle_world.x, circle_world.y
            ));
        } else {
            self.camera_state.detected_cross_world = None;
            self.camera_state.detected_circle_world = None;
            self.camera_state.detection_status =
                "Auto-detect failed: marker shapes not recognized. Use Calibration Wizard or 2-Point Align manually."
                    .to_string();
            self.log(
                "Auto marker detection failed. Manual correction workflow remains available."
                    .into(),
            );
        }
    }

    fn apply_detected_marker_align(&mut self) {
        let (Some(cross), Some(circle)) = (
            self.camera_state.detected_cross_world,
            self.camera_state.detected_circle_world,
        ) else {
            self.show_error("No detected markers to apply. Run auto-detect first.".into());
            return;
        };

        self.camera_live.point_align_picks.clear();
        self.camera_live.point_align_picks.push(cross);
        self.camera_live.point_align_picks.push(circle);
        if self.apply_point_align_from_picks() {
            self.stop_camera_point_align();
            self.sync_settings();
            self.log("Detected marker alignment applied via 2-point mapping.".into());
        }
    }

    fn align_job_to_camera(&mut self) {
        let Some(file) = self.loaded_file.as_ref() else {
            self.show_error("Load a job before Align Job to Camera.".into());
            return;
        };
        let Some((min_x, min_y, max_x, max_y)) = file.bounds() else {
            self.show_error("Loaded job has no valid bounds.".into());
            return;
        };

        let job_center = egui::Pos2::new((min_x + max_x) * 0.5, (min_y + max_y) * 0.5);
        let target = self.camera_overlay_center_world();
        self.job_transform.offset_x = target.x - job_center.x;
        self.job_transform.offset_y = target.y - job_center.y;
        self.job_transform.rotation = self.camera_state.calibration.rotation;
        self.needs_auto_fit = true;
        self.sync_settings();

        self.log(format!(
            "Job aligned to camera overlay (offset X={:.2} Y={:.2}, rot={:.2}°).",
            self.job_transform.offset_x, self.job_transform.offset_y, self.job_transform.rotation
        ));
    }

    fn world_to_shape_local(
        shape: &crate::ui::drawing::ShapeParams,
        wx: f32,
        wy: f32,
    ) -> (f32, f32) {
        let dx = wx - shape.x;
        let dy = wy - shape.y;
        let angle = -shape.rotation.to_radians();
        let lx = dx * angle.cos() - dy * angle.sin();
        let ly = dx * angle.sin() + dy * angle.cos();
        (lx, ly)
    }

    fn save_file(&mut self) {
        if self.program_lines.is_empty() {
            self.show_error("No program to save.".into());
            return;
        }

        if let Some(path) = rfd::FileDialog::new()
            .set_file_name("export.nc")
            .add_filter("GCode", &["nc", "gcode"])
            .save_file()
        {
            let data = self.program_lines.join("\n");
            match std::fs::write(&path, data) {
                Ok(_) => self.log(format!("Saved to {}", path.display())),
                Err(e) => self.show_error(format!("Failed to save: {e}")),
            }
        }
    }

    fn frame_bbox(&mut self) {
        if !self.is_connected() {
            self.show_error("Not connected.".into());
            return;
        }

        if self.framing_active {
            // Stop framing
            self.framing_active = false;
            self.framing_wait_idle = false;
            self.send_command("M5");
            self.send_command("G0 X0 Y0");
            self.log("Framing stopped.".into());
        } else {
            // Start framing
            if self.loaded_file.as_ref().and_then(|f| f.bounds()).is_some() {
                self.framing_active = true;
                self.framing_wait_idle = false;
                self.log("Continuous Framing Started...".into());
                self.send_frame_sequence();
            } else {
                self.show_error("Loaded file has no bounds.".into());
            }
        }
    }

    fn send_frame_sequence(&mut self) {
        if let Some(file) = &self.loaded_file {
            if let Some((min_x, min_y, max_x, max_y)) = file.bounds() {
                let feed = 3000.0;

                let commands = vec![
                    format!("M5"),
                    format!("G0 X{:.2} Y{:.2} F{feed}", min_x, min_y),
                    format!("M3 S10"),
                    format!("G1 X{:.2} Y{:.2} F{feed}", max_x, min_y),
                    format!("G1 X{:.2} Y{:.2} F{feed}", max_x, max_y),
                    format!("G1 X{:.2} Y{:.2} F{feed}", min_x, max_y),
                    format!("G1 X{:.2} Y{:.2} F{feed}", min_x, min_y),
                    format!("M5"),
                ];

                for cmd in commands {
                    if let Some(conn) = self.connection.as_ref() {
                        conn.send(&cmd);
                    }
                }
            }
        }
    }

    fn quick_move_to(&mut self, pos: ui::machine_state::QuickPosition) {
        if !self.is_connected() {
            self.show_error("Not connected.".into());
            return;
        }
        if let Some(file) = &self.loaded_file {
            if let Some((min_x, min_y, max_x, max_y)) = file.bounds() {
                let (x, y) = match pos {
                    ui::machine_state::QuickPosition::Center => {
                        ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0)
                    }
                    ui::machine_state::QuickPosition::TopLeft => (min_x, max_y),
                    ui::machine_state::QuickPosition::TopRight => (max_x, max_y),
                    ui::machine_state::QuickPosition::BottomLeft => (min_x, min_y),
                    ui::machine_state::QuickPosition::BottomRight => (max_x, min_y),
                };
                self.log(format!("Moving to {:?}", pos));

                // Construct standard G0 move
                let feed = 3000.0;
                let cmd = format!("G90 G0 X{:.2} Y{:.2} F{feed}", x, y);
                self.send_command(&cmd);
            } else {
                self.show_error("Loaded file has no bounds.".into());
            }
        } else {
            self.show_error("No file loaded.".into());
        }
    }

    fn run_program_with_preflight(&mut self) {
        if !self.is_connected() {
            self.show_error("Not connected".to_string());
            return;
        }
        if self.program_lines.is_empty() {
            self.show_error("No file loaded".to_string());
            return;
        }
        let issues = ui::preflight::run_checks(&self.drawing_state.shapes, &self.layers);
        if issues.is_empty() || self.preflight_state.bypass {
            self.preflight_state.bypass = false; // Ensure it doesn't stay permanently bypassed
            self.run_program_internal();
        } else {
            self.preflight_state.issues = issues;
            self.preflight_state.is_open = true;
        }
    }

    fn run_program_internal(&mut self) {
        self.program_index = 0;
        self.running = true;
        self.notify_job_done = false;
        self.framing_active = false;

        if !self.machine_profile.pre_job_hook.trim().is_empty() {
            let hook = self.machine_profile.pre_job_hook.clone();
            self.send_hook_script("pre-job", &hook);
        }

        // Air assist ON
        if self.machine_profile.air_assist {
            self.send_command("M8");
        }

        // Append return-to-origin if configured
        if self.machine_profile.return_to_origin {
            if self.program_lines.last().map(|l| l.trim()) != Some("G0 X0 Y0") {
                std::sync::Arc::make_mut(&mut self.program_lines).push("G0 X0 Y0 F3000".to_string());
            }
        }

        self.log(if self.is_dry_run {
            "Starting Dry Run (Laser OFF)…".to_string()
        } else {
            "Starting program…".to_string()
        });
        self.send_next_program_line();
    }

    fn build_preflight_report(&self) -> PreflightReport {
        let ctx = PreflightContext {
            shapes: &self.drawing_state.shapes,
            layers: &self.layers,
            loaded_file: self.loaded_file.as_ref(),
            program_lines: &self.program_lines,
            machine_profile: &self.machine_profile,
        };
        crate::ui::preflight::build_preflight_report(&ctx)
    }

    fn run_preflight(&mut self, source: &str, block_if_critical: bool) -> bool {
        let report = self.build_preflight_report();
        let critical = report.critical_count();
        let warning = report.warning_count();
        self.preflight_report = Some(report);
        self.log(format!(
            "Preflight ({source}): {critical} critical, {warning} warning."
        ));

        if block_if_critical && self.preflight_block_critical && critical > 0 {
            self.show_error(format!(
                "Preflight blocked launch: {critical} critical issue(s)."
            ));
            return false;
        }
        true
    }

    fn enqueue_current_job(&mut self) -> Option<u64> {
        if self.program_lines.is_empty() {
            self.show_error("No loaded program to enqueue.".into());
            return None;
        }

        let base_name = self
            .loaded_file
            .as_ref()
            .map(|f| f.filename.clone())
            .unwrap_or_else(|| "current_job".to_string());
        let id = self
            .job_queue_state
            .enqueue_job(base_name.clone(), self.program_lines.clone());
        self.log(format!("Queued job #{id}: {base_name}"));
        Some(id)
    }

    fn try_start_next_queued_job(&mut self) {
        if self.running {
            return;
        }
        if !self.is_connected() {
            self.show_error("Connect machine before starting queued jobs.".into());
            return;
        }

        let Some(job) = self.job_queue_state.pop_next_job() else {
            return;
        };

        let file = GCodeFile::from_lines(&job.name, &job.lines);
        self.set_loaded_file(file, job.lines.to_vec());
        if !self.run_preflight("queue", true) {
            self.job_queue_state
                .record_failure(job.clone(), "Preflight blocked launch".into());
            if self.job_queue_state.auto_run_next {
                self.try_start_next_queued_job();
            }
            return;
        }
        self.active_queue_job = Some(job.clone());
        self.run_program_internal(); // Assume enqueued jobs are preflighted
        self.log(format!("Queue start: #{} {}", job.id, job.name));
    }

    fn handle_program_completed(&mut self) {
        self.running = false;
        self.is_dry_run = false;
        if self.machine_profile.air_assist {
            self.send_command("M9");
        }
        // Track tube wear (F97)
        let burn_secs = self.estimation.total_burn_mm as f64
            / (self.machine_profile.max_rate_x.max(1.0) as f64 / 60.0);
        self.machine_profile.record_job_burn_time(burn_secs);
        if let Some(p) = self
            .profile_store
            .profiles
            .get_mut(self.profile_store.active_index)
        {
            p.tube_hours_total = self.machine_profile.tube_hours_total;
        }
        self.profile_store.save();
        let wear = self.machine_profile.tube_wear_pct();
        if wear > 80.0 {
            self.log(format!("⚠ Tube wear: {wear:.1}% — consider replacement."));
        }
        // Maintenance tracking (F27)
        self.machine_profile.record_job_completed();
        if let Some(p) = self
            .profile_store
            .profiles
            .get_mut(self.profile_store.active_index)
        {
            p.total_jobs_completed = self.machine_profile.total_jobs_completed;
            p.maintenance_jobs_since_lens_clean =
                self.machine_profile.maintenance_jobs_since_lens_clean;
            p.maintenance_jobs_since_belt_check =
                self.machine_profile.maintenance_jobs_since_belt_check;
        }
        for alert in self.machine_profile.maintenance_alerts() {
            self.console_log.push_back(alert);
        }
        // Job report CSV (F15)
        let report_csv = crate::gcode::estimation::generate_job_report_csv(
            self.loaded_file
                .as_ref()
                .map(|f| f.filename.as_str())
                .unwrap_or("unknown"),
            &self.estimation,
            &self.layers,
            &self.machine_profile.name,
            self.program_lines.len(),
        );
        let report_path = std::env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("last_job_report.csv");
        let _ = std::fs::write(&report_path, &report_csv);

        self.log("Program complete.".to_string());
        self.notify_job_done = true;

        if !self.machine_profile.post_job_hook.trim().is_empty() {
            let hook = self.machine_profile.post_job_hook.clone();
            self.send_hook_script("post-job", &hook);
        }

        if let Some(job) = self.active_queue_job.take() {
            self.job_queue_state.record_completion(job);
            if self.job_queue_state.auto_run_next {
                self.try_start_next_queued_job();
            }
        }
    }

    fn handle_program_failed(&mut self, reason: String) {
        self.running = false;
        self.is_dry_run = false;
        self.framing_active = false;
        if self.machine_profile.air_assist {
            self.send_command("M9");
        }

        if let Some(job) = self.active_queue_job.take() {
            self.job_queue_state.record_failure(job, reason);
        }
        if !self.machine_profile.post_job_hook.trim().is_empty() {
            let hook = self.machine_profile.post_job_hook.clone();
            self.send_hook_script("post-job", &hook);
        }
    }

    fn handle_program_aborted(&mut self) {
        self.running = false;
        self.is_dry_run = false;
        self.framing_active = false;
        if self.machine_profile.air_assist {
            self.send_command("M9");
        }

        if let Some(job) = self.active_queue_job.take() {
            self.job_queue_state.record_aborted(job);
        }
        if !self.machine_profile.post_job_hook.trim().is_empty() {
            let hook = self.machine_profile.post_job_hook.clone();
            self.send_hook_script("post-job", &hook);
        }
    }

    fn abort_program(&mut self) {
        if !self.send_realtime(RealtimeCommand::Reset) && self.is_connected() {
            self.log(format!(
                "Emergency reset not supported by {} backend.",
                self.machine_profile.controller_kind.label()
            ));
        }
        self.handle_program_aborted();
        self.log("Program aborted.".to_string());
    }

    fn send_command(&mut self, cmd: &str) {
        self.log(format!("> {cmd}"));
        if let Some(conn) = self.connection.as_ref() {
            conn.send(cmd);
        } else {
            self.log("Not connected".to_string());
        }
    }

    fn send_hook_script(&mut self, hook_name: &str, gcode: &str) {
        let mut sent = 0usize;
        for raw_line in gcode.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue;
            }
            self.send_command(line);
            sent += 1;
        }
        if sent > 0 {
            self.log(format!("Executed {hook_name} hook ({sent} line(s))."));
        }
    }

    fn execute_macro_script(&mut self, macro_label: &str, gcode: &str) {
        if !self.is_connected() {
            self.show_error("Connect machine before running a macro.".into());
            return;
        }

        let mut sent = 0usize;
        let mut skipped = 0usize;
        for raw_line in gcode.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                skipped += 1;
                continue;
            }
            self.send_command(line);
            sent += 1;
        }

        if sent == 0 {
            self.show_error(format!(
                "Macro '{macro_label}' has no executable command. Add at least one G-code line."
            ));
            return;
        }

        if skipped > 0 {
            self.log(format!(
                "Macro '{macro_label}' executed: {sent} command(s) sent ({skipped} line(s) skipped)."
            ));
        } else {
            self.log(format!(
                "Macro '{macro_label}' executed: {sent} command(s) sent."
            ));
        }
    }

    fn send_override(
        &mut self,
        reset_cmd: RealtimeCommand,
        plus10_cmd: RealtimeCommand,
        minus10_cmd: RealtimeCommand,
        plus1_cmd: RealtimeCommand,
        minus1_cmd: RealtimeCommand,
        pct: u8,
    ) {
        if self.send_realtime(reset_cmd) {
            let diff = pct as i32 - 100;
            let tens = diff / 10;
            let ones = diff % 10;
            let (ten_cmd, one_cmd) = if diff >= 0 {
                (plus10_cmd, plus1_cmd)
            } else {
                (minus10_cmd, minus1_cmd)
            };
            for _ in 0..tens.abs() {
                self.send_realtime(ten_cmd);
            }
            for _ in 0..ones.abs() {
                self.send_realtime(one_cmd);
            }
        }
    }

    /// Send GRBL real-time feed override to pct% (10-200)
    fn send_feed_override(&mut self, pct: u8) {
        self.send_override(
            RealtimeCommand::FeedOverrideReset,
            RealtimeCommand::FeedOverridePlus10,
            RealtimeCommand::FeedOverrideMinus10,
            RealtimeCommand::FeedOverridePlus1,
            RealtimeCommand::FeedOverrideMinus1,
            pct,
        );
    }

    /// Send GRBL real-time spindle (laser power) override to pct% (10-200)
    fn send_spindle_override(&mut self, pct: u8) {
        self.send_override(
            RealtimeCommand::SpindleOverrideReset,
            RealtimeCommand::SpindleOverridePlus10,
            RealtimeCommand::SpindleOverrideMinus10,
            RealtimeCommand::SpindleOverridePlus1,
            RealtimeCommand::SpindleOverrideMinus1,
            pct,
        );
    }

    fn jog(&mut self, dir: JogDirection) {
        if !self.is_connected() {
            return;
        }
        if let Some(cmd) = self
            .controller_backend
            .jog_command(dir, self.jog_step, self.jog_feed)
        {
            self.send_command(&cmd);
        } else {
            self.log(format!(
                "Jog is not supported by {} backend.",
                self.machine_profile.controller_kind.label()
            ));
        }
    }


    fn handle_keyboard_undo_redo(&mut self, undo_node_edit: bool, redo_node_edit: bool) {
        if undo_node_edit {
            self.handle_undo_node_edit();
        }
        if redo_node_edit {
            self.handle_redo_node_edit();
        }
    }

    fn handle_keyboard_clipboard(&mut self, copy_selection: bool, cut_selection: bool, paste_selection: bool) {
        if copy_selection {
            self.handle_copy_selection();
        }
        if cut_selection {
            self.handle_cut_selection();
        }
        if paste_selection {
            self.handle_paste_selection();
        }
    }

    fn handle_keyboard_delete(&mut self, delete_node: bool, delete_selection: bool) {
        if delete_node {
            self.handle_delete_node();
        }
        if delete_selection {
            self.handle_delete_selection();
        }
    }

    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        let mut jog_dir: Option<JogDirection> = None;
        let mut hold = false;
        let mut abort = false;
        let mut delete_selection = false;
        let mut delete_node = false;
        let mut undo_node_edit = false;
        let mut redo_node_edit = false;
        let mut copy_selection = false;
        let mut cut_selection = false;
        let mut paste_selection = false;
        let typing_in_progress = ctx.wants_keyboard_input();
        let caps = self.controller_capabilities();

        ctx.input(|i| {
            if caps.supports_jog {
                if i.key_pressed(egui::Key::ArrowUp) {
                    jog_dir = Some(JogDirection::N);
                }
                if i.key_pressed(egui::Key::ArrowDown) {
                    jog_dir = Some(JogDirection::S);
                }
                if i.key_pressed(egui::Key::ArrowLeft) {
                    jog_dir = Some(JogDirection::W);
                }
                if i.key_pressed(egui::Key::ArrowRight) {
                    jog_dir = Some(JogDirection::E);
                }
                if i.key_pressed(egui::Key::PageUp) {
                    jog_dir = Some(JogDirection::Zup);
                }
                if i.key_pressed(egui::Key::PageDown) {
                    jog_dir = Some(JogDirection::Zdown);
                }
            }
            if caps.supports_jog && caps.supports_home && i.key_pressed(egui::Key::Home) {
                jog_dir = Some(JogDirection::Home);
            }
            if caps.supports_hold_resume && i.key_pressed(egui::Key::Space) {
                hold = true;
            }
            if i.key_pressed(egui::Key::Escape) {
                abort = true;
            }
            if i.key_pressed(egui::Key::Delete) {
                if self.renderer.node_edit_mode
                    && (self.renderer.selected_node.is_some()
                        || !self.renderer.selected_nodes.is_empty())
                {
                    delete_node = true;
                } else {
                    delete_selection = true;
                }
            }

            if !typing_in_progress && i.modifiers.command && i.key_pressed(egui::Key::Z) {
                if i.modifiers.shift {
                    redo_node_edit = true;
                } else {
                    undo_node_edit = true;
                }
            }
            if !typing_in_progress && i.modifiers.command && i.key_pressed(egui::Key::Y) {
                redo_node_edit = true;
            }
            if !typing_in_progress && i.modifiers.command && i.key_pressed(egui::Key::C) {
                copy_selection = true;
            }
            if !typing_in_progress && i.modifiers.command && i.key_pressed(egui::Key::X) {
                cut_selection = true;
            }
            if !typing_in_progress && i.modifiers.command && i.key_pressed(egui::Key::V) {
                paste_selection = true;
            }
        });

        self.handle_keyboard_undo_redo(undo_node_edit, redo_node_edit);
        self.handle_keyboard_clipboard(copy_selection, cut_selection, paste_selection);
        self.handle_keyboard_delete(delete_node, delete_selection);

        if let Some(dir) = jog_dir {
            self.jog(dir);
        }
        if hold {
            self.send_realtime_or_warn(RealtimeCommand::FeedHold, "Feed hold");
        }
        if abort {
            self.abort_program();
        }
    }

    fn handle_undo_node_edit(&mut self) {
        if !self.undo_node_edit() {
            self.log("Nothing to undo.".into());
        }
    }

    fn handle_redo_node_edit(&mut self) {
        if !self.redo_node_edit() {
            self.log("Nothing to redo.".into());
        }
    }

    fn handle_copy_selection(&mut self) {
        let copied = self.copy_selected_shapes_to_clipboard();
        if copied == 0 {
            self.log("Copy: no shape selected.".into());
        } else {
            self.log(format!("Copied {copied} shape(s)."));
        }
    }

    fn handle_cut_selection(&mut self) {
        let copied = self.copy_selected_shapes_to_clipboard();
        if copied == 0 {
            self.log("Cut: no shape selected.".into());
        } else {
            self.push_node_undo_snapshot();
            let removed = self.delete_selected_shapes();
            self.log(format!("Cut {removed} shape(s)."));
        }
    }

    fn handle_paste_selection(&mut self) {
        if self.clipboard_shapes.is_empty() {
            self.log("Paste: clipboard is empty.".into());
        } else {
            self.push_node_undo_snapshot();
            let pasted = self.paste_shapes_from_clipboard();
            self.log(format!("Pasted {pasted} shape(s)."));
        }
    }

    fn handle_delete_node(&mut self) {
        if let Some((shape_idx, node_idx)) = self.renderer.selected_node {
            self.push_node_undo_snapshot();
            let mut selected_in_shape: Vec<usize> = self
                .renderer
                .selected_nodes
                .iter()
                .filter_map(|(s, n)| if *s == shape_idx { Some(*n) } else { None })
                .collect();
            selected_in_shape.sort_unstable();
            selected_in_shape.dedup();
            if selected_in_shape.is_empty() {
                selected_in_shape.push(node_idx);
            }

            let result = if selected_in_shape.len() > 1 {
                ui::vector_edit::delete_nodes(
                    &mut self.drawing_state,
                    shape_idx,
                    &selected_in_shape,
                )
            } else {
                ui::vector_edit::delete_node(&mut self.drawing_state, shape_idx, node_idx)
            };

            match result {
                Ok(()) => {
                    self.regenerate_drawing_gcode();
                    self.renderer.selected_nodes.clear();
                    self.renderer.selected_node = Some((shape_idx, node_idx.saturating_sub(1)));
                }
                Err(e) => self.log(format!("Node delete failed: {e}")),
            }
        }
    }

    fn handle_delete_selection(&mut self) {
        if !self.renderer.selected_shape_idx.is_empty() {
            self.push_node_undo_snapshot();
            let removed = self.delete_selected_shapes();
            if removed > 0 {
                self.log(format!("Deleted {removed} shape(s)."));
            }
        }
    }

    fn handle_file_drop(&mut self, ctx: &egui::Context) {
        let dropped_files = ctx.input(|i| i.raw.dropped_files.clone());
        if dropped_files.is_empty() {
            return;
        }
        let dropped: Vec<String> = dropped_files
            .into_iter()
            .filter_map(|f| f.path.as_ref().map(|p| p.to_string_lossy().to_string()))
            .collect();
        for path_str in dropped {
            let ext = std::path::Path::new(&path_str)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if matches!(
                ext.as_str(),
                "nc" | "gcode" | "ngc" | "gc" | "svg" | "png" | "jpg" | "jpeg" | "bmp"
            ) {
                self.load_file_path(&path_str);
            }
        }
    }

    fn switch_to_profile(&mut self, index: usize) {
        if index >= self.profile_store.profiles.len() {
            return;
        }
        // Save current edits back to the store before switching
        if let Some(p) = self
            .profile_store
            .profiles
            .get_mut(self.profile_store.active_index)
        {
            *p = self.machine_profile.clone();
        }
        self.profile_store.set_active(index);
        let prev_kind = self.machine_profile.controller_kind;
        self.machine_profile = self.profile_store.active().clone();
        self.use_tcp = self.machine_profile.preferred_use_tcp;
        self.tcp_host = self.machine_profile.preferred_tcp_host.clone();
        self.tcp_port_str = self.machine_profile.preferred_tcp_port.to_string();
        self.settings.output_protocol = self.machine_profile.preferred_output_protocol;
        if let Some(port_idx) = self
            .ports
            .iter()
            .position(|p| p == &self.machine_profile.preferred_port)
        {
            self.selected_port = port_idx;
        }
        if let Some(baud_idx) = self
            .baud_rates
            .iter()
            .position(|b| *b == self.machine_profile.preferred_baud)
        {
            self.selected_baud = baud_idx;
        }
        self.profile_store.save();
        if self.machine_profile.controller_kind != prev_kind {
            self.apply_controller_kind_change(prev_kind);
        }
        self.log(format!(
            "Switched to profile: {}",
            self.machine_profile.name
        ));
    }

    fn ui_profile_selector(&mut self, ui: &mut egui::Ui) -> bool {
        let mut profile_changed = false;
        let mut switch_to: Option<usize> = None;
        ui.horizontal(|ui| {
            let current_name = self.machine_profile.name.clone();
            egui::ComboBox::from_id_salt("profile_selector")
                .selected_text(&current_name)
                .width(140.0)
                .show_ui(ui, |ui| {
                    for (i, p) in self.profile_store.profiles.iter().enumerate() {
                        let is_active = i == self.profile_store.active_index;
                        if ui.selectable_label(is_active, &p.name).clicked() && !is_active {
                            switch_to = Some(i);
                        }
                    }
                });
            if ui.small_button("➕").on_hover_text("New profile").clicked() {
                let mut new_p = MachineProfile::default();
                new_p.name = format!("Machine {}", self.profile_store.profiles.len() + 1);
                self.profile_store.add(new_p);
                switch_to = Some(self.profile_store.profiles.len() - 1);
            }
            if ui.small_button("📋").on_hover_text("Duplicate").clicked() {
                // save current edits before duplicating
                if let Some(p) = self
                    .profile_store
                    .profiles
                    .get_mut(self.profile_store.active_index)
                {
                    *p = self.machine_profile.clone();
                }
                self.profile_store.duplicate_active();
                switch_to = Some(self.profile_store.profiles.len() - 1);
            }
            if self.profile_store.profiles.len() > 1 {
                if ui
                    .small_button("🗑")
                    .on_hover_text("Delete profile")
                    .clicked()
                {
                    let idx = self.profile_store.active_index;
                    self.profile_store.remove(idx);
                    switch_to = Some(self.profile_store.active_index);
                }
            }
        });
        if let Some(idx) = switch_to {
            self.switch_to_profile(idx);
            profile_changed = true;
        }
        profile_changed
    }

    fn ui_profile_import_export(&mut self, ui: &mut egui::Ui) -> bool {
        let mut profile_changed = false;
        ui.horizontal(|ui| {
            if ui.small_button("📥 Import").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Machine Profile", &["json"])
                    .pick_file()
                {
                    match self.profile_store.import_profile(&path.to_string_lossy()) {
                        Ok(name) => {
                            let new_idx = self.profile_store.profiles.len() - 1;
                            self.switch_to_profile(new_idx);
                            profile_changed = true;
                            self.log(format!("Imported profile: {name}"));
                        }
                        Err(e) => self.show_error(format!("Import failed: {e}")),
                    }
                }
            }
            if ui.small_button("📤 Export").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .set_file_name(&format!("{}.json", self.machine_profile.name))
                    .add_filter("Machine Profile", &["json"])
                    .save_file()
                {
                    match self
                        .profile_store
                        .export_profile(self.profile_store.active_index, &path.to_string_lossy())
                    {
                        Ok(()) => self.log(format!("Profile exported to {}", path.display())),
                        Err(e) => self.show_error(format!("Export failed: {e}")),
                    }
                }
            }
        });
        profile_changed
    }

    fn ui_profile_settings(&mut self, ui: &mut egui::Ui) -> bool {
        let mut profile_changed = false;
        egui::Grid::new("mp_grid")
            .num_columns(2)
            .spacing([8.0, 4.0])
            .show(ui, |ui| {
                ui.label("Name:");
                if ui
                    .text_edit_singleline(&mut self.machine_profile.name)
                    .changed()
                {
                    profile_changed = true;
                }
                ui.end_row();

                ui.label(format!("{}:", crate::i18n::tr("Controller")));
                let previous_kind = self.machine_profile.controller_kind;
                egui::ComboBox::from_id_salt("controller_kind_combo")
                    .selected_text(self.machine_profile.controller_kind.label())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.machine_profile.controller_kind,
                            ControllerKind::Grbl,
                            ControllerKind::Grbl.label(),
                        );
                        ui.selectable_value(
                            &mut self.machine_profile.controller_kind,
                            ControllerKind::Marlin,
                            ControllerKind::Marlin.label(),
                        );
                        ui.selectable_value(
                            &mut self.machine_profile.controller_kind,
                            ControllerKind::Ruida,
                            ControllerKind::Ruida.label(),
                        );
                        ui.selectable_value(
                            &mut self.machine_profile.controller_kind,
                            ControllerKind::Trocen,
                            ControllerKind::Trocen.label(),
                        );
                    });
                if self.machine_profile.controller_kind != previous_kind {
                    profile_changed = true;
                    self.apply_controller_kind_change(previous_kind);
                }
                ui.end_row();

                ui.label("Output Protocol:");
                egui::ComboBox::from_id_salt("profile_protocol_combo")
                    .selected_text(self.machine_profile.preferred_output_protocol.label())
                    .show_ui(ui, |ui| {
                        for kind in crate::gcode::output_protocol::ProtocolKind::ALL {
                            ui.selectable_value(
                                &mut self.machine_profile.preferred_output_protocol,
                                kind,
                                kind.label(),
                            );
                        }
                    });
                if self.settings.output_protocol != self.machine_profile.preferred_output_protocol {
                    self.settings.output_protocol = self.machine_profile.preferred_output_protocol;
                    profile_changed = true;
                }
                ui.end_row();

                ui.label("Preferred serial port:");
                if ui
                    .text_edit_singleline(&mut self.machine_profile.preferred_port)
                    .changed()
                {
                    profile_changed = true;
                }
                ui.end_row();

                ui.label("Preferred baud:");
                if ui
                    .add(egui::DragValue::new(&mut self.machine_profile.preferred_baud).range(1200..=2_000_000))
                    .changed()
                {
                    profile_changed = true;
                }
                ui.end_row();

                ui.label("Use TCP by default:");
                if ui
                    .checkbox(&mut self.machine_profile.preferred_use_tcp, "")
                    .changed()
                {
                    profile_changed = true;
                }
                ui.end_row();

                ui.label("Preferred TCP host:");
                if ui
                    .text_edit_singleline(&mut self.machine_profile.preferred_tcp_host)
                    .changed()
                {
                    profile_changed = true;
                }
                ui.end_row();

                ui.label("Preferred TCP port:");
                if ui
                    .add(egui::DragValue::new(&mut self.machine_profile.preferred_tcp_port).range(1..=65535))
                    .changed()
                {
                    profile_changed = true;
                }
                ui.end_row();

                ui.label("Spot size min (mm):");
                if ui
                    .add(egui::DragValue::new(&mut self.machine_profile.spot_size_min_mm).speed(0.01).range(0.01..=2.0))
                    .changed()
                {
                    profile_changed = true;
                }
                ui.end_row();

                ui.label("Spot size max (mm):");
                if ui
                    .add(egui::DragValue::new(&mut self.machine_profile.spot_size_max_mm).speed(0.01).range(0.01..=5.0))
                    .changed()
                {
                    profile_changed = true;
                }
                ui.end_row();

                ui.label("Width (mm):");
                if ui
                    .add(egui::DragValue::new(&mut self.machine_profile.workspace_x_mm).speed(5.0))
                    .changed()
                {
                    profile_changed = true;
                }
                ui.end_row();

                ui.label("Height (mm):");
                if ui
                    .add(egui::DragValue::new(&mut self.machine_profile.workspace_y_mm).speed(5.0))
                    .changed()
                {
                    profile_changed = true;
                }
                ui.end_row();

                ui.label("Max Rate X:");
                if ui
                    .add(
                        egui::DragValue::new(&mut self.machine_profile.max_rate_x)
                            .speed(50.0)
                            .suffix(" mm/min"),
                    )
                    .changed()
                {
                    profile_changed = true;
                }
                ui.end_row();

                ui.label("Max Rate Y:");
                if ui
                    .add(
                        egui::DragValue::new(&mut self.machine_profile.max_rate_y)
                            .speed(50.0)
                            .suffix(" mm/min"),
                    )
                    .changed()
                {
                    profile_changed = true;
                }
                ui.end_row();

                ui.label("Accel X:");
                if ui
                    .add(
                        egui::DragValue::new(&mut self.machine_profile.accel_x)
                            .speed(10.0)
                            .suffix(" mm/s²"),
                    )
                    .changed()
                {
                    profile_changed = true;
                }
                ui.end_row();

                ui.label("Accel Y:");
                if ui
                    .add(
                        egui::DragValue::new(&mut self.machine_profile.accel_y)
                            .speed(10.0)
                            .suffix(" mm/s²"),
                    )
                    .changed()
                {
                    profile_changed = true;
                }
                ui.end_row();
            });

        ui.horizontal(|ui| {
            if ui
                .checkbox(
                    &mut self.machine_profile.return_to_origin,
                    "Return to origin",
                )
                .changed()
            {
                profile_changed = true;
            }
        });
        ui.horizontal(|ui| {
            if ui
                .checkbox(&mut self.machine_profile.air_assist, "Air Assist (M8/M9)")
                .changed()
            {
                profile_changed = true;
            }
        });
        ui.horizontal(|ui| {
            if ui
                .checkbox(
                    &mut self.machine_profile.rotary_enabled,
                    "Enable Rotary Support",
                )
                .changed()
            {
                profile_changed = true;
            }
        });
        if self.machine_profile.rotary_enabled {
            ui.horizontal(|ui| {
                ui.label("Cylinder Ø:");
                if ui
                    .add(
                        egui::DragValue::new(&mut self.machine_profile.rotary_diameter_mm)
                            .suffix(" mm"),
                    )
                    .changed()
                {
                    profile_changed = true;
                }
            });
            ui.horizontal(|ui| {
                ui.label("Rotary Axis:");
                if ui
                    .selectable_value(&mut self.machine_profile.rotary_axis, 'Y', "Y (Roller)")
                    .changed()
                {
                    profile_changed = true;
                }
                if ui
                    .selectable_value(&mut self.machine_profile.rotary_axis, 'A', "A (Chuck)")
                    .changed()
                {
                    profile_changed = true;
                }
            });
        }

        ui.separator();
        ui.label("Pre-job G-code hook:");
        if ui
            .add(
                egui::TextEdit::multiline(&mut self.machine_profile.pre_job_hook)
                    .desired_rows(3)
                    .hint_text("Executed before sending job lines"),
            )
            .changed()
        {
            profile_changed = true;
        }
        ui.label("Post-job G-code hook:");
        if ui
            .add(
                egui::TextEdit::multiline(&mut self.machine_profile.post_job_hook)
                    .desired_rows(3)
                    .hint_text("Executed after completion/fail/abort"),
            )
            .changed()
        {
            profile_changed = true;
        }
        profile_changed
    }

    fn ui_machine_profile_editor(&mut self, ui: &mut egui::Ui) {
        let mut profile_changed = false;

        egui::CollapsingHeader::new(
            egui::RichText::new(format!("⚙ {}", crate::i18n::tr("Machine Profile")))
                .color(crate::theme::LAVENDER)
                .strong(),
        )
        .default_open(false)
        .show(ui, |ui| {
            profile_changed |= self.ui_profile_selector(ui);
            profile_changed |= self.ui_profile_import_export(ui);
            ui.separator();
            profile_changed |= self.ui_profile_settings(ui);

            // Maintenance alerts & tracking (F27/F97)
            ui.add_space(6.0);
            ui.group(|ui| {
                ui.label(egui::RichText::new("🔧 Maintenance").strong());
                let wear = self.machine_profile.tube_wear_pct();
                let wear_color = if wear > 90.0 { crate::theme::RED } else if wear > 70.0 { crate::theme::PEACH } else { crate::theme::GREEN };
                ui.label(egui::RichText::new(format!("Tube wear: {wear:.1}% ({:.1}h / {:.0}h)", self.machine_profile.tube_hours_total, self.machine_profile.tube_life_hours)).color(wear_color));
                ui.label(egui::RichText::new(format!("Jobs completed: {}", self.machine_profile.total_jobs_completed)).small().color(crate::theme::SUBTEXT));

                let alerts = self.machine_profile.maintenance_alerts();
                for alert in &alerts {
                    ui.label(egui::RichText::new(format!("⚠ {alert}")).color(crate::theme::PEACH));
                }

                ui.horizontal(|ui| {
                    if ui.button("✅ Lens Cleaned").on_hover_text("Reset lens cleaning counter").clicked() {
                        self.machine_profile.reset_lens_clean();
                        profile_changed = true;
                    }
                    if ui.button("✅ Belt Checked").on_hover_text("Reset belt check counter").clicked() {
                        self.machine_profile.reset_belt_check();
                        profile_changed = true;
                    }
                });
            });
        });

        if profile_changed {
            // Sync active profile back into the store and persist
            if let Some(p) = self
                .profile_store
                .profiles
                .get_mut(self.profile_store.active_index)
            {
                *p = self.machine_profile.clone();
            }
            self.profile_store.save();
        }
    }

    fn handle_camera_ui_actions(&mut self, ui: &mut egui::Ui) {
        let prev_cam_enabled = self.camera_state.enabled;
        let prev_cam_opacity = self.camera_state.opacity;
        let prev_cam_calib = self.camera_state.calibration.clone();
        let prev_cam_device_index = self.camera_state.device_index;
        let prev_cam_live_streaming = self.camera_state.live_streaming;
        let camera_action = ui::camera::show(ui, &mut self.camera_state);
        if camera_action.load_snapshot {
            self.load_camera_snapshot(ui.ctx());
            self.sync_settings();
        }
        if camera_action.start_live_stream {
            self.start_live_camera();
            self.sync_settings();
        }
        if camera_action.stop_live_stream {
            self.stop_live_camera();
            self.sync_settings();
            self.log("Live camera stopped.".into());
        }
        if camera_action.start_calibration_wizard {
            self.start_camera_calibration_wizard();
        }
        if camera_action.stop_calibration_wizard {
            self.stop_camera_calibration_wizard();
        }
        if camera_action.start_point_align {
            self.start_camera_point_align();
        }
        if camera_action.stop_point_align {
            self.stop_camera_point_align();
        }
        if camera_action.auto_detect_mark {
            self.auto_detect_camera_mark();
        }
        if camera_action.auto_detect_markers {
            self.auto_detect_camera_markers();
        }
        if camera_action.apply_detected_align {
            self.apply_detected_marker_align();
        }
        if self.camera_state.live_streaming
            && self.camera_state.device_index != prev_cam_device_index
        {
            self.start_live_camera();
            self.sync_settings();
        }
        if camera_action.clear_snapshot {
            self.stop_live_camera();
            self.camera_state.texture = None;
            self.camera_state.snapshot_path = None;
            self.camera_live.last_frame_rgba.clear();
            self.camera_live.last_frame_width = 0;
            self.camera_live.last_frame_height = 0;
            self.camera_state.detected_cross_world = None;
            self.camera_state.detected_circle_world = None;
            self.camera_state.detection_status = "No marker detection run yet.".to_string();
            self.sync_settings();
            self.log("Camera image cleared.".into());
        }
        if camera_action.align_job_to_camera {
            self.align_job_to_camera();
        }
        if self.camera_state.enabled != prev_cam_enabled
            || (self.camera_state.opacity - prev_cam_opacity).abs() > f32::EPSILON
            || self.camera_state.calibration.offset_x != prev_cam_calib.offset_x
            || self.camera_state.calibration.offset_y != prev_cam_calib.offset_y
            || self.camera_state.calibration.scale != prev_cam_calib.scale
            || self.camera_state.calibration.rotation != prev_cam_calib.rotation
            || self.camera_state.device_index != prev_cam_device_index
            || self.camera_state.live_streaming != prev_cam_live_streaming
        {
            self.sync_settings();
        }
    }

    /// Classic layout left panel: tools only (LightBurn-like).
    /// Selection, drawing, node editing, shape properties, manipulation.
    fn ui_left_tools_classic(&mut self, ui: &mut egui::Ui) {
        let selection: Vec<usize> = self.renderer.selected_shape_idx.iter().cloned().collect();

        // ── Selection & Node Editing ──
        ui.label(
            RichText::new(format!("🖱 {}", crate::i18n::tr("Selection")))
                .color(theme::LAVENDER)
                .strong(),
        );
        ui.add_space(2.0);
        self.ui_node_editing_tools(ui, &selection);

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(4.0);

        // ── Shape Creation ──
        ui.label(
            RichText::new(format!("🎨 {}", crate::i18n::tr("Create")))
                .color(theme::LAVENDER)
                .strong(),
        );
        ui.add_space(2.0);
        let draw_action = crate::ui::drawing::show(
            ui,
            &mut self.drawing_state,
            &self.layers,
            self.active_layer_idx,
        );
        if let Some(lines) = draw_action.generate_gcode {
            let file = GCodeFile::from_lines("drawing", &lines);
            self.set_loaded_file(file, lines);
        }

        ui.add_space(4.0);
        let text_action = ui::text::show(ui, &mut self.text_state, self.active_layer_idx);
        if let Some(shapes) = text_action.add_shapes {
            let added = shapes.len();
            if added > 0 {
                self.push_node_undo_snapshot();
                let start_idx = self.drawing_state.shapes.len();
                self.drawing_state.shapes.extend(shapes);
                self.renderer.selected_shape_idx.clear();
                self.renderer.selected_node = None;
                self.renderer.selected_nodes.clear();
                for idx in start_idx..self.drawing_state.shapes.len() {
                    self.renderer.selected_shape_idx.insert(idx);
                }
                if let Some(last_idx) = self.renderer.selected_shape_idx.iter().last().copied() {
                    if let Some(shape) = self.drawing_state.shapes.get(last_idx) {
                        self.drawing_state.current = shape.clone();
                    }
                }
                self.regenerate_drawing_gcode();
                self.log(format!("Added {added} text path shape(s)."));
            }
        }

        ui.add_space(4.0);
        let gen_action = ui::generators::show(ui, &mut self.generator_state, self.active_layer_idx);
        if let Some(lines) = gen_action.generate_gcode {
            let file = GCodeFile::from_lines("generator", &lines);
            self.set_loaded_file(file, lines);
        }
        if let Some(shapes) = gen_action.generate_shapes {
            self.push_node_undo_snapshot();
            self.renderer.selected_shape_idx.clear();
            let base = self.drawing_state.shapes.len();
            for (i, s) in shapes.into_iter().enumerate() {
                self.drawing_state.shapes.push(s);
                self.renderer.selected_shape_idx.insert(base + i);
            }
            self.regenerate_drawing_gcode();
        }

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(4.0);

        // ── Manipulation ──
        ui.label(
            RichText::new(format!("🔧 {}", crate::i18n::tr("Modify")))
                .color(theme::LAVENDER)
                .strong(),
        );
        ui.add_space(2.0);

        ui.horizontal_wrapped(|ui| {
            if ui
                .button(RichText::new(format!("🌀 {}", crate::i18n::tr("Array"))).small())
                .on_hover_text(crate::i18n::tr("Circular Array"))
                .clicked()
            {
                self.circular_array_state.is_open = true;
            }
            if ui
                .button(RichText::new(format!("🔲 {}", crate::i18n::tr("Grid"))).small())
                .on_hover_text(crate::i18n::tr("Grid Array"))
                .clicked()
            {
                self.grid_array_state.is_open = true;
            }
            if ui
                .button(RichText::new(format!("📐 {}", crate::i18n::tr("Offset"))).small())
                .on_hover_text(crate::i18n::tr("Offset Path"))
                .clicked()
            {
                self.offset_state.is_open = true;
            }
            if ui
                .button(RichText::new(format!("🧩 {}", crate::i18n::tr("Boolean"))).small())
                .on_hover_text(crate::i18n::tr("Boolean Operations"))
                .clicked()
            {
                self.boolean_ops_state.is_open = true;
            }
        });

        ui.add_space(4.0);
        // Group / Ungroup
        ui.horizontal(|ui| {
            if ui
                .add_enabled(selection.len() >= 2, egui::Button::new(format!("📦 {}", crate::i18n::tr("Group"))).small())
                .clicked()
            {
                self.push_node_undo_snapshot();
                if let Some(gid) =
                    crate::ui::drawing::group_shapes(&mut self.drawing_state.shapes, &selection)
                {
                    self.log(format!(
                        "Grouped {} shapes (group #{gid}).",
                        selection.len()
                    ));
                }
            }
            let has_group = selection.iter().any(|&i| {
                self.drawing_state
                    .shapes
                    .get(i)
                    .and_then(|s| s.group_id)
                    .is_some()
            });
            if ui
                .add_enabled(has_group, egui::Button::new(format!("📤 {}", crate::i18n::tr("Ungroup"))).small())
                .clicked()
            {
                self.push_node_undo_snapshot();
                let n = crate::ui::drawing::ungroup_shapes(
                    &mut self.drawing_state.shapes,
                    &selection,
                );
                self.log(format!("Ungrouped {n} shapes."));
            }
        });

        ui.add_space(4.0);
        // Align / Distribute (compact)
        self.ui_alignment_tools(ui);

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(4.0);

        // ── Properties ──
        self.ui_shape_properties(ui, &selection);

        ui.add_space(8.0);
        // Camera
        self.handle_camera_ui_actions(ui);
    }

    fn ui_left_content(&mut self, ui: &mut egui::Ui, connected: bool) {
        let caps = self.controller_capabilities();
        let selection: Vec<usize> = self.renderer.selected_shape_idx.iter().cloned().collect();
        egui::CollapsingHeader::new(
            RichText::new(format!("🔌 {}", crate::i18n::tr("Connection & Control")))
                .color(theme::LAVENDER)
                .strong(),
        )
        .default_open(true)
        .show(ui, |ui| {
            let conn_action = ui::connection::show(
                ui,
                &self.ports,
                &mut self.selected_port,
                &self.baud_rates,
                &mut self.selected_baud,
                connected,
                &mut self.settings.output_protocol,
                &mut self.use_tcp,
                &mut self.tcp_host,
                &mut self.tcp_port_str,
            );
            if conn_action.connect {
                self.connect();
            }
            if conn_action.connect_tcp {
                self.connect_tcp();
            }
            if conn_action.disconnect {
                self.disconnect();
            }
            if conn_action.refresh_ports {
                self.ports = connection::list_ports();
                self.log("Ports refreshed.".to_string());
            }

            ui.add_space(6.0);
            self.ui_machine_profile_editor(ui);

            ui.add_space(6.0);

            let ms_action =
                ui::machine_state::show(ui, &self.grbl_state, self.is_focus_on, connected, self.speed_unit);
            if ms_action.toggle_focus && connected {
                self.is_focus_on = !self.is_focus_on;
                if self.is_focus_on {
                    self.send_command("M3 S10");
                } else {
                    self.send_command("M5");
                }
            }
            if let Some(pos) = ms_action.quick_pos {
                self.quick_move_to(pos);
            }

            ui.add_space(6.0);

            let jog_action = ui::jog::show(
                ui,
                &mut self.jog_step,
                &mut self.jog_feed,
                caps.supports_jog,
                caps.supports_home,
                self.speed_unit,
            );
            if let Some(dir) = jog_action.direction {
                self.jog(dir);
            }
        });

        ui.add_space(6.0);

        egui::CollapsingHeader::new(
            RichText::new(format!("📐 {}", crate::i18n::tr("Job Preparation")))
                .color(theme::LAVENDER)
                .strong(),
        )
        .default_open(true)
        .show(ui, |ui| {
            ui.group(|ui| {
                let est_time_s = self.estimation.estimated_seconds;
                let h = (est_time_s / 3600.0) as u32;
                let m = ((est_time_s % 3600.0) / 60.0) as u32;
                let s = (est_time_s % 60.0) as u32;
                ui.label(
                    RichText::new(format!("⏱ Est. Time: {:02}:{:02}:{:02}", h, m, s))
                        .strong()
                        .color(theme::GREEN),
                );

                ui.add_space(4.0);
                if ui
                    .button("⚡ Optimize Path")
                    .on_hover_text("Reorder segments to minimize travel distance")
                    .clicked()
                {
                    if let Some(mut file) = self.loaded_file.clone() {
                        let optimized_lines = crate::gcode::optimizer::optimize(&file.lines);
                        let raw_lines: Vec<String> =
                            optimized_lines.iter().map(|l| l.to_gcode()).collect();
                        file.lines = optimized_lines;
                        self.set_loaded_file(file, raw_lines);
                        self.log("Path optimized.".to_string());
                    }
                }
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("📐 {}", crate::i18n::tr("Job Transformation")))
                        .color(theme::LAVENDER)
                        .strong(),
                );
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(crate::i18n::tr("Offset X:"));
                    ui.add(
                        egui::DragValue::new(&mut self.job_transform.offset_x)
                            .speed(1.0)
                            .suffix(" mm"),
                    );
                    ui.label(crate::i18n::tr("Y:"));
                    ui.add(
                        egui::DragValue::new(&mut self.job_transform.offset_y)
                            .speed(1.0)
                            .suffix(" mm"),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label(crate::i18n::tr("Rotation:"));
                    ui.add(
                        egui::Slider::new(&mut self.job_transform.rotation, -180.0..=180.0)
                            .suffix("°"),
                    );
                    if ui
                        .button(crate::i18n::tr("Reset"))
                        .clicked()
                    {
                        self.job_transform.rotation = 0.0;
                    }

                    ui.add_space(4.0);
                    if ui.button(format!("🔍 {}", crate::i18n::tr("Preflight Check"))).clicked() {
                        self.preflight_state.issues = ui::preflight::run_checks(&self.drawing_state.shapes, &self.layers);
                        self.preflight_state.is_open = true;
                    }
                });
                if ui.button(crate::i18n::tr("Center Job")).clicked() {
                    if let Some(file) = &self.loaded_file {
                        if let Some((min_x, min_y, max_x, max_y)) = file.bounds() {
                            let mid_x = (min_x + max_x) / 2.0;
                            let mid_y = (min_y + max_y) / 2.0;
                            let wm_x = self.machine_profile.workspace_x_mm / 2.0;
                            let wm_y = self.machine_profile.workspace_y_mm / 2.0;
                            self.job_transform.offset_x = wm_x - mid_x;
                            self.job_transform.offset_y = wm_y - mid_y;
                        }
                    }
                }
            });
        });

        ui.add_space(6.0);

        egui::CollapsingHeader::new(
            RichText::new(format!("🎨 {}", crate::i18n::tr("Creation & Editing")))
                .color(theme::LAVENDER)
                .strong(),
        )
        .default_open(!self.beginner_mode)
        .show(ui, |ui| {
            let draw_action = crate::ui::drawing::show(
                ui,
                &mut self.drawing_state,
                &self.layers,
                self.active_layer_idx,
            );
            if let Some(lines) = draw_action.generate_gcode {
                let file = GCodeFile::from_lines("drawing", &lines);
                self.set_loaded_file(file, lines);
            }

            ui.add_space(6.0);
            let text_action = ui::text::show(ui, &mut self.text_state, self.active_layer_idx);
            if let Some(shapes) = text_action.add_shapes {
                let added = shapes.len();
                if added > 0 {
                    self.push_node_undo_snapshot();
                    let start_idx = self.drawing_state.shapes.len();
                    self.drawing_state.shapes.extend(shapes);

                    self.renderer.selected_shape_idx.clear();
                    self.renderer.selected_node = None;
                    self.renderer.selected_nodes.clear();
                    for idx in start_idx..self.drawing_state.shapes.len() {
                        self.renderer.selected_shape_idx.insert(idx);
                    }

                    if let Some(last_idx) = self.renderer.selected_shape_idx.iter().last().copied()
                    {
                        if let Some(shape) = self.drawing_state.shapes.get(last_idx) {
                            self.drawing_state.current = shape.clone();
                        }
                    }

                    self.regenerate_drawing_gcode();
                    self.log(format!("Added {added} text path shape(s)."));
                }
            }

            ui.add_space(4.0);

            let ws = self.renderer.workspace_size;
            ui::alignment::show(ui, &mut self.drawing_state, &selection, ws);

            ui.add_space(6.0);
            self.ui_shape_properties(ui, &selection);

            ui.add_space(6.0);
            self.handle_camera_ui_actions(ui);

            ui.add_space(6.0);
            let gen_action = ui::generators::show(ui, &mut self.generator_state, self.active_layer_idx);
            if let Some(lines) = gen_action.generate_gcode {
                let file = GCodeFile::from_lines("generator", &lines);
                self.set_loaded_file(file, lines);
            }
            if let Some(shapes) = gen_action.generate_shapes {
                self.push_node_undo_snapshot();
                self.renderer.selected_shape_idx.clear();
                let base = self.drawing_state.shapes.len();
                for (i, s) in shapes.into_iter().enumerate() {
                    self.drawing_state.shapes.push(s);
                    self.renderer.selected_shape_idx.insert(base + i);
                }
                self.regenerate_drawing_gcode();
            }

            ui.add_space(6.0);
            if ui
                .button(
                    RichText::new("🌀 Circular Array")
                        .color(theme::LAVENDER)
                        .strong(),
                )
                .clicked()
            {
                self.circular_array_state.is_open = true;
            }
            if ui
                .button(
                    RichText::new("🔲 Grid Array")
                        .color(theme::LAVENDER)
                        .strong(),
                )
                .clicked()
            {
                self.grid_array_state.is_open = true;
            }
            if ui
                .button(
                    RichText::new("📐 Offset Path")
                        .color(theme::LAVENDER)
                        .strong(),
                )
                .clicked()
            {
                self.offset_state.is_open = true;
            }
            if ui
                .button(
                    RichText::new("🧩 Boolean Ops")
                        .color(theme::LAVENDER)
                        .strong(),
                )
                .clicked()
            {
                self.boolean_ops_state.is_open = true;
            }

            // Group / Ungroup (F51)
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(selection.len() >= 2, egui::Button::new("📦 Group"))
                    .clicked()
                {
                    self.push_node_undo_snapshot();
                    if let Some(gid) =
                        crate::ui::drawing::group_shapes(&mut self.drawing_state.shapes, &selection)
                    {
                        self.log(format!(
                            "Grouped {} shapes (group #{gid}).",
                            selection.len()
                        ));
                    }
                }
                let has_group = selection.iter().any(|&i| {
                    self.drawing_state
                        .shapes
                        .get(i)
                        .and_then(|s| s.group_id)
                        .is_some()
                });
                if ui
                    .add_enabled(has_group, egui::Button::new("📤 Ungroup"))
                    .clicked()
                {
                    self.push_node_undo_snapshot();
                    let n = crate::ui::drawing::ungroup_shapes(
                        &mut self.drawing_state.shapes,
                        &selection,
                    );
                    self.log(format!("Ungrouped {n} shapes."));
                }
            });

            ui.add_space(6.0);
            self.ui_node_editing_tools(ui, &selection);
        });

        ui.add_space(6.0);

        if !self.beginner_mode {
            self.ui_advanced_tools(ui, connected);
        } else {
            ui.label(
                RichText::new(format!(
                    "🧭 {}",
                    crate::i18n::tr("Beginner mode active: interface simplified. Disable it in View to show all tools.")
                ))
                .small()
                .color(theme::SUBTEXT),
            );
        }
    }

    fn ui_node_editing_tools(&mut self, ui: &mut egui::Ui, selection: &[usize]) {
        let node_edit_text = if self.renderer.node_edit_mode {
            format!("✅ {}", crate::i18n::tr("Node Editing"))
        } else {
            format!("🖱 {}", crate::i18n::tr("Node Editing"))
        };
        if ui
            .selectable_label(
                self.renderer.node_edit_mode,
                RichText::new(node_edit_text).color(theme::PEACH).strong(),
            )
            .clicked()
        {
            self.renderer.node_edit_mode = !self.renderer.node_edit_mode;
            if !self.renderer.node_edit_mode {
                self.renderer.selected_node = None;
                self.renderer.selected_nodes.clear();
            }
        }

        // Measure tool (F50)
        let measure_text = if self.renderer.measure_mode {
            format!("✅ {}", crate::i18n::tr("Measure"))
        } else {
            format!("📏 {}", crate::i18n::tr("Measure"))
        };
        if ui
            .selectable_label(
                self.renderer.measure_mode,
                RichText::new(measure_text).color(theme::TEAL).strong(),
            )
            .clicked()
        {
            self.renderer.measure_mode = !self.renderer.measure_mode;
            if !self.renderer.measure_mode {
                self.renderer.measure_start = None;
                self.renderer.measure_end = None;
            }
        }
        if self.renderer.measure_mode {
            if let (Some(s), Some(e)) = (self.renderer.measure_start, self.renderer.measure_end) {
                let dx = e.0 - s.0;
                let dy = e.1 - s.1;
                let dist = (dx * dx + dy * dy).sqrt();
                ui.label(
                    RichText::new(format!("Distance: {dist:.2} mm"))
                        .color(theme::TEAL)
                        .small(),
                );
            } else {
                ui.label(
                    RichText::new("Click two points on canvas to measure")
                        .small()
                        .color(theme::SUBTEXT),
                );
            }
        }

        if self.renderer.node_edit_mode {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(selection.len() >= 2, egui::Button::new("🔗 Join Paths"))
                    .clicked()
                {
                    self.push_node_undo_snapshot();
                    match ui::vector_edit::join_selected_paths(&mut self.drawing_state, selection) {
                        Ok(new_idx) => {
                            self.renderer.selected_shape_idx.clear();
                            self.renderer.selected_shape_idx.insert(new_idx);
                            self.renderer.selected_node = None;
                            self.renderer.selected_nodes.clear();
                            self.regenerate_drawing_gcode();
                            self.log("Paths joined.".into());
                        }
                        Err(e) => self.log(format!("Join paths failed: {e}")),
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Node smooth:");
                ui.add(egui::Slider::new(&mut self.node_smooth_strength, 0.0..=1.0));
                ui.label("Corner strength:");
                ui.add(egui::Slider::new(&mut self.node_corner_strength, 0.0..=1.5));
            });

            ui.horizontal(|ui| {
                ui.label("Simplify tol:");
                ui.add(egui::Slider::new(
                    &mut self.path_simplify_tolerance,
                    0.01..=2.0,
                ));
                ui.label("Path smooth:");
                ui.add(egui::Slider::new(&mut self.path_smooth_strength, 0.0..=1.0));
                ui.label("iter:");
                ui.add(egui::DragValue::new(&mut self.path_smooth_iterations).range(1..=8));
            });

            if let Some((shape_idx, node_idx)) = self.renderer.selected_node {
                let mut selected_nodes: Vec<usize> = self
                    .renderer
                    .selected_nodes
                    .iter()
                    .filter_map(|(s, n)| if *s == shape_idx { Some(*n) } else { None })
                    .collect();
                selected_nodes.sort_unstable();
                selected_nodes.dedup();
                if selected_nodes.is_empty() {
                    selected_nodes.push(node_idx);
                }

                ui.horizontal(|ui| {
                    if ui
                        .button("➕ Node")
                        .on_hover_text("Insert midpoint after selected node")
                        .clicked()
                    {
                        self.push_node_undo_snapshot();
                        match ui::vector_edit::insert_midpoint_after(
                            &mut self.drawing_state,
                            shape_idx,
                            node_idx,
                        ) {
                            Ok(new_idx) => {
                                self.renderer.selected_node = Some((shape_idx, new_idx));
                                self.renderer.selected_nodes.clear();
                                self.renderer.selected_nodes.insert((shape_idx, new_idx));
                                self.regenerate_drawing_gcode();
                            }
                            Err(e) => self.log(format!("Insert node failed: {e}")),
                        }
                    }
                    if ui
                        .button("✂ Split")
                        .on_hover_text("Split path at selected node")
                        .clicked()
                    {
                        self.push_node_undo_snapshot();
                        match ui::vector_edit::split_path_at_node(
                            &mut self.drawing_state,
                            shape_idx,
                            node_idx,
                        ) {
                            Ok(other_idx) => {
                                self.renderer.selected_shape_idx.clear();
                                self.renderer.selected_shape_idx.insert(shape_idx);
                                self.renderer.selected_shape_idx.insert(other_idx);
                                self.renderer.selected_node = None;
                                self.renderer.selected_nodes.clear();
                                self.regenerate_drawing_gcode();
                            }
                            Err(e) => self.log(format!("Split path failed: {e}")),
                        }
                    }
                    if ui
                        .button("🧼 Smooth")
                        .on_hover_text("Soften selected node")
                        .clicked()
                    {
                        self.push_node_undo_snapshot();
                        match ui::vector_edit::smooth_nodes_weighted(
                            &mut self.drawing_state,
                            shape_idx,
                            &selected_nodes,
                            self.node_smooth_strength,
                        ) {
                            Ok(()) => self.regenerate_drawing_gcode(),
                            Err(e) => self.log(format!("Smooth node failed: {e}")),
                        }
                    }
                    if ui
                        .button("📐 Corner")
                        .on_hover_text("Sharpen selected node")
                        .clicked()
                    {
                        self.push_node_undo_snapshot();
                        match ui::vector_edit::corner_nodes_weighted(
                            &mut self.drawing_state,
                            shape_idx,
                            &selected_nodes,
                            self.node_corner_strength,
                        ) {
                            Ok(()) => self.regenerate_drawing_gcode(),
                            Err(e) => self.log(format!("Corner node failed: {e}")),
                        }
                    }
                    if ui
                        .button("🧹 Simplify")
                        .on_hover_text("Simplify path with quality guard")
                        .clicked()
                    {
                        self.push_node_undo_snapshot();
                        match ui::vector_edit::simplify_path(
                            &mut self.drawing_state,
                            shape_idx,
                            self.path_simplify_tolerance,
                        ) {
                            Ok(removed) => {
                                self.regenerate_drawing_gcode();
                                self.log(format!("Path simplified: removed {removed} nodes."));
                            }
                            Err(e) => self.log(format!("Simplify path failed: {e}")),
                        }
                    }
                    if ui
                        .button("〰 Smooth Path")
                        .on_hover_text("Smooth entire path with iterations")
                        .clicked()
                    {
                        self.push_node_undo_snapshot();
                        match ui::vector_edit::smooth_path(
                            &mut self.drawing_state,
                            shape_idx,
                            self.path_smooth_iterations as usize,
                            self.path_smooth_strength,
                        ) {
                            Ok(()) => self.regenerate_drawing_gcode(),
                            Err(e) => self.log(format!("Smooth path failed: {e}")),
                        }
                    }
                    if ui.button("🗑 Delete Node").clicked() {
                        self.push_node_undo_snapshot();
                        let result = if selected_nodes.len() > 1 {
                            ui::vector_edit::delete_nodes(
                                &mut self.drawing_state,
                                shape_idx,
                                &selected_nodes,
                            )
                        } else {
                            ui::vector_edit::delete_node(
                                &mut self.drawing_state,
                                shape_idx,
                                node_idx,
                            )
                        };
                        match result {
                            Ok(()) => {
                                self.renderer.selected_nodes.clear();
                                self.renderer.selected_node =
                                    Some((shape_idx, node_idx.saturating_sub(1)));
                                self.regenerate_drawing_gcode();
                            }
                            Err(e) => self.log(format!("Delete node failed: {e}")),
                        }
                    }
                });

                if selected_nodes.len() > 1 {
                    ui.label(
                        RichText::new(format!("{} nodes selected", selected_nodes.len()))
                            .small()
                            .color(theme::SUBTEXT),
                    );
                }
            } else {
                ui.label(
                    RichText::new("Node mode: click a node to edit, click a segment to add.")
                        .small()
                        .color(theme::SUBTEXT),
                );
            }
        }

        if self.renderer.selected_shape_idx.len() == 1 {
            if let Some(&idx) = self.renderer.selected_shape_idx.iter().next() {
                if let Some(shape) = self.drawing_state.shapes.get(idx) {
                    if !matches!(shape.shape, ShapeKind::Path(_)) {
                        if ui.button("🛤 Convert to Path").clicked() {
                            if let Some(poly) = ui::offset::shape_to_polygon(shape) {
                                let exterior = poly.exterior();
                                let pts: Vec<(f32, f32)> = exterior
                                    .coords()
                                    .map(|c| (c.x as f32, c.y as f32))
                                    .collect();
                                let mut new_shape = shape.clone();
                                new_shape.shape = ShapeKind::Path(crate::ui::drawing::PathData::from_points(pts));
                                new_shape.x = 0.0;
                                new_shape.y = 0.0;
                                self.drawing_state.shapes[idx] = new_shape;
                                self.log("Converted to path.".into());
                            }
                        }
                    }
                }
            }
        }
    }

    fn ui_advanced_tools(&mut self, ui: &mut egui::Ui, connected: bool) {
        egui::CollapsingHeader::new(
            RichText::new(format!("🛠 {}", crate::i18n::tr("Advanced Tools")))
                .color(theme::LAVENDER)
                .strong(),
        )
        .default_open(false)
        .show(ui, |ui| {
            ui.add_space(6.0);

            let macros_action = ui::macros::show(ui, &mut self.macros_state, connected);
            if let Some(mac) = macros_action.execute_macro {
                self.execute_macro_script(&mac.label, &mac.gcode);
            }

            ui.add_space(6.0);

            let console_action = ui::console::show(
                ui,
                self.console_log.make_contiguous(),
                &mut self.console_input,
            );
            if let Some(cmd) = console_action.send_command {
                self.send_command(&cmd);
            }

            ui.add_space(6.0);

            let mat_context = self.materials_ui_context();
            let mat_action =
                ui::materials::show_with_context(ui, &mut self.materials_state, &mat_context);
            self.apply_material_action(mat_action);

            ui.add_space(6.0);

            self.ui_alignment_tools(ui);

            ui.add_space(6.0);

            self.ui_z_probe_tools(ui);
        });
    }

    fn ui_alignment_tools(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(
                RichText::new(format!("📐 {}", crate::i18n::tr("Align / Distribute")))
                    .color(theme::LAVENDER)
                    .strong(),
            );
            let sel: Vec<usize> = self.renderer.selected_shape_idx.iter().copied().collect();
            let has_sel = sel.len() >= 2;
            ui.horizontal(|ui| {
                for (label, hint, op) in [
                    ("⬅", crate::i18n::tr("Align Left"), crate::ui::drawing::AlignOp::Left),
                    ("➡", crate::i18n::tr("Align Right"), crate::ui::drawing::AlignOp::Right),
                    ("⬆", crate::i18n::tr("Align Top"), crate::ui::drawing::AlignOp::Top),
                    ("⬇", crate::i18n::tr("Align Bottom"), crate::ui::drawing::AlignOp::Bottom),
                    (
                        "⬌",
                        crate::i18n::tr("Center Horizontal"),
                        crate::ui::drawing::AlignOp::CenterH,
                    ),
                    ("⬍", crate::i18n::tr("Center Vertical"), crate::ui::drawing::AlignOp::CenterV),
                ] {
                    if ui
                        .add_enabled(has_sel, egui::Button::new(label).small())
                        .on_hover_text(hint)
                        .clicked()
                    {
                        self.push_node_undo_snapshot();
                        crate::ui::drawing::align_shapes(&mut self.drawing_state.shapes, &sel, op);
                        self.regenerate_drawing_gcode();
                    }
                }
            });
            let has_3 = sel.len() >= 3;
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(has_3, egui::Button::new(format!("⇔ {}", crate::i18n::tr("Distribute H"))).small())
                    .clicked()
                {
                    self.push_node_undo_snapshot();
                    crate::ui::drawing::align_shapes(
                        &mut self.drawing_state.shapes,
                        &sel,
                        crate::ui::drawing::AlignOp::DistributeH,
                    );
                    self.regenerate_drawing_gcode();
                }
                if ui
                    .add_enabled(has_3, egui::Button::new(format!("⇕ {}", crate::i18n::tr("Distribute V"))).small())
                    .clicked()
                {
                    self.push_node_undo_snapshot();
                    crate::ui::drawing::align_shapes(
                        &mut self.drawing_state.shapes,
                        &sel,
                        crate::ui::drawing::AlignOp::DistributeV,
                    );
                    self.regenerate_drawing_gcode();
                }
            });
        });
    }

    fn ui_z_probe_tools(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("📏 {}", crate::i18n::tr("Z-Probe")))
                        .color(theme::LAVENDER)
                        .strong(),
                );
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui
                    .button("⇊ Run Z-Probe")
                    .on_hover_text("Search for surface using G38.2 and set Z0")
                    .clicked()
                {
                    self.send_command("G38.2 Z-50 F100");
                    self.send_command("G4 P0.5");
                    self.send_command("G92 Z0");
                    self.send_command("G0 Z5 F500");
                    self.log("Probing complete. Z set to 5mm above surface.".into());
                }
                if ui
                    .button("🎯 Focus Point")
                    .on_hover_text("Move to Z focusing position (e.g. 20mm)")
                    .clicked()
                {
                    self.send_command("G0 Z20 F1000");
                }
            });
            ui.add_space(4.0);
            if ui
                .button("🔍 Generate Z Focus Test")
                .on_hover_text("Generate lines at different Z heights to find focus")
                .clicked()
            {
                let lines = ui::power_speed_test::generate_z_focus_test(
                    0.0, 10.0, 10, 40.0, 1000.0, 500.0, 5.0, 5.0,
                );
                let file = crate::gcode::file::GCodeFile::from_lines("z_focus_test", &lines);
                self.set_loaded_file(file, lines);
                self.log("Z focus test pattern loaded.".into());
            }
        });
    }

    fn ui_right_content(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(4.0);
            ui.label(
                RichText::new("📜 GCode Lines")
                    .color(theme::LAVENDER)
                    .strong(),
            );
            ui.separator();

            let text_style = egui::TextStyle::Monospace;
            let row_height = ui.text_style_height(&text_style);
            let num_rows = self.program_lines.len();

            egui::ScrollArea::vertical()
                .id_salt("gcode_scroll")
                .max_height(ui.available_height() - 10.0)
                .show_rows(ui, row_height, num_rows, |ui, row_range| {
                    for i in row_range {
                        let is_current = i == self.program_index && self.running;
                        let line = &self.program_lines[i];

                        let mut text = RichText::new(format!("{: >4} | {}", i + 1, line))
                            .monospace()
                            .size(12.0);

                        if is_current {
                            text = text.color(theme::GREEN).strong();
                        } else if self.running && i < self.program_index {
                            text = text.color(theme::SUBTEXT);
                        } else if !self.light_mode {
                            text = text.color(theme::TEXT);
                        }

                        let response = ui.selectable_label(is_current, text);
                        if is_current {
                            response.scroll_to_me(Some(egui::Align::Center));
                        }
                    }
                });
        });
    }

    fn ui_shape_properties(&mut self, ui: &mut egui::Ui, selection: &[usize]) {
        ui.label(RichText::new(crate::i18n::tr("Shape Properties")).strong());
        if selection.len() == 1 {
            let idx = selection[0];
            if let Some(shape) = self.drawing_state.shapes.get_mut(idx) {
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label("X:");
                    if ui
                        .add(egui::DragValue::new(&mut shape.x).speed(1.0))
                        .changed()
                    {
                        changed = true;
                    }
                    ui.label("Y:");
                    if ui
                        .add(egui::DragValue::new(&mut shape.y).speed(1.0))
                        .changed()
                    {
                        changed = true;
                    }
                });
                ui.horizontal(|ui| match &shape.shape {
                    ShapeKind::Circle => {
                        ui.label("R:");
                        if ui
                            .add(egui::DragValue::new(&mut shape.radius).speed(1.0))
                            .changed()
                        {
                            changed = true;
                        }
                    }
                    _ => {
                        ui.label(crate::i18n::tr("W:"));
                        if ui
                            .add(egui::DragValue::new(&mut shape.width).speed(1.0))
                            .changed()
                        {
                            changed = true;
                        }
                        ui.label(crate::i18n::tr("H:"));
                        if ui
                            .add(egui::DragValue::new(&mut shape.height).speed(1.0))
                            .changed()
                        {
                            changed = true;
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(crate::i18n::tr("Layer:"));
                    egui::ComboBox::from_id_salt("sel_layer")
                        .selected_text(format!("Layer {}", shape.layer_idx))
                        .show_ui(ui, |ui| {
                            for i in 0..self.layers.len() {
                                let name = self.layers[i].name.clone();
                                if ui.selectable_value(&mut shape.layer_idx, i, name).changed() {
                                    changed = true;
                                }
                            }
                        });
                });
                ui.horizontal(|ui| {
                    ui.label(crate::i18n::tr("Rotation:"));
                    if ui
                        .add(
                            egui::DragValue::new(&mut shape.rotation)
                                .suffix("°")
                                .range(0.0..=360.0),
                        )
                        .changed()
                    {
                        changed = true;
                    }
                });

                if let Some(msg) = shape_fill_warning(shape, &self.layers) {
                    ui.add_space(4.0);
                    ui.label(RichText::new(format!("⚠ {}", msg)).color(theme::PEACH));
                }

                ui.separator();
                ui.label(RichText::new(crate::i18n::tr("Outline Settings")).strong());
                ui.horizontal(|ui| {
                    ui.label(crate::i18n::tr("Dist:"));
                    ui.add(
                        egui::DragValue::new(&mut self.offset_state.distance)
                            .speed(0.1)
                            .range(0.1..=100.0),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label(crate::i18n::tr("Join:"));
                    egui::ComboBox::from_id_salt("sel_join")
                        .selected_text(format!("{:?}", self.offset_state.join_style))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.offset_state.join_style,
                                JoinStyle::Round,
                                "Round",
                            );
                            ui.selectable_value(
                                &mut self.offset_state.join_style,
                                JoinStyle::Miter,
                                "Miter",
                            );
                            ui.selectable_value(
                                &mut self.offset_state.join_style,
                                JoinStyle::Bevel,
                                "Bevel",
                            );
                        });
                });

                ui.add_space(4.0);
                if ui.button("🔳 Create Outline (Cut)").clicked() {
                    let selection = vec![idx];
                    ui::offset::apply_offset(
                        &self.offset_state,
                        &mut self.drawing_state,
                        &selection,
                    );
                    changed = true;
                }

                if changed {
                    self.regenerate_drawing_gcode();
                }
            }
        } else if selection.len() > 1 {
            ui.label(format!("{} {}", selection.len(), crate::i18n::tr("items selected")));
            ui.add_space(4.0);

            // Batch move
            ui.horizontal(|ui| {
                ui.label(crate::i18n::tr("Move X:"));
                ui.add(egui::DragValue::new(&mut self.batch_move_x).speed(1.0).suffix(" mm"));
                ui.label(crate::i18n::tr("Move Y:"));
                ui.add(egui::DragValue::new(&mut self.batch_move_y).speed(1.0).suffix(" mm"));
                if ui.button(crate::i18n::tr("Apply")).clicked() {
                    let dx = self.batch_move_x;
                    let dy = self.batch_move_y;
                    for &idx in selection {
                        if let Some(s) = self.drawing_state.shapes.get_mut(idx) {
                            s.x += dx;
                            s.y += dy;
                        }
                    }
                    self.batch_move_x = 0.0;
                    self.batch_move_y = 0.0;
                    self.regenerate_drawing_gcode();
                }
            });

            // Batch set layer
            ui.horizontal(|ui| {
                ui.label(crate::i18n::tr("Set Layer:"));
                egui::ComboBox::from_id_salt("batch_layer")
                    .selected_text(format!("Layer {}", self.batch_target_layer))
                    .show_ui(ui, |ui| {
                        for i in 0..self.layers.len() {
                            let name = self.layers[i].name.clone();
                            ui.selectable_value(&mut self.batch_target_layer, i, name);
                        }
                    });
                if ui.button(crate::i18n::tr("Apply")).clicked() {
                    let tgt = self.batch_target_layer;
                    for &idx in selection {
                        if let Some(s) = self.drawing_state.shapes.get_mut(idx) {
                            s.layer_idx = tgt;
                        }
                    }
                    self.regenerate_drawing_gcode();
                }
            });

            // Batch delete
            ui.add_space(4.0);
            if ui.button(RichText::new(format!("🗑 {}", crate::i18n::tr("Delete Selected"))).color(theme::RED)).clicked() {
                let mut sel_sorted: Vec<usize> = selection.to_vec();
                sel_sorted.sort_unstable();
                sel_sorted.dedup();
                for &idx in sel_sorted.iter().rev() {
                    if idx < self.drawing_state.shapes.len() {
                        self.drawing_state.shapes.remove(idx);
                    }
                }
                self.renderer.selected_shape_idx.clear();
                self.regenerate_drawing_gcode();
            }
        } else {
            ui.label(crate::i18n::tr("Select a shape to edit properties."));
        }
    }

    fn ui_right_tabs(&mut self, ui: &mut egui::Ui, connected: bool) {
        use crate::i18n::tr;
        let caps = self.controller_capabilities();

        if self.active_tab == RightPanelTab::Art {
            self.active_tab = RightPanelTab::Cuts;
        }

        egui::ScrollArea::horizontal().id_salt("tab_bar_scroll").show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_value(&mut self.active_tab, RightPanelTab::Cuts, tr("Cuts"))
                    .changed()
                {
                    self.sync_settings();
                }
                if ui
                    .selectable_value(&mut self.active_tab, RightPanelTab::Move, tr("Move"))
                    .changed()
                {
                    self.sync_settings();
                }
                if ui
                    .selectable_value(&mut self.active_tab, RightPanelTab::Console, tr("Console"))
                    .changed()
                {
                    self.sync_settings();
                }
                if ui
                    .selectable_value(&mut self.active_tab, RightPanelTab::Job, tr("Job"))
                    .changed()
                {
                    self.sync_settings();
                }
                if ui
                    .selectable_value(&mut self.active_tab, RightPanelTab::Laser, tr("Laser"))
                    .changed()
                {
                    self.sync_settings();
                }
                if ui
                    .selectable_value(&mut self.active_tab, RightPanelTab::Notes, "📝")
                    .changed()
                {
                    self.sync_settings();
                }
            });
        });
        ui.separator();

        egui::ScrollArea::vertical()
            .id_salt("tab_scroll")
            .show(ui, |ui| {
                match self.active_tab {
                    RightPanelTab::Cuts => {
                        // Layer List Table (to be implemented more fully)
                        ui.label(RichText::new(tr("Layers")).strong());
                        // Reuse palette for now, but vertical?
                        // We need a list view.
                        // For now, I'll put Materials here too.
                        ui.add_space(8.0);
                        let mat_context = self.materials_ui_context();
                        let mat_action = ui::materials::show_with_context(
                            ui,
                            &mut self.materials_state,
                            &mat_context,
                        );
                        self.apply_material_action(mat_action);

                        ui.separator();
                        // Show full layers list with details
                        let used_layers = self.preview_used_layer_indices();
                        let list_action = ui::cut_list::show(
                            ui,
                            &mut self.layers,
                            self.active_layer_idx,
                            &used_layers,
                        );
                        if let Some(idx) = list_action.select_layer {
                            self.active_layer_idx = idx;
                            self.drawing_state.current.layer_idx = idx;
                        }
                        if let Some(idx) = list_action.open_settings {
                            self.cut_settings_state.editing_layer_idx = Some(idx);
                            self.cut_settings_state.is_open = true;
                        }
                        if list_action.layers_changed && !self.drawing_state.shapes.is_empty() {
                            self.regenerate_drawing_gcode();
                        }
                    }
                    RightPanelTab::Move => {
                        // Machine State + Jog
                        let ms_action = ui::machine_state::show(
                            ui,
                            &self.grbl_state,
                            self.is_focus_on,
                            connected,
                            self.speed_unit,
                        );
                        if ms_action.toggle_focus && connected {
                            self.is_focus_on = !self.is_focus_on;
                            if self.is_focus_on {
                                self.send_command("M3 S10");
                            } else {
                                self.send_command("M5");
                            }
                        }
                        if let Some(pos) = ms_action.quick_pos {
                            self.quick_move_to(pos);
                        }

                        ui.add_space(8.0);
                        let jog_action = ui::jog::show(
                            ui,
                            &mut self.jog_step,
                            &mut self.jog_feed,
                            caps.supports_jog,
                            caps.supports_home,
                            self.speed_unit,
                        );
                        if let Some(dir) = jog_action.direction {
                            self.jog(dir);
                        }

                        ui.add_space(8.0);
                        // Z-Probe & Focus
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(format!("📏 {}", crate::i18n::tr("Z-Probe")))
                                        .color(theme::LAVENDER)
                                        .strong(),
                                );
                            });
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                if ui.button("⇊ Run Z-Probe").clicked() {
                                    self.send_command("G38.2 Z-50 F100");
                                    self.send_command("G92 Z0");
                                    self.send_command("G0 Z5 F500");
                                }
                                if ui.button("🎯 Focus Point").clicked() {
                                    self.send_command("G0 Z20 F1000");
                                }
                            });
                        });
                    }
                    RightPanelTab::Console => {
                        ui.push_id("classic_console", |ui| {
                            let console_action = ui::console::show(
                                ui,
                                self.console_log.make_contiguous(),
                                &mut self.console_input,
                            );
                            if let Some(cmd) = console_action.send_command {
                                self.send_command(&cmd);
                            }
                        });

                        ui.separator();
                        self.ui_right_content(ui);
                    }
                    RightPanelTab::Art => {}
                    RightPanelTab::Job => {
                        ui.label(RichText::new(tr("Job Preparation")).strong());
                        ui.add_space(4.0);
                        ui.group(|ui| {
                            let est_time_s = self.estimation.estimated_seconds;
                            let h = (est_time_s / 3600.0) as u32;
                            let m = ((est_time_s % 3600.0) / 60.0) as u32;
                            let s = (est_time_s % 60.0) as u32;
                            ui.label(
                                RichText::new(format!("⏱ Est. Time: {:02}:{:02}:{:02}", h, m, s))
                                    .strong()
                                    .color(theme::GREEN),
                            );
                            ui.add_space(4.0);
                            if ui
                                .button("⚡ Optimize Path")
                                .on_hover_text("Reorder segments to minimize travel distance")
                                .clicked()
                            {
                                if let Some(mut file) = self.loaded_file.clone() {
                                    let optimized_lines =
                                        crate::gcode::optimizer::optimize(&file.lines);
                                    let raw_lines: Vec<String> =
                                        optimized_lines.iter().map(|l| l.to_gcode()).collect();
                                    file.lines = optimized_lines;
                                    self.set_loaded_file(file, raw_lines);
                                    self.log("Path optimized.".to_string());
                                }
                            }
                        });

                        ui.add_space(8.0);
                        ui.group(|ui| {
                            ui.label(
                                RichText::new(format!("📐 {}", tr("Job Transformation")))
                                    .color(theme::LAVENDER)
                                    .strong(),
                            );
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.label(tr("Offset X:"));
                                ui.add(
                                    egui::DragValue::new(&mut self.job_transform.offset_x)
                                        .speed(1.0)
                                        .suffix(" mm"),
                                );
                                ui.label(tr("Y:"));
                                ui.add(
                                    egui::DragValue::new(&mut self.job_transform.offset_y)
                                        .speed(1.0)
                                        .suffix(" mm"),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label(tr("Rotation:"));
                                ui.add(
                                    egui::Slider::new(
                                        &mut self.job_transform.rotation,
                                        -180.0..=180.0,
                                    )
                                    .suffix("°"),
                                );
                                if ui.button(tr("Reset")).clicked() {
                                    self.job_transform.rotation = 0.0;
                                }
                            });
                            if ui.button(tr("Center Job")).clicked() {
                                if let Some(file) = &self.loaded_file {
                                    if let Some((min_x, min_y, max_x, max_y)) = file.bounds() {
                                        let mid_x = (min_x + max_x) / 2.0;
                                        let mid_y = (min_y + max_y) / 2.0;
                                        let wm_x = self.machine_profile.workspace_x_mm / 2.0;
                                        let wm_y = self.machine_profile.workspace_y_mm / 2.0;
                                        self.job_transform.offset_x = wm_x - mid_x;
                                        self.job_transform.offset_y = wm_y - mid_y;
                                    }
                                }
                            }
                        });

                        ui.add_space(8.0);
                        if ui.button(format!("🔍 {}", tr("Preflight Check"))).clicked() {
                            self.preflight_state.issues =
                                ui::preflight::run_checks(&self.drawing_state.shapes, &self.layers);
                            self.preflight_state.is_open = true;
                        }
                    }
                    RightPanelTab::Notes => {
                        ui.label(RichText::new(format!("📝 {}", tr("Project Notes"))).strong());
                        ui.add_space(4.0);
                        ui.add(
                            egui::TextEdit::multiline(&mut self.project_notes)
                                .desired_width(f32::INFINITY)
                                .desired_rows(12)
                                .hint_text(tr("Add notes about this project...")),
                        );
                    }
                    RightPanelTab::Laser => {
                        // Connection
                        let conn_action = ui::connection::show(
                            ui,
                            &self.ports,
                            &mut self.selected_port,
                            &self.baud_rates,
                            &mut self.selected_baud,
                            connected,
                            &mut self.settings.output_protocol,
                            &mut self.use_tcp,
                            &mut self.tcp_host,
                            &mut self.tcp_port_str,
                        );
                        if conn_action.connect {
                            self.connect();
                        }
                        if conn_action.connect_tcp {
                            self.connect_tcp();
                        }
                        if conn_action.disconnect {
                            self.disconnect();
                        }
                        if conn_action.refresh_ports {
                            self.ports = connection::list_ports();
                        }

                        ui.add_space(8.0);
                        ui.push_id("classic_profile_editor", |ui| {
                            self.ui_machine_profile_editor(ui);
                        });

                        ui.add_space(8.0);
                        ui.push_id("classic_macros", |ui| {
                            let macros_action = ui::macros::show(ui, &mut self.macros_state, connected);
                            if let Some(mac) = macros_action.execute_macro {
                                self.execute_macro_script(&mac.label, &mac.gcode);
                            }
                        });

                        ui.add_space(8.0);
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(tr("Preflight QA"))
                                        .color(theme::LAVENDER)
                                        .strong(),
                                );
                                if ui.button(format!("🧪 {}", tr("Run Preflight"))).clicked() {
                                    self.run_preflight("manual", false);
                                }
                            });
                            ui.checkbox(
                                &mut self.preflight_block_critical,
                                tr("Block critical issues"),
                            );

                            if let Some(report) = &self.preflight_report {
                                let critical = report.critical_count();
                                let warning = report.warning_count();
                                let head_color = if critical > 0 {
                                    theme::PEACH
                                } else {
                                    theme::GREEN
                                };
                                ui.label(
                                    RichText::new(format!(
                                        "Last report: {critical} critical, {warning} warning"
                                    ))
                                    .small()
                                    .color(head_color),
                                );

                                egui::ScrollArea::vertical()
                                    .max_height(120.0)
                                    .show(ui, |ui| {
                                        for issue in &report.issues {
                                            let (icon, color) = match issue.severity {
                                                PreflightSeverity::Critical => ("⛔", theme::RED),
                                                PreflightSeverity::Warning => ("⚠", theme::PEACH),
                                            };
                                            ui.label(
                                                RichText::new(format!("{icon} {}", issue.message))
                                                    .small()
                                                    .color(color),
                                            );
                                        }
                                        if report.issues.is_empty() {
                                            ui.label(
                                                RichText::new(format!("✅ {}", tr("No preflight issues detected.")))
                                                    .small()
                                                    .color(theme::GREEN),
                                            );
                                        }
                                    });
                            } else {
                                ui.label(
                                    RichText::new(tr("No preflight report yet."))
                                        .small()
                                        .color(theme::SUBTEXT),
                                );
                            }
                        });
                    }
                }
            });
    }

    fn update_modals(&mut self, ctx: &egui::Context) {
        // === Handle Cut Settings Modal ===
        {
            let action = ui::cut_settings::show(
                ctx,
                &mut self.cut_settings_state,
                &self.layers,
                &self.drawing_state.shapes,
                self.speed_unit,
            );
            if let Some((idx, new_layer)) = action.apply {
                if idx < self.layers.len() {
                    self.layers[idx] = new_layer;
                    if !self.drawing_state.shapes.is_empty() {
                        self.regenerate_drawing_gcode();
                    }
                }
            }
        }

        // === Handle Settings Modal ===
        let supports_grbl_settings = self.controller_capabilities().supports_grbl_settings;
        let mut settings_write_blocked = false;
        if let Some(state) = &mut self.settings_state {
            ui::settings_dialog::show(ctx, state);
            if !state.is_open {
                self.settings_state = None;
            } else {
                if !state.pending_writes.is_empty() {
                    if !supports_grbl_settings {
                        settings_write_blocked = true;
                        state.pending_writes.clear();
                    } else {
                        let writes = std::mem::take(&mut state.pending_writes);
                        for (id, val) in writes {
                            if id == -1 && val == "$$" {
                                self.send_command("$$");
                            } else {
                                self.send_command(&format!("${}={}", id, val));
                            }
                        }
                    }
                }
            }
        }
        if settings_write_blocked {
            self.log(format!(
                "Settings write is not supported by {} backend.",
                self.machine_profile.controller_kind.label()
            ));
        }
    }

    fn handle_toolbar_file_actions(
        &mut self,
        ctx: &egui::Context,
        actions: &ui::toolbar::ToolbarAction,
    ) {
        if actions.open_file {
            self.open_file();
        }
        if let Some(path) = &actions.open_recent {
            self.load_file_path(path);
        }
        if actions.save_file {
            self.save_file();
        }
        if actions.open_project {
            self.handle_open_project(ctx);
        }
        if actions.save_project {
            self.handle_save_project();
        }
    }

    fn handle_toolbar_job_actions(&mut self, actions: &ui::toolbar::ToolbarAction) {
        if actions.run_program {
            self.is_dry_run = false;
            if self.run_preflight("run", true) {
                self.run_program_internal();
            }
        }
        if actions.frame_bbox {
            self.frame_bbox();
        }
        if actions.dry_run {
            self.is_dry_run = true;
            if self.run_preflight("dry-run", true) {
                self.run_program_internal();
            } else {
                self.is_dry_run = false;
            }
        }
        if actions.abort_program {
            self.abort_program();
        }
        if actions.hold {
            self.send_realtime_or_warn(RealtimeCommand::FeedHold, "Feed hold");
        }
        if actions.resume {
            self.send_realtime_or_warn(RealtimeCommand::CycleStart, "Cycle start");
        }
    }

    fn handle_toolbar_machine_actions(&mut self, actions: &ui::toolbar::ToolbarAction) {
        let is_connected = self.is_connected();
        if actions.connect_toggle {
            if is_connected {
                self.disconnect();
            } else {
                self.connect();
            }
        }
        if actions.home {
            self.send_command("$H");
        }
        if actions.unlock {
            self.send_command("$X");
        }
        if actions.set_zero {
            self.send_command("G92X0Y0Z0");
        }
        if actions.reset {
            self.send_realtime_or_warn(RealtimeCommand::Reset, "Reset");
        }
    }

    fn handle_toolbar_ui_actions(
        &mut self,
        ctx: &egui::Context,
        actions: &ui::toolbar::ToolbarAction,
    ) {
        if let Some(t) = &actions.set_theme {
            self.ui_theme = *t;
            self.apply_theme(ctx);
            self.sync_settings();
        }
        if let Some(l) = &actions.set_layout {
            self.ui_layout = *l;
            self.sync_settings();
        }
        if let Some(lang) = &actions.set_language {
            self.language = *lang;
            crate::i18n::set_language(*lang);
            self.sync_settings();
        }
        if actions.toggle_light_mode {
            self.light_mode = !self.light_mode;
            self.apply_theme(ctx);
            self.sync_settings();
        }
        if actions.toggle_beginner_mode {
            self.beginner_mode = !self.beginner_mode;
            self.sync_settings();
        }
        if actions.zoom_in {
            self.renderer.zoom_in();
        }
        if actions.zoom_out {
            self.renderer.zoom_out();
        }
        if actions.undo {
            if !self.undo_node_edit() {
                self.log("Nothing to undo.".into());
            }
        }
        if actions.redo {
            if !self.redo_node_edit() {
                self.log("Nothing to redo.".into());
            }
        }
        if actions.open_about {
            self.about_open = true;
        }
    }

    fn handle_toolbar_tool_actions(&mut self, actions: &ui::toolbar::ToolbarAction) {
        if actions.open_power_speed_test {
            self.power_speed_test.is_open = true;
        }
        if actions.open_gcode_editor {
            self.gcode_editor.is_open = true;
        }
        if actions.open_shortcuts {
            self.shortcuts.is_open = true;
        }
        if actions.open_tiling {
            self.tiling.is_open = true;
        }
        if actions.open_nesting {
            self.nesting_state.is_open = true;
        }
        if actions.open_job_queue {
            self.job_queue_state.is_open = true;
        }
        if actions.open_test_fire {
            self.test_fire.is_open = true;
        }
        if actions.open_settings {
            if self.settings_state.is_none() {
                let mut state = ui::settings_dialog::SettingsDialogState::default();
                state.is_open = true;
                state.pending_writes.push((-1, "$$".to_string()));
                self.settings_state = Some(state);
            }
        }
    }

    fn handle_toolbar_export_actions(&mut self, actions: &ui::toolbar::ToolbarAction) {
        if actions.export_lbrn2 {
            self.handle_export_lbrn2();
        }
        if actions.export_svg {
            self.handle_export_svg();
        }
        if actions.export_job_report {
            self.handle_export_job_report();
        }
        if actions.save_job_template {
            self.handle_save_job_template();
        }
        if actions.load_job_template {
            self.handle_load_job_template();
        }
    }

    fn handle_toolbar_actions(&mut self, ctx: &egui::Context, actions: ui::toolbar::ToolbarAction) {
        self.handle_toolbar_file_actions(ctx, &actions);
        self.handle_toolbar_job_actions(&actions);
        self.handle_toolbar_machine_actions(&actions);
        self.handle_toolbar_ui_actions(ctx, &actions);
        self.handle_toolbar_tool_actions(&actions);
        self.handle_toolbar_export_actions(&actions);
    }

    fn handle_save_job_template(&mut self) {
        let name = format!("template_{}", self.layers.len());
        let template = crate::config::project::JobTemplate {
            name: name.clone(),
            layers: self.layers.clone(),
            description: String::new(),
        };
        match crate::config::project::JobTemplate::save(&template) {
            Ok(()) => self.log(format!("Layer template saved: {name}")),
            Err(e) => self.show_error(format!("Template save failed: {e}")),
        }
    }

    fn handle_load_job_template(&mut self) {
        let names = crate::config::project::JobTemplate::list_templates();
        if names.is_empty() {
            self.show_error("No saved templates found.".into());
            return;
        }
        // Load the most recently saved template
        if let Some(name) = names.last() {
            match crate::config::project::JobTemplate::load(name) {
                Ok(template) => {
                    self.layers = template.layers;
                    self.log(format!("Loaded layer template: {}", template.name));
                    self.regenerate_drawing_gcode();
                }
                Err(e) => self.show_error(format!("Template load failed: {e}")),
            }
        }
    }

    fn handle_export_job_report(&mut self) {
        let filename = self.loaded_file
            .as_ref()
            .map(|f| f.filename.as_str())
            .unwrap_or("unknown");
        let csv = crate::gcode::estimation::generate_job_report_csv(
            filename,
            &self.estimation,
            &self.layers,
            &self.machine_profile.name,
            self.program_lines.len(),
        );
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("CSV Report", &["csv"])
            .set_file_name("job_report.csv")
            .save_file()
        {
            match std::fs::write(&path, &csv) {
                Ok(()) => self.log(format!("Job report exported: {}", path.display())),
                Err(e) => self.show_error(format!("Report export failed: {e}")),
            }
        }
    }

    fn handle_export_lbrn2(&mut self) {
        if self.drawing_state.shapes.is_empty() {
            self.show_error("No shapes to export.".into());
            return;
        }
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("LightBurn Project", &["lbrn2"])
            .set_file_name("export.lbrn2")
            .save_file()
        {
            let xml = crate::gcode::lbrn_import::export_lbrn2(
                &self.drawing_state.shapes,
                &self.layers,
            );
            match std::fs::write(&path, &xml) {
                Ok(()) => self.log(format!("Exported .lbrn2: {}", path.display())),
                Err(e) => self.show_error(format!("Export failed: {e}")),
            }
        }
    }

    fn handle_export_svg(&mut self) {
        if self.drawing_state.shapes.is_empty() {
            self.show_error("No shapes to export.".into());
            return;
        }
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("SVG", &["svg"])
            .set_file_name("export.svg")
            .save_file()
        {
            let svg = crate::ui::drawing::export_shapes_to_svg(
                &self.drawing_state.shapes,
                &self.layers,
            );
            match std::fs::write(&path, &svg) {
                Ok(()) => self.log(format!("Exported SVG: {}", path.display())),
                Err(e) => self.show_error(format!("SVG export failed: {e}")),
            }
        }
    }

    fn handle_open_project(&mut self, ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("All4Laser Project", &["a4l"])
            .pick_file()
        {
            let path_str = path.to_string_lossy().to_string();
            match crate::config::project::ProjectFile::load(&path_str) {
                Ok(proj) => {
                    self.job_transform.offset_x = proj.offset_x;
                    self.job_transform.offset_y = proj.offset_y;
                    self.job_transform.rotation = proj.rotation_deg;
                    if let Some(mp) = proj.machine_profile {
                        let previous_kind = self.machine_profile.controller_kind;
                        self.machine_profile = mp;
                        self.apply_controller_kind_change(previous_kind);
                    }
                    self.camera_state.enabled = proj.camera_enabled;
                    self.camera_state.opacity = proj.camera_opacity;
                    self.camera_state.calibration = proj.camera_calibration;
                    self.camera_state.device_index = proj.camera_device_index;
                    self.camera_state.live_streaming = proj.camera_live_streaming;
                    self.camera_state.snapshot_path = proj.camera_snapshot_path.clone();
                    if let Some(preset_name) = proj.material_selected_preset.as_deref() {
                        self.materials_state.select_preset_by_name(preset_name);
                    }
                    if self.camera_state.live_streaming {
                        self.start_live_camera();
                    } else if let Some(path) = proj.camera_snapshot_path {
                        if self.load_camera_snapshot_from_path(ctx, &path).is_err() {
                            self.camera_state.texture = None;
                        }
                    } else {
                        self.camera_state.texture = None;
                    }
                    if let Some(gc_path) = proj.gcode_path {
                        self.load_file_path(&gc_path);
                    } else if let Some(content) = proj.gcode_content {
                        let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
                        let file = GCodeFile::from_lines("project", &lines);
                        self.set_loaded_file(file, lines);
                    }
                    self.sync_settings();
                    self.log("Project loaded.".into());
                }
                Err(e) => self.show_error(format!("Project load failed: {e}")),
            }
        }
    }

    fn handle_save_project(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("All4Laser Project", &["a4l"])
            .set_file_name("project.a4l")
            .save_file()
        {
            let path_str = path.to_string_lossy().to_string();
            let proj = crate::config::project::ProjectFile {
                version: 1,
                gcode_content: Some(self.program_lines.join("\n")),
                gcode_path: None,
                offset_x: self.job_transform.offset_x,
                offset_y: self.job_transform.offset_y,
                rotation_deg: self.job_transform.rotation,
                machine_profile: Some(self.machine_profile.clone()),
                camera_enabled: self.camera_state.enabled,
                camera_opacity: self.camera_state.opacity,
                camera_calibration: self.camera_state.calibration.clone(),
                camera_snapshot_path: self.camera_state.snapshot_path.clone(),
                camera_device_index: self.camera_state.device_index,
                camera_live_streaming: self.camera_state.live_streaming,
                material_selected_preset: self
                    .materials_state
                    .selected_preset_name()
                    .map(str::to_string),
                checkpoint_line: None,
                project_notes: self.project_notes.clone(),
            };
            match crate::config::project::ProjectFile::save(&path_str, &proj) {
                Ok(_) => {
                    // Record revision in project history (F110)
                    let mut history = crate::config::project::ProjectHistory::load(&path_str);
                    history.add_revision("Project saved");
                    let _ = history.save(&path_str);
                    self.log(format!("Project saved: {path_str}"));
                }
                Err(e) => self.show_error(format!("Project save failed: {e}")),
            }
        }
    }

    fn update_preview(&mut self, ctx: &egui::Context) {
        // Show loading overlay for background LightBurn import
        if let Some(msg) = &self.lbrn_loading_msg {
            egui::Area::new(egui::Id::new("lbrn_loading_overlay"))
                .order(egui::Order::Foreground)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    egui::Frame::popup(ui.style())
                        .inner_margin(20.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.spinner();
                                ui.label(RichText::new(msg).size(16.0));
                            });
                        });
                });
        }

        CentralPanel::default().show(ctx, |ui| {
            let segments = self
                .loaded_file
                .as_ref()
                .map(|f| {
                    f.segments
                        .iter()
                        .filter(|seg| {
                            f.layers
                                .get(seg.layer_id)
                                .map(|l| l.visible)
                                .unwrap_or(true)
                        })
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            let offset = egui::vec2(self.job_transform.offset_x, self.job_transform.offset_y);

            let preview_action = ui::preview_panel::show(
                ui,
                &mut self.renderer,
                &segments,
                &self.drawing_state.shapes,
                &self.layers,
                self.light_mode,
                offset,
                self.job_transform.rotation,
                &mut self.camera_state,
            );

            if preview_action.zoom_in {
                self.renderer.zoom_in();
            }
            if preview_action.zoom_out {
                self.renderer.zoom_out();
            }
            if preview_action.auto_fit {
                if let Some(file) = self.loaded_file.as_ref() {
                    let segments = file.segments.clone();
                    let rect = ui.max_rect();
                    self.renderer
                        .auto_fit(&segments, rect, offset, self.job_transform.rotation);
                }
            }

            if !matches!(
                preview_action.interactive_action,
                crate::preview::renderer::InteractiveAction::MoveNode { .. }
                    | crate::preview::renderer::InteractiveAction::MoveNodeHandle { .. }
            ) {
                self.node_move_undo_armed = true;
            }

            if !matches!(
                preview_action.interactive_action,
                crate::preview::renderer::InteractiveAction::DragSelection { .. }
                    | crate::preview::renderer::InteractiveAction::RotateSelection { .. }
            ) {
                self.shape_transform_undo_armed = true;
            }

            self.handle_interactive_action(preview_action.interactive_action);

            self.renderer.machine_pos =
                egui::Pos2::new(self.grbl_state.wpos.x, self.grbl_state.wpos.y);
        });
    }

    fn handle_interactive_action(&mut self, action: crate::preview::renderer::InteractiveAction) {
        use crate::preview::renderer::InteractiveAction;
        match action {
            InteractiveAction::SelectShape(idx, _is_multi) => {
                if let Some(shape) = self.drawing_state.shapes.get(idx) {
                    self.drawing_state.current = shape.clone();
                }
                // Auto-expand selection to include all shapes in the same group
                let group_indices = crate::ui::drawing::expand_group_selection(
                    &self.drawing_state.shapes, idx,
                );
                if group_indices.len() > 1 {
                    for gi in group_indices {
                        self.renderer.selected_shape_idx.insert(gi);
                    }
                }
            }
            InteractiveAction::Deselect => {}
            InteractiveAction::DragSelection { delta } => {
                if self.shape_transform_undo_armed {
                    self.push_node_undo_snapshot();
                    self.shape_transform_undo_armed = false;
                }
                for &idx in &self.renderer.selected_shape_idx {
                    if let Some(shape) = self.drawing_state.shapes.get_mut(idx) {
                        shape.x += delta.x;
                        shape.y += delta.y;
                    }
                }
                self.regenerate_drawing_gcode();
            }
            InteractiveAction::RotateSelection {
                shape_idx,
                delta_deg,
            } => {
                if self.shape_transform_undo_armed {
                    self.push_node_undo_snapshot();
                    self.shape_transform_undo_armed = false;
                }
                if let Some(shape) = self.drawing_state.shapes.get_mut(shape_idx) {
                    let local_center = shape.local_center();
                    let (old_cx, old_cy) = shape.world_pos(local_center.0, local_center.1);
                    shape.rotation += delta_deg;
                    shape.rotation = (shape.rotation + 360.0) % 360.0;
                    let (new_cx, new_cy) = shape.world_pos(local_center.0, local_center.1);
                    shape.x += old_cx - new_cx;
                    shape.y += old_cy - new_cy;
                    self.regenerate_drawing_gcode();
                }
            }
            InteractiveAction::MoveNode {
                shape_idx,
                node_idx,
                new_pos,
            } => {
                if self.node_move_undo_armed {
                    self.push_node_undo_snapshot();
                    self.node_move_undo_armed = false;
                }
                if let Some(shape_ro) = self.drawing_state.shapes.get(shape_idx) {
                    if let ShapeKind::Path(pts_ro) = &shape_ro.shape {
                        if let Some(p0) = pts_ro.get(node_idx) {
                            let (cur_wx, cur_wy) = shape_ro.world_pos(p0.0, p0.1);
                            let dx = new_pos.x - cur_wx;
                            let dy = new_pos.y - cur_wy;
                            let angle = shape_ro.rotation.to_radians();
                            let (sin_a, cos_a) = angle.sin_cos();
                            let dlx = dx * cos_a + dy * sin_a;
                            let dly = -dx * sin_a + dy * cos_a;

                            let mut selected_nodes: Vec<usize> = self
                                .renderer
                                .selected_nodes
                                .iter()
                                .filter_map(|(s, n)| if *s == shape_idx { Some(*n) } else { None })
                                .collect();
                            selected_nodes.sort_unstable();
                            selected_nodes.dedup();
                            if selected_nodes.is_empty() {
                                selected_nodes.push(node_idx);
                            }

                            if let Some(shape_rw) = self.drawing_state.shapes.get_mut(shape_idx) {
                                if let ShapeKind::Path(pts_rw) = &mut shape_rw.shape {
                                    for idx in selected_nodes {
                                        if let Some(p) = pts_rw.get_mut(idx) {
                                            p.0 += dlx;
                                            p.1 += dly;
                                        }
                                    }
                                }
                            }
                            self.regenerate_drawing_gcode();
                        }
                    }
                }
            }
            InteractiveAction::MoveNodeHandle {
                shape_idx,
                node_idx,
                handle,
                new_pos,
            } => {
                if self.node_move_undo_armed {
                    self.push_node_undo_snapshot();
                    self.node_move_undo_armed = false;
                }
                if let Some(shape_ro) = self.drawing_state.shapes.get(shape_idx) {
                    let (lx, ly) = Self::world_to_shape_local(shape_ro, new_pos.x, new_pos.y);
                    if let Some(shape_rw) = self.drawing_state.shapes.get_mut(shape_idx) {
                        if let ShapeKind::Path(pts) = &mut shape_rw.shape {
                            let target_idx = match handle {
                                crate::preview::renderer::NodeHandleKind::In => {
                                    node_idx.saturating_sub(1)
                                }
                                crate::preview::renderer::NodeHandleKind::Out => {
                                    node_idx.saturating_add(1)
                                }
                            };
                            if target_idx < pts.len() {
                                if let Some(p) = pts.get_mut(target_idx) {
                                    p.0 = lx;
                                    p.1 = ly;
                                    self.regenerate_drawing_gcode();
                                }
                            }
                        }
                    }
                }
            }
            InteractiveAction::AddNode {
                shape_idx,
                insert_after,
                new_pos,
            } => {
                self.push_node_undo_snapshot();
                if let Some(shape) = self.drawing_state.shapes.get(shape_idx) {
                    let (lx, ly) = Self::world_to_shape_local(shape, new_pos.x, new_pos.y);
                    match ui::vector_edit::insert_node_on_segment(
                        &mut self.drawing_state,
                        shape_idx,
                        insert_after,
                        (lx, ly),
                    ) {
                        Ok(insert_idx) => {
                            self.renderer.selected_shape_idx.clear();
                            self.renderer.selected_shape_idx.insert(shape_idx);
                            self.renderer.selected_node = Some((shape_idx, insert_idx));
                            self.renderer.selected_nodes.clear();
                            self.renderer.selected_nodes.insert((shape_idx, insert_idx));
                            self.regenerate_drawing_gcode();
                        }
                        Err(e) => self.log(format!("Add node failed: {e}")),
                    }
                }
            }
            InteractiveAction::CameraPickPoint(pos) => {
                self.handle_camera_pick_point(pos);
            }
            InteractiveAction::GroupSelection => {
                let selected_indices: Vec<usize> =
                    self.renderer.selected_shape_idx.iter().copied().collect();
                crate::ui::drawing::group_shapes(&mut self.drawing_state.shapes, &selected_indices);
            }
            InteractiveAction::ContextCopy => {
                self.copy_selected_shapes_to_clipboard();
            }
            InteractiveAction::ContextCut => {
                self.push_node_undo_snapshot();
                self.copy_selected_shapes_to_clipboard();
                self.delete_selected_shapes();
                self.regenerate_drawing_gcode();
            }
            InteractiveAction::ContextPaste => {
                self.handle_paste_selection();
            }
            InteractiveAction::ContextDeleteSelection => {
                self.push_node_undo_snapshot();
                let mut indices: Vec<usize> = self.renderer.selected_shape_idx.iter().copied().collect();
                indices.sort_unstable();
                for idx in indices.into_iter().rev() {
                    if idx < self.drawing_state.shapes.len() {
                        self.drawing_state.shapes.remove(idx);
                    }
                }
                self.renderer.selected_shape_idx.clear();
                self.renderer.selected_node = None;
                self.renderer.selected_nodes.clear();
                self.regenerate_drawing_gcode();
            }
            InteractiveAction::UngroupSelection => {
                let selected_indices: Vec<usize> =
                    self.renderer.selected_shape_idx.iter().copied().collect();
                crate::ui::drawing::ungroup_shapes(
                    &mut self.drawing_state.shapes,
                    &selected_indices,
                );
            }
            InteractiveAction::ContextDuplicateSelection => {
                self.push_node_undo_snapshot();
                let indices: Vec<usize> = self.renderer.selected_shape_idx.iter().copied().collect();
                let mut new_indices = Vec::with_capacity(indices.len());
                self.drawing_state.shapes.reserve(indices.len());
                for idx in &indices {
                    if let Some(shape) = self.drawing_state.shapes.get(*idx) {
                        let mut dup = shape.clone();
                        dup.x += 5.0;
                        dup.y += 5.0;
                        let new_idx = self.drawing_state.shapes.len();
                        self.drawing_state.shapes.push(dup);
                        new_indices.push(new_idx);
                    }
                }
                self.renderer.selected_shape_idx.clear();
                for ni in new_indices {
                    self.renderer.selected_shape_idx.insert(ni);
                }
                self.regenerate_drawing_gcode();
            }
            InteractiveAction::ContextGroupSelection => {
                self.push_node_undo_snapshot();
                let selected_indices: Vec<usize> =
                    self.renderer.selected_shape_idx.iter().copied().collect();
                crate::ui::drawing::group_shapes(&mut self.drawing_state.shapes, &selected_indices);
            }
            InteractiveAction::ContextUngroupSelection => {
                self.push_node_undo_snapshot();
                let selected_indices: Vec<usize> =
                    self.renderer.selected_shape_idx.iter().copied().collect();
                crate::ui::drawing::ungroup_shapes(
                    &mut self.drawing_state.shapes,
                    &selected_indices,
                );
            }
            InteractiveAction::ContextSelectAll => {
                self.renderer.selected_shape_idx.clear();
                for i in 0..self.drawing_state.shapes.len() {
                    self.renderer.selected_shape_idx.insert(i);
                }
                if let Some(last) = self.drawing_state.shapes.last() {
                    self.drawing_state.current = last.clone();
                }
            }
            _ => {}
        }
    }

    fn update_import_modal(&mut self, ctx: &egui::Context) {
        if let Some(mut state) = self.import_state.take() {
            if state.needs_texture_update {
                if let ui::image_dialog::ImportType::Raster(base_img) = &state.import_type {
                    let processed =
                        imaging::raster::preprocess_image_rgba(base_img, &state.raster_params);
                    let rgba = processed.to_rgba8();
                    let color_image = egui::ColorImage::from_rgba_unmultiplied(
                        [rgba.width() as _, rgba.height() as _],
                        rgba.as_flat_samples().as_slice(),
                    );
                    state.texture =
                        Some(ctx.load_texture(&state.filename, color_image, Default::default()));
                    state.needs_texture_update = false;
                }
            }

            let mut open = true;
            let mut import_triggered = false;
            let mut cancel_triggered = false;

            egui::Window::new("Import Settings")
                .open(&mut open)
                .collapsible(false)
                .resizable(true)
                .default_width(600.0)
                .show(ctx, |ui| {
                    let res = ui::image_dialog::show(ui, &mut state, self.speed_unit);
                    if res.imported {
                        import_triggered = true;
                    }
                    if res.cancel {
                        cancel_triggered = true;
                    }
                });

            if !open || cancel_triggered {
                self.import_state = None;
            } else if import_triggered {
                match &state.import_type {
                    ui::image_dialog::ImportType::Raster(img) => {
                        if state.vectorize {
                            let mut vector_shapes =
                                crate::imaging::tracing::trace_image(img, &state.raster_params);
                            for shape in vector_shapes.iter_mut() {
                                shape.layer_idx = self.active_layer_idx;
                            }
                            self.drawing_state.shapes.extend(vector_shapes);
                        } else {
                            let shape = crate::ui::drawing::ShapeParams {
                                shape: crate::ui::drawing::ShapeKind::RasterImage {
                                    data: crate::ui::drawing::ImageData(std::sync::Arc::new(
                                        img.clone(),
                                    )),
                                    params: state.raster_params.clone(),
                                },
                                x: 0.0,
                                y: 0.0,
                                width: state.raster_params.width_mm,
                                height: state.raster_params.height_mm,
                                ..Default::default()
                            };
                            self.drawing_state.shapes.push(shape);
                        }
                        self.regenerate_drawing_gcode();
                    }
                    ui::image_dialog::ImportType::Svg(data) => {
                        match imaging::svg::svg_to_paths(data, &state.svg_params) {
                            Ok(paths) => {
                                for (path_data, layer_idx) in paths {
                                    let shape = crate::ui::drawing::ShapeParams {
                                        shape: crate::ui::drawing::ShapeKind::Path(path_data),
                                        layer_idx,
                                        ..Default::default()
                                    };
                                    self.drawing_state.shapes.push(shape);
                                }
                                self.regenerate_drawing_gcode();
                            }
                            Err(e) => self.log(format!("SVG Conversion failed: {e}")),
                        }
                    }
                }
                self.import_state = None;
            } else {
                self.import_state = Some(state);
            }
            if self.notify_sound_enabled {
                crate::ui::sound::play_notification_sound();
            }
        }
    }

    fn update_tool_windows(&mut self, ctx: &egui::Context) {
        // === Power/Speed Test Window ===
        {
            let pst_action = ui::power_speed_test::show(ctx, &mut self.power_speed_test, self.speed_unit);
            if let Some(lines) = pst_action.generate {
                let file = crate::gcode::file::GCodeFile::from_lines("pwr_speed_test", &lines);
                self.set_loaded_file(file, lines);
            }
        }

        // === GCode Editor Window ===
        {
            let ed_action = ui::gcode_editor::show(ctx, &mut self.gcode_editor);
            if let Some(lines) = ed_action.apply {
                let file = GCodeFile::from_lines("edited", &lines);
                self.log(format!("GCode editor applied ({} lines)", lines.len()));
                self.program_lines = std::sync::Arc::new(lines);
                self.program_index = 0;
                self.loaded_file = Some(file);
                self.needs_auto_fit = true;
            }
        }

        // === Shortcuts Panel ===
        {
            if !ctx.wants_keyboard_input() {
                if ctx.input(|i| i.key_pressed(egui::Key::Questionmark)) {
                    self.shortcuts.is_open = true;
                }
            }
            egui::Area::new(egui::Id::new("shortcuts_area"))
                .interactable(false)
                .fixed_pos(egui::pos2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui::shortcuts::show(ui, &mut self.shortcuts);
                });
        }

        // === Tiling Window ===
        {
            let tile_action = ui::tiling::show(ctx, &mut self.tiling, &self.program_lines);
            if let Some(lines) = tile_action.apply {
                let file = GCodeFile::from_lines("tiled", &lines);
                self.set_loaded_file(file, lines);
                self.log("Tiling applied.".into());
            }
        }

        // === Auto Nesting Window ===
        {
            let selection = self.selected_shape_indices();
            let workspace = egui::vec2(
                self.machine_profile.workspace_x_mm,
                self.machine_profile.workspace_y_mm,
            );
            let nesting_action = ui::nesting::show(
                ctx,
                &mut self.nesting_state,
                selection.len(),
                self.drawing_state.shapes.len(),
                workspace,
            );
            if nesting_action.apply {
                let result = ui::nesting::apply_nesting(
                    &mut self.drawing_state,
                    &selection,
                    workspace,
                    self.nesting_state.options(),
                );
                if result.placed > 0 {
                    self.regenerate_drawing_gcode();
                    self.needs_auto_fit = true;
                    self.log(format!(
                        "Nesting applied: {} placed, {} skipped ({} rotated).",
                        result.placed, result.skipped, result.used_rotation
                    ));
                } else {
                    self.show_error(
                        "Nesting could not place any shape in current workspace/margins.".into(),
                    );
                }
            }
        }

        // === Job Queue Window ===
        {
            let active_name = self.active_queue_job.as_ref().map(|job| job.name.as_str());
            let queue_action = ui::job_queue::show(
                ctx,
                &mut self.job_queue_state,
                !self.program_lines.is_empty(),
                self.running,
                active_name,
            );
            if queue_action.enqueue_current {
                self.enqueue_current_job();
            }
            if queue_action.retry_last_failed {
                if let Some(id) = self.job_queue_state.retry_last_failed() {
                    self.log(format!("Requeued last failed job as #{id}."));
                } else {
                    self.show_error("No failed/aborted job in history to retry.".into());
                }
            }
            if queue_action.start_next {
                self.try_start_next_queued_job();
            }
        }

        // === Circular Array Window ===
        {
            let selection: Vec<usize> = self.renderer.selected_shape_idx.iter().cloned().collect();
            let ca_action = ui::circular_array::show(ctx, &mut self.circular_array_state);
            if ca_action.apply {
                ui::circular_array::apply_array(
                    &self.circular_array_state,
                    &mut self.drawing_state,
                    &selection,
                );
                self.regenerate_drawing_gcode();
                self.log("Circular array applied.".into());
            }
        }

        // === Grid Array Window ===
        {
            let selection: Vec<usize> = self.renderer.selected_shape_idx.iter().cloned().collect();
            let ga_action = ui::grid_array::show(ctx, &mut self.grid_array_state);
            if ga_action.apply {
                ui::grid_array::apply_array(
                    &self.grid_array_state,
                    &mut self.drawing_state,
                    &selection,
                );
                self.regenerate_drawing_gcode();
                self.log("Grid array applied.".into());
            }
        }

        // === Offset Window ===
        {
            let selection: Vec<usize> = self.renderer.selected_shape_idx.iter().cloned().collect();
            let off_action = ui::offset::show(ctx, &mut self.offset_state);
            if off_action.apply {
                ui::offset::apply_offset(&self.offset_state, &mut self.drawing_state, &selection);
                self.regenerate_drawing_gcode();
                self.log("Offset applied.".into());
            }
        }

        // === Boolean Ops Window ===
        {
            let selection: Vec<usize> = self.renderer.selected_shape_idx.iter().cloned().collect();
            let bool_action = ui::boolean_ops::show(ctx, &mut self.boolean_ops_state);
            if bool_action.apply {
                ui::boolean_ops::apply_boolean(
                    &self.boolean_ops_state,
                    &mut self.drawing_state,
                    &selection,
                );
                self.regenerate_drawing_gcode();
                self.log("Boolean operation applied.".into());
            }
        }

        // === Test Fire Window ===
        if self.test_fire.is_open {
            let connected = self.is_connected();
            let mut close_tf = false;
            egui::Window::new("🔥 Test Fire")
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    egui::Grid::new("tf_grid")
                        .num_columns(2)
                        .spacing([8.0, 4.0])
                        .show(ui, |ui| {
                            ui.label("Power (%):");
                            {
                                let mut pct = self.test_fire.power / 10.0;
                                if ui.add(
                                    egui::DragValue::new(&mut pct)
                                        .range(0.1..=100.0)
                                        .speed(0.5)
                                        .suffix("%"),
                                ).changed() {
                                    self.test_fire.power = (pct * 10.0).clamp(0.0, 1000.0);
                                }
                            }
                            ui.end_row();
                            ui.label("Duration (ms):");
                            ui.add(
                                egui::DragValue::new(&mut self.test_fire.duration_ms)
                                    .range(10.0..=5000.0)
                                    .speed(10.0),
                            );
                            ui.end_row();
                        });
                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled(
                                connected,
                                egui::Button::new(
                                    egui::RichText::new("🔥 Fire").color(theme::RED).strong(),
                                ),
                            )
                            .clicked()
                        {
                            let pow = self.test_fire.power;
                            let secs = self.test_fire.duration_ms / 1000.0;
                            self.send_command(&format!("M3 S{:.0}", pow));
                            self.send_command(&format!("G4 P{:.3}", secs));
                            self.send_command("M5");
                        }
                        if ui.button("Close").clicked() {
                            close_tf = true;
                        }
                    });
                });
            if close_tf {
                self.test_fire.is_open = false;
            }
        }

        // === Feed/Spindle Override Window ===
        if self.running {
            egui::Window::new("⚡ Overrides")
                .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-10.0, -50.0))
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new("Feed Override").small());
                    if ui
                        .add(
                            egui::Slider::new(&mut self.feed_override_pct, 10.0..=200.0)
                                .suffix("%"),
                        )
                        .drag_stopped()
                    {
                        let val = self.feed_override_pct as u8;
                        self.send_feed_override(val);
                    }
                    ui.label(egui::RichText::new("Spindle Override").small());
                    if ui
                        .add(
                            egui::Slider::new(&mut self.spindle_override_pct, 10.0..=200.0)
                                .suffix("%"),
                        )
                        .drag_stopped()
                    {
                        let val = self.spindle_override_pct as u8;
                        self.send_spindle_override(val);
                    }
                    if ui.button("Reset").clicked() {
                        self.feed_override_pct = 100.0;
                        self.spindle_override_pct = 100.0;
                        self.send_feed_override(100);
                        self.send_spindle_override(100);
                    }
                });
        }
    }
}

impl eframe::App for All4LaserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_background_updates(ctx);
        self.process_input(ctx);
        self.process_session_lifecycle(ctx);
        self.process_notifications(ctx);
        self.process_modals(ctx);
        self.render_ui_layout(ctx);
    }
}

/// Simple timestamp without external crate
fn chrono_lite() -> String {
    let elapsed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = elapsed.as_secs() % 86400;
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(
        id: f32,
        node: Option<(usize, usize)>,
        selected_nodes: Vec<(usize, usize)>,
    ) -> NodeEditSnapshot {
        let shape = ShapeParams {
            shape: ShapeKind::Path(crate::ui::drawing::PathData::from_points(vec![(id, 0.0), (id + 1.0, 1.0)])),
            ..ShapeParams::default()
        };
        NodeEditSnapshot {
            shapes: vec![shape],
            selected_shape_idx: vec![0],
            selected_node: node,
            selected_nodes,
        }
    }

    #[test]
    fn node_undo_redo_history_restores_expected_snapshots() {
        let mut undo = VecDeque::from(vec![snapshot(1.0, Some((0, 0)), vec![(0, 0)])]);
        let mut redo = VecDeque::new();
        let current = snapshot(2.0, Some((0, 1)), vec![(0, 0), (0, 1)]);

        let prev = undo_history_step(&mut undo, &mut redo, current.clone())
            .expect("undo should produce previous state");
        assert_eq!(prev.selected_node, Some((0, 0)));
        assert_eq!(redo.len(), 1);
        assert_eq!(redo[0].selected_nodes.len(), 2);

        let next = redo_history_step(&mut undo, &mut redo, prev.clone())
            .expect("redo should restore current state");
        assert_eq!(next.selected_node, Some((0, 1)));
        assert_eq!(next.selected_nodes.len(), 2);
    }

    #[test]
    fn undo_on_empty_stack_returns_none() {
        let mut undo: VecDeque<NodeEditSnapshot> = VecDeque::new();
        let mut redo: VecDeque<NodeEditSnapshot> = VecDeque::new();
        let current = snapshot(1.0, None, vec![]);
        assert!(undo_history_step(&mut undo, &mut redo, current).is_none());
    }

    #[test]
    fn redo_on_empty_stack_returns_none() {
        let mut undo: VecDeque<NodeEditSnapshot> = VecDeque::new();
        let mut redo: VecDeque<NodeEditSnapshot> = VecDeque::new();
        let current = snapshot(1.0, None, vec![]);
        assert!(redo_history_step(&mut undo, &mut redo, current).is_none());
    }

    #[test]
    fn vecdeque_log_respects_max_capacity() {
        let mut log: VecDeque<String> = VecDeque::new();
        for i in 0..MAX_LOG_LINES + 50 {
            log.push_back(format!("line {i}"));
            if log.len() > MAX_LOG_LINES {
                log.pop_front();
            }
        }
        assert_eq!(log.len(), MAX_LOG_LINES);
        assert!(log.front().unwrap().contains("50"));
    }

    #[test]
    fn chrono_lite_returns_valid_timestamp() {
        let ts = chrono_lite();
        assert_eq!(ts.len(), 8); // HH:MM:SS
        assert_eq!(ts.chars().nth(2), Some(':'));
        assert_eq!(ts.chars().nth(5), Some(':'));
    }

    #[test]
    fn shape_fill_warning_flags_open_path_on_fill_layers() {
        let shape = ShapeParams {
            shape: ShapeKind::Path(crate::ui::drawing::PathData::from_points(vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0)])),
            layer_idx: 0,
            ..ShapeParams::default()
        };
        let mut layers = ui::layers_new::CutLayer::default_palette();
        layers[0].mode = ui::layers_new::CutMode::Fill;

        let warning = shape_fill_warning(&shape, &layers);
        assert_eq!(
            warning,
            Some("Fill ignored: open path. Close contour to enable fill.")
        );
    }

    #[test]
    fn shape_fill_warning_ignores_closed_path() {
        let shape = ShapeParams {
            shape: ShapeKind::Path(crate::ui::drawing::PathData::from_points(vec![
                (0.0, 0.0),
                (10.0, 0.0),
                (10.0, 10.0),
                (0.0, 10.0),
                (0.0, 0.0),
            ])),
            layer_idx: 0,
            ..ShapeParams::default()
        };
        let mut layers = ui::layers_new::CutLayer::default_palette();
        layers[0].mode = ui::layers_new::CutMode::FillAndLine;

        let warning = shape_fill_warning(&shape, &layers);
        assert_eq!(warning, None);
    }

    #[test]
    fn normalized_segment_key_is_direction_agnostic() {
        use crate::ui::preflight::normalized_segment_key;
        let a = normalized_segment_key((0.0, 0.0), (10.0, 5.0));
        let b = normalized_segment_key((10.0, 5.0), (0.0, 0.0));
        assert_eq!(a, b);
    }

    #[test]
    fn detect_cross_and_circle_markers_finds_two_components() {
        let w = 80usize;
        let h = 80usize;
        let mut rgba = vec![255u8; w * h * 4];

        let set_black = |buf: &mut [u8], x: usize, y: usize| {
            let i = (y * w + x) * 4;
            buf[i] = 0;
            buf[i + 1] = 0;
            buf[i + 2] = 0;
            buf[i + 3] = 255;
        };

        for d in 0..9usize {
            set_black(&mut rgba, 10 + d, 18);
            set_black(&mut rgba, 14, 14 + d);
        }
        for y in 48..56usize {
            for x in 52..60usize {
                set_black(&mut rgba, x, y);
            }
        }

        let markers = detect_cross_and_circle_markers(&rgba, w, h);
        assert!(markers.is_some());
    }
}
