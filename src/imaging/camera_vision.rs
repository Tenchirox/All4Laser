use image::{Luma, RgbaImage, GrayImage, ImageBuffer};

/// Attempts to find an alignment mark (a dark circle or crosshair) in an image.
/// Returns the (x, y) coordinates of the center if found.
pub fn find_alignment_mark(img: &RgbaImage) -> Option<(f32, f32)> {
    let width = img.width();
    let height = img.height();

    // Convert to grayscale for processing
    let mut luma = GrayImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            // Simple luminosity
            let l = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;
            luma.put_pixel(x, y, Luma([l]));
        }
    }

    // A very simple blob detection approach:
    // 1. Threshold (keep dark pixels)
    let threshold = 100u8; // Assuming marks are black on a lighter background
    let mut thresholded = GrayImage::new(width, height);
    let mut sum_x: f64 = 0.0;
    let mut sum_y: f64 = 0.0;
    let mut count: f64 = 0.0;

    for y in 0..height {
        for x in 0..width {
            let p = luma.get_pixel(x, y)[0];
            if p < threshold {
                thresholded.put_pixel(x, y, Luma([255]));
                sum_x += x as f64;
                sum_y += y as f64;
                count += 1.0;
            } else {
                thresholded.put_pixel(x, y, Luma([0]));
            }
        }
    }

    // Require a minimum size to consider it a mark
    if count > 20.0 && count < (width as f64 * height as f64 * 0.1) {
        let cx = (sum_x / count) as f32;
        let cy = (sum_y / count) as f32;
        Some((cx, cy))
    } else {
        None
    }
}
