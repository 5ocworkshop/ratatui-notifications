// FILE: src/notifications/classes/mod.rs - Classes module
// VERSION: 1.1.0
// WCTX: OFPF migration - Notification class
// CLOG: Export Notification and NotificationBuilder publicly

pub(crate) mod cls_notification;
pub(crate) mod cls_notification_state;

// Public exports
pub use cls_notification::{Notification, NotificationBuilder};

// Internal exports
pub(crate) use cls_notification_state::{NotificationState, ManagerDefaults};

// FILE: src/notifications/classes/mod.rs - Classes module
// END OF VERSION: 1.1.0
