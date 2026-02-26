use egui::{Ui, RichText, ComboBox};
use crate::theme;

pub struct ConnectionAction {
    pub connect: bool,
    pub disconnect: bool,
    pub refresh_ports: bool,
}

impl Default for ConnectionAction {
    fn default() -> Self {
        Self { connect: false, disconnect: false, refresh_ports: false }
    }
}

const BAUD_RATES: &[u32] = &[9600, 19200, 38400, 57600, 115200, 230400, 250000];

pub fn show(
    ui: &mut Ui,
    ports: &[String],
    selected_port: &mut usize,
    baud_rates: &[u32],
    selected_baud: &mut usize,
    connected: bool,
) -> ConnectionAction {
    let mut action = ConnectionAction::default();

    ui.group(|ui| {
        ui.label(RichText::new("Connection").color(theme::LAVENDER).strong().size(14.0));
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label("Port:");
            let port_label = if ports.is_empty() {
                "No ports".to_string()
            } else {
                ports.get(*selected_port).cloned().unwrap_or_default()
            };
            ComboBox::from_id_salt("port_combo")
                .selected_text(&port_label)
                .show_ui(ui, |ui| {
                    for (i, port) in ports.iter().enumerate() {
                        ui.selectable_value(selected_port, i, port);
                    }
                });
        });

        ui.horizontal(|ui| {
            ui.label("Baud:");
            let baud_label = format!("{}", get_baud(baud_rates, *selected_baud));
            ComboBox::from_id_salt("baud_combo")
                .selected_text(baud_label)
                .show_ui(ui, |ui| {
                    for (i, rate) in baud_rates.iter().enumerate() {
                        ui.selectable_value(selected_baud, i, format!("{rate}"));
                    }
                });
        });

        ui.horizontal(|ui| {
            if connected {
                if ui.button(RichText::new("Disconnect").color(theme::RED)).clicked() {
                    action.disconnect = true;
                }
            } else {
                let can_connect = !ports.is_empty();
                if ui.add_enabled(can_connect, egui::Button::new(RichText::new("Connect").color(theme::GREEN))).clicked() {
                    action.connect = true;
                }
            }
            if ui.button("↻ Refresh").clicked() {
                action.refresh_ports = true;
            }
        });
    });

    action
}

pub fn get_baud(rates: &[u32], idx: usize) -> u32 {
    rates.get(idx).copied().unwrap_or(115200)
}

pub fn default_baud_rates() -> Vec<u32> {
    BAUD_RATES.to_vec()
}
