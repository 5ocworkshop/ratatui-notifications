// FILE: src/notifications/types/mod.rs - Module declarations and re-exports for notification types
// VERSION: 1.1.0
// WCTX: OFPF migration
// CLOG: Made AnimationPhase public for animation function testing

mod anchor;
mod animation;
mod animation_phase;
mod auto_dismiss;
mod error;
mod level;
mod overflow;
mod size_constraint;
mod slide_direction;
mod timing;

pub use anchor::Anchor;
pub use animation::Animation;
pub use animation_phase::AnimationPhase;
pub use auto_dismiss::AutoDismiss;
pub use error::NotificationError;
pub use level::Level;
pub use overflow::Overflow;
pub use size_constraint::SizeConstraint;
pub use slide_direction::SlideDirection;
pub use timing::Timing;

// FILE: src/notifications/types/mod.rs - Module declarations and re-exports for notification types
// END OF VERSION: 1.1.0
