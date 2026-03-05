/// Sub-struct definitions for All4LaserApp fields.
/// Extracted from app.rs to reduce file size and improve modularity.
use std::time::Instant;

/// Grouped job transform state
#[derive(Clone, Debug)]
pub struct JobTransform {
    pub offset_x: f32,
    pub offset_y: f32,
    pub rotation: f32,
    pub center: Option<egui::Pos2>,
}

impl Default for JobTransform {
    fn default() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            rotation: 0.0,
            center: None,
        }
    }
}

/// Grouped camera live frame data
pub struct CameraLiveState {
    pub stream: Option<crate::camera_stream::CameraStream>,
    pub last_frame_rgba: Vec<u8>,
    pub last_frame_width: usize,
    pub last_frame_height: usize,
    pub calibration_picks: Vec<egui::Pos2>,
    pub point_align_picks: Vec<egui::Pos2>,
}

impl Default for CameraLiveState {
    fn default() -> Self {
        Self {
            stream: None,
            last_frame_rgba: Vec::new(),
            last_frame_width: 0,
            last_frame_height: 0,
            calibration_picks: Vec::new(),
            point_align_picks: Vec::new(),
        }
    }
}

/// Grouped test fire state
pub struct TestFireState {
    pub is_open: bool,
    pub power: f32,
    pub duration_ms: f32,
}

impl Default for TestFireState {
    fn default() -> Self {
        Self {
            is_open: false,
            power: 10.0,
            duration_ms: 1000.0,
        }
    }
}

/// Grouped wizard state (F43)
pub struct WizardState {
    pub show: bool,
    pub step: u8,
}

impl Default for WizardState {
    fn default() -> Self {
        Self {
            show: false,
            step: 0,
        }
    }
}

/// Grouped auto-save state (F71)
pub struct AutosaveState {
    pub last_save: Instant,
    pub interval_secs: u64,
    pub show_recovery_prompt: bool,
    pub pending_recovery: Option<crate::config::project::ProjectFile>,
}

impl Default for AutosaveState {
    fn default() -> Self {
        Self {
            last_save: Instant::now(),
            interval_secs: 60,
            show_recovery_prompt: false,
            pending_recovery: None,
        }
    }
}

/// Helper: check if a path is closed (first ≈ last point)
pub fn path_is_closed_for_fill(points: &[(f32, f32)]) -> bool {
    if points.len() < 3 {
        return false;
    }
    let (Some(first), Some(last)) = (points.first(), points.last()) else {
        return false;
    };
    let dx = first.0 - last.0;
    let dy = first.1 - last.1;
    (dx * dx + dy * dy).sqrt() <= 0.05
}

/// Helper: check if a shape in fill mode has a valid contour
pub fn shape_fill_warning(
    shape: &crate::ui::drawing::ShapeParams,
    layers: &[crate::ui::layers_new::CutLayer],
) -> Option<&'static str> {
    use crate::ui::drawing::ShapeKind;
    use crate::ui::layers_new::CutMode;

    let layer = layers.get(shape.layer_idx)?;
    let is_fill_mode = matches!(
        layer.mode,
        CutMode::Fill | CutMode::FillAndLine | CutMode::Offset
    );
    if !is_fill_mode {
        return None;
    }

    match &shape.shape {
        ShapeKind::Path(points) if points.len() < 3 => {
            Some("Fill ignored: path needs at least 3 points.")
        }
        ShapeKind::Path(points) if !path_is_closed_for_fill(points) => {
            Some("Fill ignored: open path. Close contour to enable fill.")
        }
        _ => None,
    }
}
