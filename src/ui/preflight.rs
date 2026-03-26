use egui::{RichText, Window};
use crate::i18n::tr;
use crate::theme;
use crate::ui::drawing::{ShapeParams, ShapeKind};
use crate::ui::layers_new::{CutLayer, CutMode};

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

#[derive(Default)]
pub struct PreflightState {
    pub is_open: bool,
    pub report: Option<PreflightReport>,
}

pub struct PreflightAction {
    pub proceed: bool,
    pub cancel: bool,
}

pub fn show(
    ctx: &egui::Context,
    state: &mut PreflightState,
    block_on_critical: bool,
) -> PreflightAction {
    let mut action = PreflightAction { proceed: false, cancel: false };

    if !state.is_open {
        return action;
    }

    let mut is_open = state.is_open;

    let report = match &state.report {
        Some(r) => r.clone(),
        None => PreflightReport::default(),
    };

    let critical_count = report.critical_count();
    let warning_count = report.warning_count();
    let has_critical = critical_count > 0;
    let all_clear = report.issues.is_empty();

    Window::new("🔍 Preflight Check")
        .open(&mut is_open)
        .resizable(true)
        .collapsible(false)
        .min_width(460.0)
        .min_height(200.0)
        .show(ctx, |ui| {
            // ── Summary header ──────────────────────────────────────────
            egui::Frame::new()
                .inner_margin(egui::Margin::symmetric(8, 6))
                .corner_radius(egui::CornerRadius::same(4))
                .fill(if all_clear {
                    egui::Color32::from_rgba_unmultiplied(166, 227, 161, 20)
                } else if has_critical {
                    egui::Color32::from_rgba_unmultiplied(243, 139, 168, 20)
                } else {
                    egui::Color32::from_rgba_unmultiplied(250, 179, 135, 20)
                })
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if all_clear {
                            ui.label(
                                RichText::new("✅  All checks passed — ready to launch.")
                                    .color(theme::GREEN)
                                    .strong(),
                            );
                        } else {
                            if has_critical {
                                ui.label(
                                    RichText::new(format!("⛔  {critical_count} critical"))
                                        .color(theme::RED)
                                        .strong(),
                                );
                                ui.separator();
                            }
                            if warning_count > 0 {
                                ui.label(
                                    RichText::new(format!("⚠  {warning_count} warning"))
                                        .color(theme::PEACH)
                                        .strong(),
                                );
                            }
                        }
                    });
                });

            ui.add_space(6.0);

            // ── Issues list ─────────────────────────────────────────────
            if !all_clear {
                egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                    // Criticals first
                    for issue in report.issues.iter().filter(|i| i.severity == PreflightSeverity::Critical) {
                        egui::Frame::new()
                            .inner_margin(egui::Margin::symmetric(6, 3))
                            .corner_radius(egui::CornerRadius::same(3))
                            .fill(egui::Color32::from_rgba_unmultiplied(243, 139, 168, 12))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("⛔").color(theme::RED));
                                    ui.label(
                                        RichText::new(&issue.message).color(theme::RED).small(),
                                    );
                                });
                            });
                        ui.add_space(2.0);
                    }
                    // Warnings
                    for issue in report.issues.iter().filter(|i| i.severity == PreflightSeverity::Warning) {
                        egui::Frame::new()
                            .inner_margin(egui::Margin::symmetric(6, 3))
                            .corner_radius(egui::CornerRadius::same(3))
                            .fill(egui::Color32::from_rgba_unmultiplied(250, 179, 135, 12))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("⚠").color(theme::PEACH));
                                    ui.label(
                                        RichText::new(&issue.message).color(theme::PEACH).small(),
                                    );
                                });
                            });
                        ui.add_space(2.0);
                    }
                });
                ui.add_space(8.0);
            }

            ui.separator();
            ui.add_space(6.0);

            // ── Action buttons ──────────────────────────────────────────
            ui.horizontal(|ui| {
                if all_clear {
                    if ui
                        .button(
                            RichText::new("▶  Launch Job")
                                .color(theme::GREEN)
                                .strong(),
                        )
                        .clicked()
                    {
                        action.proceed = true;
                    }
                } else if has_critical && block_on_critical {
                    ui.label(
                        RichText::new(tr("Launch blocked — fix critical issues first."))
                            .color(theme::RED)
                            .small(),
                    );
                } else {
                    if ui
                        .button(
                            RichText::new("▶  Launch Anyway")
                                .color(theme::PEACH)
                                .strong(),
                        )
                        .clicked()
                    {
                        action.proceed = true;
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(tr("Cancel")).clicked() {
                        action.cancel = true;
                    }
                });
            });
        });

    if action.proceed || action.cancel {
        is_open = false;
    }

    state.is_open = is_open;
    action
}

use crate::config::machine_profile::MachineProfile;
use crate::gcode::file::GCodeFile;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreflightSeverity {
    Critical,
    Warning,
}

#[derive(Clone, Debug)]
pub struct ReportIssue {
    pub severity: PreflightSeverity,
    pub message: String,
}

#[derive(Clone, Debug, Default)]
pub struct PreflightReport {
    pub issues: Vec<ReportIssue>,
}

impl PreflightReport {
    pub fn add_critical(&mut self, message: impl Into<String>) {
        self.issues.push(ReportIssue {
            severity: PreflightSeverity::Critical,
            message: message.into(),
        });
    }

    pub fn add_warning(&mut self, message: impl Into<String>) {
        self.issues.push(ReportIssue {
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
        report.add_critical(tr("No program loaded."));
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
                    "{} #{} {} {}.",
                    tr("Shape"),
                    idx + 1,
                    tr("uses missing layer index"),
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
            "{} {open_paths} {}.",
            tr("Detected"),
            tr("open path(s). Close contours before launch")
        ));
    }

    let duplicate_count = duplicate_segments
        .values()
        .filter(|&&count| count > 1)
        .count();
    if duplicate_count > 0 {
        report.add_critical(format!(
            "{} {duplicate_count} {}.",
            tr("Detected"),
            tr("duplicated path segment group(s)")
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
                "{overlap_count} {} -- {}.",
                tr("pair(s) of shapes appear identical/overlapping"),
                tr("risk of double-burn")
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
                "{} #{} {} ({:.0}x{:.0}mm).",
                tr("Shape"),
                idx + 1,
                tr("extends outside workspace bounds"),
                ws_x,
                ws_y
            ));
        }
    }

    // F94: Interlock safety checks
    if ctx.machine_profile.interlock_lid_enabled {
        report.add_warning(
            tr("Lid interlock enabled -- ensure lid is closed before running.").to_string(),
        );
    }
    if ctx.machine_profile.interlock_water_enabled {
        report.add_warning(
            tr("Water cooling interlock enabled -- ensure water flow is active.").to_string(),
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
            report.add_critical(format!("{} {} {} (<= 0).", tr("Layer"), layer.name, tr("has invalid speed")));
        }
        if layer.power < 0.0 {
            report.add_critical(format!("{} {} {} (< 0).", tr("Layer"), layer.name, tr("has invalid power")));
        }
        if layer.passes == 0 {
            report.add_critical(format!("{} {} {} (= 0).", tr("Layer"), layer.name, tr("has invalid passes")));
        }
        if matches!(layer.mode, CutMode::Fill | CutMode::FillAndLine)
            && layer.fill_interval_mm <= 0.0
        {
            report.add_critical(format!(
                "{} {} {}.",
                tr("Layer"),
                layer.name,
                tr("fill interval must be > 0 for fill modes")
            ));
        }
        if !layer.visible {
            report.add_warning(format!(
                "{} {} {}.",
                tr("Layer"),
                layer.name,
                tr("is disabled but still referenced by current job")
            ));
        }
        if layer.power > 1000.0 {
            report.add_warning(format!(
                "{} {} {} ({:.0}) {}.",
                tr("Layer"),
                layer.name,
                tr("power"),
                layer.power,
                tr("exceeds nominal GRBL S1000 range")
            ));
        }
        if layer.power > 80.0 && layer.speed < 100.0 {
            report.add_warning(format!(
                "{} {} {} ({:.0}%) {} ({:.0}mm/min) — {}!",
                tr("Layer"),
                layer.name,
                tr("has high power"),
                layer.power,
                tr("and low speed"),
                layer.speed,
                tr("fire risk")
            ));
        }
    }

    report
}
