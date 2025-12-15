// FILE: tests/test_fnc_generate_code_integration.rs - Integration tests for code generation function
// VERSION: 1.0.0
// WCTX: Adding "show code" feature to demo
// CLOG: Initial creation

use std::time::Duration;

use ratatui::prelude::*;
use ratatui::widgets::{BorderType, Padding};

use ratatui_notifications::{
    generate_code, Anchor, Animation, AutoDismiss, Level, Notification, SlideDirection,
    SizeConstraint, Timing,
};

#[test]
fn test_default_notification_produces_minimal_code() {
    let notification = Notification::new("Hello").build().unwrap();
    let code = generate_code(&notification);

    // Should have builder pattern
    assert!(code.contains("Notification::builder("));
    assert!(code.contains(".build()"));

    // Content should be present
    assert!(code.contains("Hello"));

    // Default values should NOT appear (keeping code minimal)
    // Default anchor is BottomRight - should not appear
    assert!(!code.contains(".anchor("));
    // Default animation is Slide - should not appear
    assert!(!code.contains(".animation("));
    // Default level is Info - should not appear
    assert!(!code.contains(".level("));
}

#[test]
fn test_non_default_anchor_appears_in_code() {
    let notification = Notification::new("Test")
        .anchor(Anchor::TopCenter)
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".anchor(Anchor::TopCenter)"));
}

#[test]
fn test_non_default_animation_appears_in_code() {
    let notification = Notification::new("Test")
        .animation(Animation::Fade)
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".animation(Animation::Fade)"));
}

#[test]
fn test_non_default_level_appears_in_code() {
    let notification = Notification::new("Test")
        .level(Level::Error)
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".level(Level::Error)"));
}

#[test]
fn test_title_appears_in_code() {
    let notification = Notification::new("Content")
        .title("My Title")
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".title("));
    assert!(code.contains("My Title"));
}

#[test]
fn test_auto_dismiss_never_appears_in_code() {
    let notification = Notification::new("Test")
        .auto_dismiss(AutoDismiss::Never)
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".auto_dismiss(AutoDismiss::Never)"));
}

#[test]
fn test_auto_dismiss_custom_duration_appears_in_code() {
    let notification = Notification::new("Test")
        .auto_dismiss(AutoDismiss::After(Duration::from_secs(10)))
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".auto_dismiss("));
    assert!(code.contains("Duration::from_secs(10)"));
}

#[test]
fn test_default_auto_dismiss_not_in_code() {
    // Default is After(4 secs)
    let notification = Notification::new("Test")
        .auto_dismiss(AutoDismiss::After(Duration::from_secs(4)))
        .build()
        .unwrap();
    let code = generate_code(&notification);

    // Should not appear because it's the default
    assert!(!code.contains(".auto_dismiss("));
}

#[test]
fn test_slide_direction_appears_when_not_default() {
    let notification = Notification::new("Test")
        .slide_direction(SlideDirection::FromLeft)
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".slide_direction(SlideDirection::FromLeft)"));
}

#[test]
fn test_fade_effect_appears_when_true() {
    let notification = Notification::new("Test")
        .fade(true)
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".fade(true)"));
}

#[test]
fn test_fade_effect_not_in_code_when_false() {
    let notification = Notification::new("Test")
        .fade(false)
        .build()
        .unwrap();
    let code = generate_code(&notification);

    // Default is false, should not appear
    assert!(!code.contains(".fade("));
}

#[test]
fn test_border_type_appears_when_not_default() {
    let notification = Notification::new("Test")
        .border_type(BorderType::Double)
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".border_type(BorderType::Double)"));
}

#[test]
fn test_margin_appears_when_not_zero() {
    let notification = Notification::new("Test")
        .margin(5)
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".margin(5)"));
}

#[test]
fn test_timing_appears_when_fixed() {
    let notification = Notification::new("Test")
        .timing(
            Timing::Fixed(Duration::from_millis(300)),
            Timing::Fixed(Duration::from_secs(2)),
            Timing::Fixed(Duration::from_millis(500)),
        )
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".timing("));
    assert!(code.contains("Timing::Fixed"));
}

#[test]
fn test_max_size_appears_when_not_default() {
    let notification = Notification::new("Test")
        .max_size(SizeConstraint::Absolute(60), SizeConstraint::Absolute(10))
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".max_size("));
    assert!(code.contains("SizeConstraint::Absolute(60)"));
}

#[test]
fn test_padding_appears_when_not_default() {
    let notification = Notification::new("Test")
        .padding(Padding::uniform(3))
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".padding("));
}

#[test]
fn test_entry_position_appears_when_set() {
    let notification = Notification::new("Test")
        .entry_position(Position::new(10, 20))
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".entry_position("));
    assert!(code.contains("Position::new(10, 20)"));
}

#[test]
fn test_exit_position_appears_when_set() {
    let notification = Notification::new("Test")
        .exit_position(Position::new(100, 50))
        .build()
        .unwrap();
    let code = generate_code(&notification);

    assert!(code.contains(".exit_position("));
    assert!(code.contains("Position::new(100, 50)"));
}

#[test]
fn test_full_configuration_produces_complete_code() {
    let notification = Notification::new("Full config")
        .title("Alert")
        .level(Level::Warn)
        .anchor(Anchor::TopRight)
        .animation(Animation::ExpandCollapse)
        .slide_direction(SlideDirection::FromTop)
        .auto_dismiss(AutoDismiss::Never)
        .fade(true)
        .border_type(BorderType::Thick)
        .margin(2)
        .build()
        .unwrap();
    let code = generate_code(&notification);

    // All non-default values should appear
    assert!(code.contains("Full config"));
    assert!(code.contains(".title("));
    assert!(code.contains("Alert"));
    assert!(code.contains(".level(Level::Warn)"));
    assert!(code.contains(".anchor(Anchor::TopRight)"));
    assert!(code.contains(".animation(Animation::ExpandCollapse)"));
    assert!(code.contains(".slide_direction(SlideDirection::FromTop)"));
    assert!(code.contains(".auto_dismiss(AutoDismiss::Never)"));
    assert!(code.contains(".fade(true)"));
    assert!(code.contains(".border_type(BorderType::Thick)"));
    assert!(code.contains(".margin(2)"));
}

#[test]
fn test_multiline_content_is_escaped() {
    let notification = Notification::new("Line 1\nLine 2")
        .build()
        .unwrap();
    let code = generate_code(&notification);

    // Should escape newlines
    assert!(code.contains("Line 1\\nLine 2"));
}

#[test]
fn test_content_with_quotes_is_escaped() {
    let notification = Notification::new(r#"Say "Hello""#)
        .build()
        .unwrap();
    let code = generate_code(&notification);

    // Should properly escape quotes
    assert!(code.contains(r#"Say \"Hello\""#));
}

#[test]
fn test_code_is_syntactically_structured() {
    let notification = Notification::new("Test")
        .anchor(Anchor::TopLeft)
        .level(Level::Error)
        .build()
        .unwrap();
    let code = generate_code(&notification);

    // Should have proper structure
    assert!(code.starts_with("Notification::builder("));
    assert!(code.ends_with(".build()"));
    // Each method should be on separate line with indentation
    assert!(code.contains("\n    ."));
}

// FILE: tests/test_fnc_generate_code_integration.rs - Integration tests for code generation function
// END OF VERSION: 1.0.0
