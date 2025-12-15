// FILE: src/notifications/functions/fnc_expand_calculate_rect.rs - Expand/Collapse animation rect calculation
// VERSION: 1.0.0
// WCTX: TDD implementation of animation function extraction
// CLOG: Extracted from expand_collapse.rs with rect interpolation logic

use crate::notifications::types::AnimationPhase;
use crate::shared_utils::math::lerp;
use ratatui::prelude::*;

// Minimum dimensions for expand/collapse animation
const MIN_WIDTH: u16 = 3;
const MIN_HEIGHT: u16 = 3;

/// Calculates the visible rectangle for an expand/collapse animation.
///
/// This function interpolates the notification size from/to a minimum size (3x3)
/// while keeping the notification centered during the animation.
///
/// # Arguments
///
/// * `full_rect` - The full rectangle of the notification when fully expanded
/// * `_frame_area` - The frame area (ignored for expand/collapse animations)
/// * `phase` - The current animation phase
/// * `progress` - The animation progress (0.0 to 1.0)
///
/// # Returns
///
/// The interpolated rectangle at the current animation progress
///
/// # Examples
///
/// ```
/// use ratatui::prelude::*;
/// use ratatui_notifications::notifications::functions::fnc_expand_calculate_rect::calculate_rect;
/// use ratatui_notifications::notifications::types::AnimationPhase;
///
/// let full_rect = Rect::new(10, 20, 33, 13);
/// let frame_area = Rect::new(0, 0, 100, 100);
///
/// // At the start of expanding, should be minimum size (3x3) centered
/// let result = calculate_rect(full_rect, frame_area, AnimationPhase::Expanding, 0.0);
/// assert_eq!(result, Rect::new(25, 25, 3, 3));
///
/// // At the end of expanding, should be full size
/// let result = calculate_rect(full_rect, frame_area, AnimationPhase::Expanding, 1.0);
/// assert_eq!(result, full_rect);
/// ```
pub fn calculate_rect(
    full_rect: Rect,
    _frame_area: Rect,
    phase: AnimationPhase,
    progress: f32,
) -> Rect {
    let progress = progress.clamp(0.0, 1.0);

    let (start_width, start_height, end_width, end_height) = match phase {
        AnimationPhase::Expanding => (
            MIN_WIDTH as f32,
            MIN_HEIGHT as f32,
            full_rect.width as f32,
            full_rect.height as f32,
        ),
        AnimationPhase::Collapsing => (
            full_rect.width as f32,
            full_rect.height as f32,
            MIN_WIDTH as f32,
            MIN_HEIGHT as f32,
        ),
        // For other phases, just return the full rect
        _ => return full_rect,
    };

    // Interpolate dimensions
    let current_width_f32 = lerp(start_width, end_width, progress);
    let current_height_f32 = lerp(start_height, end_height, progress);

    // Round dimensions, ensuring they are at least 1x1 if progress > 0
    let current_width = (current_width_f32.round() as u16).max(if progress > 0.0 { 1 } else { 0 });
    let current_height =
        (current_height_f32.round() as u16).max(if progress > 0.0 { 1 } else { 0 });

    // Calculate top-left position to keep the rectangle centered around the full_rect's center
    let center_x_full = full_rect.x as f32 + (full_rect.width as f32 / 2.0);
    let center_y_full = full_rect.y as f32 + (full_rect.height as f32 / 2.0);

    let current_x = (center_x_full - (current_width as f32 / 2.0)).round() as u16;
    let current_y = (center_y_full - (current_height as f32 / 2.0)).round() as u16;

    // Ensure dimensions are valid
    if current_width == 0 || current_height == 0 {
        Rect::default()
    } else {
        Rect::new(current_x, current_y, current_width, current_height)
    }
}

// FILE: src/notifications/functions/fnc_expand_calculate_rect.rs - Expand/Collapse animation rect calculation
// END OF VERSION: 1.0.0
