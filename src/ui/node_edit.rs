/// Node editing undo/redo helpers and snapshot types.
/// Extracted from app.rs for maintainability.

use std::collections::VecDeque;
use crate::ui::drawing::ShapeParams;

pub const MAX_NODE_HISTORY: usize = 128;

#[derive(Clone)]
pub struct NodeEditSnapshot {
    pub shapes: Vec<ShapeParams>,
    pub selected_shape_idx: Vec<usize>,
    pub selected_node: Option<(usize, usize)>,
    pub selected_nodes: Vec<(usize, usize)>,
}

pub fn undo_history_step(
    undo_stack: &mut VecDeque<NodeEditSnapshot>,
    redo_stack: &mut VecDeque<NodeEditSnapshot>,
    current: NodeEditSnapshot,
) -> Option<NodeEditSnapshot> {
    let prev = undo_stack.pop_back()?;
    redo_stack.push_back(current);
    Some(prev)
}

pub fn redo_history_step(
    undo_stack: &mut VecDeque<NodeEditSnapshot>,
    redo_stack: &mut VecDeque<NodeEditSnapshot>,
    current: NodeEditSnapshot,
) -> Option<NodeEditSnapshot> {
    let next = redo_stack.pop_back()?;
    undo_stack.push_back(current);
    Some(next)
}

pub fn push_undo(
    undo_stack: &mut VecDeque<NodeEditSnapshot>,
    redo_stack: &mut VecDeque<NodeEditSnapshot>,
    snapshot: NodeEditSnapshot,
) {
    undo_stack.push_back(snapshot);
    if undo_stack.len() > MAX_NODE_HISTORY {
        undo_stack.pop_front();
    }
    redo_stack.clear();
}
