#![allow(dead_code)]

use crate::ai::{AiBackend, AiConfig};
use crate::i18n::tr;
use crate::ui::drawing::{PathData, ShapeKind, ShapeParams};
use egui::{RichText, Ui};
use qrcode::QrCode;
use std::sync::{Arc, Mutex};

pub struct GeneratorState {
    pub qr_text: String,
    pub box_w: f32,
    pub box_h: f32,
    pub box_d: f32,
    pub box_thickness: f32,
    pub box_tab_size: f32,
    pub fiducial_width: f32,
    pub fiducial_height: f32,
    pub fiducial_margin: f32,
    pub fiducial_mark_size: f32,
    pub fiducial_feed: f32,
    pub fiducial_power: f32,
    pub fiducial_draw_frame: bool,
    pub hinge_w: f32,
    pub hinge_h: f32,
    pub hinge_cut_length: f32,
    pub hinge_gap: f32,
    pub hinge_dist: f32,
    pub ai_prompt: String,
    pub ai_width: f32,
    pub ai_height: f32,
    pub ai_config: AiConfig,
    pub ai_use_llm: bool,
    pub ai_llm_busy: bool,
    pub ai_llm_result: Arc<Mutex<Option<Result<Vec<ShapeParams>, String>>>>,
    pub ai_llm_error: String,
    pub ai_layer_cut: usize,
    pub ai_layer_engrave: usize,
    pub ai_layer_fine: usize,
}

impl Default for GeneratorState {
    fn default() -> Self {
        Self {
            qr_text: "https://github.com/arkypita/LaserGRBL".to_string(),
            box_w: 100.0,
            box_h: 80.0,
            box_d: 50.0,
            box_thickness: 3.0,
            box_tab_size: 10.0,
            fiducial_width: 200.0,
            fiducial_height: 150.0,
            fiducial_margin: 10.0,
            fiducial_mark_size: 6.0,
            fiducial_feed: 1200.0,
            fiducial_power: 350.0,
            fiducial_draw_frame: true,
            hinge_w: 50.0,
            hinge_h: 80.0,
            hinge_cut_length: 15.0,
            hinge_gap: 3.0,
            hinge_dist: 2.0,
            ai_prompt: "eagle on a branch".to_string(),
            ai_width: 100.0,
            ai_height: 100.0,
            ai_config: AiConfig::default(),
            ai_use_llm: false,
            ai_llm_busy: false,
            ai_llm_result: Arc::new(Mutex::new(None)),
            ai_llm_error: String::new(),
            ai_layer_cut: 0,
            ai_layer_engrave: 1,
            ai_layer_fine: 2,
        }
    }
}

pub struct GeneratorAction {
    pub generate_gcode: Option<Vec<String>>,
    pub generate_shapes: Option<Vec<ShapeParams>>,
}

pub fn show(ui: &mut Ui, state: &mut GeneratorState, active_layer: usize) -> GeneratorAction {
    let mut action = GeneratorAction {
        generate_gcode: None,
        generate_shapes: None,
    };

    ui.group(|ui| {
        ui.label(
            RichText::new(format!("📦 {}", tr("Object Generators")))
                .color(crate::theme::LAVENDER)
                .strong(),
        );
        ui.add_space(4.0);

        ui.collapsing(format!("🔗 {}", tr("QR Code Generator")), |ui| {
            ui.horizontal(|ui| {
                ui.label("Data/URL:");
                ui.text_edit_singleline(&mut state.qr_text);
            });
            if ui.button("🚀 Generate QR Code").clicked() {
                if let Ok(code) = QrCode::new(&state.qr_text) {
                    let size = 1.0_f32; // 1mm per module
                    let pixels = code.to_colors();
                    let width = code.width();
                    let group_id = (std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis()
                        & 0xFFFF_FFFF) as u32;

                    let mut shapes = Vec::new();
                    for (i, color) in pixels.into_iter().enumerate() {
                        if color == qrcode::Color::Dark {
                            let x = (i % width) as f32 * size;
                            let y = (width - 1 - (i / width)) as f32 * size;
                            shapes.push(ShapeParams {
                                shape: ShapeKind::Rectangle,
                                x,
                                y,
                                width: size,
                                height: size,
                                layer_idx: active_layer,
                                group_id: Some(group_id),
                                ..ShapeParams::default()
                            });
                        }
                    }
                    action.generate_shapes = Some(shapes);
                }
            }
        });

        ui.collapsing("📥 Box Maker (Finger Joints)", |ui| {
            ui.add(egui::Slider::new(&mut state.box_w, 20.0..=500.0).text("Width (X)"));
            ui.add(egui::Slider::new(&mut state.box_h, 20.0..=500.0).text("Height (Y)"));
            ui.add(egui::Slider::new(&mut state.box_d, 20.0..=500.0).text("Depth (Z)"));
            ui.add(egui::Slider::new(&mut state.box_thickness, 1.0..=20.0).text("Thickness"));
            ui.add(egui::Slider::new(&mut state.box_tab_size, 5.0..=50.0).text("Tab Size"));

            if ui.button("🚀 Generate Box Components").clicked() {
                let w = state.box_w;
                let h = state.box_h;
                let d = state.box_d;
                let t = state.box_thickness;
                let ts = state.box_tab_size;

                let mut shapes = Vec::new();

                // Bottom face (W x H)
                shapes.push(make_tabbed_face_shape(
                    w, h, t, ts, 0.0, 0.0,
                    [true, true, true, true], active_layer, "Bottom",
                ));
                // Top face (W x H)
                shapes.push(make_tabbed_face_shape(
                    w, h, t, ts, w + 10.0, 0.0,
                    [true, true, true, true], active_layer, "Top",
                ));
                // Front face (W x D)
                shapes.push(make_tabbed_face_shape(
                    w, d, t, ts, 0.0, h + 10.0,
                    [false, true, false, true], active_layer, "Front",
                ));
                // Back face (W x D)
                shapes.push(make_tabbed_face_shape(
                    w, d, t, ts, w + 10.0, h + 10.0,
                    [false, true, false, true], active_layer, "Back",
                ));
                // Left face (H x D)
                shapes.push(make_tabbed_face_shape(
                    h, d, t, ts, 0.0, h + d + 20.0,
                    [false, false, false, false], active_layer, "Left",
                ));
                // Right face (H x D)
                shapes.push(make_tabbed_face_shape(
                    h, d, t, ts, h + 10.0, h + d + 20.0,
                    [false, false, false, false], active_layer, "Right",
                ));

                action.generate_shapes = Some(shapes);
            }
        });

        ui.collapsing("🚪 Living Hinge", |ui| {
            ui.add(egui::Slider::new(&mut state.hinge_w, 10.0..=500.0).text("Width (X)"));
            ui.add(egui::Slider::new(&mut state.hinge_h, 10.0..=500.0).text("Height (Y)"));
            ui.add(egui::Slider::new(&mut state.hinge_cut_length, 2.0..=100.0).text("Cut Length"));
            ui.add(egui::Slider::new(&mut state.hinge_gap, 1.0..=50.0).text("Gap Between Cuts"));
            ui.add(egui::Slider::new(&mut state.hinge_dist, 1.0..=20.0).text("Distance Between Rows"));

            if ui.button("🚀 Generate Hinge").clicked() {
                action.generate_shapes = Some(make_living_hinge(
                    state.hinge_w,
                    state.hinge_h,
                    state.hinge_cut_length,
                    state.hinge_gap,
                    state.hinge_dist,
                    active_layer,
                ));
            }
        });

        ui.collapsing("🤖 AI Workpiece Generator", |ui| {
            ui.label("Prompt (FR / EN):");
            ui.text_edit_singleline(&mut state.ai_prompt);
            ui.horizontal(|ui| {
                ui.label("Width:");
                ui.add(
                    egui::DragValue::new(&mut state.ai_width)
                        .range(20.0..=2000.0)
                        .suffix(" mm"),
                );
                ui.label("Height:");
                ui.add(
                    egui::DragValue::new(&mut state.ai_height)
                        .range(20.0..=2000.0)
                        .suffix(" mm"),
                );
            });

            ui.add_space(4.0);
            ui.checkbox(&mut state.ai_use_llm, "Use LLM (Ollama / OpenAI / Gemini)");

            if state.ai_use_llm {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Backend:");
                        if ui.selectable_label(state.ai_config.backend == AiBackend::Ollama, "Ollama").clicked() {
                            state.ai_config.backend = AiBackend::Ollama;
                            if !state.ai_config.endpoint.contains("localhost") {
                                state.ai_config.endpoint = "http://localhost:11434".to_string();
                            }
                            if !state.ai_config.model.contains(':') {
                                state.ai_config.model = "llama3.1:8b".to_string();
                            }
                        }
                        if ui.selectable_label(state.ai_config.backend == AiBackend::OpenAi, "OpenAI").clicked() {
                            state.ai_config.backend = AiBackend::OpenAi;
                            if !state.ai_config.endpoint.contains("openai.com") {
                                state.ai_config.endpoint = "https://api.openai.com".to_string();
                            }
                            if !state.ai_config.model.starts_with("gpt") {
                                state.ai_config.model = "gpt-4o-mini".to_string();
                            }
                        }
                        if ui.selectable_label(state.ai_config.backend == AiBackend::Gemini, "Gemini").clicked() {
                            state.ai_config.backend = AiBackend::Gemini;
                            if !state.ai_config.endpoint.contains("googleapis.com") {
                                state.ai_config.endpoint = "https://generativelanguage.googleapis.com".to_string();
                            }
                            if !state.ai_config.model.starts_with("gemini") {
                                state.ai_config.model = "gemini-2.5-flash".to_string();
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Endpoint:");
                        ui.text_edit_singleline(&mut state.ai_config.endpoint);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Model:");
                        ui.text_edit_singleline(&mut state.ai_config.model);
                    });
                    if state.ai_config.backend == AiBackend::OpenAi || state.ai_config.backend == AiBackend::Gemini {
                        ui.horizontal(|ui| {
                            ui.label("API Key:");
                            ui.add(egui::TextEdit::singleline(&mut state.ai_config.api_key).password(true));
                        });
                    }
                    ui.horizontal(|ui| {
                        ui.label("Max tokens:");
                        ui.add(egui::DragValue::new(&mut state.ai_config.max_tokens).range(256..=131072));
                    });

                    ui.separator();
                    ui.label(RichText::new("Layer mapping (color index 0-29):").small());
                    ui.horizontal(|ui| {
                        ui.label("✂ Cut:");
                        ui.add(egui::DragValue::new(&mut state.ai_layer_cut).range(0..=29));
                        ui.label("  ▪ Engrave:");
                        ui.add(egui::DragValue::new(&mut state.ai_layer_engrave).range(0..=29));
                        ui.label("  ∿ Fine:");
                        ui.add(egui::DragValue::new(&mut state.ai_layer_fine).range(0..=29));
                    });
                });
            }

            ui.add_space(4.0);
            if !state.ai_use_llm {
                ui.label(
                    RichText::new(
                        "30+ built-in shapes: eagle, cat, dog, fish, butterfly, horse, tree, flower, \
                         star, heart, house, gear, crown, skull, flame, moon, sun, mountain, \
                         leaf, arrow, key, snowflake, spiral, diamond, shield, anchor, \
                         lightning, paw, music… Combine: \"eagle on branch\", \"chat avec fleur\"",
                    )
                    .small()
                    .color(crate::theme::SUBTEXT),
                );
            } else {
                ui.label(
                    RichText::new(
                        "LLM mode: generates multi-layer SVG — Cut (outline), Engrave (details), \
                         Fine (hatching/contrast). Assign each to a color layer for different laser settings.",
                    )
                    .small()
                    .color(crate::theme::SUBTEXT),
                );
            }

            // Check for async LLM result
            if state.ai_llm_busy {
                if let Ok(mut guard) = state.ai_llm_result.try_lock() {
                    if let Some(result) = guard.take() {
                        state.ai_llm_busy = false;
                        match result {
                            Ok(shapes) => {
                                if !shapes.is_empty() {
                                    action.generate_shapes = Some(shapes);
                                    state.ai_llm_error.clear();
                                } else {
                                    state.ai_llm_error = "LLM returned no usable shapes.".to_string();
                                }
                            }
                            Err(e) => {
                                state.ai_llm_error = e;
                            }
                        }
                    }
                }
            }

            ui.add_space(2.0);
            if state.ai_llm_busy {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Generating with AI…");
                });
            } else if !state.ai_use_llm {
                if ui.button("✨ Generate (built-in)").clicked() {
                    let shapes = crate::ai::generate_from_prompt(
                        &state.ai_prompt,
                        state.ai_width.max(10.0),
                        state.ai_height.max(10.0),
                        active_layer,
                    );
                    if !shapes.is_empty() {
                        action.generate_shapes = Some(shapes);
                    }
                }
            } else {
                if ui.button("🧠 Generate with LLM").clicked() {
                    state.ai_llm_busy = true;
                    state.ai_llm_error.clear();
                    let config = state.ai_config.clone();
                    let prompt = state.ai_prompt.clone();
                    let w = state.ai_width.max(10.0);
                    let h = state.ai_height.max(10.0);
                    let layers = [state.ai_layer_cut, state.ai_layer_engrave, state.ai_layer_fine];
                    let result_slot = Arc::clone(&state.ai_llm_result);
                    std::thread::spawn(move || {
                        let res = llm_generate_shapes(&config, &prompt, w, h, &layers);
                        if let Ok(mut guard) = result_slot.lock() {
                            *guard = Some(res);
                        }
                    });
                }
            }

            if !state.ai_llm_error.is_empty() {
                ui.label(
                    RichText::new(&state.ai_llm_error)
                        .small()
                        .color(egui::Color32::from_rgb(220, 60, 60)),
                );
            }
        });

        ui.collapsing(format!("🎯 {}", tr("Print & Cut Fiducials")), |ui| {
            ui.label("Generate 4 registration marks + optional outer frame.");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Width:");
                ui.add(
                    egui::DragValue::new(&mut state.fiducial_width)
                        .range(20.0..=2000.0)
                        .suffix(" mm"),
                );
                ui.label("Height:");
                ui.add(
                    egui::DragValue::new(&mut state.fiducial_height)
                        .range(20.0..=2000.0)
                        .suffix(" mm"),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Margin:");
                ui.add(
                    egui::DragValue::new(&mut state.fiducial_margin)
                        .range(1.0..=200.0)
                        .suffix(" mm"),
                );
                ui.label("Mark size:");
                ui.add(
                    egui::DragValue::new(&mut state.fiducial_mark_size)
                        .range(1.0..=100.0)
                        .suffix(" mm"),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Feed:");
                ui.add(egui::DragValue::new(&mut state.fiducial_feed).range(50.0..=10000.0));
                ui.label("Power (%):");
                {
                    let mut pct = state.fiducial_power / 10.0;
                    if ui.add(egui::DragValue::new(&mut pct).speed(0.5).range(0.0..=100.0).suffix("%")).changed() {
                        state.fiducial_power = (pct * 10.0).clamp(0.0, 1000.0);
                    }
                }
            });
            ui.checkbox(&mut state.fiducial_draw_frame, "Include outer frame");

            if ui.button("🚀 Generate Fiducials").clicked() {
                action.generate_shapes = Some(generate_fiducials_shapes(state, active_layer));
            }
        });
    });

    action
}

fn generate_fiducials_shapes(state: &GeneratorState, active_layer: usize) -> Vec<ShapeParams> {
    let w = state.fiducial_width.max(20.0);
    let h = state.fiducial_height.max(20.0);
    let margin = state.fiducial_margin.clamp(1.0, (w.min(h) * 0.45).max(1.0));
    let mark = state.fiducial_mark_size.clamp(1.0, 100.0);
    let half = mark * 0.5;
    let group_id = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        & 0xFFFF_FFFF) as u32;

    let mut shapes = Vec::new();

    // Optional outer frame (closed rectangle path)
    if state.fiducial_draw_frame {
        shapes.push(ShapeParams {
            shape: ShapeKind::Path(PathData::from_points(vec![
                (0.0, 0.0),
                (w, 0.0),
                (w, h),
                (0.0, h),
                (0.0, 0.0),
            ])),
            x: 0.0,
            y: 0.0,
            layer_idx: active_layer,
            group_id: Some(group_id),
            ..ShapeParams::default()
        });
    }

    // 4 corner crosshairs
    let corners = [
        (margin, margin),
        (w - margin, margin),
        (w - margin, h - margin),
        (margin, h - margin),
    ];
    for (cx, cy) in corners {
        // Horizontal line
        shapes.push(ShapeParams {
            shape: ShapeKind::Path(PathData::from_points(vec![(0.0, 0.0), (mark, 0.0)])),
            x: cx - half,
            y: cy,
            layer_idx: active_layer,
            group_id: Some(group_id),
            ..ShapeParams::default()
        });
        // Vertical line
        shapes.push(ShapeParams {
            shape: ShapeKind::Path(PathData::from_points(vec![(0.0, 0.0), (0.0, mark)])),
            x: cx,
            y: cy - half,
            layer_idx: active_layer,
            group_id: Some(group_id),
            ..ShapeParams::default()
        });
    }

    shapes
}

#[allow(dead_code)]
fn generate_fiducials_gcode(state: &GeneratorState) -> Vec<String> {
    let mut gcode = Vec::new();
    let w = state.fiducial_width.max(20.0);
    let h = state.fiducial_height.max(20.0);
    let margin = state.fiducial_margin.clamp(1.0, (w.min(h) * 0.45).max(1.0));
    let mark = state.fiducial_mark_size.clamp(1.0, 100.0);
    let half = mark * 0.5;
    let feed = state.fiducial_feed.max(50.0);
    let power = state.fiducial_power.clamp(1.0, 1000.0);

    let corners = [
        (margin, margin),
        (w - margin, margin),
        (w - margin, h - margin),
        (margin, h - margin),
    ];

    gcode.push("; Print & Cut Fiducials — All4Laser".to_string());
    gcode.push("G90".to_string());
    gcode.push("G21".to_string());
    gcode.push("M5".to_string());

    if state.fiducial_draw_frame {
        gcode.push("; Outer frame".to_string());
        gcode.push("M5".to_string());
        gcode.push(format!("G0 X{:.3} Y{:.3}", 0.0, 0.0));
        gcode.push(format!("M3 S{:.0}", power));
        gcode.push(format!("G1 X{:.3} Y{:.3} F{:.0}", w, 0.0, feed));
        gcode.push(format!("G1 X{:.3} Y{:.3} F{:.0}", w, h, feed));
        gcode.push(format!("G1 X{:.3} Y{:.3} F{:.0}", 0.0, h, feed));
        gcode.push(format!("G1 X{:.3} Y{:.3} F{:.0}", 0.0, 0.0, feed));
        gcode.push("M5".to_string());
    }

    for (idx, (cx, cy)) in corners.into_iter().enumerate() {
        gcode.push(format!("; Fiducial {}", idx + 1));

        gcode.push(format!("G0 X{:.3} Y{:.3}", cx - half, cy));
        gcode.push(format!("M3 S{:.0}", power));
        gcode.push(format!("G1 X{:.3} Y{:.3} F{:.0}", cx + half, cy, feed));
        gcode.push("M5".to_string());

        gcode.push(format!("G0 X{:.3} Y{:.3}", cx, cy - half));
        gcode.push(format!("M3 S{:.0}", power));
        gcode.push(format!("G1 X{:.3} Y{:.3} F{:.0}", cx, cy + half, feed));
        gcode.push("M5".to_string());
    }

    gcode.push("M5".to_string());
    gcode.push("G0 X0 Y0 F3000".to_string());
    gcode
}

fn add_tabbed_face(
    gcode: &mut Vec<String>,
    w: f32,
    h: f32,
    t: f32,
    ts: f32,
    ox: f32,
    oy: f32,
    edges: [bool; 4],
) {
    gcode.push(format!("; Face {}x{}", w, h));
    gcode.push(format!("G0 X{} Y{}", ox, oy));
    gcode.push("M3 S1000".into());

    // Edges: 0: Bottom (+X), 1: Right (+Y), 2: Top (-X), 3: Left (-Y)
    draw_edge(gcode, ox, oy, w, 0.0, t, ts, edges[0]);
    draw_edge(gcode, ox + w, oy, 0.0, h, t, ts, edges[1]);
    draw_edge(gcode, ox + w, oy + h, -w, 0.0, t, ts, edges[2]);
    draw_edge(gcode, ox, oy + h, 0.0, -h, t, ts, edges[3]);

    gcode.push("M5".into());
}

fn draw_edge(
    gcode: &mut Vec<String>,
    sx: f32,
    sy: f32,
    dx: f32,
    dy: f32,
    t: f32,
    ts: f32,
    is_male: bool,
) {
    let length = (dx * dx + dy * dy).sqrt();
    let num_tabs = (length / ts).floor() as i32;
    if num_tabs < 1 {
        gcode.push(format!("G1 X{} Y{} F800", sx + dx, sy + dy));
        return;
    }

    let (nx, ny) = if dy == 0.0 {
        if dx > 0.0 { (0.0, -1.0) } else { (0.0, 1.0) }
    } else {
        if dy > 0.0 { (1.0, 0.0) } else { (-1.0, 0.0) }
    };

    for i in 0..num_tabs {
        let is_tab = (i % 2 == 0) == is_male;
        let offset = if is_tab { 0.0 } else { -t };

        let p_start_x = sx + (dx * (i as f32 / num_tabs as f32));
        let p_start_y = sy + (dy * (i as f32 / num_tabs as f32));

        let p_end_x = sx + (dx * ((i + 1) as f32 / num_tabs as f32));
        let p_end_y = sy + (dy * ((i + 1) as f32 / num_tabs as f32));

        // Move to offset starting point
        gcode.push(format!(
            "G1 X{} Y{}",
            p_start_x + nx * offset,
            p_start_y + ny * offset
        ));
        // Cut the segment
        gcode.push(format!(
            "G1 X{} Y{}",
            p_end_x + nx * offset,
            p_end_y + ny * offset
        ));
    }
}

/// Build a closed Path shape for a single tabbed box face.
/// The shape origin (x, y) is at (ox, oy); points are local to (0,0).
fn make_tabbed_face_shape(
    w: f32,
    h: f32,
    t: f32,
    ts: f32,
    ox: f32,
    oy: f32,
    edges: [bool; 4],
    layer_idx: usize,
    _label: &str,
) -> ShapeParams {
    let mut pts: Vec<(f32, f32)> = Vec::new();

    // Edges: 0: Bottom (+X), 1: Right (+Y), 2: Top (-X), 3: Left (-Y)
    // Each edge goes from one corner to the next, generating tabbed points
    edge_points(&mut pts, 0.0, 0.0, w, 0.0, t, ts, edges[0]);
    edge_points(&mut pts, w, 0.0, 0.0, h, t, ts, edges[1]);
    edge_points(&mut pts, w, h, -w, 0.0, t, ts, edges[2]);
    edge_points(&mut pts, 0.0, h, 0.0, -h, t, ts, edges[3]);

    // Close the path back to start
    if let Some(&first) = pts.first() {
        if let Some(&last) = pts.last() {
            if (first.0 - last.0).abs() > 0.01 || (first.1 - last.1).abs() > 0.01 {
                pts.push(first);
            }
        }
    }

    ShapeParams {
        shape: ShapeKind::Path(PathData::from_points(pts)),
        x: ox,
        y: oy,
        layer_idx,
        ..Default::default()
    }
}

/// Generate shapes for a living hinge lattice pattern.
fn make_living_hinge(
    w: f32,
    h: f32,
    cut_length: f32,
    gap: f32,
    dist: f32,
    active_layer: usize,
) -> Vec<ShapeParams> {
    let mut shapes = Vec::new();

    // The bounding box
    shapes.push(ShapeParams {
        shape: ShapeKind::Rectangle,
        x: 0.0,
        y: 0.0,
        width: w,
        height: h,
        layer_idx: active_layer,
        ..Default::default()
    });

    if cut_length <= 0.0 || dist <= 0.0 || gap <= 0.0 {
        return shapes;
    }

    let mut x = dist;
    let mut is_staggered = false;
    let period = cut_length + gap;

    while x < w {
        if is_staggered {
            // First partial cut for staggered lines
            let first_cut_len = (cut_length - gap) / 2.0;
            if first_cut_len > 0.0 {
                shapes.push(ShapeParams {
                    shape: ShapeKind::Path(PathData::from_points(vec![(x, 0.0), (x, first_cut_len)])),
                    x: 0.0,
                    y: 0.0,
                    layer_idx: active_layer,
                    ..Default::default()
                });
            }
        }

        let mut y = if is_staggered { period / 2.0 } else { 0.0 };

        while y < h {
            let next_y = (y + cut_length).min(h);

            // Generate a line path for the cut
            shapes.push(ShapeParams {
                shape: ShapeKind::Path(PathData::from_points(vec![(x, y), (x, next_y)])),
                x: 0.0, // Path coordinates are local and already set
                y: 0.0,
                layer_idx: active_layer,
                ..Default::default()
            });

            y += period;
        }

        x += dist;
        is_staggered = !is_staggered;
    }

    shapes
}

/// Call the LLM, parse SVG paths from its response, and convert to ShapeParams.
/// `layer_map` is [cut_layer, engrave_layer, fine_layer].
fn llm_generate_shapes(
    config: &AiConfig,
    prompt: &str,
    width_mm: f32,
    height_mm: f32,
    layer_map: &[usize; 3],
) -> Result<Vec<ShapeParams>, String> {
    let raw_svg = crate::ai::call_llm(config, prompt)?;

    eprintln!("[AI] LLM raw response ({} chars):\n{}", raw_svg.len(), &raw_svg[..raw_svg.len().min(1000)]);

    let layered = crate::ai::extract_layered_paths(&raw_svg);
    eprintln!("[AI] Extracted {} paths", layered.len());
    if layered.is_empty() {
        return Err(format!(
            "LLM did not return valid SVG paths. Raw response (first 500 chars):\n{}",
            &raw_svg[..raw_svg.len().min(500)]
        ));
    }

    // Log layer distribution
    let n_cut = layered.iter().filter(|lp| lp.layer == crate::ai::PathLayer::Cut).count();
    let n_eng = layered.iter().filter(|lp| lp.layer == crate::ai::PathLayer::Engrave).count();
    let n_fine = layered.iter().filter(|lp| lp.layer == crate::ai::PathLayer::Fine).count();
    eprintln!("[AI] Layers: {} cut, {} engrave, {} fine", n_cut, n_eng, n_fine);

    let sx = width_mm / 100.0;
    let sy = height_mm / 100.0;

    let group_id = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        & 0xFFFF_FFFF) as u32;

    let shapes: Vec<ShapeParams> = layered
        .into_iter()
        .filter(|lp| lp.points.len() >= 2)
        .map(|lp| {
            let layer_idx = match lp.layer {
                crate::ai::PathLayer::Cut => layer_map[0],
                crate::ai::PathLayer::Engrave => layer_map[1],
                crate::ai::PathLayer::Fine => layer_map[2],
            };
            let scaled_pts: Vec<(f32, f32)> = lp.points.into_iter()
                .map(|(x, y)| (x * sx, y * sy))
                .collect();
            ShapeParams {
                shape: ShapeKind::Path(PathData::from_points(scaled_pts)),
                x: 0.0,
                y: 0.0,
                layer_idx,
                group_id: Some(group_id),
                ..ShapeParams::default()
            }
        })
        .collect();

    eprintln!("[AI] Generated {} shape(s) for canvas", shapes.len());
    Ok(shapes)
}

/// Generate the points for one tabbed edge and append them to `pts`.
/// (sx, sy) is the start corner (local coords), (dx, dy) is the direction vector.
fn edge_points(
    pts: &mut Vec<(f32, f32)>,
    sx: f32,
    sy: f32,
    dx: f32,
    dy: f32,
    t: f32,
    ts: f32,
    is_male: bool,
) {
    let length = (dx * dx + dy * dy).sqrt();
    let num_tabs = (length / ts).floor().max(1.0) as i32;

    // Normal direction (perpendicular, pointing inward)
    let (nx, ny) = if dy == 0.0 {
        if dx > 0.0 { (0.0, -1.0) } else { (0.0, 1.0) }
    } else {
        if dy > 0.0 { (1.0, 0.0) } else { (-1.0, 0.0) }
    };

    for i in 0..num_tabs {
        let is_tab = (i % 2 == 0) == is_male;
        let offset = if is_tab { 0.0 } else { -t };

        let frac_start = i as f32 / num_tabs as f32;
        let frac_end = (i + 1) as f32 / num_tabs as f32;

        let p_start_x = sx + dx * frac_start + nx * offset;
        let p_start_y = sy + dy * frac_start + ny * offset;
        let p_end_x = sx + dx * frac_end + nx * offset;
        let p_end_y = sy + dy * frac_end + ny * offset;

        pts.push((p_start_x, p_start_y));
        pts.push((p_end_x, p_end_y));
    }
}

#[cfg(test)]
mod tests {
    use super::{GeneratorState, generate_fiducials_gcode, make_living_hinge};
    use crate::ui::drawing::ShapeKind;

    #[test]
    fn test_living_hinge_generation() {
        let shapes = make_living_hinge(50.0, 50.0, 10.0, 2.0, 5.0, 0);

        // Should have at least the bounding box and several cut lines
        assert!(shapes.len() > 1);

        // First shape should be the bounding box
        assert_eq!(shapes[0].shape, ShapeKind::Rectangle);
        assert_eq!(shapes[0].width, 50.0);
        assert_eq!(shapes[0].height, 50.0);

        // Second shape should be a path
        if let ShapeKind::Path(pts) = &shapes[1].shape {
            assert_eq!(pts.len(), 2);
        } else {
            panic!("Expected path for hinge cut");
        }
    }

    #[test]
    fn fiducials_generator_emits_four_marks() {
        let mut state = GeneratorState::default();
        state.fiducial_draw_frame = false;

        let lines = generate_fiducials_gcode(&state);
        let marks = lines
            .iter()
            .filter(|line| line.starts_with("; Fiducial "))
            .count();
        assert_eq!(marks, 4);
    }

    #[test]
    fn fiducials_generator_can_emit_outer_frame() {
        let mut state = GeneratorState::default();
        state.fiducial_draw_frame = true;

        let lines = generate_fiducials_gcode(&state);
        assert!(lines.iter().any(|line| line == "; Outer frame"));
    }
}
