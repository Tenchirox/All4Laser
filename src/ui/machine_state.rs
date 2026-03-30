use crate::grbl::types::GrblState;
use crate::theme;
use egui::{Grid, RichText, Ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickPosition {
    Center,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub struct MachineStateAction {
    pub toggle_focus: bool,
    pub quick_pos: Option<QuickPosition>,
    pub confirm_focus: bool,
}

use crate::i18n::tr;

pub fn show(
    ui: &mut Ui,
    state: &GrblState,
    is_focused: bool,
    connected: bool,
) -> MachineStateAction {
    let mut action = MachineStateAction {
        toggle_focus: false,
        quick_pos: None,
        confirm_focus: false,
    };

    ui.group(|ui| {
        ui.label(
            RichText::new(tr("Machine Profile"))
                .color(theme::LAVENDER)
                .strong()
                .size(14.0),
        );
        ui.add_space(4.0);

        Grid::new("machine_state_grid")
            .num_columns(4)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                // Header
                ui.label("");
                ui.label(RichText::new("X").color(theme::PEACH).strong());
                ui.label(RichText::new("Y").color(theme::GREEN).strong());
                ui.label(RichText::new("Z").color(theme::BLUE).strong());
                ui.end_row();

                // MPos
                ui.label(RichText::new(tr("MPos")).color(theme::SUBTEXT));
                ui.label(
                    RichText::new(format!("{:.3}", state.mpos.x))
                        .color(theme::TEXT)
                        .monospace(),
                );
                ui.label(
                    RichText::new(format!("{:.3}", state.mpos.y))
                        .color(theme::TEXT)
                        .monospace(),
                );
                ui.label(
                    RichText::new(format!("{:.3}", state.mpos.z))
                        .color(theme::TEXT)
                        .monospace(),
                );
                ui.end_row();

                // WPos
                ui.label(RichText::new(tr("WPos")).color(theme::SUBTEXT));
                ui.label(
                    RichText::new(format!("{:.3}", state.wpos.x))
                        .color(theme::TEXT)
                        .monospace(),
                );
                ui.label(
                    RichText::new(format!("{:.3}", state.wpos.y))
                        .color(theme::TEXT)
                        .monospace(),
                );
                ui.label(
                    RichText::new(format!("{:.3}", state.wpos.z))
                        .color(theme::TEXT)
                        .monospace(),
                );
                ui.end_row();
            });

        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("{}:", tr("Speed"))).color(theme::SUBTEXT));
            ui.label(
                RichText::new(format!("{:.0} mm/min", state.feed_rate))
                    .color(theme::YELLOW)
                    .monospace(),
            );
            ui.add_space(12.0);
            ui.label(RichText::new(format!("{}:", tr("Power"))).color(theme::SUBTEXT));
            ui.label(
                RichText::new(format!("S{:.0}", state.spindle_speed))
                    .color(theme::MAUVE)
                    .monospace(),
            );
        });

        ui.add_space(4.0);

        let focus_label = if is_focused {
            format!("🔥 {} ({})", tr("Laser Focus"), tr("ON"))
        } else {
            format!("🔦 {} ({})", tr("Laser Focus"), tr("OFF"))
        };
        let focus_color = if is_focused {
            theme::RED
        } else {
            theme::SUBTEXT
        };
        if ui
            .add_enabled(
                connected,
                egui::Button::new(RichText::new(focus_label).color(focus_color)),
            )
            .on_hover_text(tr("Click to toggle laser focus mode (low power laser on)"))
            .clicked()
        {
            // Show confirmation when turning ON
            if !is_focused {
                action.confirm_focus = true;
            } else {
                action.toggle_focus = true;
            }
        }

        ui.add_space(8.0);
        ui.label(
            RichText::new(tr("Quick Move (Bounds)"))
                .color(theme::LAVENDER)
                .strong(),
        );
        ui.add_space(2.0);

        let qb = egui::vec2(36.0, 24.0);
        ui.horizontal(|ui| {
            if ui.add_enabled(connected, egui::Button::new(format!("⌜ {}", tr("TL"))).min_size(qb)).clicked() {
                action.quick_pos = Some(QuickPosition::TopLeft);
            }
            ui.add_space(4.0);
            if ui.add_enabled(connected, egui::Button::new(format!("⌝ {}", tr("TR"))).min_size(qb)).clicked() {
                action.quick_pos = Some(QuickPosition::TopRight);
            }
        });
        ui.horizontal(|ui| {
            ui.add_space(16.0);
            if ui.add_enabled(connected, egui::Button::new(format!("⌂ {}", tr("C"))).min_size(qb)).clicked() {
                action.quick_pos = Some(QuickPosition::Center);
            }
        });
        ui.horizontal(|ui| {
            if ui.add_enabled(connected, egui::Button::new(format!("⌞ {}", tr("BL"))).min_size(qb)).clicked() {
                action.quick_pos = Some(QuickPosition::BottomLeft);
            }
            ui.add_space(4.0);
            if ui.add_enabled(connected, egui::Button::new(format!("⌟ {}", tr("BR"))).min_size(qb)).clicked() {
                action.quick_pos = Some(QuickPosition::BottomRight);
            }
        });
    });

    action
}
