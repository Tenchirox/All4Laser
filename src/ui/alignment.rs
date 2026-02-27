use egui::{Ui, RichText};
use crate::ui::drawing::{DrawingState, ShapeParams, ShapeKind};
use crate::theme;

pub enum AlignCmd {
    Left,
    CenterHorizontal,
    Right,
    Top,
    CenterVertical,
    Bottom,
    DistributeHorizontal,
    DistributeVertical,
}

pub fn show(ui: &mut Ui, state: &mut DrawingState, selection: &[usize], workspace_size: egui::Vec2) {
    if selection.is_empty() {
        ui.add_enabled(false, egui::Button::new("Align: (Select objects)"));
        return;
    }

    ui.horizontal(|ui| {
        ui.label(RichText::new("Align:").small().color(theme::SUBTEXT));

        if ui.button("⇤").on_hover_text("Align Left").clicked() {
            apply_align(state, selection, AlignCmd::Left, workspace_size);
        }
        if ui.button("⇹").on_hover_text("Align Center (H)").clicked() {
            apply_align(state, selection, AlignCmd::CenterHorizontal, workspace_size);
        }
        if ui.button("⇥").on_hover_text("Align Right").clicked() {
            apply_align(state, selection, AlignCmd::Right, workspace_size);
        }

        ui.separator();

        if ui.button("⤒").on_hover_text("Align Top").clicked() {
            apply_align(state, selection, AlignCmd::Top, workspace_size);
        }
        if ui.button("⇕").on_hover_text("Align Center (V)").clicked() {
            apply_align(state, selection, AlignCmd::CenterVertical, workspace_size);
        }
        if ui.button("⤓").on_hover_text("Align Bottom").clicked() {
            apply_align(state, selection, AlignCmd::Bottom, workspace_size);
        }
    });
}

fn apply_align(state: &mut DrawingState, selection: &[usize], cmd: AlignCmd, workspace_size: egui::Vec2) {
    if selection.is_empty() { return; }

    // 1. Calculate selection bounds or "Anchor" (Last selected? Or total bounds?)
    // LightBurn aligns to the *last selected* object if it's "Align to Selection".
    // Or aligns to the *page* if "Align to Page" is checked.
    // For simplicity here, if 1 object -> Align to Page. If >1 object -> Align to Selection Bounds.

    let align_to_page = selection.len() == 1;

    let target_bounds = if align_to_page {
        (0.0, 0.0, workspace_size.x, workspace_size.y)
    } else {
        // Calculate union bounds of selection
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for &idx in selection {
            if let Some(shape) = state.shapes.get(idx) {
                let (sx, sy, sw, sh) = get_shape_rect(shape);
                min_x = min_x.min(sx);
                min_y = min_y.min(sy);
                max_x = max_x.max(sx + sw);
                max_y = max_y.max(sy + sh);
            }
        }
        (min_x, min_y, max_x, max_y)
    };

    let (tx, ty, tw, th) = target_bounds;
    let t_mid_x = tx + tw / 2.0;
    let t_mid_y = ty + th / 2.0;

    for &idx in selection {
        if let Some(shape) = state.shapes.get_mut(idx) {
            let (sx, sy, sw, sh) = get_shape_rect(shape);

            match cmd {
                AlignCmd::Left => shape.x = tx,
                AlignCmd::CenterHorizontal => shape.x = t_mid_x - sw / 2.0,
                AlignCmd::Right => shape.x = tx + tw - sw,
                AlignCmd::Top => shape.y = ty + th - sh, // Y is up in our world space?
                // Wait, coordinate system check:
                // GRBL: Y+ is Up.
                // Shapes: x,y is usually bottom-left origin for Rect.
                // Let's assume bottom-left origin.
                // Top alignment means aligning the TOP edge of shape to TOP edge of target.
                // Top Y = y + h. Target Top Y = ty + th.
                // new_y + h = ty + th  => new_y = ty + th - h.
                AlignCmd::CenterVertical => shape.y = t_mid_y - sh / 2.0,
                AlignCmd::Bottom => shape.y = ty,
                _ => {}
            }

            // Adjust Circle center vs corner
            if shape.shape == ShapeKind::Circle {
                // Logic above assumes x,y is bottom-left.
                // But Circle x,y is Center usually?
                // In `drawing.rs`:
                // Rect: x,y is origin (bottom-left).
                // Circle: x,y is center. radius is radius.
                // Text: x,y is baseline start?

                // Let's correct `get_shape_rect` to handle this,
                // and then we need to map back the calculated `shape.x` (which is now bottom-left) to center if Circle.
                if matches!(shape.shape, ShapeKind::Circle) {
                    shape.x += shape.radius;
                    shape.y += shape.radius;
                }
            }
        }
    }
}

// Returns (x, y, w, h) bounding box where x,y is bottom-left
fn get_shape_rect(s: &ShapeParams) -> (f32, f32, f32, f32) {
    match &s.shape {
        ShapeKind::Rectangle => (s.x, s.y, s.width, s.height),
        ShapeKind::Circle => (s.x - s.radius, s.y - s.radius, s.radius * 2.0, s.radius * 2.0),
        ShapeKind::TextLine => {
             let char_w = s.font_size_mm * 0.6;
             let w = s.text.len() as f32 * char_w;
             (s.x, s.y, w, s.font_size_mm)
        },
        ShapeKind::Path(pts) => {
            let mut min_x = f32::MAX; let mut max_x = f32::MIN;
            let mut min_y = f32::MAX; let mut max_y = f32::MIN;
            if pts.is_empty() { return (s.x, s.y, 0.0, 0.0); }
            for p in pts {
                min_x = min_x.min(p.0); max_x = max_x.max(p.0);
                min_y = min_y.min(p.1); max_y = max_y.max(p.1);
            }
            (s.x + min_x, s.y + min_y, max_x - min_x, max_y - min_y)
        }
    }
}
