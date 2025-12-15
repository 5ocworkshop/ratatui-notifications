// FILE: tests/notifications/functions/test_fnc_calculate_anchor_position.rs - Tests for anchor position calculation
// VERSION: 1.0.0
// WCTX: Implementing layout functions for ratatui-notifications using TDD
// CLOG: Initial creation with tests for all 9 anchor positions

use ratatui::layout::{Position, Rect};
use ratatui_notifications::notifications::functions::fnc_calculate_anchor_position::calculate_anchor_position;
use ratatui_notifications::notifications::types::Anchor;

#[test]
fn test_top_left_returns_frame_origin() {
    let frame = Rect::new(10, 5, 100, 50);
    let pos = calculate_anchor_position(Anchor::TopLeft, frame);
    assert_eq!(pos, Position::new(10, 5), "TopLeft should return frame origin");
}

#[test]
fn test_top_center_returns_center_of_top_edge() {
    let frame = Rect::new(10, 5, 100, 50);
    let pos = calculate_anchor_position(Anchor::TopCenter, frame);
    // x should be frame.x + frame.width / 2 = 10 + 50 = 60
    assert_eq!(pos, Position::new(60, 5), "TopCenter should return center of top edge");
}

#[test]
fn test_top_right_returns_right_edge_of_top() {
    let frame = Rect::new(10, 5, 100, 50);
    let pos = calculate_anchor_position(Anchor::TopRight, frame);
    // x should be frame.right() - 1 = 110 - 1 = 109
    assert_eq!(pos, Position::new(109, 5), "TopRight should return right edge of top");
}

#[test]
fn test_middle_left_returns_left_edge_center() {
    let frame = Rect::new(10, 5, 100, 50);
    let pos = calculate_anchor_position(Anchor::MiddleLeft, frame);
    // y should be frame.y + frame.height / 2 = 5 + 25 = 30
    assert_eq!(pos, Position::new(10, 30), "MiddleLeft should return left edge center");
}

#[test]
fn test_middle_center_returns_center_of_frame() {
    let frame = Rect::new(10, 5, 100, 50);
    let pos = calculate_anchor_position(Anchor::MiddleCenter, frame);
    // x = 10 + 50 = 60, y = 5 + 25 = 30
    assert_eq!(pos, Position::new(60, 30), "MiddleCenter should return center of frame");
}

#[test]
fn test_middle_right_returns_right_edge_center() {
    let frame = Rect::new(10, 5, 100, 50);
    let pos = calculate_anchor_position(Anchor::MiddleRight, frame);
    // x = 109, y = 30
    assert_eq!(pos, Position::new(109, 30), "MiddleRight should return right edge center");
}

#[test]
fn test_bottom_left_returns_bottom_left_corner() {
    let frame = Rect::new(10, 5, 100, 50);
    let pos = calculate_anchor_position(Anchor::BottomLeft, frame);
    // y should be frame.bottom() - 1 = 55 - 1 = 54
    assert_eq!(pos, Position::new(10, 54), "BottomLeft should return bottom-left corner");
}

#[test]
fn test_bottom_center_returns_center_of_bottom_edge() {
    let frame = Rect::new(10, 5, 100, 50);
    let pos = calculate_anchor_position(Anchor::BottomCenter, frame);
    // x = 60, y = 54
    assert_eq!(pos, Position::new(60, 54), "BottomCenter should return center of bottom edge");
}

#[test]
fn test_bottom_right_returns_bottom_right_corner() {
    let frame = Rect::new(10, 5, 100, 50);
    let pos = calculate_anchor_position(Anchor::BottomRight, frame);
    // x = 109, y = 54
    assert_eq!(pos, Position::new(109, 54), "BottomRight should return bottom-right corner");
}

// FILE: tests/notifications/functions/test_fnc_calculate_anchor_position.rs - Tests for anchor position calculation
// END OF VERSION: 1.0.0
