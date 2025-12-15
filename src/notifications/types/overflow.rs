// FILE: src/notifications/types/overflow.rs - Notification overflow behavior enum
// VERSION: 1.0.0
// WCTX: OFPF migration
// CLOG: Initial creation

/// Behavior when notification limit is reached.
///
/// Determines which notification to discard when the maximum number
/// of concurrent notifications is exceeded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Overflow {
    /// Discard the oldest notification when limit is reached (default).
    #[default]
    DiscardOldest,

    /// Discard the newest notification when limit is reached.
    DiscardNewest,
}

// FILE: src/notifications/types/overflow.rs - Notification overflow behavior enum
// END OF VERSION: 1.0.0
