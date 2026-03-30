use crate::i18n::tr;
use crate::theme;
use egui::{ComboBox, RichText, Ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum ConnectionMode {
    #[default]
    Serial,
    Network,
}

impl ConnectionMode {
    fn label(self) -> &'static str {
        match self {
            Self::Serial => "Serial",
            Self::Network => "Network (TCP/IP)",
        }
    }
}

pub struct ConnectionAction {
    pub connect: bool,
    pub disconnect: bool,
    pub refresh_ports: bool,
    pub test_network: bool,
}

impl Default for ConnectionAction {
    fn default() -> Self {
        Self {
            connect: false,
            disconnect: false,
            refresh_ports: false,
            test_network: false,
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
    mode: &mut ConnectionMode,
    network_host: &mut String,
    network_port: &mut String,
    connected: bool,
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
            ui.label(format!("{}:", tr("Mode")));
            ComboBox::from_id_salt("connection_mode_combo")
                .selected_text(tr(mode.label()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        mode,
                        ConnectionMode::Serial,
                        tr(ConnectionMode::Serial.label()),
                    );
                    ui.selectable_value(
                        mode,
                        ConnectionMode::Network,
                        tr(ConnectionMode::Network.label()),
                    );
                });
        });

        if *mode == ConnectionMode::Serial {
            ui.horizontal(|ui| {
                ui.label(format!("{}:", tr("Port")));
                let port_label = if ports.is_empty() {
                    tr("No ports").to_string()
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
                ui.label(format!("{}:", tr("Baud")));
                let baud_label = format!("{}", get_baud(baud_rates, *selected_baud));
                ComboBox::from_id_salt("baud_combo")
                    .selected_text(baud_label)
                    .show_ui(ui, |ui| {
                        for (i, rate) in baud_rates.iter().enumerate() {
                            ui.selectable_value(selected_baud, i, format!("{rate}"));
                        }
                    });
            });
        } else {
            ui.horizontal(|ui| {
                ui.label(format!("{}:", tr("Host")));
                ui.text_edit_singleline(network_host);
            });
            ui.horizontal(|ui| {
                ui.label(format!("{}:", tr("Port")));
                ui.text_edit_singleline(network_port);
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
            } else {
                let can_connect = if *mode == ConnectionMode::Serial {
                    !ports.is_empty()
                } else {
                    !network_host.trim().is_empty() && network_port.trim().parse::<u16>().is_ok()
                };
                if ui
                    .add_enabled(
                        can_connect,
                        egui::Button::new(RichText::new(tr("Connect")).color(theme::GREEN)),
                    )
                    .clicked()
                {
                    action.connect = true;
                }
            }
            if *mode == ConnectionMode::Serial && ui.button(format!("↻ {}", tr("Refresh"))).clicked() {
                action.refresh_ports = true;
            }
            if *mode == ConnectionMode::Network
                && ui.button(format!("🧪 {}", tr("Test Network"))).clicked()
            {
                action.test_network = true;
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
