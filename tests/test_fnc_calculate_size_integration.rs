// FILE: tests/test_fnc_calculate_size_integration.rs - Integration tests for fnc_calculate_size
// VERSION: 1.1.0
// WCTX: TDD implementation of deferred functions
// CLOG: Rewrote tests to use NotificationBuilder pattern instead of direct struct construction

use ratatui::prelude::*;
use ratatui::widgets::{BorderType, Padding};
use ratatui_notifications::notifications::NotificationBuilder;
use ratatui_notifications::notifications::functions::fnc_calculate_size::calculate_size;
use ratatui_notifications::notifications::types::SizeConstraint;

#[test]
fn test_empty_content_returns_minimum_size() {
    // Empty content should return minimum 3x3 size
    let notification = NotificationBuilder::new("")
        .build()
        .unwrap();
    let frame_area = Rect::new(0, 0, 100, 100);

    let (width, height) = calculate_size(&notification, frame_area);

    assert!(width >= 3);
    assert!(height >= 3);
}

#[test]
fn test_content_respects_max_width_percentage() {
    // Long content should be constrained by percentage max_width
    let long_content = "This is a very long line of text that should be wrapped";
    let notification = NotificationBuilder::new(long_content)
        .border_type(BorderType::Plain)
        .padding(Padding::uniform(1))
        .max_size(SizeConstraint::Percentage(0.5), SizeConstraint::Absolute(100))
        .build()
        .unwrap();
    let frame_area = Rect::new(0, 0, 100, 100);

    let (width, _height) = calculate_size(&notification, frame_area);

    // Should be at most 50 (50% of 100)
    assert!(width <= 50);
    // Should be at least minimum size
    assert!(width >= 3);
}

#[test]
fn test_content_respects_max_width_absolute() {
    // Long content should be constrained by absolute max_width
    let long_content = "This is a very long line of text that should be wrapped";
    let notification = NotificationBuilder::new(long_content)
        .border_type(BorderType::Plain)
        .padding(Padding::uniform(1))
        .max_size(SizeConstraint::Absolute(30), SizeConstraint::Absolute(100))
        .build()
        .unwrap();
    let frame_area = Rect::new(0, 0, 100, 100);

    let (width, _height) = calculate_size(&notification, frame_area);

    // Should be at most 30
    assert!(width <= 30);
    assert!(width >= 3);
}

#[test]
fn test_content_respects_max_height() {
    // Multi-line content should be constrained by max_height
    let multiline_content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8";
    let notification = NotificationBuilder::new(multiline_content)
        .border_type(BorderType::Plain)
        .padding(Padding::uniform(1))
        .max_size(SizeConstraint::Absolute(20), SizeConstraint::Absolute(8))
        .build()
        .unwrap();
    let frame_area = Rect::new(0, 0, 100, 100);

    let (_width, height) = calculate_size(&notification, frame_area);

    // Should be at most 8
    assert!(height <= 8);
    assert!(height >= 3);
}

#[test]
fn test_multiline_content_calculates_correct_height() {
    // Multi-line content should calculate height based on actual wrapped lines
    let content = "Line 1\nLine 2\nLine 3";
    let notification = NotificationBuilder::new(content)
        .border_type(BorderType::Plain)
        .padding(Padding::uniform(1))
        .max_size(SizeConstraint::Absolute(40), SizeConstraint::Absolute(100))
        .build()
        .unwrap();
    let frame_area = Rect::new(0, 0, 100, 100);

    let (_width, height) = calculate_size(&notification, frame_area);

    // 3 lines of content + padding (top 1 + bottom 1) + border (2) = 7
    assert!(height >= 7);
}

#[test]
fn test_title_affects_width_calculation() {
    // Title wider than content should affect width
    let notification = NotificationBuilder::new("Short")
        .title("This is a very long title text that exceeds content")
        .border_type(BorderType::Plain)
        .padding(Padding::uniform(1))
        .max_size(SizeConstraint::Absolute(100), SizeConstraint::Absolute(50))
        .build()
        .unwrap();
    let frame_area = Rect::new(0, 0, 100, 100);

    let (width, _height) = calculate_size(&notification, frame_area);

    // Width should be wider than just "Short" (5 chars)
    // Content alone would be: 5 + padding (2) + border (2) = 9
    // With long title, width should be significantly larger
    assert!(width > 15); // Much wider than content-only would be
    assert!(width <= 100); // But within max_width constraint
}

#[test]
fn test_border_type_double_adds_correct_offset() {
    // Double border should be accounted for in size calculation
    let notification = NotificationBuilder::new("Test")
        .border_type(BorderType::Double)
        .build()
        .unwrap();
    let frame_area = Rect::new(0, 0, 100, 100);

    let (width, height) = calculate_size(&notification, frame_area);

    // Should include double border offset
    assert!(width >= 6); // "Test" (4) + double border (2)
    assert!(height >= 3); // 1 line + double border (2)
}

#[test]
fn test_no_border_has_no_offset() {
    // No border should mean no border offset
    let notification = NotificationBuilder::new("Test")
        .build()
        .unwrap();
    let frame_area = Rect::new(0, 0, 100, 100);

    let (width, height) = calculate_size(&notification, frame_area);

    // Minimum 3x3
    assert!(width >= 3);
    assert!(height >= 3);
}

#[test]
fn test_padding_affects_size() {
    // Padding should add to overall size
    let notification = NotificationBuilder::new("Test")
        .border_type(BorderType::Plain)
        .padding(Padding::new(1, 2, 3, 4)) // left=1, right=2, top=3, bottom=4
        .max_size(SizeConstraint::Absolute(50), SizeConstraint::Absolute(50))
        .build()
        .unwrap();
    let frame_area = Rect::new(0, 0, 100, 100);

    let (width, height) = calculate_size(&notification, frame_area);

    // Width: "Test" (4) + left (1) + right (2) + border (2) = 9
    assert_eq!(width, 9);
    // Height should not exceed max_height constraint
    assert!(height <= 50);
    // Height should be at least min_height
    assert!(height >= 3);
}

#[test]
fn test_wrapping_increases_height() {
    // Long content that wraps should increase height
    let long_line = "This is a very long line that will definitely wrap when constrained to a small width";
    let notification = NotificationBuilder::new(long_line)
        .border_type(BorderType::Plain)
        .padding(Padding::uniform(1))
        .max_size(SizeConstraint::Absolute(25), SizeConstraint::Absolute(100))
        .build()
        .unwrap();
    let frame_area = Rect::new(0, 0, 100, 100);

    let (_width, height) = calculate_size(&notification, frame_area);

    // Should wrap to multiple lines, height > minimum
    assert!(height > 5); // Should be significantly taller due to wrapping
}

// FILE: tests/test_fnc_calculate_size_integration.rs - Integration tests for fnc_calculate_size
// END OF VERSION: 1.1.0
