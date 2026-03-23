#![allow(dead_code)]

use image::{GrayImage, Luma};

/// Parameters for a cutting frame (outline) around the image
#[derive(Clone, Debug, PartialEq)]
pub struct OutlineParams {
    pub enabled: bool,
    pub speed: f32,
    pub power: f32,
    pub passes: u32,
}

fn compensated_power(base_power: f32, params: &RasterParams) -> f32 {
    let mut power = base_power.clamp(0.0, params.max_power);
    if !params.spot_interp_enabled || params.max_power <= 0.0 {
        return power;
    }

    let min_spot = params.spot_size_min_mm.max(0.001);
    let max_spot = params.spot_size_max_mm.max(min_spot);
    let ratio = (power / params.max_power).clamp(0.0, 1.0);
    let spot = min_spot + (max_spot - min_spot) * ratio;

    let density_comp = (min_spot / spot).clamp(0.1, 1.0);
    power = (power * density_comp).clamp(0.0, params.max_power);
    power
}

impl Default for OutlineParams {
    fn default() -> Self {
        Self {
            enabled: false,
            speed: 500.0,
            power: 1000.0,
            passes: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DitherMode {
    None,
    FloydSteinberg,
    Atkinson,
}

/// Parameters for raster-to-GCode conversion
#[derive(Clone, Debug, PartialEq)]
pub struct RasterParams {
    pub width_mm: f32,
    pub height_mm: f32,
    pub dpi: f32,
    pub max_speed: f32,
    pub max_power: f32,
    pub spot_interp_enabled: bool,
    pub spot_size_min_mm: f32,
    pub spot_size_max_mm: f32,
    pub brightness: f32, // -1.0 to 1.0
    pub contrast: f32,   // 0.0 to 5.0 (1.0 is neutral)
    pub threshold: u8,   // 0-255 for vectorization
    pub smoothing: f32, // 0.0 to 1.0 for path smoothing
    pub flip_h: bool,
    pub flip_v: bool,
    pub rotation: i32, // 0, 90, 180, 270
    pub _line_spacing: f32,
    pub dither: DitherMode,
    pub use_skeleton: bool, // New flag for skeletonization
    pub outline: OutlineParams,
}

impl Default for RasterParams {
    fn default() -> Self {
        Self {
            width_mm: 50.0,
            height_mm: 50.0,
            dpi: 254.0, // 10 lines/mm
            max_speed: 1000.0,
            max_power: 1000.0,
            spot_interp_enabled: false,
            spot_size_min_mm: 0.08,
            spot_size_max_mm: 0.2,
            brightness: 0.0,
            contrast: 1.0,
            threshold: 128,
            smoothing: 0.5,
            flip_h: false,
            flip_v: false,
            rotation: 0,
            _line_spacing: 0.1, // mm
            dither: DitherMode::FloydSteinberg,
            use_skeleton: false,
            outline: OutlineParams::default(),
        }
    }
}

pub fn vectorize_image(img: &image::DynamicImage, params: &RasterParams) -> Vec<String> {
    let processed = preprocess_image(img, params);
    let gray = processed.to_luma8();
    let (iw, ih) = gray.dimensions();

    let target_w = (params.width_mm * params.dpi / 25.4) as u32;
    let target_h = (params.height_mm * params.dpi / 25.4) as u32;

    let resized = image::imageops::resize(
        &gray,
        target_w,
        target_h,
        image::imageops::FilterType::Lanczos3,
    );
    let (rw, rh) = resized.dimensions();

    let mut gcode = Vec::new();
    gcode.push("; Vectorized by All4Laser".to_string());
    gcode.push(format!("; Source: {}x{} → {}x{}", iw, ih, rw, rh));
    gcode.push("G90".into());
    gcode.push("G21".into());
    gcode.push("M3".into());

    let x_scale = params.width_mm / rw as f32;
    let y_scale = params.height_mm / rh as f32;

    for y_idx in 0..rh {
        let py = (rh - 1 - y_idx) as f32 * y_scale;
        let mut in_run = false;
        let mut start_x = 0.0;

        for x_idx in 0..rw {
            let pixel = resized.get_pixel(x_idx, y_idx)[0];
            let is_black = pixel < params.threshold;

            if is_black && !in_run {
                in_run = true;
                start_x = x_idx as f32 * x_scale;
            } else if !is_black && in_run {
                in_run = false;
                let end_x = (x_idx - 1) as f32 * x_scale;
                gcode.push(format!("G0X{:.3}Y{:.3}S0", start_x, py));
                let power = compensated_power(params.max_power, params);
                gcode.push(format!(
                    "G1X{:.3}Y{:.3}S{:.0}F{:.0}",
                    end_x, py, power, params.max_speed
                ));
            }
        }
        if in_run {
            let end_x = (rw - 1) as f32 * x_scale;
            gcode.push(format!("G0X{:.3}Y{:.3}S0", start_x, py));
            let power = compensated_power(params.max_power, params);
            gcode.push(format!(
                "G1X{:.3}Y{:.3}S{:.0}F{:.0}",
                end_x, py, power, params.max_speed
            ));
        }
    }

    gcode.push("M5".into());
    gcode.push("G0X0Y0".into());
    gcode
}

/// Convert a raster image to GCode using line-scan with variable power
pub fn image_to_gcode(img: &image::DynamicImage, params: &RasterParams) -> Vec<String> {
    // 1. Preprocess (Brightness, Contrast, Grayscale)
    let processed = preprocess_image(img, params);
    let gray = processed.to_luma8();
    let (iw, ih) = gray.dimensions();

    // Resize to match target dimensions at given DPI
    let target_w = (params.width_mm * params.dpi / 25.4) as u32;
    let target_h = (params.height_mm * params.dpi / 25.4) as u32;

    let resized = image::imageops::resize(
        &gray,
        target_w,
        target_h,
        image::imageops::FilterType::Lanczos3,
    );

    let dithered = match params.dither {
        DitherMode::FloydSteinberg => floyd_steinberg_dither(&resized),
        DitherMode::Atkinson => atkinson_dither(&resized),
        DitherMode::None => resized,
    };
    let (rw, rh) = dithered.dimensions();

    let mut gcode = Vec::new();
    gcode.push("; Generated by All4Laser".to_string());
    gcode.push(format!("; Source: {}x{} → {}x{} pixels", iw, ih, rw, rh));
    gcode.push(format!(
        "; Target: {:.1}x{:.1} mm",
        params.width_mm, params.height_mm
    ));
    gcode.push("G90".to_string()); // Absolute
    gcode.push("G21".to_string()); // Millimeters
    gcode.push("M4".to_string()); // Dynamic laser mode

    let x_scale = params.width_mm / rw as f32;
    let y_scale = params.height_mm / rh as f32;

    for row in 0..rh {
        let y = row as f32 * y_scale;
        let reverse = row % 2 == 1; // Bidirectional scanning

        // Move to start of line
        let start_x = if reverse { params.width_mm } else { 0.0 };
        gcode.push(format!("G0X{start_x:.3}Y{y:.3}S0"));

        // Scan across - using G1 for the whole line and only modulating S
        let cols: Box<dyn Iterator<Item = u32>> = if reverse {
            Box::new((0..rw).rev())
        } else {
            Box::new(0..rw)
        };

        let mut first_col = true;
        for col in cols {
            let pixel = dithered.get_pixel(col, rh - 1 - row); // rh-1-row because row 0 is at Y=0
            let brightness = pixel[0];
            let base_power = (255 - brightness) as f32 / 255.0 * params.max_power;
            let power = compensated_power(base_power, params) as u32;
            let x = col as f32 * x_scale;

            if first_col {
                gcode.push(format!("G1X{x:.3}Y{y:.3}S{power}F{:.0}", params.max_speed));
                first_col = false;
            } else {
                gcode.push(format!("X{x:.3}S{power}"));
            }
        }
    }

    // --- Cutting Frame (Outline) ---
    if params.outline.enabled && params.outline.passes > 0 {
        gcode.push("; Cutting Frame".to_string());
        let w = params.width_mm;
        let h = params.height_mm;
        let s = params.outline.speed;
        let p = params.outline.power;

        for i in 0..params.outline.passes {
            gcode.push(format!("; Pass {}", i + 1));
            // Move to start of frame (bottom-left)
            gcode.push(format!("G0X0Y0S0"));
            // Trace rectangle
            gcode.push(format!("G1X{w:.3}Y0S{p}F{s}"));
            gcode.push(format!("G1X{w:.3}Y{h:.3}"));
            gcode.push(format!("G1X0Y{h:.3}"));
            gcode.push(format!("G1X0Y0"));
        }
        gcode.push("M5".to_string());
    }

    gcode.push("M5".to_string());
    gcode.push("G0X0Y0".to_string());
    gcode
}

/// Floyd-Steinberg dithering on a grayscale image
fn floyd_steinberg_dither(img: &GrayImage) -> GrayImage {
    let (w, h) = img.dimensions();
    let wu = w as usize;
    let hu = h as usize;

    let mut buf: Vec<f32> = vec![0.0; wu * hu];
    for y in 0..h {
        let offset = (y as usize) * wu;
        for x in 0..w {
            buf[offset + (x as usize)] = img.get_pixel(x, y)[0] as f32;
        }
    }

    for y in 0..h {
        let y_offset = (y as usize) * wu;
        for x in 0..w {
            let idx = y_offset + (x as usize);
            let old = buf[idx];
            let new_val = if old > 127.0 { 255.0 } else { 0.0 };
            buf[idx] = new_val;
            let error = old - new_val;

            let spread = |buf: &mut [f32], px: u32, py: u32, factor: f32| {
                if px < w && py < h {
                    buf[(py as usize) * wu + (px as usize)] += error * factor;
                }
            };

            spread(&mut buf, x + 1, y, 7.0 / 16.0);
            if x > 0 {
                spread(&mut buf, x - 1, y + 1, 3.0 / 16.0);
            }
            spread(&mut buf, x, y + 1, 5.0 / 16.0);
            spread(&mut buf, x + 1, y + 1, 1.0 / 16.0);
        }
    }

    let mut output = GrayImage::new(w, h);
    for y in 0..h {
        let offset = (y as usize) * wu;
        for x in 0..w {
            let v = buf[offset + (x as usize)].clamp(0.0, 255.0) as u8;
            output.put_pixel(x, y, Luma([v]));
        }
    }
    output
}

/// Atkinson dithering (preserves more detail, higher contrast)
fn atkinson_dither(img: &GrayImage) -> GrayImage {
    let (w, h) = img.dimensions();
    let wu = w as usize;
    let hu = h as usize;

    let mut buf: Vec<f32> = vec![0.0; wu * hu];
    for y in 0..h {
        let offset = (y as usize) * wu;
        for x in 0..w {
            buf[offset + (x as usize)] = img.get_pixel(x, y)[0] as f32;
        }
    }

    for y in 0..h {
        let y_offset = (y as usize) * wu;
        for x in 0..w {
            let idx = y_offset + (x as usize);
            let old = buf[idx];
            let new_val = if old > 127.0 { 255.0 } else { 0.0 };
            buf[idx] = new_val;
            let error = (old - new_val) / 8.0; // Atkinson spreads 1/8 to each neighbor

            let spread = |buf: &mut [f32], px: u32, py: u32| {
                if px < w && py < h {
                    buf[(py as usize) * wu + (px as usize)] += error;
                }
            };

            spread(&mut buf, x + 1, y);
            spread(&mut buf, x + 2, y);
            if x > 0 {
                spread(&mut buf, x - 1, y + 1);
            }
            spread(&mut buf, x, y + 1);
            spread(&mut buf, x + 1, y + 1);
            spread(&mut buf, x, y + 2);
        }
    }

    let mut output = GrayImage::new(w, h);
    for y in 0..h {
        let offset = (y as usize) * wu;
        for x in 0..w {
            let v = buf[offset + (x as usize)].clamp(0.0, 255.0) as u8;
            output.put_pixel(x, y, Luma([v]));
        }
    }
    output
}

/// Composite RGBA onto white background, then convert to grayscale.
/// Transparent pixels become white (255 = no engraving).
fn alpha_composite_to_luma(img: &image::DynamicImage) -> GrayImage {
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let mut gray = GrayImage::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let px = rgba.get_pixel(x, y);
            let r = px[0] as f32;
            let g = px[1] as f32;
            let b = px[2] as f32;
            let a = px[3] as f32 / 255.0;
            // Composite onto white (255) background
            let rb = r * a + 255.0 * (1.0 - a);
            let gb = g * a + 255.0 * (1.0 - a);
            let bb = b * a + 255.0 * (1.0 - a);
            // Standard luminance conversion
            let luma = (0.299 * rb + 0.587 * gb + 0.114 * bb).clamp(0.0, 255.0) as u8;
            gray.put_pixel(x, y, Luma([luma]));
        }
    }
    gray
}

/// Apply brightness/contrast/flip/rotation while preserving the alpha channel (for preview).
/// Unlike `preprocess_image`, this does NOT composite onto white — transparent pixels stay transparent.
pub fn preprocess_image_rgba(img: &image::DynamicImage, params: &RasterParams) -> image::DynamicImage {
    let mut rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();

    // Apply brightness & contrast to RGB only, preserve alpha
    if params.brightness != 0.0 || params.contrast != 1.0 {
        let bright_offset = params.brightness * 255.0;
        for y in 0..h {
            for x in 0..w {
                let px = rgba.get_pixel_mut(x, y);
                for ch in 0..3 {
                    let mut v = px[ch] as f32;
                    // Contrast: scale around midpoint 128
                    if params.contrast != 1.0 {
                        v = (v - 128.0) * params.contrast + 128.0;
                    }
                    // Brightness
                    v += bright_offset;
                    px[ch] = v.clamp(0.0, 255.0) as u8;
                }
                // px[3] (alpha) unchanged
            }
        }
    }

    let mut processed = image::DynamicImage::ImageRgba8(rgba);

    if params.flip_h {
        processed = processed.fliph();
    }
    if params.flip_v {
        processed = processed.flipv();
    }

    processed = match params.rotation {
        90 => processed.rotate90(),
        180 => processed.rotate180(),
        270 => processed.rotate270(),
        _ => processed,
    };

    processed
}

/// Apply brightness and contrast adjustments (for GCode — composites alpha onto white)
pub fn preprocess_image(img: &image::DynamicImage, params: &RasterParams) -> image::DynamicImage {
    // Alpha-composite onto white, then grayscale — transparent = white = no burn
    let mut processed = image::DynamicImage::ImageLuma8(alpha_composite_to_luma(img));

    if params.brightness != 0.0 {
        // brightness(f32) where 0.0 is no change, -1.0 is black, 1.0 is white
        // The image crate uses i32 for brightness but it's easier to use f32 here
        processed = processed.brighten((params.brightness * 255.0) as i32);
    }

    if params.contrast != 1.0 {
        // contrast(f32) where 1.0 is no change
        // Wait, image::DynamicImage has adjust_contrast(f32)
        processed = processed.adjust_contrast(params.contrast);
    }

    if params.flip_h {
        processed = processed.fliph();
    }
    if params.flip_v {
        processed = processed.flipv();
    }

    processed = match params.rotation {
        90 => processed.rotate90(),
        180 => processed.rotate180(),
        270 => processed.rotate270(),
        _ => processed,
    };

    processed
}
