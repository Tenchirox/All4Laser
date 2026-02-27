#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod theme;
mod grbl;
mod serial;
mod gcode;
mod preview;
mod imaging;
mod ui;
mod config;

fn main() -> eframe::Result {
    env_logger::init();

    let icon = load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("All4Laser")
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "All4Laser",
        options,
        Box::new(|cc| Ok(Box::new(app::All4LaserApp::new(cc)))),
    )
}

fn load_icon() -> egui::IconData {
    let icon_data = include_bytes!("../A4L.ico");
    let img = image::load_from_memory(icon_data).expect("Failed to load icon");
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    egui::IconData {
        rgba: rgba.into_raw(),
        width,
        height,
    }
}
