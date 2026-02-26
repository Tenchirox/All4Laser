use egui::{Ui, RichText};
use crate::gcode::types::LayerSettings;

pub fn show(ui: &mut Ui, layers: &mut [LayerSettings]) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("📑 Project Layers").color(crate::theme::LAVENDER).strong());
        });
        ui.add_space(4.0);

        if layers.is_empty() {
            ui.label("No layers detected.");
            return;
        }

        egui::ScrollArea::vertical().id_source("layers_scroll").max_height(200.0).show(ui, |ui| {
            for (i, layer) in layers.iter_mut().enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        // Color swatch
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, 2.0, layer.color);
                        
                        ui.checkbox(&mut layer.visible, "");
                        
                        let name = if layer.name.is_empty() { format!("Layer {}", i) } else { layer.name.clone() };
                        ui.label(RichText::new(name).strong());
                    });
                    
                    if layer.visible {
                        ui.horizontal(|ui| {
                            ui.label("Passes:");
                            ui.add(egui::DragValue::new(&mut layer.passes).range(1..=50));
                            ui.label("Pwr:");
                            ui.add(egui::DragValue::new(&mut layer.power).speed(10.0).range(0.0..=1000.0));
                        });
                    }
                });
            }
        });
    });
}
