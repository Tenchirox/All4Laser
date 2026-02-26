use egui::{Ui, RichText, Vec2};
use crate::grbl::types::JogDirection;
use crate::theme;

pub struct JogAction {
    pub direction: Option<JogDirection>,
}

pub fn show(ui: &mut Ui, step: &mut f32, feed: &mut f32) -> JogAction {
    let mut action = JogAction { direction: None };

    ui.group(|ui| {
        ui.label(RichText::new("Jog Control").color(theme::LAVENDER).strong().size(14.0));
        ui.add_space(4.0);

        let btn_size = Vec2::new(36.0, 36.0);

        // Row 1: NW N NE
        ui.horizontal(|ui| {
            if ui.add_sized(btn_size, egui::Button::new("↖")).clicked() {
                action.direction = Some(JogDirection::NW);
            }
            if ui.add_sized(btn_size, egui::Button::new("↑")).clicked() {
                action.direction = Some(JogDirection::N);
            }
            if ui.add_sized(btn_size, egui::Button::new("↗")).clicked() {
                action.direction = Some(JogDirection::NE);
            }
            ui.add_space(8.0);
            if ui.add_sized(btn_size, egui::Button::new("Z↑")).clicked() {
                action.direction = Some(JogDirection::Zup);
            }
        });

        // Row 2: W Home E
        ui.horizontal(|ui| {
            if ui.add_sized(btn_size, egui::Button::new("←")).clicked() {
                action.direction = Some(JogDirection::W);
            }
            if ui.add_sized(btn_size, egui::Button::new("⌂")).clicked() {
                action.direction = Some(JogDirection::Home);
            }
            if ui.add_sized(btn_size, egui::Button::new("→")).clicked() {
                action.direction = Some(JogDirection::E);
            }
            ui.add_space(8.0);
            if ui.add_sized(btn_size, egui::Button::new("Z↓")).clicked() {
                action.direction = Some(JogDirection::Zdown);
            }
        });

        // Row 3: SW S SE
        ui.horizontal(|ui| {
            if ui.add_sized(btn_size, egui::Button::new("↙")).clicked() {
                action.direction = Some(JogDirection::SW);
            }
            if ui.add_sized(btn_size, egui::Button::new("↓")).clicked() {
                action.direction = Some(JogDirection::S);
            }
            if ui.add_sized(btn_size, egui::Button::new("↘")).clicked() {
                action.direction = Some(JogDirection::SE);
            }
        });

        ui.add_space(8.0);

        // Step size selector
        ui.horizontal(|ui| {
            ui.label(RichText::new("Step:").color(theme::SUBTEXT));
            for &s in &[0.1_f32, 1.0, 5.0, 10.0] {
                let label = if s < 1.0 { format!("{s}") } else { format!("{}", s as i32) };
                let selected = (*step - s).abs() < 0.01;
                if ui.selectable_label(selected, &label).clicked() {
                    *step = s;
                }
            }
            ui.label("mm");
        });

        // Feed rate
        ui.horizontal(|ui| {
            ui.label(RichText::new("Feed:").color(theme::SUBTEXT));
            ui.add(egui::DragValue::new(feed).range(10.0..=10000.0).speed(50.0).suffix(" mm/min"));
        });
    });

    action
}
