use egui::{Ui, RichText, Color32, Grid, Sense, StrokeKind};
use crate::ui::layers_new::{CutLayer, CutMode};
use crate::theme;

pub struct CutListAction {
    pub select_layer: Option<usize>,
    pub open_settings: Option<usize>,
}

use std::collections::HashSet;

pub fn show(ui: &mut Ui, layers: &mut [CutLayer], active_idx: usize, used_layers: &HashSet<usize>, hide_unused: &mut bool) -> CutListAction {
    let mut action = CutListAction { select_layer: None, open_settings: None };

    ui.horizontal(|ui| {
        ui.checkbox(hide_unused, "Hide Unused");
    });

    ui.group(|ui| {
        Grid::new("cut_list_grid")
            .num_columns(5)
            .spacing([8.0, 4.0])
            .striped(true)
            .min_col_width(20.0)
            .show(ui, |ui| {
                // Header
                ui.label("#");
                ui.label("Mode");
                ui.label("Spd/Pwr");
                ui.label("Out");
                ui.label("Show");
                ui.end_row();

                // Only show active layers? Or allow user to see all?
                // LightBurn only shows used layers usually, but we don't track "used" status easily unless we scan all shapes.
                // For simplicity, let's show ALL 30 layers but maybe compact or scrollable.
                // Or better: Show only layers that are "visible" OR have been modified from default?
                // Let's show all for now inside the ScrollArea provided by the parent tab.

                // Note: The parent `ui` is already in a scroll area in `app.rs`.
                // Grid inside ScrollArea works fine.

                for (i, layer) in layers.iter_mut().enumerate() {
                    let is_active = i == active_idx;

                    if *hide_unused && !is_active && !used_layers.contains(&i) {
                        continue;
                    }

                    // Row background for active
                    // Grid doesn't support full row selection easily without custom paint or tricks.
                    // We will just highlight the text or use a button for the first cell.

                    // 1. Color Swatch + ID
                    let (rect, response) = ui.allocate_exact_size(egui::vec2(24.0, 16.0), Sense::click());
                    if ui.is_rect_visible(rect) {
                        ui.painter().rect_filled(rect, 2.0, layer.color);
                        if is_active {
                            ui.painter().rect_stroke(rect, 2.0, egui::Stroke::new(2.0, theme::GREEN), StrokeKind::Inside);
                        }
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            format!("{:02}", i),
                            egui::FontId::monospace(10.0),
                            if is_light(layer.color) { Color32::BLACK } else { Color32::WHITE }
                        );
                    }
                    if response.clicked() {
                        action.select_layer = Some(i);
                    }
                    if response.double_clicked() {
                        action.open_settings = Some(i);
                    }

                    // 2. Mode
                    ui.label(match layer.mode {
                        CutMode::Line => "Line",
                        CutMode::Fill => "Fill",
                        CutMode::FillAndLine => "Fill+Line",
                        CutMode::Offset => "Offset",
                    });

                    // 3. Speed / Power
                    ui.label(format!("{:.0} / {:.0}", layer.speed, layer.power));

                    // 4. Output Toggle
                    ui.checkbox(&mut layer.visible, "");

                    // 5. Show (Visibility in preview).
                    ui.checkbox(&mut layer.show, "");

                    ui.end_row();
                }
            });
    });

    action
}

fn is_light(c: Color32) -> bool {
    let brightness = (c.r() as f32 * 299.0 + c.g() as f32 * 587.0 + c.b() as f32 * 114.0) / 1000.0;
    brightness > 128.0
}
