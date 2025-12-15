// FILE: src/notifications/types/slide_direction.rs - Slide direction enum
// VERSION: 1.0.0
// WCTX: OFPF migration
// CLOG: Initial creation

/// Direction from which a notification slides in.
///
/// Used with the `Slide` animation type to control the entry direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SlideDirection {
    /// Auto-select direction based on anchor point (default).
    ///
    /// For example, `BottomRight` anchor will slide from the right,
    /// `TopLeft` anchor will slide from the left, etc.
    #[default]
    Default,

    /// Slide in from the top edge.
    FromTop,

    /// Slide in from the bottom edge.
    FromBottom,

    /// Slide in from the left edge.
    FromLeft,

    /// Slide in from the right edge.
    FromRight,

    /// Slide in diagonally from top-left corner.
    FromTopLeft,

    /// Slide in diagonally from top-right corner.
    FromTopRight,

    /// Slide in diagonally from bottom-left corner.
    FromBottomLeft,

    /// Slide in diagonally from bottom-right corner.
    FromBottomRight,
}

// FILE: src/notifications/types/slide_direction.rs - Slide direction enum
// END OF VERSION: 1.0.0
