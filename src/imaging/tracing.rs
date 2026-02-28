/// Image Tracing Logic (Bitmap to Vector)
/// Uses a simplified skeletonization (thinning) algorithm followed by path tracing.

use crate::ui::drawing::{ShapeParams, ShapeKind};
use crate::imaging::raster::RasterParams;
use image::{DynamicImage, GenericImageView, GrayImage, Luma};

/// Traces a bitmap image into a set of Vector Shapes
pub fn trace_image(img: &DynamicImage, params: &RasterParams) -> Vec<ShapeParams> {
    // 1. Preprocess (Thresholding)
    let processed = crate::imaging::raster::preprocess_image(img, params);
    let mut gray = processed.to_luma8();
    let (width, height) = gray.dimensions();
    let threshold = params.threshold;

    // Binarize
    for y in 0..height {
        for x in 0..width {
            let p = gray.get_pixel_mut(x, y);
            p.0 = if p.0[0] < threshold { [0] } else { [255] }; // Black is object (0)
        }
    }

    // 2. Skeletonize if requested
    if params.use_skeleton {
        // Invert for Zhang-Suen (usually expects foreground=1, background=0)
        // Here we have Black=0 (Object).
        // Let's invert so Object=255/1 for the algo.
        for p in gray.pixels_mut() {
            p.0 = [255 - p.0[0]];
        }

        zhang_suen_thinning(&mut gray);

        // Invert back so Object is Black (if tracing expects black) or just trace white.
        // Let's trace white pixels (255) as paths.
    } else {
        // If not skeletonizing, we just invert so we trace white pixels?
        // Original logic traced black pixels (< threshold).
        // Let's standardise on: We trace foreground pixels.
        // If skeleton: foreground is white (thin lines).
        // If contour: foreground is black?
        // Let's just assume we trace White pixels (255) from here on.
        for p in gray.pixels_mut() {
            p.0 = [255 - p.0[0]];
        }
    }

    // 3. Trace paths from foreground pixels
    // For skeleton, we just follow connected neighbors.
    let mut visited = vec![false; (width * height) as usize];
    let mut shapes = Vec::new();

    let x_scale = params.width_mm / width as f32;
    let y_scale = params.height_mm / height as f32;

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            if visited[idx] { continue; }

            let pixel = gray.get_pixel(x, y)[0];
            if pixel > 128 { // Foreground found
                let path_pixels = trace_line(&gray, x, y, &mut visited);
                if path_pixels.len() > 2 {
                    // Convert to mm
                    let points: Vec<(f32, f32)> = path_pixels.iter().map(|(px, py)| {
                        // Flip Y for G-code coordinate system (bottom-left origin vs top-left image)
                        let mm_x = *px as f32 * x_scale;
                        let mm_y = (height - 1 - *py) as f32 * y_scale;
                        (mm_x, mm_y)
                    }).collect();

                    // Optimize/Simplify points (naive subsampling)
                    let simplified = if points.len() > 10 {
                        points.into_iter().step_by(2).collect()
                    } else {
                        points
                    };

                    shapes.push(ShapeParams {
                        shape: ShapeKind::Path(simplified),
                        x: 0.0, // Path points are absolute relative to image origin
                        y: 0.0,
                        width: 0.0, height: 0.0, radius: 0.0,
                        layer_idx: 0,
                        text: "".into(),
                        font_size_mm: 0.0,
                        rotation: 0.0,
                    });
                }
            }
        }
    }

    shapes
}

/// Simple line follower for skeleton
fn trace_line(img: &GrayImage, start_x: u32, start_y: u32, visited: &mut [bool]) -> Vec<(u32, u32)> {
    let (w, h) = img.dimensions();
    let mut path = Vec::new();
    let mut stack = vec![(start_x, start_y)];

    while let Some((cx, cy)) = stack.pop() {
        let idx = (cy * w + cx) as usize;
        if visited[idx] { continue; }
        visited[idx] = true;
        path.push((cx, cy));

        // Check 8 neighbors
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = cx as i32 + dx;
                let ny = cy as i32 + dy;

                if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                    let ni = (ny * w as i32 + nx) as usize;
                    if !visited[ni] && img.get_pixel(nx as u32, ny as u32)[0] > 128 {
                        stack.push((nx as u32, ny as u32));
                        // For a simple line trace, we might want to break here to follow one branch,
                        // but managing junctions is hard.
                        // DFS will follow one branch to end, then backtrack.
                        // This produces a single long path potentially doubling back or jumping.
                        // For a "Centerline" trace, strict ordering matters.
                        // A proper graph builder is needed for perfect results.
                        // For this iteration, DFS is "okay" but might jump.
                        // Let's greedily pick ONE neighbor to follow to maintain line order.
                        // break;
                    }
                }
            }
        }
        // If we broke above, we only pushed one. If we didn't, we pushed all.
        // Pushing all makes it flood fill, order is lost.
        // Let's refine:
        // Ideally we want to follow the "best" unvisited neighbor.
    }
    // Re-ordering path is hard if it branched.
    // For now, return what we have.
    path
}

/// Zhang-Suen Thinning Algorithm
/// Modifies image in-place. Expects foreground=255, background=0.
fn zhang_suen_thinning(img: &mut GrayImage) {
    let (w, h) = img.dimensions();
    let mut changing = true;
    let mut to_clear = Vec::new();

    while changing {
        changing = false;

        // Step 1
        for y in 1..h-1 {
            for x in 1..w-1 {
                if img.get_pixel(x, y)[0] == 0 { continue; }

                let p2 = img.get_pixel(x, y-1)[0] > 0;
                let p3 = img.get_pixel(x+1, y-1)[0] > 0;
                let p4 = img.get_pixel(x+1, y)[0] > 0;
                let p5 = img.get_pixel(x+1, y+1)[0] > 0;
                let p6 = img.get_pixel(x, y+1)[0] > 0;
                let p7 = img.get_pixel(x-1, y+1)[0] > 0;
                let p8 = img.get_pixel(x-1, y)[0] > 0;
                let p9 = img.get_pixel(x-1, y-1)[0] > 0;

                let neighbors = [p2, p3, p4, p5, p6, p7, p8, p9];
                let b = neighbors.iter().filter(|&&v| v).count();
                if b < 2 || b > 6 { continue; }

                let a = count_transitions(&neighbors);
                if a != 1 { continue; }

                if (p2 && p4 && p6) { continue; }
                if (p4 && p6 && p8) { continue; }

                to_clear.push((x, y));
                changing = true;
            }
        }

        for (x, y) in to_clear.drain(..) {
            img.put_pixel(x, y, Luma([0]));
        }

        // Step 2
        for y in 1..h-1 {
            for x in 1..w-1 {
                if img.get_pixel(x, y)[0] == 0 { continue; }

                let p2 = img.get_pixel(x, y-1)[0] > 0;
                let p3 = img.get_pixel(x+1, y-1)[0] > 0;
                let p4 = img.get_pixel(x+1, y)[0] > 0;
                let p5 = img.get_pixel(x+1, y+1)[0] > 0;
                let p6 = img.get_pixel(x, y+1)[0] > 0;
                let p7 = img.get_pixel(x-1, y+1)[0] > 0;
                let p8 = img.get_pixel(x-1, y)[0] > 0;
                let p9 = img.get_pixel(x-1, y-1)[0] > 0;

                let neighbors = [p2, p3, p4, p5, p6, p7, p8, p9];
                let b = neighbors.iter().filter(|&&v| v).count();
                if b < 2 || b > 6 { continue; }

                let a = count_transitions(&neighbors);
                if a != 1 { continue; }

                if (p2 && p4 && p8) { continue; }
                if (p2 && p6 && p8) { continue; }

                to_clear.push((x, y));
                changing = true;
            }
        }

        for (x, y) in to_clear.drain(..) {
            img.put_pixel(x, y, Luma([0]));
        }
    }
}

fn count_transitions(n: &[bool; 8]) -> usize {
    let mut c = 0;
    // P2->P3, P3->P4, ... P9->P2
    let indices = [0, 1, 2, 3, 4, 5, 6, 7, 0];
    for i in 0..8 {
        if !n[indices[i]] && n[indices[i+1]] {
            c += 1;
        }
    }
    c
}
