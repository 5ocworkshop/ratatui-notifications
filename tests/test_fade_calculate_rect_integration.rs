// FILE: tests/test_fade_calculate_rect_integration.rs - Integration tests for fade rect calculation
// VERSION: 1.0.0
// WCTX: TDD implementation of animation function extraction
// CLOG: Created integration test for fade calculate_rect

use ratatui::prelude::*;
use ratatui_notifications::notifications::functions::fnc_fade_calculate_rect::calculate_rect;
use ratatui_notifications::notifications::types::AnimationPhase;

#[test]
fn test_fade_calculate_rect_returns_full_rect_fading_in() {
    let full_rect = Rect::new(10, 20, 30, 40);
    let frame_area = Rect::new(0, 0, 100, 100);

    let result = calculate_rect(full_rect, frame_area, AnimationPhase::FadingIn, 0.0);
    assert_eq!(result, full_rect);

    let result = calculate_rect(full_rect, frame_area, AnimationPhase::FadingIn, 0.5);
    assert_eq!(result, full_rect);

    let result = calculate_rect(full_rect, frame_area, AnimationPhase::FadingIn, 1.0);
    assert_eq!(result, full_rect);
}

#[test]
fn test_fade_calculate_rect_returns_full_rect_fading_out() {
    let full_rect = Rect::new(5, 10, 25, 35);
    let frame_area = Rect::new(0, 0, 100, 100);

    let result = calculate_rect(full_rect, frame_area, AnimationPhase::FadingOut, 0.0);
    assert_eq!(result, full_rect);

    let result = calculate_rect(full_rect, frame_area, AnimationPhase::FadingOut, 0.5);
    assert_eq!(result, full_rect);

    let result = calculate_rect(full_rect, frame_area, AnimationPhase::FadingOut, 1.0);
    assert_eq!(result, full_rect);
}

#[test]
fn test_fade_calculate_rect_returns_full_rect_other_phases() {
    let full_rect = Rect::new(15, 25, 20, 30);
    let frame_area = Rect::new(0, 0, 100, 100);

    // Dwelling phase
    let result = calculate_rect(full_rect, frame_area, AnimationPhase::Dwelling, 0.5);
    assert_eq!(result, full_rect);

    // Expanding phase
    let result = calculate_rect(full_rect, frame_area, AnimationPhase::Expanding, 0.5);
    assert_eq!(result, full_rect);

    // Pending phase
    let result = calculate_rect(full_rect, frame_area, AnimationPhase::Pending, 0.0);
    assert_eq!(result, full_rect);
}

#[test]
fn test_fade_calculate_rect_ignores_frame_area() {
    let full_rect = Rect::new(10, 20, 30, 40);
    let frame_area1 = Rect::new(0, 0, 50, 50);
    let frame_area2 = Rect::new(0, 0, 200, 200);

    // Fade doesn't care about frame_area, should always return full_rect
    let result1 = calculate_rect(full_rect, frame_area1, AnimationPhase::FadingIn, 0.5);
    let result2 = calculate_rect(full_rect, frame_area2, AnimationPhase::FadingIn, 0.5);

    assert_eq!(result1, full_rect);
    assert_eq!(result2, full_rect);
    assert_eq!(result1, result2);
}

// FILE: tests/test_fade_calculate_rect_integration.rs - Integration tests for fade rect calculation
// END OF VERSION: 1.0.0
