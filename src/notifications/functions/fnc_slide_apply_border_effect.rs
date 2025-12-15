// FILE: src/notifications/functions/fnc_slide_apply_border_effect.rs - Applies vanishing edge border effect during slide animation
// VERSION: 1.0.0
// WCTX: Implementing slide animation functions with TDD
// CLOG: Extracted from slide.rs:150-448, initial creation

use crate::notifications::functions::fnc_slide_offscreen_position::slide_offscreen_position;
use crate::notifications::functions::fnc_slide_resolve_direction::resolve_slide_direction;
use crate::notifications::types::{Anchor, AnimationPhase, SlideDirection};
use ratatui::{prelude::*, symbols::border, widgets::Block};

/// Applies vanishing edge effect to block borders during slide animation.
///
/// This function modifies the block's border symbols to create a "vanishing edge"
/// effect when the notification slides past the frame boundary. The edge that
/// crosses the boundary has its symbols replaced to appear as if it's disappearing.
///
/// # Arguments
///
/// * `block` - The base block to modify
/// * `anchor` - The anchor position of the notification
/// * `slide_direction_cfg` - The configured slide direction
/// * `progress` - Animation progress (0.0 to 1.0)
/// * `phase` - Current animation phase
/// * `full_rect` - The full rectangle of the notification
/// * `custom_slide_in_start_pos` - Optional custom starting position for slide-in
/// * `custom_slide_out_end_pos` - Optional custom ending position for slide-out
/// * `frame_area` - The visible frame area
/// * `base_set` - The base border symbol set
///
/// # Returns
///
/// The modified block with border effect applied
///
/// # Examples
///
/// ```ignore
/// use ratatui::prelude::*;
/// use ratatui::symbols::border;
/// use ratatui::widgets::{Block, BorderType, Borders};
/// use ratatui_notifications::notifications::functions::fnc_slide_apply_border_effect::slide_apply_border_effect;
/// use ratatui_notifications::notifications::types::{Anchor, AnimationPhase, SlideDirection};
///
/// let full_rect = Rect::new(90, 25, 20, 10);
/// let frame_area = Rect::new(0, 0, 100, 50);
/// let base_block = Block::default()
///     .borders(Borders::ALL)
///     .border_type(BorderType::Rounded);
/// let base_set = border::ROUNDED;
///
/// let result = slide_apply_border_effect(
///     base_block,
///     Anchor::MiddleRight,
///     SlideDirection::FromRight,
///     0.9,
///     AnimationPhase::SlidingOut,
///     full_rect,
///     None,
///     None,
///     frame_area,
///     &base_set,
/// );
/// ```
#[allow(clippy::too_many_arguments)]
pub fn slide_apply_border_effect<'a>(
    block: Block<'a>,
    anchor: Anchor,
    slide_direction_cfg: SlideDirection,
    progress: f32,
    phase: AnimationPhase,
    full_rect: Rect,
    custom_slide_in_start_pos: Option<(f32, f32)>,
    custom_slide_out_end_pos: Option<(f32, f32)>,
    frame_area: Rect,
    base_set: &border::Set,
) -> Block<'a> {
    const PROGRESS_OFFSET: f32 = 0.0;

    if full_rect.width == 0 || full_rect.height == 0 {
        return block;
    }

    let slide_direction = resolve_slide_direction(slide_direction_cfg, anchor);

    let (actual_start_x, actual_start_y, actual_end_x, actual_end_y) = match phase {
        AnimationPhase::SlidingIn => {
            let (sx, sy) = custom_slide_in_start_pos.unwrap_or_else(|| {
                slide_offscreen_position(anchor, slide_direction, full_rect, frame_area)
            });
            (sx, sy, full_rect.x as f32, full_rect.y as f32)
        }
        AnimationPhase::SlidingOut => {
            let (ex, ey) = custom_slide_out_end_pos.unwrap_or_else(|| {
                slide_offscreen_position(anchor, slide_direction, full_rect, frame_area)
            });
            (full_rect.x as f32, full_rect.y as f32, ex, ey)
        }
        _ => return block,
    };

    let frame_x1 = frame_area.x as f32;
    let frame_y1 = frame_area.y as f32;
    let frame_x2 = frame_area.right() as f32;
    let frame_y2 = frame_area.bottom() as f32;
    let width = full_rect.width as f32;
    let height = full_rect.height as f32;

    let (trigger_start, trigger_end) = match slide_direction {
        SlideDirection::FromRight => {
            let crosses = actual_start_x + width > frame_x2 || actual_end_x + width > frame_x2;
            let trigger_s = if !crosses {
                2.0
            } else {
                let travel_dist = actual_end_x - actual_start_x;
                if travel_dist <= 0.0 {
                    0.0
                } else {
                    let required_pos = frame_x2 - width;
                    let dist_to_reach = required_pos - actual_start_x;
                    if dist_to_reach <= 0.0 {
                        0.0
                    } else {
                        (dist_to_reach / travel_dist).clamp(0.0, 1.0)
                    }
                }
            };
            let trigger_e = if !crosses {
                0.0
            } else {
                let travel_dist = actual_end_x - actual_start_x;
                if travel_dist >= 0.0 {
                    1.0
                } else {
                    let required_pos_left_edge = frame_x2 - width - actual_start_x;
                    if required_pos_left_edge >= 0.0 {
                        1.0
                    } else {
                        (required_pos_left_edge / travel_dist).clamp(0.0, 1.0)
                    }
                }
            };
            (trigger_s, trigger_e)
        }
        SlideDirection::FromLeft => {
            let crosses = actual_start_x < frame_x1 || actual_end_x < frame_x1;
            let trigger_s = if !crosses {
                2.0
            } else {
                let travel_dist = actual_end_x - actual_start_x;
                if travel_dist >= 0.0 {
                    0.0
                } else {
                    let required_pos = frame_x1;
                    let dist_to_reach = required_pos - actual_start_x;
                    if dist_to_reach >= 0.0 {
                        0.0
                    } else {
                        (dist_to_reach / travel_dist).clamp(0.0, 1.0)
                    }
                }
            };
            let trigger_e = if !crosses {
                0.0
            } else {
                let travel_dist = actual_end_x - actual_start_x;
                if travel_dist <= 0.0 {
                    1.0
                } else {
                    let required_pos_left_edge = frame_x1 - actual_start_x;
                    if required_pos_left_edge <= 0.0 {
                        1.0
                    } else {
                        (required_pos_left_edge / travel_dist).clamp(0.0, 1.0)
                    }
                }
            };
            (trigger_s, trigger_e)
        }
        SlideDirection::FromTop => {
            let crosses = actual_start_y < frame_y1 || actual_end_y < frame_y1;
            if crosses {
                (0.0, 1.0)
            } else {
                (2.0, 0.0)
            }
        }
        SlideDirection::FromBottom => {
            let crosses = actual_start_y + height > frame_y2 || actual_end_y + height > frame_y2;
            if crosses {
                (0.0, 1.0)
            } else {
                (2.0, 0.0)
            }
        }
        SlideDirection::FromTopLeft => {
            let cx = actual_start_x < frame_x1 || actual_end_x < frame_x1;
            let cy = actual_start_y < frame_y1 || actual_end_y < frame_y1;
            if cx || cy {
                (0.0, 1.0)
            } else {
                (2.0, 0.0)
            }
        }
        SlideDirection::FromTopRight => {
            let cx = actual_start_x + width > frame_x2 || actual_end_x + width > frame_x2;
            let cy = actual_start_y < frame_y1 || actual_end_y < frame_y1;
            if cx || cy {
                (0.0, 1.0)
            } else {
                (2.0, 0.0)
            }
        }
        SlideDirection::FromBottomLeft => {
            let cx = actual_start_x < frame_x1 || actual_end_x < frame_x1;
            let cy = actual_start_y + height > frame_y2 || actual_end_y + height > frame_y2;
            if cx || cy {
                (0.0, 1.0)
            } else {
                (2.0, 0.0)
            }
        }
        SlideDirection::FromBottomRight => {
            let cx = actual_start_x + width > frame_x2 || actual_end_x + width > frame_x2;
            let cy = actual_start_y + height > frame_y2 || actual_end_y + height > frame_y2;
            if cx || cy {
                (0.0, 1.0)
            } else {
                (2.0, 0.0)
            }
        }
        SlideDirection::Default => (2.0, 0.0),
    };

    let apply_effect = match phase {
        AnimationPhase::SlidingIn => progress < trigger_end - PROGRESS_OFFSET,
        AnimationPhase::SlidingOut => progress >= trigger_start - PROGRESS_OFFSET,
        _ => false,
    };

    if !apply_effect {
        return block;
    }

    let (
        top_left_mod,
        top_right_mod,
        bottom_left_mod,
        bottom_right_mod,
        vertical_left_mod,
        vertical_right_mod,
        horizontal_top_mod,
        horizontal_bottom_mod,
    ) = match slide_direction {
        SlideDirection::FromRight => (
            None,
            Some(base_set.horizontal_top),
            None,
            Some(base_set.horizontal_bottom),
            None,
            Some(" "),
            None,
            None,
        ),
        SlideDirection::FromLeft => (
            Some(base_set.horizontal_top),
            None,
            Some(base_set.horizontal_bottom),
            None,
            Some(" "),
            None,
            None,
            None,
        ),
        SlideDirection::FromTop => (
            Some(base_set.vertical_left),
            Some(base_set.vertical_right),
            None,
            None,
            None,
            None,
            Some(" "),
            None,
        ),
        SlideDirection::FromBottom => (
            None,
            None,
            Some(base_set.vertical_left),
            Some(base_set.vertical_right),
            None,
            None,
            None,
            Some(" "),
        ),
        SlideDirection::FromTopLeft => (
            Some(base_set.horizontal_top),
            Some(base_set.vertical_right),
            Some(base_set.horizontal_bottom),
            None,
            Some(" "),
            None,
            Some(" "),
            None,
        ),
        SlideDirection::FromTopRight => (
            Some(base_set.vertical_left),
            Some(base_set.horizontal_top),
            None,
            Some(base_set.horizontal_bottom),
            None,
            Some(" "),
            Some(" "),
            None,
        ),
        SlideDirection::FromBottomLeft => (
            Some(base_set.horizontal_top),
            None,
            Some(base_set.vertical_left),
            Some(base_set.vertical_right),
            Some(" "),
            None,
            None,
            Some(" "),
        ),
        SlideDirection::FromBottomRight => (
            None,
            Some(base_set.horizontal_top),
            Some(base_set.vertical_left),
            Some(base_set.horizontal_bottom),
            None,
            Some(" "),
            None,
            Some(" "),
        ),
        _ => return block,
    };

    let custom_set = border::Set {
        top_left: top_left_mod.unwrap_or(base_set.top_left),
        top_right: top_right_mod.unwrap_or(base_set.top_right),
        bottom_left: bottom_left_mod.unwrap_or(base_set.bottom_left),
        bottom_right: bottom_right_mod.unwrap_or(base_set.bottom_right),
        vertical_left: vertical_left_mod.unwrap_or(base_set.vertical_left),
        vertical_right: vertical_right_mod.unwrap_or(base_set.vertical_right),
        horizontal_top: horizontal_top_mod.unwrap_or(base_set.horizontal_top),
        horizontal_bottom: horizontal_bottom_mod.unwrap_or(base_set.horizontal_bottom),
    };
    block.border_set(custom_set)
}

// FILE: src/notifications/functions/fnc_slide_apply_border_effect.rs - Applies vanishing edge border effect during slide animation
// END OF VERSION: 1.0.0
