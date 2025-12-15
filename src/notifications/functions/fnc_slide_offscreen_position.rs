// FILE: src/notifications/functions/fnc_slide_offscreen_position.rs - Calculates offscreen starting position for slide animations
// VERSION: 1.0.0
// WCTX: Implementing slide animation functions with TDD
// CLOG: Extracted from slide.rs:36-72, initial creation

use crate::notifications::types::{Anchor, SlideDirection};
use ratatui::prelude::Rect;

/// Calculates the default off-screen starting/ending coordinates for sliding.
///
/// This function determines where the notification should start (when sliding in)
/// or end (when sliding out) based on the slide direction. The position is calculated
/// to be just outside the frame area with a small margin.
///
/// # Arguments
///
/// * `_anchor` - The anchor position (currently unused, kept for API consistency)
/// * `slide_direction` - The direction from which to slide
/// * `full_rect` - The full rectangle of the notification
/// * `frame_area` - The visible frame area
///
/// # Returns
///
/// A tuple `(x, y)` representing the off-screen position
///
/// # Examples
///
/// ```
/// use ratatui::prelude::Rect;
/// use ratatui_notifications::notifications::functions::fnc_slide_offscreen_position::slide_offscreen_position;
/// use ratatui_notifications::notifications::types::{Anchor, SlideDirection};
///
/// let full_rect = Rect::new(50, 25, 20, 10);
/// let frame_area = Rect::new(0, 0, 100, 50);
/// let (x, y) = slide_offscreen_position(
///     Anchor::MiddleLeft,
///     SlideDirection::FromLeft,
///     full_rect,
///     frame_area,
/// );
/// assert_eq!(x, -21.0); // Left of frame
/// ```
pub fn slide_offscreen_position(
    _anchor: Anchor,
    slide_direction: SlideDirection,
    full_rect: Rect,
    frame_area: Rect,
) -> (f32, f32) {
    const EDGE_MARGIN: i16 = 1;
    let width = full_rect.width as i16;
    let height = full_rect.height as i16;
    let frame_x = frame_area.x as i16;
    let frame_y = frame_area.y as i16;
    let frame_right = frame_area.right() as i16;
    let frame_bottom = frame_area.bottom() as i16;
    let full_x = full_rect.x as i16;
    let full_y = full_rect.y as i16;

    let start_x = match slide_direction {
        SlideDirection::FromLeft | SlideDirection::FromTopLeft | SlideDirection::FromBottomLeft => {
            frame_x.saturating_sub(width).saturating_sub(EDGE_MARGIN)
        }
        SlideDirection::FromRight
        | SlideDirection::FromTopRight
        | SlideDirection::FromBottomRight => frame_right.saturating_add(EDGE_MARGIN),
        _ => full_x,
    };
    let start_y = match slide_direction {
        SlideDirection::FromTop | SlideDirection::FromTopLeft | SlideDirection::FromTopRight => {
            frame_y.saturating_sub(height).saturating_sub(EDGE_MARGIN)
        }
        SlideDirection::FromBottom
        | SlideDirection::FromBottomLeft
        | SlideDirection::FromBottomRight => frame_bottom.saturating_add(EDGE_MARGIN),
        _ => full_y,
    };
    (start_x as f32, start_y as f32)
}

// FILE: src/notifications/functions/fnc_slide_offscreen_position.rs - Calculates offscreen starting position for slide animations
// END OF VERSION: 1.0.0
