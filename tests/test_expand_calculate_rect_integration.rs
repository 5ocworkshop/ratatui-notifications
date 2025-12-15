// FILE: tests/test_expand_calculate_rect_integration.rs - Integration tests for expand rect calculation
// VERSION: 1.0.0
// WCTX: TDD implementation of animation function extraction
// CLOG: Created integration test for expand calculate_rect

use ratatui::prelude::*;
use ratatui_notifications::notifications::functions::fnc_expand_calculate_rect::calculate_rect;
use ratatui_notifications::notifications::types::AnimationPhase;

#[test]
fn test_expand_calculate_rect_expanding_at_0() {
    let full_rect = Rect::new(10, 20, 33, 13);
    let frame_area = Rect::new(0, 0, 100, 100);

    let result = calculate_rect(full_rect, frame_area, AnimationPhase::Expanding, 0.0);

    // At progress 0.0, should be minimum size (3x3) centered
    // Center of full_rect: x = 10 + 33/2 = 26.5, y = 20 + 13/2 = 26.5
    // Centered 3x3: x = 26.5 - 1.5 = 25, y = 26.5 - 1.5 = 25
    assert_eq!(result, Rect::new(25, 25, 3, 3));
}

#[test]
fn test_expand_calculate_rect_expanding_at_50() {
    let full_rect = Rect::new(10, 20, 33, 13);
    let frame_area = Rect::new(0, 0, 100, 100);

    let result = calculate_rect(full_rect, frame_area, AnimationPhase::Expanding, 0.5);

    // At progress 0.5, should be halfway: lerp(3, 33, 0.5) = 18, lerp(3, 13, 0.5) = 8
    // Centered: x = 26.5 - 9 = 18 (rounded), y = 26.5 - 4 = 23 (rounded)
    assert_eq!(result, Rect::new(18, 23, 18, 8));
}

#[test]
fn test_expand_calculate_rect_expanding_at_100() {
    let full_rect = Rect::new(10, 20, 33, 13);
    let frame_area = Rect::new(0, 0, 100, 100);

    let result = calculate_rect(full_rect, frame_area, AnimationPhase::Expanding, 1.0);

    // At progress 1.0, should be full size
    assert_eq!(result, full_rect);
}

#[test]
fn test_expand_calculate_rect_collapsing_at_0() {
    let full_rect = Rect::new(10, 20, 33, 13);
    let frame_area = Rect::new(0, 0, 100, 100);

    let result = calculate_rect(full_rect, frame_area, AnimationPhase::Collapsing, 0.0);

    // At progress 0.0 of collapsing, should be full size
    assert_eq!(result, full_rect);
}

#[test]
fn test_expand_calculate_rect_collapsing_at_50() {
    let full_rect = Rect::new(10, 20, 33, 13);
    let frame_area = Rect::new(0, 0, 100, 100);

    let result = calculate_rect(full_rect, frame_area, AnimationPhase::Collapsing, 0.5);

    // At progress 0.5, should be halfway: lerp(33, 3, 0.5) = 18, lerp(13, 3, 0.5) = 8
    // Centered: x = 26.5 - 9 = 18, y = 26.5 - 4 = 23
    assert_eq!(result, Rect::new(18, 23, 18, 8));
}

#[test]
fn test_expand_calculate_rect_collapsing_at_100() {
    let full_rect = Rect::new(10, 20, 33, 13);
    let frame_area = Rect::new(0, 0, 100, 100);

    let result = calculate_rect(full_rect, frame_area, AnimationPhase::Collapsing, 1.0);

    // At progress 1.0 of collapsing, should be minimum size (3x3) centered
    assert_eq!(result, Rect::new(25, 25, 3, 3));
}

#[test]
fn test_expand_calculate_rect_stays_centered() {
    let full_rect = Rect::new(10, 20, 33, 13);
    let frame_area = Rect::new(0, 0, 100, 100);

    // Test multiple progress values to ensure centering is maintained
    let progress_values = [0.0, 0.25, 0.5, 0.75, 1.0];

    for &progress in &progress_values {
        let result = calculate_rect(full_rect, frame_area, AnimationPhase::Expanding, progress);

        // Calculate expected center
        let full_center_x = full_rect.x as f32 + (full_rect.width as f32 / 2.0);
        let full_center_y = full_rect.y as f32 + (full_rect.height as f32 / 2.0);

        // Calculate actual center
        let result_center_x = result.x as f32 + (result.width as f32 / 2.0);
        let result_center_y = result.y as f32 + (result.height as f32 / 2.0);

        // Centers should be approximately equal (within 0.5 due to rounding)
        assert!((full_center_x - result_center_x).abs() <= 0.5);
        assert!((full_center_y - result_center_y).abs() <= 0.5);
    }
}

#[test]
fn test_expand_calculate_rect_other_phases_return_full() {
    let full_rect = Rect::new(10, 20, 33, 13);
    let frame_area = Rect::new(0, 0, 100, 100);

    // Non-expand/collapse phases should return full_rect
    let result_dwelling = calculate_rect(full_rect, frame_area, AnimationPhase::Dwelling, 0.5);
    assert_eq!(result_dwelling, full_rect);

    let result_fading = calculate_rect(full_rect, frame_area, AnimationPhase::FadingIn, 0.5);
    assert_eq!(result_fading, full_rect);

    let result_pending = calculate_rect(full_rect, frame_area, AnimationPhase::Pending, 0.0);
    assert_eq!(result_pending, full_rect);
}

#[test]
fn test_expand_calculate_rect_with_different_sizes() {
    let frame_area = Rect::new(0, 0, 100, 100);

    // Test with a larger rect
    let large_rect = Rect::new(5, 10, 60, 40);
    let result = calculate_rect(large_rect, frame_area, AnimationPhase::Expanding, 0.5);

    // Should interpolate: lerp(3, 60, 0.5) = 31.5 -> 32, lerp(3, 40, 0.5) = 21.5 -> 22
    // Center: x = 5 + 30 - 16 = 19, y = 10 + 20 - 11 = 19
    assert!(result.width > 3 && result.width < 60);
    assert!(result.height > 3 && result.height < 40);
}

// FILE: tests/test_expand_calculate_rect_integration.rs - Integration tests for expand rect calculation
// END OF VERSION: 1.0.0
