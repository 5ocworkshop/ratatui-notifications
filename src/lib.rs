// FILE: src/lib.rs - Ratatui Notifications library root
// VERSION: 2.1.0
// WCTX: Adding code generation feature
// CLOG: Added generate_code export

//! # Ratatui Notifications
//!
//! Animated notification widgets for [ratatui](https://ratatui.rs) terminal applications.
//!
//! This library provides a flexible notification system with multiple animation styles,
//! customizable appearance, and automatic stacking of multiple notifications.
//!
//! ## Features
//!
//! - **Multiple animation styles**: Slide, Fade, Expand
//! - **Flexible anchoring**: Position notifications at any corner or edge of the screen
//! - **Auto-dismiss**: Configurable automatic dismissal with countdown indicators
//! - **Stacking**: Automatically manages multiple notifications without overlap
//! - **Customizable appearance**: Icons, colors, borders, and styling
//! - **Level-based styling**: Info, Success, Warning, Error with distinct visual cues
//!
//! ## Quick Start
//!
//! ```no_run
//! use ratatui_notifications::{Notification, Notifications, Level, Anchor};
//! use std::time::Duration;
//!
//! // Create the notification manager
//! let mut notifications = Notifications::new();
//!
//! // Add a notification
//! let notif = Notification::new("Operation completed!")
//!     .title("Success")
//!     .level(Level::Info)
//!     .anchor(Anchor::BottomRight)
//!     .build()
//!     .unwrap();
//!
//! notifications.add(notif).unwrap();
//!
//! // In your render loop:
//! // notifications.tick(Duration::from_millis(16));
//! // notifications.render(&mut frame, frame.area());
//! ```
//!
//! ## Examples
//!
//! ### Different Animation Styles
//!
//! ```no_run
//! use ratatui_notifications::{Notification, Animation, SlideDirection, Level};
//!
//! // Slide animation (direction set via slide_direction)
//! let slide_notif = Notification::new("Sliding in!")
//!     .animation(Animation::Slide)
//!     .slide_direction(SlideDirection::FromRight)
//!     .build()
//!     .unwrap();
//!
//! // Fade animation
//! let fade_notif = Notification::new("Fading in...")
//!     .animation(Animation::Fade)
//!     .build()
//!     .unwrap();
//!
//! // Expand/collapse animation
//! let expand_notif = Notification::new("Expanding!")
//!     .animation(Animation::ExpandCollapse)
//!     .build()
//!     .unwrap();
//! ```
//!
//! ### Auto-dismiss with Countdown
//!
//! ```no_run
//! use ratatui_notifications::{Notification, AutoDismiss};
//! use std::time::Duration;
//!
//! let notif = Notification::new("This will disappear...")
//!     .auto_dismiss(AutoDismiss::After(Duration::from_secs(5)))
//!     .build()
//!     .unwrap();
//! ```
//!
//! ### Custom Positioning
//!
//! ```no_run
//! use ratatui_notifications::{Notification, Anchor};
//!
//! // Top-left corner
//! let top_left = Notification::new("Top left")
//!     .anchor(Anchor::TopLeft)
//!     .build()
//!     .unwrap();
//!
//! // Bottom center
//! let bottom_center = Notification::new("Bottom center")
//!     .anchor(Anchor::BottomCenter)
//!     .build()
//!     .unwrap();
//! ```

pub mod notifications;
pub(crate) mod shared_utils;

// Re-export public API at crate root for ergonomic imports
pub use notifications::{
    // Core types
    Notification,
    NotificationBuilder,
    Notifications,

    // Configuration enums
    Anchor,
    Animation,
    AutoDismiss,
    Level,
    Overflow,
    SizeConstraint,
    SlideDirection,
    Timing,

    // Error type
    NotificationError,

    // Layout utilities (for custom positioning)
    calculate_anchor_position,
    calculate_rect,
    calculate_size,

    // Code generation utility
    generate_code,
};

// Re-export ratatui Position for custom positioning
pub use ratatui::layout::Position;

// FILE: src/lib.rs - Ratatui Notifications library root
// END OF VERSION: 2.1.0
