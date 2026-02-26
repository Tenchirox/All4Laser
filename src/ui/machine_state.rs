use egui::{Ui, RichText, Grid};
use crate::grbl::types::GrblState;
use crate::theme;

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
}

pub fn show(ui: &mut Ui, state: &GrblState, is_focused: bool, connected: bool) -> MachineStateAction {
    let mut action = MachineStateAction { toggle_focus: false, quick_pos: None };

    ui.group(|ui| {
        ui.label(RichText::new("Machine State").color(theme::LAVENDER).strong().size(14.0));
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
                ui.label(RichText::new("MPos").color(theme::SUBTEXT));
                ui.label(RichText::new(format!("{:.3}", state.mpos.x)).color(theme::TEXT).monospace());
                ui.label(RichText::new(format!("{:.3}", state.mpos.y)).color(theme::TEXT).monospace());
                ui.label(RichText::new(format!("{:.3}", state.mpos.z)).color(theme::TEXT).monospace());
                ui.end_row();

                // WPos
                ui.label(RichText::new("WPos").color(theme::SUBTEXT));
                ui.label(RichText::new(format!("{:.3}", state.wpos.x)).color(theme::TEXT).monospace());
                ui.label(RichText::new(format!("{:.3}", state.wpos.y)).color(theme::TEXT).monospace());
                ui.label(RichText::new(format!("{:.3}", state.wpos.z)).color(theme::TEXT).monospace());
                ui.end_row();
            });

        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label(RichText::new("Feed:").color(theme::SUBTEXT));
            ui.label(RichText::new(format!("{:.0} mm/min", state.feed_rate)).color(theme::YELLOW).monospace());
            ui.add_space(12.0);
            ui.label(RichText::new("Spindle:").color(theme::SUBTEXT));
            ui.label(RichText::new(format!("{:.0} RPM", state.spindle_speed)).color(theme::MAUVE).monospace());
        });

        ui.add_space(4.0);

        let focus_label = if is_focused { "🔥 Laser Focus (ON)" } else { "🔦 Laser Focus (OFF)" };
        let focus_color = if is_focused { theme::RED } else { theme::SUBTEXT };
        if ui.add_enabled(connected, egui::Button::new(RichText::new(focus_label).color(focus_color))).clicked() {
            action.toggle_focus = true;
        }

        ui.add_space(8.0);
        ui.label(RichText::new("Quick Move (Bounds)").color(theme::LAVENDER).strong());
        
        ui.horizontal(|ui| {
            if ui.add_enabled(connected, egui::Button::new("⌜ TL")).clicked() { action.quick_pos = Some(QuickPosition::TopLeft); }
            if ui.add_enabled(connected, egui::Button::new("⌂ C")).clicked() { action.quick_pos = Some(QuickPosition::Center); }
            if ui.add_enabled(connected, egui::Button::new("⌝ TR")).clicked() { action.quick_pos = Some(QuickPosition::TopRight); }
        });
        ui.horizontal(|ui| {
            if ui.add_enabled(connected, egui::Button::new("⌞ BL")).clicked() { action.quick_pos = Some(QuickPosition::BottomLeft); }
            if ui.add_enabled(connected, egui::Button::new("⌟ BR")).clicked() { action.quick_pos = Some(QuickPosition::BottomRight); }
        });
    });

    action
}
