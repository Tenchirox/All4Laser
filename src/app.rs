use std::time::{Duration, Instant};

use egui::{CentralPanel, SidePanel, TopBottomPanel, RichText};

use crate::config::machine_profile::MachineProfile;
use crate::config::recent_files::RecentFiles;
use crate::gcode::file::GCodeFile;
use crate::grbl::types::*;
use crate::grbl::protocol;
use crate::preview::renderer::PreviewRenderer;
use crate::serial::connection::{self, SerialConnection, SerialMsg};
use crate::theme;
use crate::ui;
use crate::imaging;

const MAX_LOG_LINES: usize = 500;
const STATUS_POLL_MS: u64 = 250;
const LEFT_PANEL_WIDTH: f32 = 300.0;

pub struct All4LaserApp {
    // GRBL state
    grbl_state: GrblState,

    // Serial
    connection: Option<SerialConnection>,
    ports: Vec<String>,
    selected_port: usize,
    baud_rates: Vec<u32>,
    selected_baud: usize,

    // GCode
    loaded_file: Option<GCodeFile>,
    program_lines: Vec<String>,
    program_index: usize,
    running: bool,
    is_dry_run: bool, // new flag for dry run

    // Preview
    renderer: PreviewRenderer,
    needs_auto_fit: bool,

    // Console
    console_log: Vec<String>,
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
    drawing_state: ui::drawing::DrawingState,

    // Power/Speed Test
    power_speed_test: ui::power_speed_test::PowerSpeedTestState,

    // Recent Files (MRU)
    recent_files: RecentFiles,

    // Machine Profile
    machine_profile: MachineProfile,

    // Job Transform
    job_offset_x: f32,
    job_offset_y: f32,
    job_rotation: f32, // degrees
    job_center: Option<egui::Pos2>,

    // Air Assist
    air_assist_on: bool,

    // End-of-job notification
    notify_job_done: bool,

    // Error Notification
    last_error: Option<String>,

    // GCode Editor
    gcode_editor: ui::gcode_editor::GCodeEditorState,

    // Shortcuts panel
    shortcuts: ui::shortcuts::ShortcutsState,

    // Tiling
    tiling: ui::tiling::TilingState,

    // Material Library
    materials_state: ui::materials::MaterialsState,

    // Test Fire
    test_fire_open: bool,
    test_fire_power: f32,
    test_fire_ms: f32,

    // Feed / Spindle RT override (0-100%, 100 = no change)
    feed_override_pct: f32,
    spindle_override_pct: f32,

    // Theme
    ui_theme: theme::UiTheme,
    ui_layout: theme::UiLayout,
    light_mode: bool,

    // Focus Target
    is_focus_on: bool,
    framing_active: bool,
    framing_wait_idle: bool,

    // Estimation
    estimation: crate::gcode::estimation::EstimationResult,

    // Camera
    camera_state: ui::camera::CameraState,

    // Professional Tier
    text_state: ui::text::TextToolState,
    generator_state: ui::generators::GeneratorState,

    // Layers (New System)
    layers: Vec<ui::layers_new::CutLayer>,
    active_layer_idx: usize,
    #[cfg_attr(feature = "persistence", serde(skip))]
    cut_settings_state: ui::cut_settings::CutSettingsState,

    // Timing
    last_poll: Instant,
}

impl All4LaserApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ports = connection::list_ports();

        let app = Self {
            grbl_state: GrblState::default(),
            connection: None,
            ports,
            selected_port: 0,
            baud_rates: ui::connection::default_baud_rates(),
            selected_baud: 4, // 115200
            loaded_file: None,
            program_lines: Vec::new(),
            program_index: 0,
            running: false,
            is_dry_run: false,
            renderer: PreviewRenderer::default(),
            needs_auto_fit: false,
            console_log: vec!["All4Laser ready.".to_string()],
            console_input: String::new(),
            jog_step: 1.0,
            jog_feed: 1000.0,
            import_state: None,
            settings_state: None,
            macros_state: ui::macros::MacrosState::default(),
            drawing_state: ui::drawing::DrawingState::default(),
            power_speed_test: ui::power_speed_test::PowerSpeedTestState::default(),
            recent_files: RecentFiles::load(),
            machine_profile: MachineProfile::load(),
            job_offset_x: 0.0,
            job_offset_y: 0.0,
            job_rotation: 0.0,
            job_center: None,
            air_assist_on: false,
            notify_job_done: false,
            last_error: None,
            gcode_editor: ui::gcode_editor::GCodeEditorState::default(),
            shortcuts: ui::shortcuts::ShortcutsState::default(),
            tiling: ui::tiling::TilingState::default(),
            materials_state: ui::materials::MaterialsState::default(),
            test_fire_open: false,
            test_fire_power: 10.0,
            test_fire_ms: 1000.0,
            ui_theme: theme::UiTheme::Modern,
            ui_layout: theme::UiLayout::Modern,
            light_mode: false,
            is_focus_on: false,
            framing_active: false,
            framing_wait_idle: false,
            last_poll: Instant::now(),
            feed_override_pct: 100.0,
            spindle_override_pct: 100.0,
            estimation: crate::gcode::estimation::EstimationResult::default(),
            camera_state: ui::camera::CameraState::default(),
            text_state: ui::text::TextToolState::default(),
            generator_state: ui::generators::GeneratorState::default(),
            layers: ui::layers_new::CutLayer::default_palette(),
            active_layer_idx: 0,
            cut_settings_state: ui::cut_settings::CutSettingsState::default(),
        };

        app.apply_theme(&cc.egui_ctx);
        app
    }

    pub fn apply_theme(&self, ctx: &egui::Context) {
        theme::apply_theme(ctx, &theme::AppTheme {
            ui_theme: self.ui_theme,
            is_light: self.light_mode,
        });
    }

    fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    fn log(&mut self, msg: String) {
        let timestamp = chrono_lite();
        self.console_log.push(format!("[{timestamp}] {msg}"));
        if self.console_log.len() > MAX_LOG_LINES {
            self.console_log.remove(0);
        }
    }

    fn show_error(&mut self, msg: String) {
        self.log(format!("ERROR: {}", msg));
        self.last_error = Some(msg);
    }

    fn poll_serial(&mut self) {
        // Poll for status periodically
        if self.is_connected() && self.last_poll.elapsed() > Duration::from_millis(STATUS_POLL_MS) {
            if let Some(conn) = self.connection.as_ref() {
                conn.send_byte(protocol::CMD_STATUS_REPORT);
            }
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
                SerialMsg::Response(GrblResponse::Status(state)) => {
                    let old_status = self.grbl_state.status;
                    // Store machine position in renderer for crosshair display
                    self.renderer.machine_pos = egui::pos2(state.mpos.x, state.mpos.y);
                    self.grbl_state = state;
                    
                    if self.framing_active {
                        if self.grbl_state.status == MacStatus::Run {
                            self.framing_wait_idle = true;
                        } else if self.grbl_state.status == MacStatus::Idle && old_status == MacStatus::Run && self.framing_wait_idle {
                            self.framing_wait_idle = false;
                            self.send_frame_sequence();
                        }
                    }
                }
                SerialMsg::Response(GrblResponse::Ok) => {
                    if self.running && self.program_index < self.program_lines.len() {
                        self.send_next_program_line();
                    } else if self.running && self.program_index >= self.program_lines.len() {
                        self.running = false;
                        self.is_dry_run = false;
                        // Air assist OFF
                        if self.machine_profile.air_assist {
                            self.send_command("M9");
                        }
                        self.log("Program complete.".to_string());
                        self.notify_job_done = true;
                    }
                }
                SerialMsg::Response(GrblResponse::Error(code)) => {
                    self.log(format!("error:{code}"));
                }
                SerialMsg::Response(GrblResponse::Alarm(code)) => {
                    self.log(format!("ALARM:{code}"));
                    self.running = false;
                    self.is_dry_run = false;
                }
                SerialMsg::Response(GrblResponse::GrblVersion(ver)) => {
                    self.log(format!("Grbl {ver}"));
                }
                SerialMsg::Response(GrblResponse::Setting(id, val)) => {
                    if let Some(state) = &mut self.settings_state {
                        if state.is_open {
                            state.settings.insert(id, val);
                        }
                    }
                }
                SerialMsg::RawLine(line) => {
                    self.log(line);
                }
                SerialMsg::Connected(port) => {
                    self.grbl_state.status = MacStatus::Idle;
                    self.log(format!("Connected to {port}"));
                }
                SerialMsg::Disconnected(reason) => {
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
                _ => {}
            }
        }
    }

    fn send_next_program_line(&mut self) {
        while self.program_index < self.program_lines.len() {
            let line_idx = self.program_index;
            self.program_index += 1;

            let mut cmd = if let (Some(file), Some(center)) = (&self.loaded_file, self.job_center) {
                if let Some(parsed) = file.lines.get(line_idx) {
                    let rotary_scale = if self.machine_profile.rotary_enabled && self.machine_profile.rotary_diameter_mm > 0.1 {
                        50.0 / self.machine_profile.rotary_diameter_mm // Default reference 50mm
                    } else {
                        1.0
                    };
                    parsed.transform(egui::vec2(self.job_offset_x, self.job_offset_y), self.job_rotation, center, rotary_scale)
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
        self.log(format!("Connecting to {port} @ {baud}…"));
        self.grbl_state.status = MacStatus::Connecting;

        match SerialConnection::connect(&port, baud) {
            Ok(conn) => {
                self.connection = Some(conn);
            }
            Err(e) => {
                self.grbl_state.status = MacStatus::Disconnected;
                self.show_error(format!("Connection failed: {e}"));
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
            .add_filter("All files", &["*"])
            .pick_file()
        {
            let path_str = path.to_string_lossy().to_string();
            self.load_file_path(&path_str);
        }
    }

    fn load_file_path(&mut self, path: &str) {
        let path_obj = std::path::Path::new(path);
        let ext = path_obj.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        let filename = path_obj.file_name().and_then(|n| n.to_str()).unwrap_or(path).to_string();
        
        // Push to recent files immediately (before any failure)
        self.recent_files.push(path);

        match ext.as_str() {
            "svg" => {
                let data = match std::fs::read(path) {
                    Ok(d) => d,
                    Err(e) => { self.show_error(format!("Error reading SVG: {e}")); return; }
                };
                let layers = imaging::svg::extract_layers(&data);
                let mut svg_params = imaging::svg::SvgParams::default();
                svg_params.layers = layers;
                self.import_state = Some(ui::image_dialog::ImageImportState {
                    import_type: ui::image_dialog::ImportType::Svg(data),
                    filename,
                    raster_params: imaging::raster::RasterParams::default(),
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
                    Err(e) => { self.show_error(format!("Error opening image: {e}")); return; }
                };
                
                self.import_state = Some(ui::image_dialog::ImageImportState {
                    import_type: ui::image_dialog::ImportType::Raster(image::DynamicImage::ImageRgba8(img)),
                    filename,
                    raster_params: imaging::raster::RasterParams::default(),
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
                    Err(e) => { self.show_error(format!("Error reading DXF: {e}")); return; }
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

    fn set_loaded_file(&mut self, file: GCodeFile, lines: Vec<String>) {
        self.log(format!("Loaded {} ({} lines)", file.filename, file.line_count()));
        // Populate GCode editor text
        self.gcode_editor.text = lines.join("\n");
        self.gcode_editor.dirty = false;
        self.program_lines = lines;
        self.program_index = 0;
        // Calculate job center for rotation
        if let Some((min_x, min_y, max_x, max_y)) = file.bounds() {
            self.job_center = Some(egui::Pos2::new((min_x + max_x) / 2.0, (min_y + max_y) / 2.0));
        } else {
            self.job_center = Some(egui::Pos2::ZERO);
        }
        
        self.loaded_file = Some(file);
        if let Some(file) = self.loaded_file.as_ref() {
           self.estimation = crate::gcode::estimation::estimate(&file.lines);
        }
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
            if self.loaded_file.as_ref().map(|f| f.bounds()).flatten().is_some() {
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
                    ui::machine_state::QuickPosition::Center => ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0),
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

    fn run_program(&mut self) {
        if !self.is_connected() {
            self.show_error("Not connected".to_string());
            return;
        }
        if self.program_lines.is_empty() {
            self.show_error("No file loaded".to_string());
            return;
        }
        self.program_index = 0;
        self.running = true;
        self.notify_job_done = false;
        self.framing_active = false;
        
        // Air assist ON
        if self.machine_profile.air_assist {
            self.send_command("M8");
        }
        
        // Append return-to-origin if configured
        if self.machine_profile.return_to_origin {
            if self.program_lines.last().map(|l| l.trim()) != Some("G0 X0 Y0") {
                self.program_lines.push("G0 X0 Y0 F3000".to_string());
            }
        }
        
        self.log(if self.is_dry_run { "Starting Dry Run (Laser OFF)…".to_string() } else { "Starting program…".to_string() });
        self.send_next_program_line();
    }

    fn abort_program(&mut self) {
        if let Some(conn) = self.connection.as_ref() {
            conn.send_byte(protocol::CMD_RESET);
        }
        self.running = false;
        self.is_dry_run = false;
        self.framing_active = false;
        // Air assist OFF
        if self.machine_profile.air_assist {
            self.send_command("M9");
        }
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

    /// Send GRBL real-time feed override to pct% (10-200)
    fn send_feed_override(&mut self, pct: u8) {
        if let Some(conn) = self.connection.as_ref() {
            conn.send_byte(protocol::FEED_OV_RESET);
            let diff = pct as i32 - 100;
            let tens = diff / 10;
            let ones = diff % 10;
            let (ten_cmd, one_cmd) = if diff >= 0 {
                (protocol::FEED_OV_PLUS_10, protocol::FEED_OV_PLUS_1)
            } else {
                (protocol::FEED_OV_MINUS_10, protocol::FEED_OV_MINUS_1)
            };
            for _ in 0..tens.abs() { conn.send_byte(ten_cmd); }
            for _ in 0..ones.abs() { conn.send_byte(one_cmd); }
        }
    }

    /// Send GRBL real-time spindle (laser power) override to pct% (10-200)
    fn send_spindle_override(&mut self, pct: u8) {
        if let Some(conn) = self.connection.as_ref() {
            conn.send_byte(protocol::SPINDLE_OV_RESET);
            let diff = pct as i32 - 100;
            let tens = diff / 10;
            let ones = diff % 10;
            let (ten_cmd, one_cmd) = if diff >= 0 {
                (protocol::SPINDLE_OV_PLUS_10, protocol::SPINDLE_OV_PLUS_1)
            } else {
                (protocol::SPINDLE_OV_MINUS_10, protocol::SPINDLE_OV_MINUS_1)
            };
            for _ in 0..tens.abs() { conn.send_byte(ten_cmd); }
            for _ in 0..ones.abs() { conn.send_byte(one_cmd); }
        }
    }

    fn jog(&mut self, dir: JogDirection) {
        if !self.is_connected() { return; }
        let cmd = protocol::jog_command(dir, self.jog_step, self.jog_feed);
        self.send_command(&cmd);
    }

    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        let mut jog_dir: Option<JogDirection> = None;
        let mut hold = false;
        let mut abort = false;

        ctx.input(|i| {
            if i.key_pressed(egui::Key::ArrowUp) { jog_dir = Some(JogDirection::N); }
            if i.key_pressed(egui::Key::ArrowDown) { jog_dir = Some(JogDirection::S); }
            if i.key_pressed(egui::Key::ArrowLeft) { jog_dir = Some(JogDirection::W); }
            if i.key_pressed(egui::Key::ArrowRight) { jog_dir = Some(JogDirection::E); }
            if i.key_pressed(egui::Key::PageUp) { jog_dir = Some(JogDirection::Zup); }
            if i.key_pressed(egui::Key::PageDown) { jog_dir = Some(JogDirection::Zdown); }
            if i.key_pressed(egui::Key::Home) { jog_dir = Some(JogDirection::Home); }
            if i.key_pressed(egui::Key::Space) { hold = true; }
            if i.key_pressed(egui::Key::Escape) { abort = true; }
        });

        if let Some(dir) = jog_dir {
            self.jog(dir);
        }
        if hold {
            if let Some(conn) = self.connection.as_ref() {
                conn.send_byte(protocol::CMD_FEED_HOLD);
            }
        }
        if abort {
            self.abort_program();
        }
    }

    fn handle_file_drop(&mut self, ctx: &egui::Context) {
        let dropped: Vec<String> = ctx.input(|i| {
            i.raw.dropped_files.iter()
                .filter_map(|f| f.path.as_ref().map(|p| p.to_string_lossy().to_string()))
                .collect()
        });
        for path_str in dropped {
            let ext = std::path::Path::new(&path_str)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if matches!(ext.as_str(), "nc" | "gcode" | "ngc" | "gc" | "svg" | "png" | "jpg" | "jpeg" | "bmp") {
                self.load_file_path(&path_str);
            }
        }
    }

    fn ui_left_content(&mut self, ui: &mut egui::Ui, connected: bool) {
        // Connection
        let conn_action = ui::connection::show(
            ui,
            &self.ports,
            &mut self.selected_port,
            &self.baud_rates,
            &mut self.selected_baud,
            connected,
        );
        if conn_action.connect { self.connect(); }
        if conn_action.disconnect { self.disconnect(); }
        if conn_action.refresh_ports {
            self.ports = connection::list_ports();
            self.log("Ports refreshed.".to_string());
        }

        ui.add_space(8.0);

        // Machine state
        let ms_action = ui::machine_state::show(ui, &self.grbl_state, self.is_focus_on, connected);
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

        // Machine Profile settings
        let mut profile_changed = false;
        egui::CollapsingHeader::new(egui::RichText::new("⚙ Machine Profile").color(crate::theme::LAVENDER).strong())
            .show(ui, |ui| {
                egui::Grid::new("mp_grid").num_columns(2).spacing([8.0, 4.0]).show(ui, |ui| {
                    ui.label("Name:"); 
                    if ui.text_edit_singleline(&mut self.machine_profile.name).changed() { profile_changed = true; }
                    ui.end_row();
                    ui.label("Width (mm):");
                    if ui.add(egui::DragValue::new(&mut self.machine_profile.workspace_x_mm).speed(5.0)).changed() { profile_changed = true; }
                    ui.end_row();
                    ui.label("Height (mm):");
                    if ui.add(egui::DragValue::new(&mut self.machine_profile.workspace_y_mm).speed(5.0)).changed() { profile_changed = true; }
                    ui.end_row();
                    ui.label("Max Rate X:");
                    if ui.add(egui::DragValue::new(&mut self.machine_profile.max_rate_x).speed(50.0).suffix(" mm/min")).changed() { profile_changed = true; }
                    ui.end_row();
                    ui.label("Max Rate Y:");
                    if ui.add(egui::DragValue::new(&mut self.machine_profile.max_rate_y).speed(50.0).suffix(" mm/min")).changed() { profile_changed = true; }
                    ui.end_row();
                    ui.label("Accel X:");
                    if ui.add(egui::DragValue::new(&mut self.machine_profile.accel_x).speed(10.0).suffix(" mm/s²")).changed() { profile_changed = true; }
                    ui.end_row();
                    ui.label("Accel Y:");
                    if ui.add(egui::DragValue::new(&mut self.machine_profile.accel_y).speed(10.0).suffix(" mm/s²")).changed() { profile_changed = true; }
                    ui.end_row();
                });
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut self.machine_profile.return_to_origin, "Return to origin").changed() { profile_changed = true; }
                });
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut self.machine_profile.air_assist, "Air Assist (M8/M9)").changed() { profile_changed = true; }
                });
            });
        if profile_changed { self.machine_profile.save(); }

        // Jog pad
        let jog_action = ui::jog::show(ui, &mut self.jog_step, &mut self.jog_feed);
        if let Some(dir) = jog_action.direction {
            self.jog(dir);
        }

        ui.add_space(8.0);

        // Custom Macros
        let macros_action = ui::macros::show(ui, &mut self.macros_state, connected);
        if let Some(gcode) = macros_action.execute_macro {
            for line in gcode.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    self.send_command(trimmed);
                }
            }
        }

        ui.add_space(8.0);

        if self.ui_layout == theme::UiLayout::Modern {
            // Drawing Tools (Only in sidebar in modern)
            let draw_action = ui::drawing::show(ui, &mut self.drawing_state, &self.layers, self.active_layer_idx);
            if let Some(lines) = draw_action.generate_gcode {
                let file = GCodeFile::from_lines("drawing", &lines);
                self.set_loaded_file(file, lines);
            }
            ui.add_space(8.0);

            // Camera Overlay
            ui::camera::show(ui, &mut self.camera_state);
            ui.add_space(8.0);
            
            // Text Tool
            let text_action = ui::text::show(ui, &mut self.text_state);
            if let Some(lines) = text_action.generate_gcode {
                let file = GCodeFile::from_lines("text", &lines);
                self.set_loaded_file(file, lines);
            }
            ui.add_space(8.0);

            // Generators
            let gen_action = ui::generators::show(ui, &mut self.generator_state);
            if let Some(lines) = gen_action.generate_gcode {
                let file = GCodeFile::from_lines("generator", &lines);
                self.set_loaded_file(file, lines);
            }
        }

        // Job Transformation
        ui.group(|ui| {
            let (h, m, s) = if let Some(file) = &self.loaded_file {
                let est_time_s = crate::gcode::estimation::estimate(&file.lines).estimated_seconds;
                let h = (est_time_s / 3600.0) as u32;
                let m = ((est_time_s % 3600.0) / 60.0) as u32;
                let s = (est_time_s % 60.0) as u32;
                (h, m, s)
            } else {
                (0, 0, 0)
            };
            ui.label(RichText::new(format!("⏱ Est. Time: {:02}:{:02}:{:02}", h, m, s)).strong().color(theme::GREEN));

            ui.add_space(4.0);
            if ui.button("⚡ Optimize Path").on_hover_text("Reorder segments to minimize travel distance").clicked() {
                if let Some(mut file) = self.loaded_file.clone() {
                    let optimized_lines = crate::gcode::optimizer::optimize(&file.lines);
                    let raw_lines: Vec<String> = optimized_lines.iter().map(|l| l.to_gcode()).collect();
                    file.lines = optimized_lines;
                    self.set_loaded_file(file, raw_lines);
                    self.log("Path optimized.".to_string());
                }
            }
            ui.add_space(4.0);
            ui.label(RichText::new("📐 Job Transformation").color(theme::LAVENDER).strong());
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("Offset X:");
                ui.add(egui::DragValue::new(&mut self.job_offset_x).speed(1.0).suffix(" mm"));
                ui.label("Y:");
                ui.add(egui::DragValue::new(&mut self.job_offset_y).speed(1.0).suffix(" mm"));
            });
            ui.horizontal(|ui| {
                ui.label("Rotation:");
                ui.add(egui::Slider::new(&mut self.job_rotation, -180.0..=180.0).suffix("°"));
                if ui.button("⟲").clicked() { self.job_rotation = 0.0; }
            });
            if ui.button("Center Job").clicked() {
                if let Some(file) = &self.loaded_file {
                    if let Some((min_x, min_y, max_x, max_y)) = file.bounds() {
                        let mid_x = (min_x + max_x) / 2.0;
                        let mid_y = (min_y + max_y) / 2.0;
                        let wm_x = self.machine_profile.workspace_x_mm / 2.0;
                        let wm_y = self.machine_profile.workspace_y_mm / 2.0;
                        self.job_offset_x = wm_x - mid_x;
                        self.job_offset_y = wm_y - mid_y;
                    }
                }
            }
        });

        ui.add_space(8.0);

        // Console
        let console_action = ui::console::show(
            ui,
            &self.console_log,
            &mut self.console_input,
        );
        if let Some(cmd) = console_action.send_command {
            self.send_command(&cmd);
        }

        ui.add_space(8.0);

        // Material Library
        let mat_action = ui::materials::show(ui, &mut self.materials_state);
        if let Some(s) = mat_action.apply_speed {
            self.jog_feed = s; // Current assumption: speed is engrave/jog speed
        }
        if let Some(p) = mat_action.apply_power {
            self.test_fire_power = p / 10.0; // Assume 1000 scale to 100%
        }

        ui.add_space(8.0);
        
        // Z-Probe & Focus (Pro Tier)
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("📏 Z-Probe & Focus").color(theme::LAVENDER).strong());
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.button("⇊ Run Z-Probe").on_hover_text("Search for surface using G38.2 and set Z0").clicked() {
                    self.send_command("G38.2 Z-50 F100");
                    self.send_command("G4 P0.5");
                    self.send_command("G92 Z0");
                    self.send_command("G0 Z5 F500");
                    self.log("Probing complete. Z set to 5mm above surface.".into());
                }
                if ui.button("🎯 Focus Point").on_hover_text("Move to Z focusing position (e.g. 20mm)").clicked() {
                    self.send_command("G0 Z20 F1000");
                }
            });
            ui.checkbox(&mut self.machine_profile.rotary_enabled, "Enable Rotary Support");
            if self.machine_profile.rotary_enabled {
                ui.horizontal(|ui| {
                    ui.label("Cylinder Ø:");
                    ui.add(egui::DragValue::new(&mut self.machine_profile.rotary_diameter_mm).suffix(" mm"));
                });
            }
        });
    }

    fn ui_right_content(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(4.0);
            ui.label(RichText::new("📜 GCode Lines").color(theme::LAVENDER).strong());
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
}

impl eframe::App for All4LaserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Poll serial
        self.poll_serial();

        // Handle keyboard shortcuts (only when no text input is focused)
        if !ctx.wants_keyboard_input() {
            self.handle_keyboard(ctx);
        }

        // Handle drag-and-drop
        self.handle_file_drop(ctx);

        // Request repaint for live updates
        ctx.request_repaint_after(Duration::from_millis(50));

        // Auto-fit after file load
        if self.needs_auto_fit {
            if let Some(file) = &self.loaded_file {
                let rect = ctx.available_rect();
                let offset = egui::vec2(self.job_offset_x, self.job_offset_y);
                self.renderer.auto_fit(&file.segments, rect, offset, self.job_rotation);
                self.needs_auto_fit = false;
            }
        }

        // Sync workspace size from machine profile to renderer
        self.renderer.workspace_size = egui::vec2(
            self.machine_profile.workspace_x_mm,
            self.machine_profile.workspace_y_mm,
        );

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
            egui::Window::new("✅ Job Complete")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new("Program finished successfully!").size(16.0));
                    ui.add_space(8.0);
                    if ui.button("OK").clicked() {
                        self.notify_job_done = false;
                    }
                });
            // Also ring the terminal bell
            print!("\x07");
        }

        // === Power/Speed Test Window ===
        {
            let pst_action = ui::power_speed_test::show(ctx, &mut self.power_speed_test);
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
                // Don't re-sync editor text (it's the source)
                self.log(format!("GCode editor applied ({} lines)", lines.len()));
                self.program_lines = lines.clone();
                self.program_index = 0;
                self.loaded_file = Some(file);
                self.needs_auto_fit = true;
            }
        }

        // === Shortcuts Panel ===
        {
            // Allow '?' on keyboard to open
            if !ctx.wants_keyboard_input() {
                if ctx.input(|i| i.key_pressed(egui::Key::Questionmark)) {
                    self.shortcuts.is_open = true;
                }
            }
            // Render via Ui trick: pass a dummy Ui context using CentralPanel? No — use ctx wrapper
            egui::Area::new(egui::Id::new("shortcuts_area"))
                .interactable(false)
                .fixed_pos(egui::pos2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui::shortcuts::show(ui, &mut self.shortcuts);
                });
        }

        // === Tiling Window ===
        {
            let source = self.program_lines.clone();
            let tile_action = ui::tiling::show(ctx, &mut self.tiling, &source);
            if let Some(lines) = tile_action.apply {
                let file = GCodeFile::from_lines("tiled", &lines);
                self.set_loaded_file(file, lines);
                self.log("Tiling applied.".into());
            }
        }

        // === Test Fire Window ===
        if self.test_fire_open {
            let connected = self.is_connected();
            let mut close_tf = false;
            egui::Window::new("🔥 Test Fire")
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    egui::Grid::new("tf_grid").num_columns(2).spacing([8.0, 4.0]).show(ui, |ui| {
                        ui.label("Power (S):");
                        ui.add(egui::DragValue::new(&mut self.test_fire_power).range(1.0..=1000.0).speed(5.0));
                        ui.end_row();
                        ui.label("Duration (ms):");
                        ui.add(egui::DragValue::new(&mut self.test_fire_ms).range(10.0..=5000.0).speed(10.0));
                        ui.end_row();
                    });
                    ui.horizontal(|ui| {
                        if ui.add_enabled(connected, egui::Button::new(
                            egui::RichText::new("🔥 Fire").color(theme::RED).strong()
                        )).clicked() {
                            let pow = self.test_fire_power;
                            let secs = self.test_fire_ms / 1000.0;
                            self.send_command(&format!("M3 S{:.0}", pow));
                            self.send_command(&format!("G4 P{:.3}", secs));
                            self.send_command("M5");
                        }
                        if ui.button("Close").clicked() { close_tf = true; }
                    });
                });
            if close_tf { self.test_fire_open = false; }
        }

        // === Feed/Spindle Override Window ===
        if self.running {
            egui::Window::new("⚡ Overrides")
                .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-10.0, -50.0))
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new("Feed Override").small());
                    if ui.add(egui::Slider::new(&mut self.feed_override_pct, 10.0..=200.0).suffix("%")).drag_stopped() {
                        let val = self.feed_override_pct as u8;
                        self.send_feed_override(val);
                    }
                    ui.label(egui::RichText::new("Spindle Override").small());
                    if ui.add(egui::Slider::new(&mut self.spindle_override_pct, 10.0..=200.0).suffix("%")).drag_stopped() {
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

        // --- Handle Import Modal ---
        if let Some(mut state) = self.import_state.take() {
            // Lazy load or Refresh texture
            if state.needs_texture_update {
                if let ui::image_dialog::ImportType::Raster(base_img) = &state.import_type {
                    // Apply adjustments for preview
                    let processed = imaging::raster::preprocess_image(base_img, &state.raster_params);
                    let rgba = processed.to_rgba8();
                    let color_image = egui::ColorImage::from_rgba_unmultiplied(
                        [rgba.width() as _, rgba.height() as _],
                        rgba.as_flat_samples().as_slice(),
                    );
                    state.texture = Some(ctx.load_texture(
                        &state.filename,
                        color_image,
                        Default::default(),
                    ));
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
                    let res = ui::image_dialog::show(ui, &mut state);
                    if res.imported { import_triggered = true; }
                    if res.cancel { cancel_triggered = true; }
                });

            if !open || cancel_triggered {
                self.import_state = None;
            } else if import_triggered {
                match &state.import_type {
                    ui::image_dialog::ImportType::Raster(img) => {
                        let lines = if state.vectorize {
                            imaging::raster::vectorize_image(img, &state.raster_params)
                        } else {
                            imaging::raster::image_to_gcode(img, &state.raster_params)
                        };
                        let file = GCodeFile::from_lines(&state.filename, &lines);
                        self.set_loaded_file(file, lines);
                    }
                    ui::image_dialog::ImportType::Svg(data) => {
                        match imaging::svg::svg_to_gcode(data, &state.svg_params) {
                            Ok(lines) => {
                                let file = GCodeFile::from_lines(&state.filename, &lines);
                                self.set_loaded_file(file, lines);
                            }
                            Err(e) => self.log(format!("SVG Conversion failed: {e}")),
                        }
                    }
                }
                self.import_state = None;
            } else {
                // Keep it open
                self.import_state = Some(state);
            }
        }

        // === Handle Cut Settings Modal ===
        {
            let action = ui::cut_settings::show(ctx, &mut self.cut_settings_state, &self.layers);
            if let Some((idx, new_layer)) = action.apply {
                if idx < self.layers.len() {
                    self.layers[idx] = new_layer;
                }
            }
        }

        // === Handle Settings Modal ===
        if let Some(state) = &mut self.settings_state {
            ui::settings_dialog::show(ctx, state);
            if !state.is_open {
                self.settings_state = None;
            } else {
                // If there are pending writes, send them
                if !state.pending_writes.is_empty() {
                    let writes = std::mem::take(&mut state.pending_writes);
                    for (id, val) in writes {
                        if id == -1 && val == "$$" {
                            self.send_command("$$"); // Refresh
                        } else {
                            self.send_command(&format!("${}={}", id, val));
                        }
                    }
                }
            }
        }

        // === TOP: Toolbar ===
        let is_connected = self.is_connected();
        let is_running = self.running;
        let is_light = self.light_mode;

        TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(4.0);
            let has_file = self.loaded_file.is_some();
            let actions = ui::toolbar::show(ui, is_connected, is_running, is_light, self.framing_active, &self.recent_files, has_file);
            ui.add_space(4.0);

            if actions.connect_toggle {
                if is_connected { self.disconnect(); } else { self.connect(); }
            }
            if actions.open_file { self.open_file(); }
            if let Some(path) = actions.open_recent { self.load_file_path(&path); }
            if actions.save_file { self.save_file(); }
            if actions.run_program { self.run_program(); }
            if actions.frame_bbox { self.frame_bbox(); }
            if actions.abort_program { self.abort_program(); }
            if actions.hold {
                if let Some(conn) = self.connection.as_ref() {
                    conn.send_byte(protocol::CMD_FEED_HOLD);
                }
            }
            if actions.resume {
                if let Some(conn) = self.connection.as_ref() {
                    conn.send_byte(protocol::CMD_CYCLE_START);
                }
            }
            if actions.home { self.send_command("$H"); }
            if actions.unlock { self.send_command("$X"); }
            if actions.set_zero { self.send_command("G92X0Y0Z0"); }
            if actions.reset {
                if let Some(conn) = self.connection.as_ref() {
                    conn.send_byte(protocol::CMD_RESET);
                }
            }
            if let Some(t) = actions.set_theme {
                self.ui_theme = t;
                self.apply_theme(ctx);
            }
            if let Some(l) = actions.set_layout {
                self.ui_layout = l;
            }
            if actions.toggle_light_mode {
                self.light_mode = !self.light_mode;
                self.apply_theme(ctx);
            }
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
            if actions.open_test_fire {
                self.test_fire_open = true;
            }
            if actions.open_project {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("All4Laser Project", &["a4l"])
                    .pick_file()
                {
                    let path_str = path.to_string_lossy().to_string();
                    match crate::config::project::ProjectFile::load(&path_str) {
                        Ok(proj) => {
                            self.job_offset_x = proj.offset_x;
                            self.job_offset_y = proj.offset_y;
                            if let Some(mp) = proj.machine_profile { self.machine_profile = mp; }
                            if let Some(gc_path) = proj.gcode_path {
                                self.load_file_path(&gc_path);
                            } else if let Some(content) = proj.gcode_content {
                                let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
                                let file = GCodeFile::from_lines("project", &lines);
                                self.set_loaded_file(file, lines);
                            }
                            self.log("Project loaded.".into());
                        }
                        Err(e) => self.show_error(format!("Project load failed: {e}")),
                    }
                }
            }
            if actions.save_project {
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
                        offset_x: self.job_offset_x,
                        offset_y: self.job_offset_y,
                        machine_profile: Some(self.machine_profile.clone()),
                    };
                    match crate::config::project::ProjectFile::save(&path_str, &proj) {
                        Ok(_) => self.log(format!("Project saved: {path_str}")),
                        Err(e) => self.show_error(format!("Project save failed: {e}")),
                    }
                }
            }
            if actions.open_settings {
                if self.settings_state.is_none() {
                    let mut state = ui::settings_dialog::SettingsDialogState::default();
                    state.is_open = true;
                    // Trigger a refresh automatically on open
                    state.pending_writes.push((-1, "$$".to_string()));
                    self.settings_state = Some(state);
                }
            }
        });

        // === BOTTOM: Progress bar + Status bar ===
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            if self.running && !self.program_lines.is_empty() {
                let progress = self.program_index as f32 / self.program_lines.len() as f32;
                let bar = egui::ProgressBar::new(progress)
                    .text(format!("{}/{}", self.program_index, self.program_lines.len()));
                ui.add(bar);
            }

            let file_info = self.loaded_file.as_ref().map(|f| {
                (f.filename.as_str(), f.line_count(), f.estimated_time)
            });
            let progress = if self.running {
                Some((self.program_index, self.program_lines.len()))
            } else {
                None
            };

            let sb_actions = ui::status_bar::show(ui, &self.grbl_state, file_info, progress);

            ui.add_space(2.0);
            ui.separator();
            ui.add_space(2.0);

            // Palette
            let pal_action = ui::cut_palette::show(ui, &self.layers, self.active_layer_idx);
            if let Some(idx) = pal_action.select_layer {
                self.active_layer_idx = idx;
                // Automatically set drawing tool to this layer
                self.drawing_state.current.layer_idx = idx;
            }
            if let Some(idx) = pal_action.open_settings {
                self.cut_settings_state.editing_layer_idx = Some(idx);
                self.cut_settings_state.is_open = true;
            }

            if let Some(conn) = self.connection.as_ref() {
                if sb_actions.feed_up { conn.send_byte(protocol::FEED_OV_PLUS_10); }
                if sb_actions.feed_down { conn.send_byte(protocol::FEED_OV_MINUS_10); }
                if sb_actions.rapid_up { conn.send_byte(protocol::RAPID_OV_100); }
                if sb_actions.rapid_down { conn.send_byte(protocol::RAPID_OV_25); }
                if sb_actions.spindle_up { conn.send_byte(protocol::SPINDLE_OV_PLUS_10); }
                if sb_actions.spindle_down { conn.send_byte(protocol::SPINDLE_OV_MINUS_10); }
            }
        });

        // === LEFT: Control panels ===
        let connected = self.is_connected();
        // === LEFT SIDEBAR ===
        let left_panel_id = "left_panel";
        let left_panel = SidePanel::left(left_panel_id)
            .resizable(self.ui_layout == theme::UiLayout::Modern)
            .default_width(if self.ui_layout == theme::UiLayout::Modern { LEFT_PANEL_WIDTH } else { 45.0 });

        left_panel.show(ctx, |ui| {
            if self.ui_layout == theme::UiLayout::Modern {
                egui::ScrollArea::vertical().id_salt("left_scroll").show(ui, |ui| {
                    self.ui_left_content(ui, connected);
                });
            } else {
                // Classic Left: Thin drawing toolbar
                ui.vertical_centered(|ui| {
                    ui.add_space(4.0);
                    let draw_action = ui::drawing::show(ui, &mut self.drawing_state, &self.layers, self.active_layer_idx);
                    if let Some(lines) = draw_action.generate_gcode {
                        let file = GCodeFile::from_lines("drawing", &lines);
                        self.set_loaded_file(file, lines);
                    }
                });
            }
        });

        // === RIGHT SIDEBAR ===
        let right_panel_id = "right_panel";
        let right_panel = SidePanel::right(right_panel_id)
            .resizable(true)
            .default_width(if self.ui_layout == theme::UiLayout::Modern { 220.0 } else { 320.0 });

        right_panel.show(ctx, |ui| {
            if self.ui_layout == theme::UiLayout::Modern {
                self.ui_right_content(ui);
            } else {
                // Classic Right: Machine, Jog, Transform, Console, GCode
                egui::ScrollArea::vertical().id_salt("right_scroll").show(ui, |ui| {
                    self.ui_left_content(ui, connected); // Reuse most content
                    self.ui_right_content(ui);       // Plus GCode viewer
                });
            }
        });

        // === CENTER: Preview ===
        CentralPanel::default().show(ctx, |ui| {
            let segments = self.loaded_file
                .as_ref()
                .map(|f| {
                    // Filter segments by layer visibility
                    f.segments.iter()
                        .filter(|seg| {
                            f.layers.get(seg.layer_id).map(|l| l.visible).unwrap_or(true)
                        })
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            let offset = egui::vec2(self.job_offset_x, self.job_offset_y);

            // Simulation Slider Overlay
            if self.loaded_file.is_some() {
                egui::Area::new(egui::Id::new("sim_controls"))
                    .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(10.0, -10.0))
                    .show(ctx, |ui| {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("🎬 Sim:");
                                let mut sim_on = self.renderer.simulation_progress.is_some();
                                if ui.checkbox(&mut sim_on, "").changed() {
                                    if sim_on { self.renderer.simulation_progress = Some(0.0); }
                                    else { self.renderer.simulation_progress = None; }
                                }
                                if let Some(p) = self.renderer.simulation_progress.as_mut() {
                                    ui.add(egui::Slider::new(p, 0.0..=1.0).show_value(false));
                                }
                            });
                        });
                    });
            }

            let preview_action = ui::preview_panel::show(
                ui, 
                &mut self.renderer, 
                &segments,
                &self.drawing_state.shapes,
                self.light_mode, 
                offset, 
                self.job_rotation,
                &mut self.camera_state
            );

            if preview_action.zoom_in { self.renderer.zoom_in(); }
            if preview_action.zoom_out { self.renderer.zoom_out(); }
            if preview_action.auto_fit {
                if let Some(file) = self.loaded_file.as_ref() {
                    let segments = file.segments.clone();
                    let rect = ui.max_rect();
                    self.renderer.auto_fit(&segments, rect, offset, self.job_rotation);
                }
            }

            // Handle Interaction from Preview
            match preview_action.interactive_action {
                crate::preview::renderer::InteractiveAction::SelectShape(idx) => {
                    // Update drawing tool to reflect selection?
                    // Ideally we should have a "Select" mode in Drawing Tools.
                    // For now, let's just log it.
                    self.log(format!("Selected Shape #{}", idx));
                    if let Some(shape) = self.drawing_state.shapes.get(idx) {
                        self.drawing_state.current = shape.clone();
                    }
                }
                crate::preview::renderer::InteractiveAction::Deselect => {
                    // self.log("Deselected".into());
                }
                crate::preview::renderer::InteractiveAction::DragShape { idx, delta } => {
                    if let Some(shape) = self.drawing_state.shapes.get_mut(idx) {
                        shape.x += delta.x;
                        shape.y += delta.y;

                        // Update current if it was the one dragged
                        self.drawing_state.current = shape.clone();

                        // Regenerate GCode if needed (debounce this in real app)
                        // But for now, we only regenerate on release?
                        // Actually, dragging logic in renderer is continuous.
                        // We need a "DragEnd" event or just live update.
                        // Live update is fine for small shape counts.

                        let lines = ui::drawing::generate_all_gcode(&self.drawing_state, &self.layers);
                        let file = GCodeFile::from_lines("drawing", &lines);
                        self.set_loaded_file(file, lines);
                    }
                }
                _ => {}
            }

            // Update machine position in preview
            self.renderer.machine_pos = egui::Pos2::new(
                self.grbl_state.wpos.x,
                self.grbl_state.wpos.y,
            );
        });
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
