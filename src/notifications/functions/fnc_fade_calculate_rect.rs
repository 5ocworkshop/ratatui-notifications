// FILE: src/notifications/functions/fnc_fade_calculate_rect.rs - Fade animation rect calculation
// VERSION: 1.0.0
// WCTX: TDD implementation of animation function extraction
// CLOG: Extracted from fade.rs - fade doesn't change rect, just returns full_rect

use crate::notifications::types::AnimationPhase;
use ratatui::prelude::*;

/// Calculates the visible rectangle for a fade animation.
///
/// Fade animations do not change the size or position of the notification,
/// so this function always returns the full_rect unchanged.
///
/// # Arguments
///
/// * `full_rect` - The full rectangle of the notification
/// * `_frame_area` - The frame area (ignored for fade animations)
/// * `_phase` - The current animation phase (ignored for fade animations)
/// * `_progress` - The animation progress (ignored for fade animations)
///
/// # Returns
///
/// The full_rect unchanged
///
/// # Examples
///
/// ```
/// use ratatui::prelude::*;
/// use ratatui_notifications::notifications::functions::fnc_fade_calculate_rect::calculate_rect;
/// use ratatui_notifications::notifications::types::AnimationPhase;
///
/// let full_rect = Rect::new(10, 20, 30, 40);
/// let frame_area = Rect::new(0, 0, 100, 100);
/// let result = calculate_rect(full_rect, frame_area, AnimationPhase::FadingIn, 0.5);
/// assert_eq!(result, full_rect);
/// ```
pub fn calculate_rect(
    full_rect: Rect,
    _frame_area: Rect,
    _phase: AnimationPhase,
    _progress: f32,
) -> Rect {
    full_rect
}

// FILE: src/notifications/functions/fnc_fade_calculate_rect.rs - Fade animation rect calculation
// END OF VERSION: 1.0.0
