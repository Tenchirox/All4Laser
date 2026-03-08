use egui::{Ui, RichText, Window};
use crate::theme;
use crate::ui::drawing::{ShapeParams, ShapeKind};
use crate::ui::layers_new::{CutLayer, CutMode};

#[derive(Default, Clone, PartialEq)]
pub enum Severity {
    #[default]
    Info,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct PreflightIssue {
    pub severity: Severity,
    pub message: String,
}

#[derive(Default)]
pub struct PreflightState {
    pub is_open: bool,
    pub block_on_critical: bool,
    pub bypass: bool,
    pub issues: Vec<PreflightIssue>,
}

pub struct PreflightAction {
    pub proceed: bool,
    pub cancel: bool,
}

fn path_is_closed(points: &[(f32, f32)]) -> bool {
    if points.len() < 3 {
        return false;
    }
    let first = points.first().unwrap();
    let last = points.last().unwrap();
    let dx = first.0 - last.0;
    let dy = first.1 - last.1;
    (dx * dx + dy * dy).sqrt() <= 0.05
}

pub fn run_checks(shapes: &[ShapeParams], layers: &[CutLayer]) -> Vec<PreflightIssue> {
    let mut issues = Vec::new();

    let mut used_layers = vec![false; layers.len()];

    for (i, shape) in shapes.iter().enumerate() {
        if shape.layer_idx < layers.len() {
            used_layers[shape.layer_idx] = true;
            let layer = &layers[shape.layer_idx];

            if !layer.visible {
                continue;
            }

            // Check for valid paths in Fill mode
            if matches!(layer.mode, CutMode::Fill | CutMode::FillAndLine | CutMode::Offset) {
                if let ShapeKind::Path(pts) = &shape.shape {
                    if pts.len() < 3 {
                        issues.push(PreflightIssue {
                            severity: Severity::Warning,
                            message: format!("Shape {} on layer {} (Fill) has fewer than 3 points and will be ignored.", i + 1, layer.name),
                        });
                    } else if !path_is_closed(pts) {
                        issues.push(PreflightIssue {
                            severity: Severity::Warning,
                            message: format!("Shape {} on layer {} (Fill) is an open path. Close contour to enable fill.", i + 1, layer.name),
                        });
                    }
                }
            }

            // Check for invalid shapes overall
            if let ShapeKind::Path(pts) = &shape.shape {
                if pts.is_empty() {
                    issues.push(PreflightIssue {
                        severity: Severity::Error,
                        message: format!("Shape {} on layer {} has 0 points.", i + 1, layer.name),
                    });
                }
            }
        } else {
            issues.push(PreflightIssue {
                severity: Severity::Error,
                message: format!("Shape {} refers to invalid layer index {}.", i + 1, shape.layer_idx),
            });
        }
    }

    // Layer-level checks
    for (idx, layer) in layers.iter().enumerate() {
        if used_layers[idx] && layer.visible {
            if layer.power > 80.0 && layer.speed < 100.0 {
                issues.push(PreflightIssue {
                    severity: Severity::Warning,
                    message: format!("Layer {} has high power ({}) and low speed ({}). Fire risk!", layer.name, layer.power, layer.speed),
                });
            }
            if layer.speed <= 0.0 {
                issues.push(PreflightIssue {
                    severity: Severity::Error,
                    message: format!("Layer {} has speed <= 0.", layer.name),
                });
            }
        }
    }

    issues
}

pub fn show(
    ctx: &egui::Context,
    state: &mut PreflightState,
) -> PreflightAction {
    let mut action = PreflightAction { proceed: false, cancel: false };

    if !state.is_open {
        return action;
    }

    let mut is_open = state.is_open;
    let has_errors = state.issues.iter().any(|i| i.severity == Severity::Error);

    Window::new("🔍 Preflight Check")
        .open(&mut is_open)
        .resizable(true)
        .collapsible(false)
        .min_width(400.0)
        .show(ctx, |ui| {
            if state.issues.is_empty() {
                ui.label(RichText::new("✅ No issues found. Ready to launch.").color(theme::GREEN).strong());
                ui.add_space(8.0);
                if ui.button("Launch Job").clicked() {
                    action.proceed = true;
                }
            } else {
                ui.label("Issues found before running job:");
                ui.add_space(4.0);

                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for issue in &state.issues {
                        let (icon, color) = match issue.severity {
                            Severity::Info => ("ℹ", theme::BLUE),
                            Severity::Warning => ("⚠", theme::PEACH),
                            Severity::Error => ("❌", theme::RED),
                        };
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(icon).color(color));
                            ui.label(&issue.message);
                        });
                    }
                });

                ui.add_space(8.0);
                ui.checkbox(&mut state.block_on_critical, "Block launch on critical errors");

                ui.horizontal(|ui| {
                    if has_errors && state.block_on_critical {
                        ui.label(RichText::new("Cannot launch job with critical errors.").color(theme::RED));
                    } else {
                        if ui.button(RichText::new("Launch Anyway").color(theme::GREEN)).clicked() {
                            action.proceed = true;
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        action.cancel = true;
                    }
                });
            }
        });

    if action.proceed || action.cancel {
        is_open = false;
    }

    state.is_open = is_open;
    action
}
