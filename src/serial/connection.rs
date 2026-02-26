use crossbeam_channel::{Receiver, Sender, unbounded};
use serialport::SerialPort;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::grbl::parser;
use crate::grbl::types::GrblResponse;

/// Messages from serial reader thread to the main app
#[derive(Debug, Clone)]
pub enum SerialMsg {
    Response(GrblResponse),
    RawLine(String),
    Connected(String),
    Disconnected(String),
    Error(String),
}

/// Messages from the main app to the serial writer
#[derive(Debug)]
pub enum SerialCmd {
    SendLine(String),
    SendByte(u8),
    Disconnect,
}

pub struct SerialConnection {
    pub rx: Receiver<SerialMsg>,
    pub cmd_tx: Sender<SerialCmd>,
    port_handle: Arc<Mutex<Option<Box<dyn SerialPort>>>>,
}

impl SerialConnection {
    /// Connect to a serial port and spawn reader/writer threads
    pub fn connect(port_name: &str, baud_rate: u32) -> Result<Self, String> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()
            .map_err(|e| format!("Failed to open {port_name}: {e}"))?;

        let (msg_tx, msg_rx) = unbounded::<SerialMsg>();
        let (cmd_tx, cmd_rx) = unbounded::<SerialCmd>();
        let port_handle = Arc::new(Mutex::new(Some(port.try_clone().unwrap())));

        // Reader thread
        let reader_tx = msg_tx.clone();
        let port_name_owned = port_name.to_string();
        std::thread::spawn(move || {
            let reader = BufReader::new(port);
            let _ = reader_tx.send(SerialMsg::Connected(port_name_owned.clone()));

            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        let trimmed = line.trim().to_string();
                        if trimmed.is_empty() {
                            continue;
                        }
                        let response = parser::parse_response(&trimmed);
                        let _ = reader_tx.send(SerialMsg::RawLine(trimmed));
                        let _ = reader_tx.send(SerialMsg::Response(response));
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::TimedOut {
                            continue;
                        }
                        let _ = reader_tx.send(SerialMsg::Disconnected(e.to_string()));
                        break;
                    }
                }
            }
        });

        // Writer thread
        let port_for_writer = port_handle.clone();
        std::thread::spawn(move || {
            loop {
                match cmd_rx.recv() {
                    Ok(SerialCmd::SendLine(line)) => {
                        if let Some(ref mut port) = *port_for_writer.lock().unwrap() {
                            let data = format!("{line}\n");
                            let _ = port.write_all(data.as_bytes());
                            let _ = port.flush();
                        }
                    }
                    Ok(SerialCmd::SendByte(byte)) => {
                        if let Some(ref mut port) = *port_for_writer.lock().unwrap() {
                            let _ = port.write_all(&[byte]);
                            let _ = port.flush();
                        }
                    }
                    Ok(SerialCmd::Disconnect) | Err(_) => {
                        if let Some(ref mut _port) = port_for_writer.lock().unwrap().take() {
                            // port dropped = closed
                        }
                        break;
                    }
                }
            }
        });

        Ok(Self {
            rx: msg_rx,
            cmd_tx,
            port_handle,
        })
    }

    pub fn send(&self, line: &str) {
        let _ = self.cmd_tx.send(SerialCmd::SendLine(line.to_string()));
    }

    pub fn send_byte(&self, byte: u8) {
        let _ = self.cmd_tx.send(SerialCmd::SendByte(byte));
    }

    pub fn disconnect(&self) {
        let _ = self.cmd_tx.send(SerialCmd::Disconnect);
    }

    pub fn is_connected(&self) -> bool {
        self.port_handle.lock().unwrap().is_some()
    }
}

/// Enumerate available serial ports
pub fn list_ports() -> Vec<String> {
    let mut ports = Vec::new();

    // System serial port API
    if let Ok(system_ports) = serialport::available_ports() {
        for p in system_ports {
            ports.push(p.port_name);
        }
    }

    // Fallback: scan /dev
    if ports.is_empty() {
        for pattern in &["ttyUSB", "ttyACM"] {
            if let Ok(entries) = std::fs::read_dir("/dev") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with(pattern) {
                        ports.push(format!("/dev/{name}"));
                    }
                }
            }
        }
    }

    ports.sort();
    ports
}
