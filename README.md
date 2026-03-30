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

## 🔌 Driver Profiles
All4Laser includes multiple pipeline driver profiles inspired by LibLaserCut. Profiles can be selected per machine controller in the UI.

### Stable Profiles
- `Auto (by controller)`
- `GRBL Generic`
- `GRBL Device-Safe`
- `Marlin Line Protocol`
- `Ruida Line Bridge`
- `Trocen Line Bridge`

### Experimental Profiles
- `Smoothie GCode`
- `Lasersaur GCode`
- `K40 Nano Bridge`
- `GoldCut HPGL`
- `Epilog Zing Bridge`
- `Epilog Helix Bridge`
- `Full Spectrum Bridge`
- `Laos Cutter Bridge`
- `MakeBlock XY Plotter`
- `LaserTools Technics`
- `Dummy Driver`
- `Sample Driver`
- `IModela Mill`
- `K3 Engraver`

Experimental profiles are provided for compatibility research and adapter workflows; validate on non-production jobs before machine use.


## 🤝 Contributing
Contributions are welcome! Please feel free to submit Pull Requests or open issues for feature requests and bug reports.

## � Acknowledgements
Special thanks to the open-source laser ecosystem and the following projects:

- [LibLaserCut](https://github.com/t-oster/LibLaserCut/)
- [LaserGRBL](https://lasergrbl.com/)
- [Rayforge](https://rayforge.org/)
- [LaserMagic](https://lasermagic.ci-yow.com/)

## �� License
This project is licensed under the GNU GPL V3.0 - see the [LICENSE](LICENSE) file for details.

---

It's a work in progress, so expect things to break !
