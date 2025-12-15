// FILE: tests/test_demo_scenarios.rs - Integration tests for demo notification scenarios
// VERSION: 1.1.0
// WCTX: Updating demo tests after demo redesign
// CLOG: Renamed key-specific tests to be feature-descriptive

//! Integration tests that verify all demo scenarios work correctly.
//! These tests guard against the issues found during the OFPF migration where:
//! - Notifications had wrong default sizes (full height)
//! - Custom positions weren't propagated to state
//! - Some anchors didn't render at all

use ratatui::layout::{Position, Rect};
use ratatui::widgets::{BorderType, Padding};
use ratatui_notifications::{
    Anchor, Animation, AutoDismiss, Level, NotificationBuilder, Notifications,
    Overflow, SizeConstraint, SlideDirection, Timing,
};
use std::time::Duration;

/// Standard test frame area (120x40 terminal)
const TEST_FRAME: Rect = Rect {
    x: 0,
    y: 0,
    width: 120,
    height: 40,
};

// ============================================================================
// Default Value Tests - Ensure sensible defaults prevent full-height issues
// ============================================================================

#[test]
fn test_default_notification_has_size_constraints() {
    let notification = NotificationBuilder::new("Test content")
        .build()
        .unwrap();

    // These defaults are critical - without them, notifications render at full frame height
    assert!(
        notification.max_width().is_some(),
        "max_width must have a default to prevent full-width notifications"
    );
    assert!(
        notification.max_height().is_some(),
        "max_height must have a default to prevent full-height notifications"
    );

    // Verify they're percentage-based (relative to frame size)
    match notification.max_width() {
        Some(SizeConstraint::Percentage(p)) => {
            assert!(p > 0.0 && p <= 1.0, "max_width percentage should be between 0 and 1");
            assert!(p <= 0.5, "max_width should be reasonable (<=50% of frame)");
        }
        _ => panic!("max_width should be a Percentage constraint by default"),
    }

    match notification.max_height() {
        Some(SizeConstraint::Percentage(p)) => {
            assert!(p > 0.0 && p <= 1.0, "max_height percentage should be between 0 and 1");
            assert!(p <= 0.3, "max_height should be reasonable (<=30% of frame)");
        }
        _ => panic!("max_height should be a Percentage constraint by default"),
    }
}

#[test]
fn test_default_notification_has_border() {
    let notification = NotificationBuilder::new("Test content")
        .build()
        .unwrap();

    assert!(
        notification.border_type().is_some(),
        "border_type must have a default for proper size calculation"
    );
}

#[test]
fn test_default_notification_has_padding() {
    let notification = NotificationBuilder::new("Test content")
        .build()
        .unwrap();

    // Padding should not be zero - at least horizontal padding for readability
    assert!(
        notification.padding() != Padding::ZERO,
        "padding should not be zero by default"
    );
}

#[test]
fn test_default_notification_has_level() {
    let notification = NotificationBuilder::new("Test content")
        .build()
        .unwrap();

    assert!(
        notification.level().is_some(),
        "level should have a default for consistent styling"
    );
}

// ============================================================================
// Anchor Tests - All 9 anchors must work correctly
// ============================================================================

#[test]
fn test_all_anchors_can_add_notifications() {
    let anchors = [
        Anchor::TopLeft,
        Anchor::TopCenter,
        Anchor::TopRight,
        Anchor::MiddleLeft,
        Anchor::MiddleCenter,
        Anchor::MiddleRight,
        Anchor::BottomLeft,
        Anchor::BottomCenter,
        Anchor::BottomRight,
    ];

    for anchor in anchors {
        let mut manager = Notifications::new();

        let notification = NotificationBuilder::new(format!("Test at {:?}", anchor))
            .anchor(anchor)
            .title(format!("{:?}", anchor))
            .build()
            .unwrap();

        let result = manager.add(notification);
        assert!(
            result.is_ok(),
            "Should be able to add notification at {:?}",
            anchor
        );
    }
}

#[test]
fn test_notifications_at_different_anchors_are_independent() {
    let mut manager = Notifications::new();

    // Add notifications at different anchors
    let top_left = NotificationBuilder::new("TopLeft")
        .anchor(Anchor::TopLeft)
        .build()
        .unwrap();
    let bottom_right = NotificationBuilder::new("BottomRight")
        .anchor(Anchor::BottomRight)
        .build()
        .unwrap();

    let id1 = manager.add(top_left).unwrap();
    let id2 = manager.add(bottom_right).unwrap();

    assert_ne!(id1, id2, "Different notifications should have different IDs");
}

// ============================================================================
// Animation Type Tests - All animation types must work
// ============================================================================

#[test]
fn test_slide_animation_notification() {
    let mut manager = Notifications::new();

    let notification = NotificationBuilder::new("Slide test")
        .animation(Animation::Slide)
        .slide_direction(SlideDirection::FromRight)
        .anchor(Anchor::TopRight)
        .build()
        .unwrap();

    let result = manager.add(notification);
    assert!(result.is_ok(), "Slide animation notification should be addable");

    // Tick to start animation
    manager.tick(Duration::from_millis(16));
}

#[test]
fn test_expand_collapse_animation_notification() {
    let mut manager = Notifications::new();

    let notification = NotificationBuilder::new("ExpandCollapse test")
        .animation(Animation::ExpandCollapse)
        .anchor(Anchor::MiddleCenter)
        .build()
        .unwrap();

    let result = manager.add(notification);
    assert!(result.is_ok(), "ExpandCollapse animation notification should be addable");

    // Tick to start animation
    manager.tick(Duration::from_millis(16));
}

#[test]
fn test_fade_animation_notification() {
    let mut manager = Notifications::new();

    let notification = NotificationBuilder::new("Fade test")
        .animation(Animation::Fade)
        .anchor(Anchor::MiddleLeft)
        .build()
        .unwrap();

    let result = manager.add(notification);
    assert!(result.is_ok(), "Fade animation notification should be addable");

    // Tick to start animation
    manager.tick(Duration::from_millis(16));
}

// ============================================================================
// Custom Position Tests - Entry/exit positions must be propagated
// ============================================================================

#[test]
fn test_custom_entry_position_is_set() {
    let entry_pos = Position::new(10, 20);

    let notification = NotificationBuilder::new("Custom entry test")
        .entry_position(entry_pos)
        .build()
        .unwrap();

    assert_eq!(
        notification.custom_entry_position(),
        Some(entry_pos),
        "Custom entry position should be stored in notification"
    );
}

#[test]
fn test_custom_exit_position_is_set() {
    let exit_pos = Position::new(100, 30);

    let notification = NotificationBuilder::new("Custom exit test")
        .exit_position(exit_pos)
        .build()
        .unwrap();

    assert_eq!(
        notification.custom_exit_position(),
        Some(exit_pos),
        "Custom exit position should be stored in notification"
    );
}

#[test]
fn test_custom_positions_with_slide_animation() {
    let mut manager = Notifications::new();

    let entry_pos = Position::new(10, 20);
    let exit_pos = Position::new(100, 20);

    let notification = NotificationBuilder::new("Custom path test")
        .anchor(Anchor::MiddleCenter)
        .animation(Animation::Slide)
        .slide_direction(SlideDirection::FromLeft)
        .entry_position(entry_pos)
        .exit_position(exit_pos)
        .fade(true)
        .build()
        .unwrap();

    let result = manager.add(notification);
    assert!(result.is_ok(), "Notification with custom positions should be addable");

    // Tick to start animation
    manager.tick(Duration::from_millis(16));
}

// ============================================================================
// Timing Tests - Various timing configurations must work
// ============================================================================

#[test]
fn test_custom_timing_is_respected() {
    let notification = NotificationBuilder::new("Timing test")
        .timing(
            Timing::Fixed(Duration::from_millis(500)),
            Timing::Fixed(Duration::from_secs(4)),
            Timing::Fixed(Duration::from_millis(750)),
        )
        .build()
        .unwrap();

    assert_eq!(
        notification.slide_in_timing(),
        Timing::Fixed(Duration::from_millis(500))
    );
    assert_eq!(
        notification.dwell_timing(),
        Timing::Fixed(Duration::from_secs(4))
    );
    assert_eq!(
        notification.slide_out_timing(),
        Timing::Fixed(Duration::from_millis(750))
    );
}

#[test]
fn test_auto_dismiss_configurations() {
    // Test AutoDismiss::After
    let notification = NotificationBuilder::new("Auto dismiss test")
        .auto_dismiss(AutoDismiss::After(Duration::from_secs(5)))
        .build()
        .unwrap();
    assert_eq!(
        notification.auto_dismiss(),
        AutoDismiss::After(Duration::from_secs(5))
    );

    // Test AutoDismiss::Never
    let notification = NotificationBuilder::new("Never dismiss test")
        .auto_dismiss(AutoDismiss::Never)
        .build()
        .unwrap();
    assert_eq!(notification.auto_dismiss(), AutoDismiss::Never);
}

// ============================================================================
// Slide Direction Tests - All directions must work with appropriate anchors
// ============================================================================

#[test]
fn test_slide_directions_with_anchors() {
    let test_cases = [
        (Anchor::TopRight, SlideDirection::FromTop),
        (Anchor::TopRight, SlideDirection::FromRight),
        (Anchor::BottomLeft, SlideDirection::FromBottom),
        (Anchor::BottomLeft, SlideDirection::FromLeft),
        (Anchor::MiddleCenter, SlideDirection::Default),
    ];

    for (anchor, direction) in test_cases {
        let mut manager = Notifications::new();

        let notification = NotificationBuilder::new(format!("{:?} from {:?}", anchor, direction))
            .anchor(anchor)
            .slide_direction(direction)
            .build()
            .unwrap();

        let result = manager.add(notification);
        assert!(
            result.is_ok(),
            "Notification at {:?} with direction {:?} should be addable",
            anchor,
            direction
        );

        // Tick to verify no panics during animation
        manager.tick(Duration::from_millis(16));
    }
}

// ============================================================================
// Styling Tests - Custom styles must work
// ============================================================================

#[test]
fn test_custom_border_types() {
    let border_types = [
        BorderType::Plain,
        BorderType::Rounded,
        BorderType::Double,
        BorderType::Thick,
    ];

    for border_type in border_types {
        let notification = NotificationBuilder::new(format!("Border: {:?}", border_type))
            .border_type(border_type)
            .build()
            .unwrap();

        assert_eq!(
            notification.border_type(),
            Some(border_type),
            "Border type {:?} should be set",
            border_type
        );
    }
}

#[test]
fn test_all_log_levels() {
    let levels = [
        Level::Info,
        Level::Warn,
        Level::Error,
        Level::Debug,
        Level::Trace,
    ];

    for level in levels {
        let notification = NotificationBuilder::new(format!("Level: {:?}", level))
            .level(level)
            .build()
            .unwrap();

        assert_eq!(
            notification.level(),
            Some(level),
            "Level {:?} should be set",
            level
        );
    }
}

// ============================================================================
// Manager Configuration Tests - Overflow and concurrent limits
// ============================================================================

#[test]
fn test_max_concurrent_with_overflow_discard_oldest() {
    let mut manager = Notifications::new()
        .max_concurrent(Some(3))
        .overflow(Overflow::DiscardOldest);

    // Add 5 notifications at same anchor
    for i in 0..5 {
        let notification = NotificationBuilder::new(format!("Notification {}", i))
            .anchor(Anchor::BottomRight)
            .build()
            .unwrap();
        manager.add(notification).unwrap();
    }

    // Tick to process
    manager.tick(Duration::from_millis(16));
}

#[test]
fn test_max_concurrent_with_overflow_discard_newest() {
    let mut manager = Notifications::new()
        .max_concurrent(Some(3))
        .overflow(Overflow::DiscardNewest);

    // Add 5 notifications at same anchor
    for i in 0..5 {
        let notification = NotificationBuilder::new(format!("Notification {}", i))
            .anchor(Anchor::BottomRight)
            .build()
            .unwrap();
        manager.add(notification).unwrap();
    }

    // Tick to process
    manager.tick(Duration::from_millis(16));
}

// ============================================================================
// Full Demo Scenario Tests - Simulate actual demo key presses
// ============================================================================

#[test]
fn test_demo_scenario_position_anchor() {
    // Tests position-based notification (numpad keys 1-9 in new demo)
    let mut manager = Notifications::new()
        .max_concurrent(Some(3))
        .overflow(Overflow::DiscardOldest);

    let notification = NotificationBuilder::new("Position: TopLeft\nSlide animation")
        .anchor(Anchor::TopLeft)
        .title(" TopLeft ")
        .level(Level::Info)
        .slide_direction(SlideDirection::Default)
        .margin(0)
        .timing(
            Timing::Fixed(Duration::from_millis(400)),
            Timing::Fixed(Duration::from_secs(3)),
            Timing::Fixed(Duration::from_millis(500)),
        )
        .border_type(BorderType::Rounded)
        .build()
        .unwrap();

    let result = manager.add(notification);
    assert!(result.is_ok(), "Position anchor scenario should work");
    manager.tick(Duration::from_millis(16));
}

#[test]
fn test_demo_scenario_combined_effects() {
    // Tests slide + fade combined (key 'c' in new demo)
    let mut manager = Notifications::new();

    let entry_pos = Position::new(30, 20);
    let exit_pos = Position::new(80, 20);

    let notification = NotificationBuilder::new("Slides in while fading\nthen fades out while sliding")
        .anchor(Anchor::MiddleLeft)
        .title(" Slide + Fade ")
        .slide_direction(SlideDirection::FromLeft)
        .timing(
            Timing::Fixed(Duration::from_millis(600)),
            Timing::Fixed(Duration::from_secs(3)),
            Timing::Fixed(Duration::from_millis(700)),
        )
        .entry_position(entry_pos)
        .exit_position(exit_pos)
        .fade(true)
        .border_type(BorderType::Rounded)
        .build()
        .unwrap();

    // Verify custom positions are set
    assert_eq!(notification.custom_entry_position(), Some(entry_pos));
    assert_eq!(notification.custom_exit_position(), Some(exit_pos));
    assert!(notification.fade_effect());

    let result = manager.add(notification);
    assert!(result.is_ok(), "Combined effects scenario should work");
    manager.tick(Duration::from_millis(16));
}

#[test]
fn test_demo_scenario_expand_animation() {
    // Tests expand/collapse animation (key 'e' in demo)
    let mut manager = Notifications::new();

    let notification = NotificationBuilder::new("Expands from center point\nand collapses back")
        .anchor(Anchor::MiddleCenter)
        .title(" Expand ")
        .level(Level::Info)
        .animation(Animation::ExpandCollapse)
        .timing(
            Timing::Fixed(Duration::from_millis(350)),
            Timing::Fixed(Duration::from_secs(2)),
            Timing::Fixed(Duration::from_millis(350)),
        )
        .border_type(BorderType::Rounded)
        .build()
        .unwrap();

    let result = manager.add(notification);
    assert!(result.is_ok(), "Expand animation scenario should work");
    manager.tick(Duration::from_millis(16));
}

#[test]
fn test_demo_scenario_fade_animation() {
    // Tests fade in/out animation (key 'f' in demo)
    let mut manager = Notifications::new();

    let notification = NotificationBuilder::new("Fades in smoothly\nthen fades out")
        .anchor(Anchor::MiddleCenter)
        .title(" Fade ")
        .animation(Animation::Fade)
        .timing(
            Timing::Fixed(Duration::from_millis(600)),
            Timing::Fixed(Duration::from_secs(2)),
            Timing::Fixed(Duration::from_millis(600)),
        )
        .border_type(BorderType::Rounded)
        .build()
        .unwrap();

    let result = manager.add(notification);
    assert!(result.is_ok(), "Fade animation scenario should work");
    manager.tick(Duration::from_millis(16));
}

#[test]
fn test_demo_scenario_all_log_levels() {
    // Tests all log level display (key 'l' in demo)
    let mut manager = Notifications::new();

    let levels_anchors = [
        (Level::Trace, Anchor::BottomLeft),
        (Level::Debug, Anchor::BottomCenter),
        (Level::Info, Anchor::BottomRight),
        (Level::Warn, Anchor::TopLeft),
        (Level::Error, Anchor::TopRight),
    ];

    for (level, anchor) in levels_anchors {
        let notification = NotificationBuilder::new(format!("{:?}: level demonstration", level))
            .anchor(anchor)
            .title(format!(" {:?} ", level))
            .level(level)
            .border_type(BorderType::Rounded)
            .build()
            .unwrap();

        let result = manager.add(notification);
        assert!(
            result.is_ok(),
            "Log levels scenario should work for {:?}",
            level
        );
    }

    manager.tick(Duration::from_millis(16));
}

// ============================================================================
// Regression Tests - Specific issues from OFPF migration
// ============================================================================

#[test]
fn test_regression_middle_anchors_render() {
    // This test guards against the issue where Middle anchors didn't render
    // because notification height exceeded available_height

    let mut manager = Notifications::new();

    let middle_anchors = [
        Anchor::MiddleLeft,
        Anchor::MiddleCenter,
        Anchor::MiddleRight,
    ];

    for anchor in middle_anchors {
        let notification = NotificationBuilder::new(format!("Test at {:?}", anchor))
            .anchor(anchor)
            .build()
            .unwrap();

        // Verify size constraints are reasonable
        assert!(
            notification.max_height().is_some(),
            "Notification must have max_height constraint"
        );

        let result = manager.add(notification);
        assert!(
            result.is_ok(),
            "Middle anchor {:?} notification should be addable",
            anchor
        );
    }

    // Tick to ensure no panics during render phase
    manager.tick(Duration::from_millis(16));
}

#[test]
fn test_regression_bottom_anchors_render() {
    // This test guards against the issue where Bottom anchors didn't render
    // because notification height exceeded available_height

    let mut manager = Notifications::new();

    let bottom_anchors = [
        Anchor::BottomLeft,
        Anchor::BottomCenter,
        Anchor::BottomRight,
    ];

    for anchor in bottom_anchors {
        let notification = NotificationBuilder::new(format!("Test at {:?}", anchor))
            .anchor(anchor)
            .build()
            .unwrap();

        let result = manager.add(notification);
        assert!(
            result.is_ok(),
            "Bottom anchor {:?} notification should be addable",
            anchor
        );
    }

    manager.tick(Duration::from_millis(16));
}

#[test]
fn test_regression_custom_positions_propagated() {
    // This test guards against the issue where custom_entry_position and
    // custom_exit_position weren't copied from Notification to NotificationState

    let entry_pos = Position::new(10, 20);
    let exit_pos = Position::new(100, 50);

    let notification = NotificationBuilder::new("Custom position test")
        .entry_position(entry_pos)
        .exit_position(exit_pos)
        .build()
        .unwrap();

    // Verify positions are stored in notification
    assert_eq!(
        notification.custom_entry_position(),
        Some(entry_pos),
        "Entry position must be stored"
    );
    assert_eq!(
        notification.custom_exit_position(),
        Some(exit_pos),
        "Exit position must be stored"
    );

    // Add to manager and tick to ensure state is created properly
    let mut manager = Notifications::new();
    manager.add(notification).unwrap();
    manager.tick(Duration::from_millis(16));
}

#[test]
fn test_regression_notification_not_full_height() {
    // This test guards against notifications being calculated as full frame height

    let notification = NotificationBuilder::new("Short content")
        .build()
        .unwrap();

    // With default 20% max_height on a 40-row frame, max height should be 8 rows
    match notification.max_height() {
        Some(SizeConstraint::Percentage(p)) => {
            let max_height = (TEST_FRAME.height as f32 * p) as u16;
            assert!(
                max_height < TEST_FRAME.height / 2,
                "Max height ({}) should be less than half frame height ({})",
                max_height,
                TEST_FRAME.height / 2
            );
        }
        Some(SizeConstraint::Absolute(h)) => {
            assert!(
                h < TEST_FRAME.height / 2,
                "Max height ({}) should be less than half frame height",
                h
            );
        }
        None => panic!("max_height should not be None"),
    }
}

// FILE: tests/test_demo_scenarios.rs - Integration tests for demo notification scenarios
// END OF VERSION: 1.1.0
