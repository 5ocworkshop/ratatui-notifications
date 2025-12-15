// FILE: src/notifications/mod.rs - Notifications module
// VERSION: 1.7.0
// WCTX: Adding code generation feature
// CLOG: Added generate_code re-export

pub mod types;
pub mod functions;
pub(crate) mod classes;
pub mod orc_stacking;
pub mod orc_render;
pub mod orc_manager;

// Re-export main types for convenient access
pub use classes::{Notification, NotificationBuilder};
pub use orc_manager::Notifications;
pub use types::{
    Anchor, Animation, AnimationPhase, AutoDismiss, Level,
    NotificationError, Overflow, SlideDirection, SizeConstraint, Timing,
};

// Re-export layout utilities for custom positioning
pub use functions::fnc_calculate_anchor_position::calculate_anchor_position;
pub use functions::fnc_calculate_rect::calculate_rect;
pub use functions::fnc_calculate_size::calculate_size;

// Re-export code generation utility
pub use functions::fnc_generate_code::generate_code;

// FILE: src/notifications/mod.rs - Notifications module
// END OF VERSION: 1.7.0
