use egui::{Ui, RichText, TextureHandle, Vec2, Color32};
use crate::imaging::raster::RasterParams;
use crate::imaging::svg::SvgParams;
use crate::ui::materials::MaterialsState;
use crate::theme;

pub enum ImportType {
    Raster(image::DynamicImage),
    Svg(Vec<u8>),
}

pub struct ImageImportState {
    pub import_type: ImportType,
    pub filename: String,
    pub raster_params: RasterParams,
    pub svg_params: SvgParams,
    pub materials: MaterialsState,
    pub texture: Option<TextureHandle>,
    pub needs_texture_update: bool,
    pub vectorize: bool,
}

#[derive(Default)]
pub struct ImageImportResult {
    pub imported: bool,
    pub cancel: bool,
}

pub fn show(ui: &mut Ui, state: &mut ImageImportState) -> ImageImportResult {
    let mut result = ImageImportResult::default();

    ui.vertical_centered(|ui| {
        ui.heading(format!("Import {}", state.filename));
    });

    ui.add_space(8.0);

    ui.columns(2, |cols| {
        // Left: Preview
        cols[0].vertical_centered(|ui| {
            // --- Material Presets ---
            let mat_action = crate::ui::materials::show(ui, &mut state.materials);
            if let Some(spd) = mat_action.apply_speed {
                state.raster_params.max_speed = spd;
                state.svg_params.layers.iter_mut().for_each(|l| l.speed = spd);
            }
            if let Some(pwr) = mat_action.apply_power {
                state.raster_params.max_power = pwr;
                state.svg_params.layers.iter_mut().for_each(|l| l.power = pwr);
            }
            if let Some(cs) = mat_action.apply_cut_speed {
                state.raster_params.outline.speed = cs;
                state.svg_params.outline.speed = cs;
            }
            if let Some(cp) = mat_action.apply_cut_power {
                state.raster_params.outline.power = cp;
                state.svg_params.outline.power = cp;
            }
            ui.add_space(8.0);

            if let Some(texture) = &state.texture {
                let _size = texture.size_vec2();
                let max_size = Vec2::new(300.0, 300.0);
                ui.add(egui::Image::new(texture).max_size(max_size).shrink_to_fit());
            } else {
                ui.add_space(100.0);
                ui.label("No preview available");
                ui.add_space(100.0);
            }
        });

        // Right: Settings
        cols[1].vertical(|ui| {
            match &state.import_type {
                ImportType::Raster(_) => {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Bitmap Import Mode:").strong());
                            ui.selectable_value(&mut state.vectorize, false, "Raster");
                            ui.selectable_value(&mut state.vectorize, true, "Vectorize (Stencil)");
                        });
                        ui.add_space(8.0);

                        ui.label(RichText::new(if state.vectorize { "Vectorize Settings" } else { "Raster / Photo Settings" })
                            .color(theme::LAVENDER).strong());
                        ui.add_space(4.0);
                        
                        ui.label("Size:");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut state.raster_params.width_mm).speed(1.0).suffix(" mm"));
                            ui.label("x");
                            ui.add(egui::DragValue::new(&mut state.raster_params.height_mm).speed(1.0).suffix(" mm"));
                        });

                        ui.add_space(4.0);
                        ui.label("Resolution:");
                        ui.add(egui::Slider::new(&mut state.raster_params.dpi, 25.4..=1270.0).text("DPI"));
                        ui.label(format!("({:.2} lines/mm)", state.raster_params.dpi / 25.4));

                        ui.add_space(8.0);
                        ui.label("Image Adjustments:");
                        if state.vectorize {
                            ui.add(egui::Slider::new(&mut state.raster_params.threshold, 0..=255).text("Threshold"));
                        } else {
                            let b_res = ui.add(egui::Slider::new(&mut state.raster_params.brightness, -1.0..=1.0).text("Brightness"));
                            let c_res = ui.add(egui::Slider::new(&mut state.raster_params.contrast, 0.0..=5.0).text("Contrast"));
                            
                            ui.horizontal(|ui| {
                                ui.label("Dithering:");
                                use crate::imaging::raster::DitherMode;
                                if ui.selectable_value(&mut state.raster_params.dither, DitherMode::FloydSteinberg, "Floyd").changed() { state.needs_texture_update = true; }
                                if ui.selectable_value(&mut state.raster_params.dither, DitherMode::Atkinson, "Atkinson").changed() { state.needs_texture_update = true; }
                                if ui.selectable_value(&mut state.raster_params.dither, DitherMode::None, "Grayscale").changed() { state.needs_texture_update = true; }
                            });

                            if b_res.changed() || c_res.changed() { state.needs_texture_update = true; }
                        }
                        
                        let mut force_update = false;
                        ui.horizontal(|ui| {
                            if ui.checkbox(&mut state.raster_params.flip_h, "Flip H").changed() { force_update = true; }
                            if ui.checkbox(&mut state.raster_params.flip_v, "Flip V").changed() { force_update = true; }
                        });
                        
                        egui::ComboBox::from_label("Rotation")
                            .selected_text(format!("{}°", state.raster_params.rotation))
                            .show_ui(ui, |ui| {
                                if ui.selectable_value(&mut state.raster_params.rotation, 0, "0°").changed() { force_update = true; }
                                if ui.selectable_value(&mut state.raster_params.rotation, 90, "90°").changed() { force_update = true; }
                                if ui.selectable_value(&mut state.raster_params.rotation, 180, "180°").changed() { force_update = true; }
                                if ui.selectable_value(&mut state.raster_params.rotation, 270, "270°").changed() { force_update = true; }
                            });
                        
                        if force_update {
                            state.needs_texture_update = true;
                        }

                        ui.add_space(8.0);
                        ui.label("Laser Settings:");
                        ui.add(egui::Slider::new(&mut state.raster_params.max_speed, 100.0..=10000.0).text("Speed (mm/min)"));
                        ui.add(egui::Slider::new(&mut state.raster_params.max_power, 0.0..=1000.0).text("Max Power (S)"));

                        // --- Cutting Frame ---
                        ui.add_space(12.0);
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut state.raster_params.outline.enabled, "");
                                ui.label(RichText::new("Cutting Frame (Outline)").color(theme::PEACH).strong());
                            });
                            
                            if state.raster_params.outline.enabled {
                                ui.add_space(4.0);
                                ui.add(egui::Slider::new(&mut state.raster_params.outline.speed, 50.0..=5000.0).text("Cut Speed"));
                                ui.add(egui::Slider::new(&mut state.raster_params.outline.power, 0.0..=1000.0).text("Cut Power"));
                                ui.horizontal(|ui| {
                                    ui.label("Passes:");
                                    ui.add(egui::DragValue::new(&mut state.raster_params.outline.passes).range(1..=50));
                                });
                            }
                        });
                    });
                }
                ImportType::Svg(_) => {
                    ui.group(|ui| {
                        ui.label(RichText::new("Vector / SVG Settings").color(theme::LAVENDER).strong());
                        ui.add_space(4.0);

                        ui.label("Scaling:");
                        ui.add(egui::Slider::new(&mut state.svg_params.scale, 0.01..=100.0).text("Scale X"));
                        
                        ui.add_space(8.0);
                        ui.label("Layers / Color Mapping:");
                        
                        egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                            let mut any_changed = false;
                            for (i, layer) in state.svg_params.layers.iter_mut().enumerate() {
                                ui.group(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.checkbox(&mut layer.enabled, "");
                                        
                                        // Try to parse hex color for a little swatch
                                        if layer.color_ha.starts_with('#') && layer.color_ha.len() == 7 {
                                            if let (Ok(r), Ok(g), Ok(b)) = (
                                                u8::from_str_radix(&layer.color_ha[1..3], 16),
                                                u8::from_str_radix(&layer.color_ha[3..5], 16),
                                                u8::from_str_radix(&layer.color_ha[5..7], 16)
                                            ) {
                                                let color = Color32::from_rgb(r, g, b);
                                                let (rect, _response) = ui.allocate_exact_size(Vec2::new(12.0, 12.0), egui::Sense::hover());
                                                ui.painter().rect_filled(rect, 2.0, color);
                                            }
                                        }
                                        
                                        ui.label(RichText::new(&layer.color_ha).strong());
                                    });
                                    if layer.enabled {
                                        ui.horizontal(|ui| {
                                            ui.label("Speed:");
                                            ui.add(egui::DragValue::new(&mut layer.speed).speed(10.0).clamp_range(100.0..=10000.0));
                                            ui.add_space(8.0);
                                            ui.label("Power:");
                                            ui.add(egui::DragValue::new(&mut layer.power).speed(1.0).clamp_range(0.0..=1000.0));
                                        });
                                    }
                                });
                            }
                        });

                        // --- Cutting Frame ---
                        ui.add_space(12.0);
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut state.svg_params.outline.enabled, "");
                                ui.label(RichText::new("Cutting Frame (Outline)").color(theme::PEACH).strong());
                            });
                            
                            if state.svg_params.outline.enabled {
                                ui.add_space(4.0);
                                ui.add(egui::Slider::new(&mut state.svg_params.outline.speed, 50.0..=5000.0).text("Cut Speed"));
                                ui.add(egui::Slider::new(&mut state.svg_params.outline.power, 0.0..=1000.0).text("Cut Power"));
                                ui.horizontal(|ui| {
                                    ui.label("Passes:");
                                    ui.add(egui::DragValue::new(&mut state.svg_params.outline.passes).range(1..=50));
                                });
                            }
                        });
                    });
                }
            }
        });
    });

    ui.add_space(20.0);

    ui.horizontal(|ui| {
        let btn_import = egui::Button::new(RichText::new("✔ Import").strong())
            .fill(Color32::from_rgb(64, 160, 43)) // Catppuccin Green-ish
            .min_size(Vec2::new(100.0, 30.0));
        
        if ui.add(btn_import).clicked() {
            result.imported = true;
        }

        let btn_cancel = egui::Button::new("✘ Cancel")
            .min_size(Vec2::new(100.0, 30.0));
        
        if ui.add(btn_cancel).clicked() {
            result.cancel = true;
        }
    });

    result
}
