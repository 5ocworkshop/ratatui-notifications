// FILE: src/notifications/types/level.rs - Notification severity level enum
// VERSION: 1.0.0
// WCTX: OFPF migration
// CLOG: Initial creation

/// Severity level of a notification.
///
/// Affects the visual styling of the notification (colors, borders).
/// Higher severity levels typically use more prominent colors to draw attention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Level {
    /// Informational message (default).
    #[default]
    Info,

    /// Warning message.
    Warn,

    /// Error message.
    Error,

    /// Debug message.
    Debug,

    /// Trace message.
    Trace,
}

// FILE: src/notifications/types/level.rs - Notification severity level enum
// END OF VERSION: 1.0.0
