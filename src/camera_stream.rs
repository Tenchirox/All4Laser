use std::thread;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender, TrySendError};

#[derive(Clone, Debug)]
pub struct CameraDeviceInfo {
    pub index: u32,
    pub label: String,
}

pub fn list_camera_devices() -> Vec<CameraDeviceInfo> {
    #[cfg(target_os = "linux")]
    {
        list_linux_camera_devices()
    }

    #[cfg(not(target_os = "linux"))]
    {
        Vec::new()
    }
}

pub struct CameraFrame {
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
}

enum CameraCommand {
    Stop,
}

pub struct CameraStream {
    frame_rx: Receiver<CameraFrame>,
    error_rx: Receiver<String>,
    cmd_tx: Sender<CameraCommand>,
    worker: Option<thread::JoinHandle<()>>,
}

impl CameraStream {
    pub fn start(device_index: u32) -> Result<Self, String> {
        #[cfg(target_os = "linux")]
        {
            start_linux(device_index)
        }

        #[cfg(not(target_os = "linux"))]
        {
            let _ = device_index;
            Err("Live camera is currently supported only on Linux builds.".to_string())
        }
    }

    pub fn try_recv_latest_frame(&self) -> Option<CameraFrame> {
        let mut latest = None;
        while let Ok(frame) = self.frame_rx.try_recv() {
            latest = Some(frame);
        }
        latest
    }

    pub fn try_recv_error(&self) -> Option<String> {
        let mut latest = None;
        while let Ok(err) = self.error_rx.try_recv() {
            latest = Some(err);
        }
        latest
    }

    pub fn stop(&mut self) {
        let _ = self.cmd_tx.send(CameraCommand::Stop);
        if let Some(handle) = self.worker.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for CameraStream {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(target_os = "linux")]
fn list_linux_camera_devices() -> Vec<CameraDeviceInfo> {
    let mut devices = Vec::new();
    let Ok(entries) = std::fs::read_dir("/sys/class/video4linux") else {
        return devices;
    };

    for entry in entries.flatten() {
        let name_os = entry.file_name();
        let Some(name) = name_os.to_str() else {
            continue;
        };
        if !name.starts_with("video") {
            continue;
        }
        let Ok(index) = name.trim_start_matches("video").parse::<u32>() else {
            continue;
        };

        let label_path = entry.path().join("name");
        let label = std::fs::read_to_string(label_path)
            .map(|s| s.trim().to_string())
            .ok()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| format!("/dev/video{index}"));

        devices.push(CameraDeviceInfo { index, label });
    }

    devices.sort_by_key(|d| d.index);
    devices
}

#[cfg(target_os = "linux")]
fn start_linux(device_index: u32) -> Result<CameraStream, String> {
    let (frame_tx, frame_rx) = crossbeam_channel::bounded(2);
    let (error_tx, error_rx) = crossbeam_channel::bounded(8);
    let (cmd_tx, cmd_rx) = crossbeam_channel::bounded(1);

    let worker = thread::Builder::new()
        .name(format!("camera-stream-{device_index}"))
        .spawn(move || run_linux_capture(device_index, frame_tx, error_tx, cmd_rx))
        .map_err(|e| e.to_string())?;

    Ok(CameraStream {
        frame_rx,
        error_rx,
        cmd_tx,
        worker: Some(worker),
    })
}

#[cfg(target_os = "linux")]
fn run_linux_capture(
    device_index: u32,
    frame_tx: Sender<CameraFrame>,
    error_tx: Sender<String>,
    cmd_rx: Receiver<CameraCommand>,
) {
    use v4l::FourCC;
    use v4l::buffer::Type;
    use v4l::io::traits::CaptureStream;
    use v4l::prelude::*;
    use v4l::video::Capture;

    let dev = match Device::new(device_index as usize) {
        Ok(dev) => dev,
        Err(e) => {
            let _ = error_tx.send(format!("Failed to open /dev/video{device_index}: {e}"));
            return;
        }
    };

    let format = match configure_format(&dev) {
        Ok(fmt) => fmt,
        Err(e) => {
            let _ = error_tx.send(e);
            return;
        }
    };

    let width = format.width as usize;
    let height = format.height as usize;
    let fourcc = format.fourcc;

    let mut stream = match MmapStream::with_buffers(&dev, Type::VideoCapture, 4) {
        Ok(stream) => stream,
        Err(e) => {
            let _ = error_tx.send(format!("Failed to start camera stream on /dev/video{device_index}: {e}"));
            return;
        }
    };

    let mut sent_decode_error = false;

    loop {
        if let Ok(CameraCommand::Stop) = cmd_rx.try_recv() {
            break;
        }

        let frame_data = match stream.next() {
            Ok((data, _meta)) => data,
            Err(e) => {
                let _ = error_tx.send(format!("Camera read error (/dev/video{device_index}): {e}"));
                thread::sleep(Duration::from_millis(80));
                continue;
            }
        };

        match decode_frame(frame_data, width, height, fourcc) {
            Ok(frame) => {
                sent_decode_error = false;
                push_latest_frame(&frame_tx, frame);
            }
            Err(e) => {
                if !sent_decode_error {
                    let _ = error_tx.send(format!("Camera decode error (/dev/video{device_index}): {e}"));
                    sent_decode_error = true;
                }
                thread::sleep(Duration::from_millis(40));
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn configure_format(dev: &v4l::Device) -> Result<v4l::Format, String> {
    use v4l::FourCC;
    use v4l::video::Capture;

    let desired_formats = [b"MJPG", b"YUYV", b"RGB3"];

    for desired in desired_formats {
        let mut fmt = dev.format().map_err(|e| e.to_string())?;
        fmt.width = 1280;
        fmt.height = 720;
        fmt.fourcc = FourCC::new(desired);

        if let Ok(applied) = dev.set_format(&fmt) {
            if applied.fourcc == FourCC::new(desired) {
                return Ok(applied);
            }
        }
    }

    let fallback = dev.format().map_err(|e| e.to_string())?;
    if fallback.fourcc == v4l::FourCC::new(b"MJPG")
        || fallback.fourcc == v4l::FourCC::new(b"YUYV")
        || fallback.fourcc == v4l::FourCC::new(b"RGB3")
    {
        Ok(fallback)
    } else {
        Err(format!(
            "Unsupported camera pixel format: {:?}. Supported: MJPG, YUYV, RGB3",
            fallback.fourcc
        ))
    }
}

#[cfg(target_os = "linux")]
fn push_latest_frame(frame_tx: &Sender<CameraFrame>, frame: CameraFrame) {
    match frame_tx.try_send(frame) {
        Ok(()) => {}
        Err(TrySendError::Full(_)) => {}
        Err(TrySendError::Disconnected(_)) => {}
    }
}

#[cfg(target_os = "linux")]
fn decode_frame(
    data: &[u8],
    width: usize,
    height: usize,
    fourcc: v4l::FourCC,
) -> Result<CameraFrame, String> {
    if fourcc == v4l::FourCC::new(b"MJPG") {
        let img = image::load_from_memory(data).map_err(|e| e.to_string())?;
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        return Ok(CameraFrame {
            width: w as usize,
            height: h as usize,
            rgba: rgba.into_raw(),
        });
    }

    if fourcc == v4l::FourCC::new(b"RGB3") {
        let expected = width * height * 3;
        if data.len() < expected {
            return Err(format!(
                "RGB3 frame too small: got {} bytes, expected at least {expected}",
                data.len()
            ));
        }
        let mut rgba = vec![0u8; width * height * 4];
        let mut src = 0usize;
        let mut dst = 0usize;
        while dst < rgba.len() && src + 2 < data.len() {
            rgba[dst] = data[src];
            rgba[dst + 1] = data[src + 1];
            rgba[dst + 2] = data[src + 2];
            rgba[dst + 3] = 255;
            src += 3;
            dst += 4;
        }
        return Ok(CameraFrame {
            width,
            height,
            rgba,
        });
    }

    if fourcc == v4l::FourCC::new(b"YUYV") {
        let expected = width * height * 2;
        if data.len() < expected {
            return Err(format!(
                "YUYV frame too small: got {} bytes, expected at least {expected}",
                data.len()
            ));
        }

        let mut rgba = vec![0u8; width * height * 4];
        let mut out = 0usize;

        for px in data.chunks_exact(4) {
            let y0 = px[0];
            let u = px[1];
            let y1 = px[2];
            let v = px[3];

            let (r0, g0, b0) = yuv_to_rgb(y0, u, v);
            let (r1, g1, b1) = yuv_to_rgb(y1, u, v);

            rgba[out] = r0;
            rgba[out + 1] = g0;
            rgba[out + 2] = b0;
            rgba[out + 3] = 255;
            out += 4;

            if out + 3 < rgba.len() {
                rgba[out] = r1;
                rgba[out + 1] = g1;
                rgba[out + 2] = b1;
                rgba[out + 3] = 255;
                out += 4;
            }
        }

        return Ok(CameraFrame {
            width,
            height,
            rgba,
        });
    }

    Err(format!(
        "Unsupported frame format: {:?} (supported: MJPG, YUYV, RGB3)",
        fourcc
    ))
}

#[cfg(target_os = "linux")]
fn yuv_to_rgb(y: u8, u: u8, v: u8) -> (u8, u8, u8) {
    let yf = y as f32;
    let uf = (u as f32) - 128.0;
    let vf = (v as f32) - 128.0;

    let r = yf + 1.402 * vf;
    let g = yf - 0.344_136 * uf - 0.714_136 * vf;
    let b = yf + 1.772 * uf;

    (clamp_u8(r), clamp_u8(g), clamp_u8(b))
}

#[cfg(target_os = "linux")]
fn clamp_u8(v: f32) -> u8 {
    v.clamp(0.0, 255.0) as u8
}
