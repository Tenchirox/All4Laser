use crate::i18n::tr;
use crate::theme;
use crate::ui::layers_new::CutLayer;
use egui::{Color32, RichText, Sense, Stroke, StrokeKind, Ui};

pub struct PaletteAction {
    pub select_layer: Option<usize>,
    pub open_settings: Option<usize>,
}

pub fn show(ui: &mut Ui, layers: &[CutLayer], active_idx: usize, is_light_mode: bool) -> PaletteAction {
    let mut action = PaletteAction {
        select_layer: None,
        open_settings: None,
    };

    ui.horizontal(|ui| {
        ui.label(RichText::new("🎨 Palette:").small().color(theme::SUBTEXT));

        egui::ScrollArea::horizontal()
            .id_salt("palette_scroll")
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.x = 2.0;

                for (i, layer) in layers.iter().enumerate() {
                    let is_active = i == active_idx;

                    // Button appearance
                    let (rect, response) =
                        ui.allocate_exact_size(egui::vec2(24.0, 24.0), Sense::click());

                    // Draw color box
                    if ui.is_rect_visible(rect) {
                        // Background
                        let bg = if is_active {
                            crate::theme::get_text(is_light_mode)
                        } else {
                            crate::theme::get_surface0(is_light_mode)
                        };
                        ui.painter().rect_filled(rect, 2.0, bg);

                        // Inner color
                        let inner = rect.shrink(2.0);
                        ui.painter().rect_filled(inner, 1.0, layer.color);

                        // Text ID
                        let text_color = if theme::is_light(layer.color) {
                            Color32::BLACK
                        } else {
                            Color32::WHITE
                        };
                        ui.painter().text(
                            inner.center(),
                            egui::Align2::CENTER_CENTER,
                            format!("{:02}", i),
                            egui::FontId::monospace(10.0),
                            text_color,
                        );

                        // Active border
                        if is_active {
                            ui.painter().rect_stroke(
                                rect,
                                2.0,
                                Stroke::new(2.0, theme::GREEN),
                                StrokeKind::Middle,
                            );
                        } else if response.hovered() {
                            ui.painter().rect_stroke(
                                rect,
                                2.0,
                                Stroke::new(1.0, theme::OVERLAY2),
                                StrokeKind::Middle,
                            );
                        }
                    }

                    // Interactions
                    if response.clicked() {
                        action.select_layer = Some(i);
                    }
                    if response.double_clicked() {
                        action.open_settings = Some(i);
                    }

                    response.on_hover_text(format!(
                        "{} C{:02}\n{}: {}\n{}: {}",
                        tr("Layer"), i,
                        tr("Speed"), layer.speed,
                        tr("Power"), layer.power
                    ));
                }
            });
    });

    action
}
