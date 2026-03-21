#![allow(dead_code)]

use crossbeam_channel::{Receiver, Sender, unbounded};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crate::controller::{ControllerBackend, ControllerResponse};

/// Messages from serial reader thread to the main app
#[derive(Debug, Clone)]
pub enum SerialMsg {
    Parsed {
        raw: String,
        response: ControllerResponse,
    },
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
    connected: Arc<AtomicBool>,
}

impl SerialConnection {
    fn spawn_reader_writer<R, W>(
        label: String,
        reader: R,
        mut writer: W,
        backend: Arc<dyn ControllerBackend>,
    ) -> Self
    where
        R: Read + Send + 'static,
        W: Write + Send + 'static,
    {
        let (msg_tx, msg_rx) = unbounded::<SerialMsg>();
        let (cmd_tx, cmd_rx) = unbounded::<SerialCmd>();
        let connected = Arc::new(AtomicBool::new(true));

        // Reader thread
        let reader_tx = msg_tx.clone();
        let connected_r = connected.clone();
        std::thread::spawn(move || {
            let mut buf_reader = BufReader::new(reader);
            let _ = reader_tx.send(SerialMsg::Connected(label));
            let mut line = String::new();
            loop {
                line.clear();
                match buf_reader.read_line(&mut line) {
                    Ok(0) => {
                        connected_r.store(false, Ordering::Relaxed);
                        let _ = reader_tx.send(SerialMsg::Disconnected("Connection closed".into()));
                        break;
                    }
                    Ok(_) => {
                        let trimmed = line.trim().to_string();
                        if trimmed.is_empty() {
                            continue;
                        }
                        let response = backend.parse_response(&trimmed);
                        let _ = reader_tx.send(SerialMsg::Parsed {
                            raw: trimmed,
                            response,
                        });
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::TimedOut
                            || e.kind() == std::io::ErrorKind::WouldBlock
                        {
                            continue;
                        }
                        connected_r.store(false, Ordering::Relaxed);
                        let _ = reader_tx.send(SerialMsg::Disconnected(e.to_string()));
                        break;
                    }
                }
            }
        });

        // Writer thread
        let connected_w = connected.clone();
        std::thread::spawn(move || {
            loop {
                match cmd_rx.recv() {
                    Ok(SerialCmd::SendLine(line)) => {
                        let data = format!("{line}\n");
                        if writer.write_all(data.as_bytes()).is_err() {
                            break;
                        }
                        let _ = writer.flush();
                    }
                    Ok(SerialCmd::SendByte(byte)) => {
                        if writer.write_all(&[byte]).is_err() {
                            break;
                        }
                        let _ = writer.flush();
                    }
                    Ok(SerialCmd::Disconnect) | Err(_) => {
                        connected_w.store(false, Ordering::Relaxed);
                        break;
                    }
                }
            }
        });

        Self { rx: msg_rx, cmd_tx, connected }
    }

    /// Connect via serial port
    pub fn connect(
        port_name: &str,
        baud_rate: u32,
        backend: Arc<dyn ControllerBackend>,
    ) -> Result<Self, String> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()
            .map_err(|e| format!("Failed to open {port_name}: {e}"))?;
        let writer = port.try_clone().map_err(|e| format!("Failed to clone port: {e}"))?;
        Ok(Self::spawn_reader_writer(
            port_name.to_string(),
            port,
            writer,
            backend,
        ))
    }

    /// Connect via TCP (WiFi/Ethernet, e.g. FluidNC/ESP32)
    pub fn connect_tcp(
        host: &str,
        port: u16,
        backend: Arc<dyn ControllerBackend>,
    ) -> Result<Self, String> {
        let addr = format!("{host}:{port}");
        let stream = TcpStream::connect(&addr)
            .map_err(|e| format!("TCP connect to {addr} failed: {e}"))?;
        stream
            .set_read_timeout(Some(Duration::from_millis(200)))
            .map_err(|e| format!("set_read_timeout: {e}"))?;
        let writer = stream.try_clone().map_err(|e| format!("TCP clone: {e}"))?;
        Ok(Self::spawn_reader_writer(
            addr,
            stream,
            writer,
            backend,
        ))
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
        self.connected.load(Ordering::Relaxed)
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
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut fallback_ports = Vec::new();
            if let Ok(entries) = std::fs::read_dir("/dev") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    if name.starts_with("ttyUSB") || name.starts_with("ttyACM") {
                        fallback_ports.push(format!("/dev/{name}"));
                    }
                }
            }
            let _ = tx.send(fallback_ports);
        });

        if let Ok(fallback_ports) = rx.recv_timeout(Duration::from_millis(250)) {
            ports.extend(fallback_ports);
        }
    }

    ports.sort();
    ports
}
