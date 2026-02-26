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

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("All4Laser"),
        ..Default::default()
    };

    eframe::run_native(
        "All4Laser",
        options,
        Box::new(|cc| Ok(Box::new(app::All4LaserApp::new(cc)))),
    )
}
