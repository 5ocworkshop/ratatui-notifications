// FILE: tests/test_fnc_calculate_rect_integration.rs - Integration tests for rect calculation
// VERSION: 1.0.0
// WCTX: Implementing layout functions for ratatui-notifications using TDD
// CLOG: Initial creation with tests for rect placement and clamping

use ratatui::layout::{Position, Rect};
use ratatui_notifications::notifications::functions::fnc_calculate_rect::calculate_rect;
use ratatui_notifications::notifications::types::Anchor;

#[test]
fn test_top_left_anchor_places_rect_at_anchor_position() {
    let frame = Rect::new(0, 0, 100, 50);
    let anchor_pos = Position::new(10, 5);
    let width = 20;
    let height = 10;
    let exterior_padding = 0;

    let result = calculate_rect(Anchor::TopLeft, anchor_pos, width, height, frame, exterior_padding);

    // TopLeft: rect starts at anchor position
    assert_eq!(result, Rect::new(10, 5, 20, 10));
}

#[test]
fn test_top_center_anchor_centers_rect_horizontally() {
    let frame = Rect::new(0, 0, 100, 50);
    let anchor_pos = Position::new(50, 5);  // Center of top edge
    let width = 20;
    let height = 10;
    let exterior_padding = 0;

    let result = calculate_rect(Anchor::TopCenter, anchor_pos, width, height, frame, exterior_padding);

    // TopCenter: rect should be centered on anchor_pos.x
    // x = anchor_pos.x - width / 2 = 50 - 10 = 40
    assert_eq!(result, Rect::new(40, 5, 20, 10));
}

#[test]
fn test_top_right_anchor_aligns_right_edge() {
    let frame = Rect::new(0, 0, 100, 50);
    let anchor_pos = Position::new(99, 5);  // Right edge
    let width = 20;
    let height = 10;
    let exterior_padding = 0;

    let result = calculate_rect(Anchor::TopRight, anchor_pos, width, height, frame, exterior_padding);

    // TopRight: rect's right edge aligns with anchor_pos.x
    // x = anchor_pos.x - (width - 1) = 99 - 19 = 80
    assert_eq!(result, Rect::new(80, 5, 20, 10));
}

#[test]
fn test_middle_center_centers_rect_both_axes() {
    let frame = Rect::new(0, 0, 100, 50);
    let anchor_pos = Position::new(50, 25);  // Center of frame
    let width = 20;
    let height = 10;
    let exterior_padding = 0;

    let result = calculate_rect(Anchor::MiddleCenter, anchor_pos, width, height, frame, exterior_padding);

    // MiddleCenter: rect centered on both axes
    // x = 50 - 10 = 40, y = 25 - 5 = 20
    assert_eq!(result, Rect::new(40, 20, 20, 10));
}

#[test]
fn test_bottom_right_anchor_aligns_bottom_right() {
    let frame = Rect::new(0, 0, 100, 50);
    let anchor_pos = Position::new(99, 49);  // Bottom-right corner
    let width = 20;
    let height = 10;
    let exterior_padding = 0;

    let result = calculate_rect(Anchor::BottomRight, anchor_pos, width, height, frame, exterior_padding);

    // BottomRight: rect's bottom-right corner aligns with anchor_pos
    // x = 99 - 19 = 80, y = 49 - 9 = 40
    assert_eq!(result, Rect::new(80, 40, 20, 10));
}

#[test]
fn test_exterior_padding_pushes_top_left_inward() {
    let frame = Rect::new(0, 0, 100, 50);
    let anchor_pos = Position::new(0, 0);
    let width = 20;
    let height = 10;
    let exterior_padding = 2;

    let result = calculate_rect(Anchor::TopLeft, anchor_pos, width, height, frame, exterior_padding);

    // TopLeft with padding: rect should be offset by padding
    assert_eq!(result, Rect::new(2, 2, 20, 10));
}

#[test]
fn test_exterior_padding_pushes_bottom_right_inward() {
    let frame = Rect::new(0, 0, 100, 50);
    let anchor_pos = Position::new(99, 49);
    let width = 20;
    let height = 10;
    let exterior_padding = 2;

    let result = calculate_rect(Anchor::BottomRight, anchor_pos, width, height, frame, exterior_padding);

    // BottomRight with padding: rect should be offset inward by padding
    // x = 99 - 19 - 2 = 78, y = 49 - 9 - 2 = 38
    assert_eq!(result, Rect::new(78, 38, 20, 10));
}

#[test]
fn test_exterior_padding_does_not_affect_middle_center() {
    let frame = Rect::new(0, 0, 100, 50);
    let anchor_pos = Position::new(50, 25);
    let width = 20;
    let height = 10;
    let exterior_padding = 2;

    let result = calculate_rect(Anchor::MiddleCenter, anchor_pos, width, height, frame, exterior_padding);

    // MiddleCenter: padding should not affect (special case)
    assert_eq!(result, Rect::new(40, 20, 20, 10));
}

#[test]
fn test_rect_clamped_when_too_large() {
    let frame = Rect::new(0, 0, 30, 20);
    let anchor_pos = Position::new(0, 0);
    let width = 50;  // Larger than frame
    let height = 30;  // Larger than frame
    let exterior_padding = 0;

    let result = calculate_rect(Anchor::TopLeft, anchor_pos, width, height, frame, exterior_padding);

    // Rect should be clamped to frame size
    assert_eq!(result, Rect::new(0, 0, 30, 20));
}

#[test]
fn test_rect_clamped_to_frame_bounds() {
    let frame = Rect::new(10, 10, 50, 30);
    let anchor_pos = Position::new(0, 0);  // Outside frame
    let width = 20;
    let height = 10;
    let exterior_padding = 0;

    let result = calculate_rect(Anchor::TopLeft, anchor_pos, width, height, frame, exterior_padding);

    // Rect should be clamped to start within frame bounds
    assert_eq!(result, Rect::new(10, 10, 20, 10));
}

#[test]
fn test_rect_stays_within_frame_right_edge() {
    let frame = Rect::new(0, 0, 30, 20);
    let anchor_pos = Position::new(25, 10);  // Near right edge
    let width = 20;  // Would extend past frame
    let height = 10;
    let exterior_padding = 0;

    let result = calculate_rect(Anchor::TopLeft, anchor_pos, width, height, frame, exterior_padding);

    // Rect should be pushed left to fit within frame
    // final_x should be max x that allows width to fit: 30 - 20 = 10
    assert_eq!(result, Rect::new(10, 10, 20, 10));
}

#[test]
fn test_rect_stays_within_frame_bottom_edge() {
    let frame = Rect::new(0, 0, 30, 20);
    let anchor_pos = Position::new(5, 15);  // Near bottom edge
    let width = 10;
    let height = 10;  // Would extend past frame
    let exterior_padding = 0;

    let result = calculate_rect(Anchor::TopLeft, anchor_pos, width, height, frame, exterior_padding);

    // Rect should be pushed up to fit within frame
    // final_y should be max y that allows height to fit: 20 - 10 = 10
    assert_eq!(result, Rect::new(5, 10, 10, 10));
}

// FILE: tests/test_fnc_calculate_rect_integration.rs - Integration tests for rect calculation
// END OF VERSION: 1.0.0
