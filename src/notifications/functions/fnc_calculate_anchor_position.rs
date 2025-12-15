// FILE: src/notifications/functions/fnc_calculate_anchor_position.rs - Calculate anchor position from frame area
// VERSION: 1.0.0
// WCTX: Implementing layout functions for ratatui-notifications using TDD
// CLOG: Initial creation - calculates anchor position based on frame area and anchor type

use crate::notifications::types::Anchor;
use ratatui::layout::{Position, Rect};

/// Calculate the anchor position within a frame area.
///
/// Given an anchor point and a frame area, this function returns the exact
/// position (x, y) that corresponds to that anchor point within the frame.
///
/// # Arguments
///
/// * `anchor` - The anchor position (e.g., TopLeft, BottomRight, etc.)
/// * `frame_area` - The rectangular area within which to calculate the anchor position
///
/// # Returns
///
/// A `Position` representing the (x, y) coordinates of the anchor point.
///
/// # Examples
///
/// ```
/// use ratatui::layout::{Position, Rect};
/// use ratatui_notifications::notifications::types::Anchor;
/// use ratatui_notifications::notifications::functions::fnc_calculate_anchor_position::calculate_anchor_position;
///
/// let frame = Rect::new(0, 0, 100, 50);
/// let pos = calculate_anchor_position(Anchor::TopLeft, frame);
/// assert_eq!(pos, Position::new(0, 0));
/// ```
pub fn calculate_anchor_position(anchor: Anchor, frame_area: Rect) -> Position {
    match anchor {
        Anchor::TopLeft => Position::new(frame_area.x, frame_area.y),
        Anchor::TopCenter => Position::new(frame_area.x + frame_area.width / 2, frame_area.y),
        Anchor::TopRight => Position::new(frame_area.right().saturating_sub(1), frame_area.y),
        Anchor::MiddleLeft => Position::new(frame_area.x, frame_area.y + frame_area.height / 2),
        Anchor::MiddleCenter => Position::new(
            frame_area.x + frame_area.width / 2,
            frame_area.y + frame_area.height / 2,
        ),
        Anchor::MiddleRight => Position::new(
            frame_area.right().saturating_sub(1),
            frame_area.y + frame_area.height / 2,
        ),
        Anchor::BottomLeft => Position::new(frame_area.x, frame_area.bottom().saturating_sub(1)),
        Anchor::BottomCenter => Position::new(
            frame_area.x + frame_area.width / 2,
            frame_area.bottom().saturating_sub(1),
        ),
        Anchor::BottomRight => Position::new(
            frame_area.right().saturating_sub(1),
            frame_area.bottom().saturating_sub(1),
        ),
    }
}

// FILE: src/notifications/functions/fnc_calculate_anchor_position.rs - Calculate anchor position from frame area
// END OF VERSION: 1.0.0
