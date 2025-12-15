// FILE: src/notifications/functions/fnc_calculate_rect.rs - Calculate notification rectangle from anchor and size
// VERSION: 1.0.0
// WCTX: Implementing layout functions for ratatui-notifications using TDD
// CLOG: Initial creation - calculates final rect position with anchor alignment and exterior padding

use crate::notifications::types::Anchor;
use ratatui::layout::{Position, Rect};

/// Calculate the final rectangular area for a notification.
///
/// Given an anchor point, anchor position, dimensions, frame area, and exterior padding,
/// this function calculates where the notification rectangle should be placed, accounting for:
/// - Anchor alignment (TopLeft, Center, BottomRight, etc.)
/// - Exterior padding (offset from screen edges)
/// - Frame boundary clamping (ensures rect stays within frame)
///
/// # Arguments
///
/// * `anchor` - The anchor type (determines alignment behavior)
/// * `anchor_pos` - The position of the anchor point
/// * `width` - Desired width of the notification
/// * `height` - Desired height of the notification
/// * `frame_area` - The frame/screen area to place the notification within
/// * `exterior_padding` - Padding from screen edges (in cells)
///
/// # Returns
///
/// A `Rect` representing the final position and size of the notification, clamped to frame bounds.
///
/// # Examples
///
/// ```
/// use ratatui::layout::{Position, Rect};
/// use ratatui_notifications::notifications::types::Anchor;
/// use ratatui_notifications::notifications::functions::fnc_calculate_rect::calculate_rect;
///
/// let frame = Rect::new(0, 0, 100, 50);
/// let anchor_pos = Position::new(0, 0);
/// let rect = calculate_rect(Anchor::TopLeft, anchor_pos, 20, 10, frame, 2);
/// // Rect will be at (2, 2) with exterior padding of 2
/// ```
pub fn calculate_rect(
    anchor: Anchor,
    anchor_pos: Position,
    width: u16,
    height: u16,
    frame_area: Rect,
    exterior_padding: u16,
) -> Rect {
    let mut x = anchor_pos.x;
    let mut y = anchor_pos.y;

    // Adjust x based on horizontal anchor alignment
    match anchor {
        Anchor::TopCenter | Anchor::MiddleCenter | Anchor::BottomCenter => {
            // Center-aligned: offset by half width
            x = x.saturating_sub(width / 2);
        }
        Anchor::TopRight | Anchor::MiddleRight | Anchor::BottomRight => {
            // Right-aligned: offset by width minus 1
            x = x.saturating_sub(width.saturating_sub(1));
        }
        _ => {}
    }

    // Adjust y based on vertical anchor alignment
    match anchor {
        Anchor::MiddleLeft | Anchor::MiddleCenter | Anchor::MiddleRight => {
            // Middle-aligned: offset by half height
            y = y.saturating_sub(height / 2);
        }
        Anchor::BottomLeft | Anchor::BottomCenter | Anchor::BottomRight => {
            // Bottom-aligned: offset by height minus 1
            y = y.saturating_sub(height.saturating_sub(1));
        }
        _ => {}
    }

    // Apply exterior padding based on anchor position
    match anchor {
        Anchor::TopLeft => {
            x = x.saturating_add(exterior_padding);
            y = y.saturating_add(exterior_padding);
        }
        Anchor::TopCenter => {
            y = y.saturating_add(exterior_padding);
        }
        Anchor::TopRight => {
            x = x.saturating_sub(exterior_padding);
            y = y.saturating_add(exterior_padding);
        }
        Anchor::MiddleLeft => {
            x = x.saturating_add(exterior_padding);
        }
        Anchor::MiddleCenter => {
            // No padding for center
        }
        Anchor::MiddleRight => {
            x = x.saturating_sub(exterior_padding);
        }
        Anchor::BottomLeft => {
            x = x.saturating_add(exterior_padding);
            y = y.saturating_sub(exterior_padding);
        }
        Anchor::BottomCenter => {
            y = y.saturating_sub(exterior_padding);
        }
        Anchor::BottomRight => {
            x = x.saturating_sub(exterior_padding);
            y = y.saturating_sub(exterior_padding);
        }
    }

    // Clamp dimensions to frame size
    let clamped_width = width.min(frame_area.width);
    let clamped_height = height.min(frame_area.height);

    // Clamp position to frame bounds
    let final_x = x
        .max(frame_area.x)
        .min(frame_area.right().saturating_sub(clamped_width));

    let final_y = y
        .max(frame_area.y)
        .min(frame_area.bottom().saturating_sub(clamped_height));

    // Ensure we don't go negative (double-check frame bounds)
    let final_x = final_x.max(frame_area.x);
    let final_y = final_y.max(frame_area.y);

    Rect::new(final_x, final_y, clamped_width, clamped_height)
}

// FILE: src/notifications/functions/fnc_calculate_rect.rs - Calculate notification rectangle from anchor and size
// END OF VERSION: 1.0.0
