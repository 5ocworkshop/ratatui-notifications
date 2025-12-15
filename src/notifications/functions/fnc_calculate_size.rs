// FILE: src/notifications/functions/fnc_calculate_size.rs - Calculate notification size
// VERSION: 1.0.0
// WCTX: TDD implementation of deferred functions
// CLOG: Initial creation - extracts size calculation logic from layout.rs

use crate::notifications::classes::Notification;
use crate::notifications::types::SizeConstraint;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap};
use ratatui::buffer::{Buffer, Cell};

/// Calculates the size of a notification based on its content and constraints.
///
/// This function determines the width and height needed to display a notification,
/// taking into account borders, padding, content wrapping, and size constraints.
///
/// # Arguments
///
/// * `notification` - The notification to calculate size for
/// * `frame_area` - The available frame area (used for percentage constraints)
///
/// # Returns
///
/// A tuple `(width, height)` representing the calculated notification dimensions
///
/// # Examples
///
/// ```ignore
/// // Internal function - use through Notifications manager
/// use ratatui::prelude::*;
/// use ratatui_notifications::notifications::classes::Notification;
/// use ratatui_notifications::notifications::functions::fnc_calculate_size::calculate_size;
///
/// let notification = Notification::default();
/// let frame_area = Rect::new(0, 0, 100, 50);
/// let (width, height) = calculate_size(&notification, frame_area);
/// ```
pub fn calculate_size(notification: &Notification, frame_area: Rect) -> (u16, u16) {
    // 1. Get border dimensions based on border_type
    let border_v_offset = match notification.border_type {
        Some(BorderType::Double) => 2,
        Some(_) => 2,
        None => 0,
    };
    let border_h_offset = match notification.border_type {
        Some(BorderType::Double) => 2,
        Some(_) => 2,
        None => 0,
    };

    // 2. Get padding dimensions
    let h_padding = notification.padding.left + notification.padding.right;
    let v_padding = notification.padding.top + notification.padding.bottom;

    // 3. Calculate minimum size (at least 3x3)
    let min_width = (1 + h_padding + border_h_offset).max(3);
    let min_height = (1 + v_padding + border_v_offset).max(3);

    // 4. Apply max_width constraint (Percentage or Absolute)
    let max_width_constraint = notification
        .max_width
        .map(|c| match c {
            SizeConstraint::Absolute(w) => w.min(frame_area.width),
            SizeConstraint::Percentage(p) => {
                ((frame_area.width as f32 * p.clamp(0.0, 1.0)) as u16).max(1)
            }
        })
        .unwrap_or(frame_area.width)
        .max(min_width);

    // 5. Calculate intrinsic width from content
    let content_max_line_width = notification
        .content
        .lines
        .iter()
        .map(|l| l.width())
        .max()
        .unwrap_or(0) as u16;

    let title_width = notification.title.as_ref().map_or(0, |t| t.width()) as u16;

    let intrinsic_width =
        (content_max_line_width.max(title_width) + border_h_offset + h_padding).max(min_width);

    let final_width = intrinsic_width.min(max_width_constraint);

    // 6. Apply max_height constraint
    let max_height_constraint = notification
        .max_height
        .map(|c| match c {
            SizeConstraint::Absolute(h) => h.min(frame_area.height),
            SizeConstraint::Percentage(p) => {
                ((frame_area.height as f32 * p.clamp(0.0, 1.0)) as u16).max(1)
            }
        })
        .unwrap_or(frame_area.height)
        .max(min_height);

    // 7. Render content to buffer to measure actual height with wrapping
    let mut temp_block = Block::default();
    if let Some(border_type) = notification.border_type {
        temp_block = temp_block.borders(Borders::ALL).border_type(border_type);
    }
    if let Some(title) = &notification.title {
        temp_block = temp_block.title(title.clone());
    }
    temp_block = temp_block.padding(notification.padding);

    let temp_paragraph = Paragraph::new(notification.content.clone())
        .wrap(Wrap { trim: true })
        .block(temp_block);

    let buffer_height = max_height_constraint;
    let mut buffer = Buffer::empty(Rect::new(0, 0, final_width, buffer_height));
    temp_paragraph.render(buffer.area, &mut buffer);

    let default_cell = Cell::default();
    let measured_height = buffer
        .content
        .iter()
        .enumerate()
        .filter(|(_, cell)| *cell != &default_cell)
        .map(|(idx, _)| buffer.pos_of(idx).1)
        .max()
        .map_or(0, |row_index| row_index + 1);

    // 8. Return (width, height) tuple
    let final_height = measured_height.max(min_height).min(max_height_constraint);
    (final_width, final_height)
}

// FILE: src/notifications/functions/fnc_calculate_size.rs - Calculate notification size
// END OF VERSION: 1.0.0
