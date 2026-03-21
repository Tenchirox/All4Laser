# All4Laser ⚡

**All4Laser** is a high-performance, laser control and GCode sender built with **Rust** and **egui**. Designed for efficiency and precision, it provides a modern alternative to traditional laser software with a focus on speed, cross-platform compatibility, and advanced job preparation tools.

---

## ✨ Key Features

### 🛠 Professional Job Preparation
*   **GCode Path Optimization**: Integrated greedy algorithm to minimize non-burning travel distance, significantly reducing job time.
*   **Project Layers & Multi-Pass**: Organize your design into layers with granular control over visibility, nested passes, and specific power/speed overrides.
*   **Material Library**: A persistent database of presets (Wood, Acrylic, Metal, etc.) to apply optimized settings instantly.
*   **Job Time Estimation**: Real-time feedback on total travel, burn distance, and precise duration calculation based on machine kinematics.

### 🖼 Versatile Imaging & CAD
*   **Native DXF Support**: Import and process `.dxf` files (LINE, ARC, CIRCLE, LWPOLYLINE) natively.
*   **Advanced Vectorization**: Convert bitmaps to GCode using high-speed stencil tracing or traditional raster scanning.
*   **Image Transformations**: Precise rotation, flipping, and scaling with real-time preview.
*   **Tiling & Grids**: Easily repeat jobs in an N×M grid with configurable spacing.

### 🎮 Real-Time Machine Control
*   **Live Overrides**: Adjust Feed Rate and Spindle Power (10% - 200%) on-the-fly without pausing the job.
*   **Continuous Framing**: Visual bounding box framing to align your material perfectly before starting.
*   **Camera Overlay**: Framework for real-time workspace alignment with calibration controls (offset, scale, rotation).
*   **Console & Macros**: Fully featured GRBL console with command history and customizable JSON macros.


---

## 🚀 Getting Started

### Prerequisites
*   **Rust Toolchain**: [Install Rust](https://rustup.rs/) (latest stable version recommended).
*   **Serial Port Permissions**: (Linux) Ensure your user is in the `dialout` or `uucp` group.

### Windows Notes (GNU toolchain)
If you build with `x86_64-pc-windows-gnu`, install an MSYS2 GCC toolchain and ensure `gcc`/`ar` are available in your terminal `PATH`.

Example session setup:
```powershell
$env:PATH = "C:\msys64\mingw64\bin;$env:PATH"
rustup default stable-x86_64-pc-windows-gnu
cargo run
```
Or use windows sdk...

For camera live preview on Windows:
- Enable **Camera access** in Windows Privacy settings.
- Close other apps that may lock the webcam.
- Select the camera from the **Camera Overlay > Device** list.

### Installation
1.  Clone the repository:
    ```bash
    git clone https://github.com/Tenchirox/All4Laser.git
    cd All4Laser
    ```
2.  Build and run:
    ```bash
    cargo run --release
    ```


## 🙏 Acknowledgments

All4Laser is built on the shoulders of many outstanding open-source projects. We extend our sincere thanks to:

### Inspiration
| Project | Description |
|---------|-------------|
| [LaserMagic](https://gitlab.com/MadSquirrels/lasermagic/lasermagic) | Multi-protocol laser control library (GPL-v3). Inspired our `OutputProtocol` trait, Marlin/EGV support, SVG filter pipeline, speed-proportional overscan, and preview energy modulation. |
| [LightBurn](https://lightburnsoftware.com/) | Commercial laser software whose UI/UX patterns inspired our layer system, cut settings, and workspace layout. |
| [LaserGRBL](https://lasergrbl.com/) | Open-source GCode streamer for GRBL that pioneered accessible laser control on desktop. |

### Core Framework
| Crate | Role |
|-------|------|
| [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) | Immediate-mode GUI framework powering the entire UI. |
| [Rust](https://www.rust-lang.org/) | Systems programming language providing safety and performance. |

### Graphics & Imaging
| Crate | Role |
|-------|------|
| [image](https://github.com/image-rs/image) | Image decoding, encoding, and pixel manipulation. |
| [usvg](https://github.com/RazrFalcon/resvg) / [tiny-skia](https://github.com/RazrFalcon/tiny-skia) | SVG parsing and 2D rendering. |
| [rusttype](https://github.com/redox-os/rusttype) | Font rasterization for text-to-path conversion. |
| [font-kit](https://github.com/servo/font-kit) | Cross-platform font discovery and loading. |
| [qrcode](https://github.com/nickel-org/qrcode-rust) | QR code generation for laser engraving. |

### Geometry & Math
| Crate | Role |
|-------|------|
| [geo](https://github.com/georust/geo) | Computational geometry (buffering, boolean ops, polygon handling). |

### Communication & I/O
| Crate | Role |
|-------|------|
| [serialport](https://github.com/serialport/serialport-rs) | Cross-platform serial port communication with GRBL/Marlin controllers. |
| [ureq](https://github.com/algesten/ureq) | Lightweight HTTP client for update checking and API calls. |
| [v4l](https://github.com/nickel-org/v4l-rs) (Linux) / [nokhwa](https://github.com/nickel-org/nokhwa) (Windows) | Camera capture for workspace alignment overlay. |

### Data & Serialization
| Crate | Role |
|-------|------|
| [serde](https://github.com/serde-rs/serde) / [serde_json](https://github.com/serde-rs/json) | Settings persistence, project files, and material library serialization. |
| [lopdf](https://github.com/nickel-org/lopdf) | PDF import support. |
| [base64](https://github.com/marshallpierce/rust-base64) | Binary data encoding for embedded assets. |

### Utilities
| Crate | Role |
|-------|------|
| [crossbeam-channel](https://github.com/crossbeam-rs/crossbeam) | Lock-free channels for background task communication. |
| [rfd](https://github.com/nickel-org/rfd) | Native file dialogs (open/save). |
| [webbrowser](https://github.com/nickel-org/webbrowser-rs) | Opening URLs in the system browser. |
| [indexmap](https://github.com/bluss/indexmap) | Insertion-ordered maps for selection tracking. |
| [log](https://github.com/rust-lang/log) / [env_logger](https://github.com/rust-cli/env_logger) | Structured logging. |

Thank you to all the maintainers and contributors of these projects!

---

## 🤝 Contributing
Contributions are welcome! Please feel free to submit Pull Requests or open issues for feature requests and bug reports.

## 📄 License
This project is licensed under the GNU GPL V3.0 - see the [LICENSE](LICENSE) file for details.

---

It's a work in progress, so expect things to break !
