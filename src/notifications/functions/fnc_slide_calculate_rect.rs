// FILE: src/notifications/functions/fnc_slide_calculate_rect.rs - Calculates visible rect during slide animation
// VERSION: 1.0.0
// WCTX: Implementing slide animation functions with TDD
// CLOG: Extracted from slide.rs:77-148, initial creation

use crate::notifications::functions::fnc_slide_offscreen_position::slide_offscreen_position;
use crate::notifications::functions::fnc_slide_resolve_direction::resolve_slide_direction;
use crate::notifications::types::{Anchor, AnimationPhase, SlideDirection};
use crate::shared_utils::math::lerp;
use ratatui::prelude::Rect;

/// Calculates the visible rectangle during slide animation.
///
/// This function interpolates between the start and end positions based on
/// animation progress, then clips the result to the visible frame area.
///
/// # Arguments
///
/// * `full_rect` - The full rectangle of the notification when fully visible
/// * `frame_area` - The visible frame area
/// * `progress` - Animation progress (0.0 to 1.0)
/// * `phase` - Current animation phase
/// * `anchor` - The anchor position of the notification
/// * `slide_direction` - The configured slide direction
/// * `custom_slide_in_start_pos` - Optional custom starting position for slide-in
/// * `custom_slide_out_end_pos` - Optional custom ending position for slide-out
///
/// # Returns
///
/// The visible rectangle at the current animation progress, clipped to frame bounds
///
/// # Examples
///
/// ```
/// use ratatui::prelude::Rect;
/// use ratatui_notifications::notifications::functions::fnc_slide_calculate_rect::slide_calculate_rect;
/// use ratatui_notifications::notifications::types::{Anchor, AnimationPhase, SlideDirection};
///
/// let full_rect = Rect::new(100, 25, 10, 5);
/// let frame_area = Rect::new(0, 0, 120, 50);
/// let rect = slide_calculate_rect(
///     full_rect,
///     frame_area,
///     1.0, // Full progress
///     AnimationPhase::SlidingIn,
///     Anchor::MiddleRight,
///     SlideDirection::FromRight,
///     None,
///     None,
/// );
/// assert_eq!(rect, full_rect); // Should be fully visible
/// ```
#[allow(clippy::too_many_arguments)]
pub fn slide_calculate_rect(
    full_rect: Rect,
    frame_area: Rect,
    progress: f32,
    phase: AnimationPhase,
    anchor: Anchor,
    slide_direction: SlideDirection,
    custom_slide_in_start_pos: Option<(f32, f32)>,
    custom_slide_out_end_pos: Option<(f32, f32)>,
) -> Rect {
    let progress = progress.clamp(0.0, 1.0);

    let (start_x_f32, start_y_f32, end_x_f32, end_y_f32) = match phase {
        AnimationPhase::SlidingIn => {
            let (sx, sy) = custom_slide_in_start_pos.unwrap_or_else(|| {
                let dir = resolve_slide_direction(slide_direction, anchor);
                slide_offscreen_position(anchor, dir, full_rect, frame_area)
            });
            (sx, sy, full_rect.x as f32, full_rect.y as f32)
        }
        AnimationPhase::SlidingOut => {
            let (ex, ey) = custom_slide_out_end_pos.unwrap_or_else(|| {
                let dir = resolve_slide_direction(slide_direction, anchor);
                slide_offscreen_position(anchor, dir, full_rect, frame_area)
            });
            (full_rect.x as f32, full_rect.y as f32, ex, ey)
        }
        _ => return full_rect,
    };

    let current_x_f32 = lerp(start_x_f32, end_x_f32, progress);
    let current_y_f32 = lerp(start_y_f32, end_y_f32, progress);

    let anim_x1 = current_x_f32;
    let anim_y1 = current_y_f32;
    let anim_x2 = current_x_f32 + full_rect.width as f32;
    let anim_y2 = current_y_f32 + full_rect.height as f32;
    let frame_x1 = frame_area.x as f32;
    let frame_y1 = frame_area.y as f32;
    let frame_x2 = frame_area.right() as f32;
    let frame_y2 = frame_area.bottom() as f32;
    let intersect_x1 = anim_x1.max(frame_x1);
    let intersect_y1 = anim_y1.max(frame_y1);
    let intersect_x2 = anim_x2.min(frame_x2);
    let intersect_y2 = anim_y2.min(frame_y2);
    let intersect_width = (intersect_x2 - intersect_x1).max(0.0);
    let intersect_height = (intersect_y2 - intersect_y1).max(0.0);

    let final_x = intersect_x1.round() as u16;
    let final_y = intersect_y1.round() as u16;
    let final_width = intersect_width.round() as u16;
    let final_height = intersect_height.round() as u16;

    let final_rect = Rect {
        x: final_x,
        y: final_y,
        width: final_width.min(frame_area.width.saturating_sub(final_x)),
        height: final_height.min(frame_area.height.saturating_sub(final_y)),
    };

    if final_rect.width > 0 && final_rect.height > 0 {
        final_rect
    } else {
        Rect::default()
    }
}

// FILE: src/notifications/functions/fnc_slide_calculate_rect.rs - Calculates visible rect during slide animation
// END OF VERSION: 1.0.0
