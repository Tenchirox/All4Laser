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
use crate::config::machine_profile::MachineProfile;
use crate::gcode::file::GCodeFile;
use crate::ui::drawing::{ShapeKind, ShapeParams};
use crate::ui::layers_new::{CutLayer, CutMode};
use std::collections::{HashMap, HashSet};

type SegmentKey = ((i32, i32), (i32, i32));

fn quantize_mm(v: f32) -> i32 {
    (v * 100.0).round() as i32
}

pub fn normalized_segment_key(a: (f32, f32), b: (f32, f32)) -> SegmentKey {
    let qa = (quantize_mm(a.0), quantize_mm(a.1));
    let qb = (quantize_mm(b.0), quantize_mm(b.1));
    if qa <= qb { (qa, qb) } else { (qb, qa) }
}

fn path_is_closed(pts: &[(f32, f32)]) -> bool {
    if pts.len() < 3 {
        return false;
    }
    let first = pts.first().unwrap();
    let last = pts.last().unwrap();
    (first.0 - last.0).abs() < 0.01 && (first.1 - last.1).abs() < 0.01
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreflightSeverity {
    Critical,
    Warning,
}

#[derive(Clone, Debug)]
pub struct PreflightIssue {
    pub severity: PreflightSeverity,
    pub message: String,
}

#[derive(Clone, Debug, Default)]
pub struct PreflightReport {
    pub issues: Vec<PreflightIssue>,
}

impl PreflightReport {
    pub fn add_critical(&mut self, message: impl Into<String>) {
        self.issues.push(PreflightIssue {
            severity: PreflightSeverity::Critical,
            message: message.into(),
        });
    }

    pub fn add_warning(&mut self, message: impl Into<String>) {
        self.issues.push(PreflightIssue {
            severity: PreflightSeverity::Warning,
            message: message.into(),
        });
    }

    pub fn critical_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == PreflightSeverity::Critical)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == PreflightSeverity::Warning)
            .count()
    }
}

pub struct PreflightContext<'a> {
    pub shapes: &'a [ShapeParams],
    pub layers: &'a [CutLayer],
    pub loaded_file: Option<&'a GCodeFile>,
    pub program_lines: &'a [String],
    pub machine_profile: &'a MachineProfile,
}

pub fn build_preflight_report(ctx: &PreflightContext) -> PreflightReport {
    let mut report = PreflightReport::default();

    if ctx.program_lines.is_empty() {
        report.add_critical("No program loaded.");
        return report;
    }

    let mut duplicate_segments: HashMap<SegmentKey, usize> = HashMap::new();
    let mut open_paths = 0usize;
    let mut used_layers: HashSet<usize> = HashSet::new();

    if !ctx.shapes.is_empty() {
        for (idx, shape) in ctx.shapes.iter().enumerate() {
            used_layers.insert(shape.layer_idx);
            if shape.layer_idx >= ctx.layers.len() {
                report.add_critical(format!(
                    "Shape #{} uses missing layer index {}.",
                    idx + 1,
                    shape.layer_idx
                ));
            }

            if let ShapeKind::Path(points) = &shape.shape {
                if points.len() >= 2 && !path_is_closed(points) {
                    open_paths += 1;
                }
                for seg in points.windows(2) {
                    let p0 = shape.world_pos(seg[0].0, seg[0].1);
                    let p1 = shape.world_pos(seg[1].0, seg[1].1);
                    let key = normalized_segment_key(p0, p1);
                    *duplicate_segments.entry(key).or_insert(0) += 1;
                }
            }
        }
    } else if let Some(file) = ctx.loaded_file {
        for seg in &file.segments {
            used_layers.insert(seg.layer_id);
            let key = normalized_segment_key((seg.x1, seg.y1), (seg.x2, seg.y2));
            *duplicate_segments.entry(key).or_insert(0) += 1;
        }
    }

    if open_paths > 0 {
        report.add_critical(format!(
            "Detected {open_paths} open path(s). Close contours before launch."
        ));
    }

    let duplicate_count = duplicate_segments
        .values()
        .filter(|&&count| count > 1)
        .count();
    if duplicate_count > 0 {
        report.add_critical(format!(
            "Detected {duplicate_count} duplicated path segment group(s)."
        ));
    }

    // F84: Detect fully overlapping shapes
    if ctx.shapes.len() >= 2 {
        let mut overlap_count = 0usize;
        for i in 0..ctx.shapes.len() {
            for j in (i + 1)..ctx.shapes.len() {
                let a = &ctx.shapes[i];
                let b = &ctx.shapes[j];
                if (a.x - b.x).abs() < 0.01
                    && (a.y - b.y).abs() < 0.01
                    && (a.width - b.width).abs() < 0.01
                    && (a.height - b.height).abs() < 0.01
                    && (a.radius - b.radius).abs() < 0.01
                    && std::mem::discriminant(&a.shape) == std::mem::discriminant(&b.shape)
                {
                    overlap_count += 1;
                }
            }
        }
        if overlap_count > 0 {
            report.add_warning(format!(
                "{overlap_count} pair(s) of shapes appear identical/overlapping -- risk of double-burn."
            ));
        }
    }

    // F59: Workspace bounds collision detection
    let ws_x = ctx.machine_profile.workspace_x_mm;
    let ws_y = ctx.machine_profile.workspace_y_mm;
    for (idx, shape) in ctx.shapes.iter().enumerate() {
        let (min_x, min_y, max_x, max_y) = crate::ui::drawing::shape_world_bounds_pub(shape);
        if min_x < -0.1 || min_y < -0.1 || max_x > ws_x + 0.1 || max_y > ws_y + 0.1 {
            report.add_warning(format!(
                "Shape #{} extends outside workspace bounds ({:.0}x{:.0}mm).",
                idx + 1,
                ws_x,
                ws_y
            ));
        }
    }

    // F94: Interlock safety checks
    if ctx.machine_profile.interlock_lid_enabled {
        report.add_warning(
            "Lid interlock enabled -- ensure lid is closed before running.".to_string(),
        );
    }
    if ctx.machine_profile.interlock_water_enabled {
        report.add_warning(
            "Water cooling interlock enabled -- ensure water flow is active.".to_string(),
        );
    }

    // Layer validation
    for layer_idx in used_layers {
        let Some(layer) = ctx.layers.get(layer_idx) else {
            report.add_critical(format!(
                "Used layer index {layer_idx} is missing from layer list."
            ));
            continue;
        };

        if layer.speed <= 0.0 {
            report.add_critical(format!("Layer {} has invalid speed (<= 0).", layer.name));
        }
        if layer.power < 0.0 {
            report.add_critical(format!("Layer {} has invalid power (< 0).", layer.name));
        }
        if layer.passes == 0 {
            report.add_critical(format!("Layer {} has invalid passes (= 0).", layer.name));
        }
        if matches!(layer.mode, CutMode::Fill | CutMode::FillAndLine)
            && layer.fill_interval_mm <= 0.0
        {
            report.add_critical(format!(
                "Layer {} fill interval must be > 0 for fill modes.",
                layer.name
            ));
        }
        if !layer.visible {
            report.add_warning(format!(
                "Layer {} is disabled but still referenced by current job.",
                layer.name
            ));
        }
        if layer.power > 1000.0 {
            report.add_warning(format!(
                "Layer {} power ({:.0}) exceeds nominal GRBL S1000 range.",
                layer.name, layer.power
            ));
        }
    }

    report
}
