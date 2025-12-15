// FILE: src/notifications/types/timing.rs - Animation timing enum
// VERSION: 1.0.0
// WCTX: OFPF migration
// CLOG: Initial creation

use std::time::Duration;

/// Animation duration specification.
///
/// Controls whether animation durations are explicitly specified or
/// automatically calculated based on content or system defaults.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Timing {
    /// Fixed duration specified by user.
    Fixed(Duration),

    /// Automatically calculated duration.
    ///
    /// Duration may be based on content length, animation type,
    /// or system-wide defaults.
    #[default]
    Auto,
}

// FILE: src/notifications/types/timing.rs - Animation timing enum
// END OF VERSION: 1.0.0
