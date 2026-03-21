use crate::gcode::output_protocol::ProtocolKind;
use crate::i18n::tr;
use crate::theme;
use egui::{ComboBox, RichText, Ui};

pub struct ConnectionAction {
    pub connect: bool,
    pub disconnect: bool,
    pub refresh_ports: bool,
    pub connect_tcp: bool,
}

impl Default for ConnectionAction {
    fn default() -> Self {
        Self {
            connect: false,
            disconnect: false,
            refresh_ports: false,
            connect_tcp: false,
        }
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
    output_protocol: &mut ProtocolKind,
    use_tcp: &mut bool,
    tcp_host: &mut String,
    tcp_port: &mut String,
) -> ConnectionAction {
    let mut action = ConnectionAction::default();

    ui.group(|ui| {
        ui.label(
            RichText::new(tr("Connection"))
                .color(theme::LAVENDER)
                .strong()
                .size(14.0),
        );
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label(tr("Protocol:"));
            ComboBox::from_id_salt("protocol_combo")
                .selected_text(output_protocol.label())
                .show_ui(ui, |ui| {
                    for kind in ProtocolKind::ALL {
                        ui.selectable_value(output_protocol, kind, kind.label());
                    }
                });
        });

        ui.horizontal(|ui| {
            ui.label(tr("Mode:"));
            ui.selectable_value(use_tcp, false, "Serial");
            ui.selectable_value(use_tcp, true, "WiFi/TCP");
        });

        if *use_tcp {
            ui.horizontal(|ui| {
                ui.label(tr("Host:"));
                ui.text_edit_singleline(tcp_host);
            });
            ui.horizontal(|ui| {
                ui.label(tr("Port:"));
                ui.text_edit_singleline(tcp_port);
            });
        } else {
            ui.horizontal(|ui| {
                ui.label(tr("Port:"));
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
                ui.label(tr("Baud:"));
                let baud_label = format!("{}", get_baud(baud_rates, *selected_baud));
                ComboBox::from_id_salt("baud_combo")
                    .selected_text(baud_label)
                    .show_ui(ui, |ui| {
                        for (i, rate) in baud_rates.iter().enumerate() {
                            ui.selectable_value(selected_baud, i, format!("{rate}"));
                        }
                    });
            });
        }

        ui.horizontal(|ui| {
            if connected {
                if ui
                    .button(RichText::new(tr("Disconnect")).color(theme::RED))
                    .clicked()
                {
                    action.disconnect = true;
                }
            } else if *use_tcp {
                let can_connect = !tcp_host.is_empty();
                if ui
                    .add_enabled(
                        can_connect,
                        egui::Button::new(RichText::new(tr("Connect")).color(theme::GREEN)),
                    )
                    .clicked()
                {
                    action.connect_tcp = true;
                }
            } else {
                let can_connect = !ports.is_empty();
                if ui
                    .add_enabled(
                        can_connect,
                        egui::Button::new(RichText::new(tr("Connect")).color(theme::GREEN)),
                    )
                    .clicked()
                {
                    action.connect = true;
                }
                if ui.button(format!("↻ {}", tr("Refresh"))).clicked() {
                    action.refresh_ports = true;
                }
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
