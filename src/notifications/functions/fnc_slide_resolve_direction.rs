// FILE: src/notifications/functions/fnc_slide_resolve_direction.rs - Resolves slide direction based on config and anchor
// VERSION: 1.0.0
// WCTX: Implementing slide animation functions with TDD
// CLOG: Extracted from slide.rs:19-34, initial creation

use crate::notifications::types::{Anchor, SlideDirection};

/// Resolves the actual slide direction based on config and anchor.
///
/// When `SlideDirection::Default` is specified, this function determines
/// the appropriate slide direction based on the anchor position. For example,
/// a `TopLeft` anchor defaults to `FromTopLeft`, while `MiddleRight` defaults
/// to `FromRight`.
///
/// # Arguments
///
/// * `direction` - The configured slide direction (may be Default)
/// * `anchor` - The anchor position of the notification
///
/// # Returns
///
/// The resolved slide direction (never Default)
///
/// # Examples
///
/// ```
/// use ratatui_notifications::notifications::functions::fnc_slide_resolve_direction::resolve_slide_direction;
/// use ratatui_notifications::notifications::types::{Anchor, SlideDirection};
///
/// // Non-default direction returns unchanged
/// assert_eq!(
///     resolve_slide_direction(SlideDirection::FromLeft, Anchor::TopRight),
///     SlideDirection::FromLeft
/// );
///
/// // Default direction is resolved based on anchor
/// assert_eq!(
///     resolve_slide_direction(SlideDirection::Default, Anchor::TopLeft),
///     SlideDirection::FromTopLeft
/// );
/// ```
pub fn resolve_slide_direction(direction: SlideDirection, anchor: Anchor) -> SlideDirection {
    if direction != SlideDirection::Default {
        return direction;
    }
    match anchor {
        Anchor::TopLeft => SlideDirection::FromTopLeft,
        Anchor::TopCenter => SlideDirection::FromTop,
        Anchor::TopRight => SlideDirection::FromTopRight,
        Anchor::MiddleLeft => SlideDirection::FromLeft,
        Anchor::MiddleCenter => SlideDirection::FromLeft,
        Anchor::MiddleRight => SlideDirection::FromRight,
        Anchor::BottomLeft => SlideDirection::FromBottomLeft,
        Anchor::BottomCenter => SlideDirection::FromBottom,
        Anchor::BottomRight => SlideDirection::FromBottomRight,
    }
}

// FILE: src/notifications/functions/fnc_slide_resolve_direction.rs - Resolves slide direction based on config and anchor
// END OF VERSION: 1.0.0
