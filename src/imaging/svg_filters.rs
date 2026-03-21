#![allow(dead_code)]

//! SVG Filter pipeline — ported & adapted from LaserMagic (GPL-v3).
//!
//! Provides image-processing primitives that correspond to SVG filter elements:
//! `feGaussianBlur`, `feColorMatrix`, `feComponentTransfer`, `feBlend`,
//! `feComposite`, and `feFlood`.  These can be chained to reproduce the
//! rendering of filtered SVG images before raster engraving.

use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};

// ─── Color Matrix ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum ColorMatrixType {
    Matrix([[f32; 5]; 4]),
    Saturate(f32),
    HueRotate(f32),
    LuminanceToAlpha,
}

pub fn apply_color_matrix(img: &DynamicImage, matrix_type: &ColorMatrixType) -> DynamicImage {
    let buf = img.to_rgba8();
    let (width, height) = buf.dimensions();
    let mut out = ImageBuffer::new(width, height);

    for (x, y, pixel) in buf.enumerate_pixels() {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;
        let a = pixel[3] as f32 / 255.0;

        let (ro, go, bo, ao) = if a == 0.0 {
            (r, g, b, a)
        } else {
            match matrix_type {
                ColorMatrixType::Matrix(m) => (
                    (m[0][0] * r + m[0][1] * g + m[0][2] * b + m[0][3] * a + m[0][4]).clamp(0.0, 1.0),
                    (m[1][0] * r + m[1][1] * g + m[1][2] * b + m[1][3] * a + m[1][4]).clamp(0.0, 1.0),
                    (m[2][0] * r + m[2][1] * g + m[2][2] * b + m[2][3] * a + m[2][4]).clamp(0.0, 1.0),
                    (m[3][0] * r + m[3][1] * g + m[3][2] * b + m[3][3] * a + m[3][4]).clamp(0.0, 1.0),
                ),
                ColorMatrixType::Saturate(s) => {
                    let (lr, lg, lb) = (0.2126, 0.7152, 0.0722);
                    let lum = r * lr + g * lg + b * lb;
                    let inv = 1.0 - s;
                    (
                        (r * s + lum * inv).clamp(0.0, 1.0),
                        (g * s + lum * inv).clamp(0.0, 1.0),
                        (b * s + lum * inv).clamp(0.0, 1.0),
                        a,
                    )
                }
                ColorMatrixType::HueRotate(angle) => {
                    let rad = angle.to_radians();
                    let (cos_a, sin_a) = (rad.cos(), rad.sin());
                    let (lr, lg, lb) = (0.213, 0.715, 0.072);
                    let ro = (lr + cos_a * (1.0 - lr) + sin_a * (-lr)) * r
                        + (lg + cos_a * (-lg) + sin_a * (-lg)) * g
                        + (lb + cos_a * (-lb) + sin_a * (1.0 - lb)) * b;
                    let go = (lr + cos_a * (-lr) + sin_a * 0.143) * r
                        + (lg + cos_a * (1.0 - lg) + sin_a * 0.140) * g
                        + (lb + cos_a * (-lb) + sin_a * (-0.283)) * b;
                    let bo = (lr + cos_a * (-lr) + sin_a * (-(1.0 - lr))) * r
                        + (lg + cos_a * (-lg) + sin_a * lg) * g
                        + (lb + cos_a * (1.0 - lb) + sin_a * lb) * b;
                    (ro.clamp(0.0, 1.0), go.clamp(0.0, 1.0), bo.clamp(0.0, 1.0), a)
                }
                ColorMatrixType::LuminanceToAlpha => {
                    let lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
                    (0.0, 0.0, 0.0, lum.clamp(0.0, 1.0))
                }
            }
        };

        out.put_pixel(x, y, Rgba([
            (ro * 255.0) as u8,
            (go * 255.0) as u8,
            (bo * 255.0) as u8,
            (ao * 255.0) as u8,
        ]));
    }

    DynamicImage::ImageRgba8(out)
}

// ─── Gaussian Blur (separable) ───────────────────────────────────────────────

fn gaussian_kernel_1d(sigma: f32) -> Vec<f32> {
    if sigma <= 0.0 {
        return vec![1.0];
    }
    let radius = (sigma * 3.0).ceil() as i32;
    let mut kernel = Vec::with_capacity((2 * radius + 1) as usize);
    let mut sum = 0.0f32;
    for i in -radius..=radius {
        let v = (-((i as f32).powi(2)) / (2.0 * sigma * sigma)).exp();
        kernel.push(v);
        sum += v;
    }
    for v in &mut kernel {
        *v /= sum;
    }
    kernel
}

pub fn apply_gaussian_blur(img: &DynamicImage, sigma_x: f32, sigma_y: f32) -> DynamicImage {
    let buf = img.to_rgba8();
    let (width, height) = buf.dimensions();

    let kx = gaussian_kernel_1d(sigma_x);
    let ky = gaussian_kernel_1d(sigma_y);
    let rx = (kx.len() / 2) as i32;
    let ry = (ky.len() / 2) as i32;

    // Horizontal pass
    let mut temp: RgbaImage = ImageBuffer::new(width, height);
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let mut acc = [0.0f32; 4];
            for (k, weight) in kx.iter().enumerate() {
                let px = (x + k as i32 - rx).clamp(0, width as i32 - 1) as u32;
                let p = buf.get_pixel(px, y as u32).0;
                for c in 0..4 {
                    acc[c] += p[c] as f32 * weight;
                }
            }
            temp.put_pixel(
                x as u32,
                y as u32,
                Rgba([
                    acc[0].clamp(0.0, 255.0) as u8,
                    acc[1].clamp(0.0, 255.0) as u8,
                    acc[2].clamp(0.0, 255.0) as u8,
                    acc[3].clamp(0.0, 255.0) as u8,
                ]),
            );
        }
    }

    // Vertical pass
    let mut out: RgbaImage = ImageBuffer::new(width, height);
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let mut acc = [0.0f32; 4];
            for (k, weight) in ky.iter().enumerate() {
                let py = (y + k as i32 - ry).clamp(0, height as i32 - 1) as u32;
                let p = temp.get_pixel(x as u32, py).0;
                for c in 0..4 {
                    acc[c] += p[c] as f32 * weight;
                }
            }
            out.put_pixel(
                x as u32,
                y as u32,
                Rgba([
                    acc[0].clamp(0.0, 255.0) as u8,
                    acc[1].clamp(0.0, 255.0) as u8,
                    acc[2].clamp(0.0, 255.0) as u8,
                    acc[3].clamp(0.0, 255.0) as u8,
                ]),
            );
        }
    }

    DynamicImage::ImageRgba8(out)
}

// ─── Blend Modes ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Darken,
    Lighten,
}

pub fn apply_blend(
    bottom: &DynamicImage,
    top: &DynamicImage,
    mode: BlendMode,
) -> DynamicImage {
    let b1 = bottom.to_rgba8();
    let b2 = top.to_rgba8();
    let (width, height) = b1.dimensions();
    let mut out = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let p1 = b1.get_pixel(x, y).0.map(|c| c as f32 / 255.0);
            let p2 = if x < b2.width() && y < b2.height() {
                b2.get_pixel(x, y).0.map(|c| c as f32 / 255.0)
            } else {
                [0.0, 0.0, 0.0, 0.0]
            };

            let (r1, g1, b_1, a1) = (p1[0], p1[1], p1[2], p1[3]);
            let (r2, g2, b_2, a2) = (p2[0], p2[1], p2[2], p2[3]);

            let (r, g, b) = match mode {
                BlendMode::Normal => (r2, g2, b_2),
                BlendMode::Multiply => (r1 * r2, g1 * g2, b_1 * b_2),
                BlendMode::Screen => (
                    1.0 - (1.0 - r1) * (1.0 - r2),
                    1.0 - (1.0 - g1) * (1.0 - g2),
                    1.0 - (1.0 - b_1) * (1.0 - b_2),
                ),
                BlendMode::Darken => (r1.min(r2), g1.min(g2), b_1.min(b_2)),
                BlendMode::Lighten => (r1.max(r2), g1.max(g2), b_1.max(b_2)),
            };

            // Source-over alpha compositing
            let out_a = a2 + a1 * (1.0 - a2);
            let (out_r, out_g, out_b) = if out_a > 0.0 {
                (
                    (r * a2 + r1 * a1 * (1.0 - a2)) / out_a,
                    (g * a2 + g1 * a1 * (1.0 - a2)) / out_a,
                    (b * a2 + b_1 * a1 * (1.0 - a2)) / out_a,
                )
            } else {
                (0.0, 0.0, 0.0)
            };

            out.put_pixel(x, y, Rgba([
                (out_r.clamp(0.0, 1.0) * 255.0) as u8,
                (out_g.clamp(0.0, 1.0) * 255.0) as u8,
                (out_b.clamp(0.0, 1.0) * 255.0) as u8,
                (out_a.clamp(0.0, 1.0) * 255.0) as u8,
            ]));
        }
    }

    DynamicImage::ImageRgba8(out)
}

// ─── Composite (Porter-Duff) ─────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub enum CompositeOp {
    Over,
    In,
    Out,
    Atop,
    Xor,
    Arithmetic { k1: f32, k2: f32, k3: f32, k4: f32 },
}

pub fn apply_composite(
    img1: &DynamicImage,
    img2: &DynamicImage,
    op: CompositeOp,
) -> DynamicImage {
    let b1 = img1.to_rgba8();
    let b2 = img2.to_rgba8();
    let (width, height) = b1.dimensions();
    let mut out = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let p1 = b1.get_pixel(x, y).0.map(|c| c as f32 / 255.0);
            let p2 = if x < b2.width() && y < b2.height() {
                b2.get_pixel(x, y).0.map(|c| c as f32 / 255.0)
            } else {
                [0.0, 0.0, 0.0, 0.0]
            };

            let (r1, g1, b_1, a1) = (p1[0], p1[1], p1[2], p1[3]);
            let (r2, g2, b_2, a2) = (p2[0], p2[1], p2[2], p2[3]);

            let (r, g, b, a) = match op {
                CompositeOp::Over => {
                    let ao = a2 + a1 * (1.0 - a2);
                    if ao > 0.0 {
                        (
                            (r2 * a2 + r1 * a1 * (1.0 - a2)) / ao,
                            (g2 * a2 + g1 * a1 * (1.0 - a2)) / ao,
                            (b_2 * a2 + b_1 * a1 * (1.0 - a2)) / ao,
                            ao,
                        )
                    } else {
                        (0.0, 0.0, 0.0, 0.0)
                    }
                }
                CompositeOp::In => (r1 * a2, g1 * a2, b_1 * a2, a1 * a2),
                CompositeOp::Out => {
                    let f = 1.0 - a2;
                    (r1 * f, g1 * f, b_1 * f, a1 * f)
                }
                CompositeOp::Atop => {
                    let ao = a2;
                    if ao > 0.0 {
                        (
                            (r1 * a2 + r2 * (1.0 - a1)) / ao,
                            (g1 * a2 + g2 * (1.0 - a1)) / ao,
                            (b_1 * a2 + b_2 * (1.0 - a1)) / ao,
                            ao,
                        )
                    } else {
                        (0.0, 0.0, 0.0, 0.0)
                    }
                }
                CompositeOp::Xor => (
                    r1 * (1.0 - a2) + r2 * (1.0 - a1),
                    g1 * (1.0 - a2) + g2 * (1.0 - a1),
                    b_1 * (1.0 - a2) + b_2 * (1.0 - a1),
                    a1 * (1.0 - a2) + a2 * (1.0 - a1),
                ),
                CompositeOp::Arithmetic { k1, k2, k3, k4 } => (
                    (k1 * r1 * r2 + k2 * r1 + k3 * r2 + k4).clamp(0.0, 1.0),
                    (k1 * g1 * g2 + k2 * g1 + k3 * g2 + k4).clamp(0.0, 1.0),
                    (k1 * b_1 * b_2 + k2 * b_1 + k3 * b_2 + k4).clamp(0.0, 1.0),
                    (k1 * a1 * a2 + k2 * a1 + k3 * a2 + k4).clamp(0.0, 1.0),
                ),
            };

            out.put_pixel(x, y, Rgba([
                (r.clamp(0.0, 1.0) * 255.0) as u8,
                (g.clamp(0.0, 1.0) * 255.0) as u8,
                (b.clamp(0.0, 1.0) * 255.0) as u8,
                (a.clamp(0.0, 1.0) * 255.0) as u8,
            ]));
        }
    }

    DynamicImage::ImageRgba8(out)
}

// ─── Component Transfer (feFuncR/G/B/A) ─────────────────────────────────────

#[derive(Debug, Clone)]
pub enum TransferFunction {
    Identity,
    Table(Vec<f32>),
    Discrete(Vec<f32>),
    Linear { slope: f32, intercept: f32 },
    Gamma { amplitude: f32, exponent: f32, offset: f32 },
}

#[derive(Debug, Clone)]
pub struct ComponentTransfer {
    pub r: TransferFunction,
    pub g: TransferFunction,
    pub b: TransferFunction,
    pub a: TransferFunction,
}

impl Default for ComponentTransfer {
    fn default() -> Self {
        Self {
            r: TransferFunction::Identity,
            g: TransferFunction::Identity,
            b: TransferFunction::Identity,
            a: TransferFunction::Identity,
        }
    }
}

fn apply_transfer(f: &TransferFunction, x: f32) -> f32 {
    match f {
        TransferFunction::Identity => x,
        TransferFunction::Table(values) => {
            if values.is_empty() {
                return x;
            }
            let idx = x * (values.len() as f32 - 1.0);
            let i0 = idx.floor() as usize;
            let i1 = (i0 + 1).min(values.len() - 1);
            let t = idx - i0 as f32;
            values[i0] * (1.0 - t) + values[i1] * t
        }
        TransferFunction::Discrete(values) => {
            if values.is_empty() {
                return x;
            }
            let idx = (x * values.len() as f32).floor() as usize;
            values[idx.min(values.len() - 1)]
        }
        TransferFunction::Linear { slope, intercept } => slope * x + intercept,
        TransferFunction::Gamma { amplitude, exponent, offset } => {
            amplitude * x.powf(*exponent) + offset
        }
    }
}

pub fn apply_component_transfer(
    img: &DynamicImage,
    transfer: &ComponentTransfer,
) -> DynamicImage {
    let buf = img.to_rgba8();
    let (width, height) = buf.dimensions();
    let mut out = ImageBuffer::new(width, height);

    for (x, y, p) in buf.enumerate_pixels() {
        let (r, g, b, a) = (
            p[0] as f32 / 255.0,
            p[1] as f32 / 255.0,
            p[2] as f32 / 255.0,
            p[3] as f32 / 255.0,
        );

        out.put_pixel(x, y, Rgba([
            (apply_transfer(&transfer.r, r).clamp(0.0, 1.0) * 255.0) as u8,
            (apply_transfer(&transfer.g, g).clamp(0.0, 1.0) * 255.0) as u8,
            (apply_transfer(&transfer.b, b).clamp(0.0, 1.0) * 255.0) as u8,
            (apply_transfer(&transfer.a, a).clamp(0.0, 1.0) * 255.0) as u8,
        ]));
    }

    DynamicImage::ImageRgba8(out)
}

// ─── Flood Fill (feFlood) ────────────────────────────────────────────────────

pub fn create_flood(
    width: u32,
    height: u32,
    color: (u8, u8, u8),
    opacity: f32,
) -> DynamicImage {
    let a = (opacity.clamp(0.0, 1.0) * 255.0) as u8;
    let img = ImageBuffer::from_pixel(width, height, Rgba([color.0, color.1, color.2, a]));
    DynamicImage::ImageRgba8(img)
}

// ─── Filter Pipeline ─────────────────────────────────────────────────────────

/// A single filter step in the pipeline.
#[derive(Debug, Clone)]
pub enum FilterStep {
    GaussianBlur { sigma_x: f32, sigma_y: f32 },
    ColorMatrix(ColorMatrixType),
    ComponentTransfer(ComponentTransfer),
    Blend { mode: BlendMode },
    Composite { op: CompositeOp },
    Flood { color: (u8, u8, u8), opacity: f32 },
}

/// Apply a sequence of filter steps to an image.
/// For two-input operations (Blend, Composite), the second input is the
/// accumulated result from the previous step.
pub fn apply_pipeline(source: &DynamicImage, steps: &[FilterStep]) -> DynamicImage {
    let mut current = source.clone();

    for step in steps {
        current = match step {
            FilterStep::GaussianBlur { sigma_x, sigma_y } => {
                apply_gaussian_blur(&current, *sigma_x, *sigma_y)
            }
            FilterStep::ColorMatrix(cm) => apply_color_matrix(&current, cm),
            FilterStep::ComponentTransfer(ct) => apply_component_transfer(&current, ct),
            FilterStep::Blend { mode } => apply_blend(source, &current, *mode),
            FilterStep::Composite { op } => apply_composite(source, &current, *op),
            FilterStep::Flood { color, opacity } => {
                let (w, h) = (current.width(), current.height());
                let flood = create_flood(w, h, *color, *opacity);
                apply_composite(&current, &flood, CompositeOp::Over)
            }
        };
    }

    current
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn test_image() -> DynamicImage {
        let mut img = RgbaImage::new(4, 4);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = Rgba([
                ((x * 60) as u8).wrapping_add(50),
                ((y * 60) as u8).wrapping_add(50),
                128,
                255,
            ]);
        }
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn identity_color_matrix_preserves_image() {
        let img = test_image();
        let identity = ColorMatrixType::Matrix([
            [1.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0, 0.0],
        ]);
        let result = apply_color_matrix(&img, &identity);
        let src = img.to_rgba8();
        let dst = result.to_rgba8();
        for (p1, p2) in src.pixels().zip(dst.pixels()) {
            assert_eq!(p1, p2);
        }
    }

    #[test]
    fn saturate_zero_is_grayscale() {
        let img = test_image();
        let result = apply_color_matrix(&img, &ColorMatrixType::Saturate(0.0));
        let buf = result.to_rgba8();
        for pixel in buf.pixels() {
            // All channels should be equal (grayscale) within rounding
            let diff_rg = (pixel[0] as i32 - pixel[1] as i32).unsigned_abs();
            let diff_rb = (pixel[0] as i32 - pixel[2] as i32).unsigned_abs();
            assert!(diff_rg <= 1, "R-G diff: {diff_rg}");
            assert!(diff_rb <= 1, "R-B diff: {diff_rb}");
        }
    }

    #[test]
    fn gaussian_blur_preserves_dimensions() {
        let img = test_image();
        let blurred = apply_gaussian_blur(&img, 1.0, 1.0);
        assert_eq!(img.width(), blurred.width());
        assert_eq!(img.height(), blurred.height());
    }

    #[test]
    fn blend_normal_returns_top() {
        let bottom = test_image();
        let top = create_flood(4, 4, (255, 0, 0), 1.0);
        let result = apply_blend(&bottom, &top, BlendMode::Normal);
        let buf = result.to_rgba8();
        for pixel in buf.pixels() {
            assert_eq!(pixel[0], 255);
            assert_eq!(pixel[1], 0);
            assert_eq!(pixel[2], 0);
        }
    }

    #[test]
    fn composite_in_masks_by_alpha() {
        let img = test_image();
        let mask = create_flood(4, 4, (0, 0, 0), 0.0);
        let result = apply_composite(&img, &mask, CompositeOp::In);
        let buf = result.to_rgba8();
        for pixel in buf.pixels() {
            assert_eq!(pixel[3], 0, "alpha should be 0 after In with transparent mask");
        }
    }

    #[test]
    fn component_transfer_linear() {
        let img = create_flood(2, 2, (128, 128, 128), 1.0);
        let transfer = ComponentTransfer {
            r: TransferFunction::Linear { slope: 2.0, intercept: 0.0 },
            g: TransferFunction::Identity,
            b: TransferFunction::Identity,
            a: TransferFunction::Identity,
        };
        let result = apply_component_transfer(&img, &transfer);
        let buf = result.to_rgba8();
        let pixel = buf.get_pixel(0, 0);
        // 128/255 ≈ 0.502, *2 = 1.004 → clamped to 1.0 → 255
        assert_eq!(pixel[0], 255);
        assert_eq!(pixel[1], 128);
    }

    #[test]
    fn pipeline_chain_works() {
        let img = test_image();
        let steps = vec![
            FilterStep::GaussianBlur { sigma_x: 0.5, sigma_y: 0.5 },
            FilterStep::ColorMatrix(ColorMatrixType::Saturate(0.5)),
        ];
        let result = apply_pipeline(&img, &steps);
        assert_eq!(result.width(), img.width());
        assert_eq!(result.height(), img.height());
    }

    #[test]
    fn flood_creates_solid_color() {
        let img = create_flood(10, 10, (100, 200, 50), 0.8);
        let buf = img.to_rgba8();
        let px = buf.get_pixel(5, 5);
        assert_eq!(px[0], 100);
        assert_eq!(px[1], 200);
        assert_eq!(px[2], 50);
        assert_eq!(px[3], 204); // 0.8 * 255 = 204
    }
}
