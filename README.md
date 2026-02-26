# All4Laser ⚡

**All4Laser** is a high-performance, professional-grade laser control and GCode sender built with **Rust** and **egui**. Designed for efficiency and precision, it provides a modern alternative to traditional laser software with a focus on speed, cross-platform compatibility, and advanced job preparation tools.

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

### 🎨 Modern & Flexible UI
*   **Dual Layouts**: Switch between a **Modern** unified sidebar or a **Classic (Industrial)** layout inspired by industry standards like LightBurn.
*   **Custom Themes**: Beautiful dark/light modes (Catppuccin inspired) or high-contrast "Mercury/Cobalt" industrial themes.
*   **Integrated GCode Editor**: Tweak and apply changes to your programs directly within the app.

---

## 🚀 Getting Started

### Prerequisites
*   **Rust Toolchain**: [Install Rust](https://rustup.rs/) (latest stable version recommended).
*   **Serial Port Permissions**: (Linux) Ensure your user is in the `dialout` or `uucp` group.

### Installation
1.  Clone the repository:
    ```bash
    git clone https://github.com/your-username/All4Laser.git
    cd All4Laser
    ```
2.  Build and run:
    ```bash
    cargo run --release
    ```

---

## 🛠 Technology Stack
*   **Language**: [Rust](https://www.rust-lang.org/) (Safety and Performance).
*   **GUI Framework**: [egui](https://github.com/emilk/egui) (Instant-mode, GPU accelerated).
*   **Serial**: [serialport-rs](https://github.com/serialport/serialport-rs) for robust machine communication.
*   **Config**: Persistent JSON-based settings for machine profiles and material libraries.

---

## 🤝 Contributing
Contributions are welcome! Please feel free to submit Pull Requests or open issues for feature requests and bug reports.

## 📄 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
*Built with ❤️ for the laser community using Rust.*
