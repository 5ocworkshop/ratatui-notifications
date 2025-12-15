// FILE: src/notifications/functions/fnc_generate_code.rs - Code generation for notifications
// VERSION: 1.0.0
// WCTX: Adding "show code" feature to demo
// CLOG: Initial creation

use std::time::Duration;

use ratatui::widgets::Padding;

use crate::notifications::classes::cls_notification::Notification;
use crate::notifications::types::{AutoDismiss, SizeConstraint, Timing};

/// Generates Rust code to recreate the given notification configuration.
///
/// Only includes non-default values in the output to keep code minimal.
/// The generated code uses the builder pattern and is formatted for readability.
///
/// # Arguments
///
/// * `notification` - The notification to generate code for
///
/// # Returns
///
/// A `String` containing valid Rust code that recreates the notification.
///
/// # Example
///
/// ```no_run
/// use ratatui_notifications::{Notification, generate_code, Level};
///
/// let notif = Notification::new("Hello!")
///     .level(Level::Error)
///     .build()
///     .unwrap();
///
/// let code = generate_code(&notif);
/// // code contains: Notification::builder("Hello!")\n    .level(Level::Error)\n    .build()
/// ```
pub fn generate_code(notification: &Notification) -> String {
    let defaults = Notification::default();
    let mut lines = Vec::new();

    // Start with builder and content
    let content_str = escape_string(&notification.content().to_string());
    lines.push(format!("Notification::builder(\"{}\")", content_str));

    // Title (no default - None)
    if let Some(title) = notification.title() {
        let title_str = escape_string(&title.to_string());
        lines.push(format!("    .title(\"{}\")", title_str));
    }

    // Level - default is Some(Level::Info)
    if notification.level() != defaults.level {
        if let Some(level) = notification.level() {
            lines.push(format!("    .level(Level::{:?})", level));
        }
    }

    // Anchor - default is BottomRight
    if notification.anchor() != defaults.anchor {
        lines.push(format!("    .anchor(Anchor::{:?})", notification.anchor()));
    }

    // Animation - default is Slide
    if notification.animation() != defaults.animation {
        lines.push(format!(
            "    .animation(Animation::{:?})",
            notification.animation()
        ));
    }

    // SlideDirection - default is Default
    if notification.slide_direction() != defaults.slide_direction {
        lines.push(format!(
            "    .slide_direction(SlideDirection::{:?})",
            notification.slide_direction()
        ));
    }

    // Timing - default is all Auto
    let timing_changed = notification.slide_in_timing() != defaults.slide_in_timing
        || notification.dwell_timing() != defaults.dwell_timing
        || notification.slide_out_timing() != defaults.slide_out_timing;

    if timing_changed {
        let slide_in = format_timing(notification.slide_in_timing());
        let dwell = format_timing(notification.dwell_timing());
        let slide_out = format_timing(notification.slide_out_timing());
        lines.push(format!("    .timing({}, {}, {})", slide_in, dwell, slide_out));
    }

    // AutoDismiss - default is After(4 secs)
    if notification.auto_dismiss() != defaults.auto_dismiss {
        lines.push(format!(
            "    .auto_dismiss({})",
            format_auto_dismiss(notification.auto_dismiss())
        ));
    }

    // MaxSize - default is Percentage(0.4), Percentage(0.2)
    let size_changed = notification.max_width() != defaults.max_width
        || notification.max_height() != defaults.max_height;

    if size_changed {
        if let (Some(w), Some(h)) = (notification.max_width(), notification.max_height()) {
            lines.push(format!(
                "    .max_size({}, {})",
                format_size_constraint(w),
                format_size_constraint(h)
            ));
        }
    }

    // Padding - default is Padding::horizontal(1)
    if notification.padding() != defaults.padding {
        lines.push(format!(
            "    .padding({})",
            format_padding(notification.padding())
        ));
    }

    // Margin - default is 0
    if notification.exterior_margin() != defaults.exterior_margin {
        lines.push(format!("    .margin({})", notification.exterior_margin()));
    }

    // BorderType - default is Some(BorderType::Rounded)
    if notification.border_type() != defaults.border_type {
        if let Some(bt) = notification.border_type() {
            lines.push(format!("    .border_type(BorderType::{:?})", bt));
        }
    }

    // Custom entry position - default is None
    if let Some(pos) = notification.custom_entry_position() {
        lines.push(format!("    .entry_position(Position::new({}, {}))", pos.x, pos.y));
    }

    // Custom exit position - default is None
    if let Some(pos) = notification.custom_exit_position() {
        lines.push(format!("    .exit_position(Position::new({}, {}))", pos.x, pos.y));
    }

    // Fade effect - default is false
    if notification.fade_effect() != defaults.fade_effect {
        lines.push(format!("    .fade({})", notification.fade_effect()));
    }

    // End with build()
    lines.push("    .build()".to_string());

    lines.join("\n")
}

/// Escapes a string for use in Rust code.
fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Formats a Timing value as Rust code.
fn format_timing(timing: Timing) -> String {
    match timing {
        Timing::Auto => "Timing::Auto".to_string(),
        Timing::Fixed(d) => format_duration_as_timing(d),
    }
}

/// Formats a Duration as Timing::Fixed code.
fn format_duration_as_timing(d: Duration) -> String {
    let millis = d.as_millis();
    if millis % 1000 == 0 {
        format!("Timing::Fixed(Duration::from_secs({}))", millis / 1000)
    } else {
        format!("Timing::Fixed(Duration::from_millis({}))", millis)
    }
}

/// Formats an AutoDismiss value as Rust code.
fn format_auto_dismiss(ad: AutoDismiss) -> String {
    match ad {
        AutoDismiss::Never => "AutoDismiss::Never".to_string(),
        AutoDismiss::After(d) => {
            let millis = d.as_millis();
            if millis % 1000 == 0 {
                format!("AutoDismiss::After(Duration::from_secs({}))", millis / 1000)
            } else {
                format!("AutoDismiss::After(Duration::from_millis({}))", millis)
            }
        }
    }
}

/// Formats a SizeConstraint as Rust code.
fn format_size_constraint(sc: SizeConstraint) -> String {
    match sc {
        SizeConstraint::Absolute(n) => format!("SizeConstraint::Absolute({})", n),
        SizeConstraint::Percentage(p) => format!("SizeConstraint::Percentage({})", p),
    }
}

/// Formats a Padding as Rust code.
fn format_padding(p: Padding) -> String {
    // Padding fields are: top, right, bottom, left
    if p.top == p.bottom && p.left == p.right && p.top == p.left {
        format!("Padding::uniform({})", p.top)
    } else if p.top == p.bottom && p.left == p.right {
        format!("Padding::symmetric({}, {})", p.top, p.left)
    } else if p.top == 0 && p.bottom == 0 {
        format!("Padding::horizontal({})", p.left)
    } else if p.left == 0 && p.right == 0 {
        format!("Padding::vertical({})", p.top)
    } else {
        format!("Padding::new({}, {}, {}, {})", p.top, p.right, p.bottom, p.left)
    }
}

// FILE: src/notifications/functions/fnc_generate_code.rs - Code generation for notifications
// END OF VERSION: 1.0.0
