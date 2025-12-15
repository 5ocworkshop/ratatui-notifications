// FILE: src/notifications/types/anchor.rs - Screen anchor position enum
// VERSION: 1.0.0
// WCTX: OFPF migration
// CLOG: Initial creation

/// Screen position from which notifications expand.
///
/// Notifications are anchored to a corner or edge of the screen and expand
/// outward from that anchor point. For example, `BottomRight` means notifications
/// appear in the bottom-right corner and stack upward/leftward.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    /// Default anchor position. Notifications expand from bottom-right.
    #[default]
    BottomRight,
}

// FILE: src/notifications/types/anchor.rs - Screen anchor position enum
// END OF VERSION: 1.0.0
