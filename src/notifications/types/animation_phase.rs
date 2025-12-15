// FILE: src/notifications/types/animation_phase.rs - Animation phase enum
// VERSION: 1.1.0
// WCTX: OFPF migration
// CLOG: Made public for animation function testing

/// Animation phase tracking.
///
/// Represents the current stage of a notification's lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationPhase {
    #[default]
    Pending,
    SlidingIn,
    Expanding,
    FadingIn,
    Dwelling,
    SlidingOut,
    Collapsing,
    FadingOut,
    Finished,
}

// FILE: src/notifications/types/animation_phase.rs - Animation phase enum
// END OF VERSION: 1.1.0
