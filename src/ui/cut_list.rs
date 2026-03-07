use crate::theme;
use crate::ui::layers_new::{CutLayer, CutMode};
use egui::{Color32, Grid, RichText, Sense, StrokeKind, Ui};

pub struct CutListAction {
    pub select_layer: Option<usize>,
    pub open_settings: Option<usize>,
    pub layers_changed: bool,
}

fn effective_layer_indices(
    layers_len: usize,
    active_idx: usize,
    used_layers: &[usize],
) -> Vec<usize> {
    let mut indices: Vec<usize> = used_layers
        .iter()
        .copied()
        .filter(|&idx| idx < layers_len)
        .collect();

    indices.sort_unstable();
    indices.dedup();

    if indices.is_empty() && active_idx < layers_len {
        indices.push(active_idx);
    }

    indices
}

pub fn show(
    ui: &mut Ui,
    layers: &mut [CutLayer],
    active_idx: usize,
    used_layers: &[usize],
) -> CutListAction {
    let mut action = CutListAction {
        select_layer: None,
        open_settings: None,
        layers_changed: false,
    };
    let visible_indices = effective_layer_indices(layers.len(), active_idx, used_layers);

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
                ui.label("View");
                ui.end_row();

                for &i in &visible_indices {
                    let Some(layer) = layers.get_mut(i) else {
                        continue;
                    };
                    let is_active = i == active_idx;

                    // Row background for active
                    // Grid doesn't support full row selection easily without custom paint or tricks.
                    // We will just highlight the text or use a button for the first cell.

                    // 1. Color Swatch + ID
                    let (rect, response) =
                        ui.allocate_exact_size(egui::vec2(24.0, 16.0), Sense::click());
                    if ui.is_rect_visible(rect) {
                        ui.painter().rect_filled(rect, 2.0, layer.color);
                        if is_active {
                            ui.painter().rect_stroke(
                                rect,
                                2.0,
                                egui::Stroke::new(2.0, theme::GREEN),
                                StrokeKind::Inside,
                            );
                        }
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            format!("{:02}", i),
                            egui::FontId::monospace(10.0),
                            if is_light(layer.color) {
                                Color32::BLACK
                            } else {
                                Color32::WHITE
                            },
                        );
                    }
                    if response.clicked() {
                        action.select_layer = Some(i);
                    }
                    if response.double_clicked() {
                        action.open_settings = Some(i);
                    }
                    response.on_hover_text(format!(
                        "Layer C{:02} — click to select, double-click for settings",
                        i
                    ));

                    // 2. Mode
                    let mode_before = layer.mode;
                    egui::ComboBox::from_id_salt(format!("layer_mode_{i}"))
                        .selected_text(match layer.mode {
                            CutMode::Line => "Line",
                            CutMode::Fill => "Fill",
                            CutMode::FillAndLine => "Fill+Line",
                            CutMode::Offset => "Offset",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut layer.mode, CutMode::Line, "Line");
                            ui.selectable_value(&mut layer.mode, CutMode::Fill, "Fill");
                            ui.selectable_value(&mut layer.mode, CutMode::FillAndLine, "Fill+Line");
                            ui.selectable_value(&mut layer.mode, CutMode::Offset, "Offset");
                        });
                    if layer.mode != mode_before {
                        action.layers_changed = true;
                    }

                    // 3. Speed / Power
                    ui.label(format!("{:.0} / {:.0}", layer.speed, layer.power));

                    // 4. Output Toggle
                    if ui
                        .checkbox(&mut layer.visible, "")
                        .on_hover_text("Enable/disable layer output")
                        .changed()
                    {
                        action.layers_changed = true;
                    }

                    // 5. Preview visibility (currently tied to layer.enabled in this app)
                    ui.label(if layer.visible { "👁" } else { "Ø" });

                    ui.end_row();
                }
            });

        ui.add_space(4.0);
        ui.label(
            RichText::new(format!(
                "{} layer(s) visible in Cuts (filtered by preview)",
                visible_indices.len()
            ))
            .small()
            .color(theme::SUBTEXT),
        );
    });

    action
}

fn is_light(c: Color32) -> bool {
    let brightness = (c.r() as f32 * 299.0 + c.g() as f32 * 587.0 + c.b() as f32 * 114.0) / 1000.0;
    brightness > 128.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effective_layer_indices_returns_sorted_unique_used_layers() {
        let result = effective_layer_indices(30, 0, &[4, 2, 4, 1, 31]);
        assert_eq!(result, vec![1, 2, 4]);
    }

    #[test]
    fn effective_layer_indices_falls_back_to_active_when_empty() {
        let result = effective_layer_indices(30, 7, &[]);
        assert_eq!(result, vec![7]);
    }
}
