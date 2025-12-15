// FILE: src/notifications/types/auto_dismiss.rs - Auto-dismiss behavior enum
// VERSION: 1.0.0
// WCTX: OFPF migration
// CLOG: Initial creation

use std::time::Duration;

/// Controls automatic dismissal of notifications.
///
/// Determines whether a notification will automatically dismiss after
/// a specified duration or remain visible until manually dismissed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoDismiss {
    /// Notification remains visible until manually dismissed.
    Never,

    /// Notification automatically dismisses after the specified duration.
    After(Duration),
}

impl Default for AutoDismiss {
    fn default() -> Self {
        Self::After(Duration::from_secs(4))
    }
}

// FILE: src/notifications/types/auto_dismiss.rs - Auto-dismiss behavior enum
// END OF VERSION: 1.0.0
