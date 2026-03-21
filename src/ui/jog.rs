use crate::grbl::types::JogDirection;
use crate::i18n::tr;
use crate::theme;
use egui::{RichText, Ui, Vec2};

pub struct JogAction {
    pub direction: Option<JogDirection>,
}

pub fn show(
    ui: &mut Ui,
    step: &mut f32,
    feed: &mut f32,
    can_jog: bool,
    can_home: bool,
    speed_unit: crate::config::settings::SpeedUnit,
) -> JogAction {
    let mut action = JogAction { direction: None };

    ui.group(|ui| {
        ui.label(
            RichText::new(tr("Jog Control"))
                .color(theme::LAVENDER)
                .strong()
                .size(14.0),
        );
        ui.add_space(4.0);

        let btn_size = Vec2::new(36.0, 36.0);

        // Row 1: NW N NE
        ui.horizontal(|ui| {
            if ui
                .add_enabled(can_jog, egui::Button::new("↖").min_size(btn_size))
                .clicked()
            {
                action.direction = Some(JogDirection::NW);
            }
            if ui
                .add_enabled(can_jog, egui::Button::new("↑").min_size(btn_size))
                .clicked()
            {
                action.direction = Some(JogDirection::N);
            }
            if ui
                .add_enabled(can_jog, egui::Button::new("↗").min_size(btn_size))
                .clicked()
            {
                action.direction = Some(JogDirection::NE);
            }
            ui.add_space(8.0);
            if ui
                .add_enabled(can_jog, egui::Button::new("Z↑").min_size(btn_size))
                .clicked()
            {
                action.direction = Some(JogDirection::Zup);
            }
        });

        // Row 2: W Home E
        ui.horizontal(|ui| {
            if ui
                .add_enabled(can_jog, egui::Button::new("←").min_size(btn_size))
                .clicked()
            {
                action.direction = Some(JogDirection::W);
            }
            if ui
                .add_enabled(
                    can_jog && can_home,
                    egui::Button::new("⌂").min_size(btn_size),
                )
                .clicked()
            {
                action.direction = Some(JogDirection::Home);
            }
            if ui
                .add_enabled(can_jog, egui::Button::new("→").min_size(btn_size))
                .clicked()
            {
                action.direction = Some(JogDirection::E);
            }
            ui.add_space(8.0);
            if ui
                .add_enabled(can_jog, egui::Button::new("Z↓").min_size(btn_size))
                .clicked()
            {
                action.direction = Some(JogDirection::Zdown);
            }
        });

        // Row 3: SW S SE
        ui.horizontal(|ui| {
            if ui
                .add_enabled(can_jog, egui::Button::new("↙").min_size(btn_size))
                .clicked()
            {
                action.direction = Some(JogDirection::SW);
            }
            if ui
                .add_enabled(can_jog, egui::Button::new("↓").min_size(btn_size))
                .clicked()
            {
                action.direction = Some(JogDirection::S);
            }
            if ui
                .add_enabled(can_jog, egui::Button::new("↘").min_size(btn_size))
                .clicked()
            {
                action.direction = Some(JogDirection::SE);
            }
        });

        ui.add_space(8.0);

        // Step size selector
        ui.horizontal(|ui| {
            ui.label(RichText::new(tr("Step:")).color(theme::SUBTEXT));
            for &s in &[0.01_f32, 0.1, 0.5, 1.0, 5.0, 10.0, 50.0] {
                let label = if s < 0.1 {
                    format!("{s}")
                } else if s < 1.0 {
                    format!("{s}")
                } else {
                    format!("{}", s as i32)
                };
                let selected = (*step - s).abs() < 0.001;
                if ui.selectable_label(selected, &label).clicked() {
                    *step = s;
                }
            }
            ui.add(
                egui::DragValue::new(step)
                    .speed(0.1)
                    .range(0.001..=200.0)
                    .suffix(" mm"),
            );
        });

        // Feed rate
        ui.horizontal(|ui| {
            ui.label(RichText::new(tr("Feed:")).color(theme::SUBTEXT));
            let mut display_feed = speed_unit.from_mmpm(*feed);
            let (drag_spd, max_val, suffix) = match speed_unit {
                crate::config::settings::SpeedUnit::MmPerMin => (50.0, 10000.0, " mm/min"),
                crate::config::settings::SpeedUnit::MmPerSec => (1.0, 167.0, " mm/s"),
            };
            if ui.add(
                egui::DragValue::new(&mut display_feed)
                    .range(0.1..=max_val)
                    .speed(drag_spd)
                    .suffix(suffix),
            ).changed() {
                *feed = speed_unit.to_mmpm(display_feed);
            }
        });
    });

    action
}
