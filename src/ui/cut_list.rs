use crate::i18n::tr;
use crate::theme;
use crate::ui::layers_new::{CutLayer, CutMode};
use egui::{Color32, RichText, Sense, StrokeKind, Ui};
use std::cell::RefCell;

pub struct CutListAction {
    pub select_layer: Option<usize>,
    pub open_settings: Option<usize>,
    pub layers_changed: bool,
    pub select_all_shapes: Option<usize>,
    pub move_layer: Option<(usize, i32)>, // (layer_idx, direction: -1=up, +1=down)
}

// Track which layer is currently being renamed and the temporary edit buffer
thread_local! {
    static RENAME_LAYER: RefCell<Option<usize>> = RefCell::new(None);
    static RENAME_BUFFER: RefCell<String> = RefCell::new(String::new());
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
    is_light_mode: bool,
) -> CutListAction {
    let mut action = CutListAction {
        select_layer: None,
        open_settings: None,
        layers_changed: false,
        select_all_shapes: None,
        move_layer: None,
    };
    let visible_indices = effective_layer_indices(layers.len(), active_idx, used_layers);

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!("✂ {}", tr("Layers")))
                    .color(theme::LAVENDER)
                    .strong()
                    .size(14.0),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Badge with layer count - more visible chip style
                let count = visible_indices.len();
                let badge_color = if count > 0 { theme::BLUE } else { theme::SUBTEXT };
                egui::Frame::new()
                    .fill(theme::SURFACE1)
                    .corner_radius(egui::CornerRadius::same(10))
                    .inner_margin(egui::Margin::symmetric(8, 2))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(format!("{} {}", count, tr("layers")))
                                .small()
                                .color(badge_color)
                                .strong(),
                        );
                    });
            });
        });
        ui.add_space(4.0);

        let layer_count = layers.len();
        for &i in &visible_indices {
            let Some(layer) = layers.get_mut(i) else {
                continue;
            };
            let is_active = i == active_idx;

            let frame_fill = if is_active { 
    crate::theme::get_surface1(is_light_mode) 
} else { 
    crate::theme::get_surface0(is_light_mode) 
};
            egui::Frame::new()
                .fill(frame_fill)
                .inner_margin(egui::Margin::same(4))
                .corner_radius(egui::CornerRadius::same(4))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Color Swatch + ID
                        let (rect, swatch_resp) =
                            ui.allocate_exact_size(egui::vec2(32.0, 20.0), Sense::click());
                        if ui.is_rect_visible(rect) {
                            ui.painter().rect_filled(rect, 3.0, layer.color);
                            if is_active {
                                ui.painter().rect_stroke(
                                    rect,
                                    3.0,
                                    egui::Stroke::new(2.0, theme::GREEN),
                                    StrokeKind::Inside,
                                );
                            }
                            ui.painter().text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                format!("{:02}", i),
                                egui::FontId::monospace(11.0),
                                if theme::is_light(layer.color) {
                                    theme::CRUST
                                } else {
                                    Color32::WHITE
                                },
                            );
                        }
                        if swatch_resp.clicked() {
                            action.select_layer = Some(i);
                        }
                        if swatch_resp.double_clicked() {
                            action.open_settings = Some(i);
                        }
                        swatch_resp.on_hover_text(format!(
                            "{} C{:02} — {}",
                            tr("Layer"), i,
                            tr("click to select, double-click for settings")
                        ));

                        // Layer name + mode (with inline editing)
                        ui.vertical(|ui| {
                            let is_renaming = RENAME_LAYER.with(|r| *r.borrow() == Some(i));
                            if is_renaming {
                                ui.horizontal(|ui| {
                                    let text_edit = RENAME_BUFFER.with(|b| {
                                        ui.text_edit_singleline(&mut *b.borrow_mut())
                                    });
                                    let should_commit = text_edit.lost_focus() || ui.button("✓").clicked();
                                    if should_commit {
                                        let new_name = RENAME_BUFFER.with(|b| {
                                            let buf = b.borrow();
                                            if !buf.is_empty() { Some(buf.clone()) } else { None }
                                        });
                                        if let Some(name) = new_name {
                                            layer.name = name;
                                            action.layers_changed = true;
                                        }
                                        RENAME_LAYER.with(|r| *r.borrow_mut() = None);
                                        RENAME_BUFFER.with(|b| b.borrow_mut().clear());
                                    }
                                });
                            } else {
                                let name_label = ui.add(
                                    egui::Label::new(
                                        RichText::new(&layer.name)
                                            .strong()
                                            .size(12.0)
                                            .color(if is_active { 
    crate::theme::get_text(is_light_mode) 
} else { 
    crate::theme::get_subtext(is_light_mode) 
}),
                                    )
                                    .sense(Sense::click()),
                                );
                                if name_label.double_clicked() {
                                    RENAME_LAYER.with(|r| *r.borrow_mut() = Some(i));
                                    RENAME_BUFFER.with(|b| *b.borrow_mut() = layer.name.clone());
                                }
                            }
                            ui.add_space(25.0); // Space between name and mode/speed row
                            ui.horizontal(|ui| {
                                let mode_before = layer.mode;
                                egui::ComboBox::from_id_salt(format!("layer_mode_{i}"))
                                    .width(70.0)
                                    .selected_text(match layer.mode {
                                        CutMode::Line => tr("Line"),
                                        CutMode::Fill => tr("Fill"),
                                        CutMode::FillAndLine => tr("F+L"),
                                        CutMode::Offset => tr("Offset"),
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut layer.mode, CutMode::Line, tr("Line"));
                                        ui.selectable_value(&mut layer.mode, CutMode::Fill, tr("Fill"));
                                        ui.selectable_value(&mut layer.mode, CutMode::FillAndLine, tr("Fill+Line"));
                                        ui.selectable_value(&mut layer.mode, CutMode::Offset, tr("Offset"));
                                    });
                                if layer.mode != mode_before {
                                    action.layers_changed = true;
                                }

                                ui.add_space(8.0); // Space between mode and speed/power

                                // Inline Speed / Power+Passes editing
                                let pwr_pct = (layer.power / 1000.0).clamp(0.0, 1.0);
                                let pwr_color = if pwr_pct > 0.8 {
                                    theme::RED
                                } else if pwr_pct > 0.5 {
                                    theme::PEACH
                                } else {
                                    theme::GREEN
                                };
                                
                                let speed_before = layer.speed;
                                let power_before = layer.power;
                                let passes_before = layer.passes;
                                
                                ui.horizontal(|ui| {
                                    ui.add(
                                        egui::DragValue::new(&mut layer.speed)
                                            .speed(10.0)
                                            .range(1.0..=20000.0)
                                            .suffix(" mm/min")
                                            .clamp_existing_to_range(true),
                                    );
                                    ui.add_space(4.0);
                                    ui.label("/");
                                    ui.add_space(4.0);
                                    ui.add(
                                        egui::DragValue::new(&mut layer.power)
                                            .speed(1.0)
                                            .range(0.0..=100.0)
                                            .suffix("%")
                                            .clamp_existing_to_range(true)
                                            .custom_formatter(|val, _| format!("{:.0}", val)),
                                    );
                                    ui.colored_label(pwr_color, "⚡");
                                    ui.add_space(4.0);
                                    // Pass count always visible next to power
                                    ui.add(
                                        egui::DragValue::new(&mut layer.passes)
                                            .speed(1.0)
                                            .range(1..=100)
                                            .prefix("×")
                                            .clamp_existing_to_range(true),
                                    );
                                });
                                
                                if layer.speed != speed_before || layer.power != power_before || layer.passes != passes_before {
                                    action.layers_changed = true;
                                }
                            });
                        });

                        // Horizontal space between left settings and right buttons
                        ui.allocate_exact_size(egui::vec2(5.0, 0.0), egui::Sense::hover());

                        // Right side: output toggle + settings + select all + reorder buttons
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Reorder buttons
                            let can_move_up = i > 0;
                            let can_move_down = i < layer_count.saturating_sub(1);
                            
                            if ui
                                .add_enabled(can_move_up, egui::Button::new("↑").small())
                                .on_hover_text(tr("Move layer up"))
                                .clicked()
                            {
                                action.move_layer = Some((i, -1));
                            }
                            if ui
                                .add_enabled(can_move_down, egui::Button::new("↓").small())
                                .on_hover_text(tr("Move layer down"))
                                .clicked()
                            {
                                action.move_layer = Some((i, 1));
                            }
                            
                            if ui.small_button("⚙").on_hover_text(tr("Settings")).clicked() {
                                action.open_settings = Some(i);
                            }
                            if ui.small_button("◇").on_hover_text(tr("Select all shapes on this layer")).clicked() {
                                action.select_all_shapes = Some(i);
                            }
                            if ui
                                .checkbox(&mut layer.visible, "")
                                .on_hover_text(tr("Enable/disable layer output"))
                                .changed()
                            {
                                action.layers_changed = true;
                            }
                        });
                    });
                });
            ui.add_space(10.0);
        }
    });

    action
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
