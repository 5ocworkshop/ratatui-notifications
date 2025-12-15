// FILE: tests/test_fade_interpolate_color_integration.rs - Integration tests for fade color interpolation
// VERSION: 1.0.0
// WCTX: TDD implementation of animation function extraction
// CLOG: Created standalone integration test to verify fade color interpolation

use ratatui::style::Color;
use ratatui_notifications::notifications::functions::fnc_fade_interpolate_color::{
    interpolate_color, FadeHandler,
};
use ratatui_notifications::notifications::types::AnimationPhase;

#[test]
fn test_interpolate_color_black_to_white_at_0() {
    let result = interpolate_color(Some(Color::Black), Some(Color::White), 0.0, true);
    assert_eq!(result, Some(Color::Rgb(0, 0, 0)));
}

#[test]
fn test_interpolate_color_black_to_white_at_50() {
    let result = interpolate_color(Some(Color::Black), Some(Color::White), 0.5, true);
    // With ease_out_quad at 0.5: 0.5 * (2.0 - 0.5) = 0.75
    // lerp(0, 255, 0.75) = 191.25 -> rounds to 191
    assert_eq!(result, Some(Color::Rgb(191, 191, 191)));
}

#[test]
fn test_interpolate_color_black_to_white_at_100() {
    let result = interpolate_color(Some(Color::Black), Some(Color::White), 1.0, true);
    assert_eq!(result, Some(Color::Rgb(255, 255, 255)));
}

#[test]
fn test_interpolate_color_rgb_values() {
    // Test with custom RGB values
    let from = Some(Color::Rgb(100, 50, 200));
    let to = Some(Color::Rgb(200, 150, 100));

    // At progress 0.0, should be at start
    let result_0 = interpolate_color(from, to, 0.0, true);
    assert_eq!(result_0, Some(Color::Rgb(100, 50, 200)));

    // At progress 1.0, should be at end
    let result_1 = interpolate_color(from, to, 1.0, true);
    assert_eq!(result_1, Some(Color::Rgb(200, 150, 100)));
}

#[test]
fn test_interpolate_color_fading_in_vs_fading_out() {
    let from = Some(Color::Black);
    let to = Some(Color::White);

    // FadingIn uses ease_out_quad
    let fading_in = interpolate_color(from, to, 0.5, true);

    // FadingOut uses ease_in_quad
    let fading_out = interpolate_color(from, to, 0.5, false);

    // These should be different due to different easing
    // ease_out_quad(0.5) = 0.75, ease_in_quad(0.5) = 0.25
    assert_eq!(fading_in, Some(Color::Rgb(191, 191, 191))); // lerp(0, 255, 0.75)
    assert_eq!(fading_out, Some(Color::Rgb(64, 64, 64))); // lerp(0, 255, 0.25)
}

#[test]
fn test_interpolate_color_non_rgb_fallback() {
    // Test with colors that can't be converted to RGB (e.g., Indexed)
    let from = Some(Color::Indexed(1));
    let to = Some(Color::Indexed(2));

    // Should snap at midpoint: < 0.5 -> from, >= 0.5 -> to
    let result_below = interpolate_color(from, to, 0.4, true);
    assert_eq!(result_below, Some(Color::Indexed(1)));

    let result_above = interpolate_color(from, to, 0.5, true);
    assert_eq!(result_above, Some(Color::Indexed(2)));
}

#[test]
fn test_interpolate_color_clamping() {
    // Test that values are clamped within min/max range
    let from = Some(Color::Rgb(100, 100, 100));
    let to = Some(Color::Rgb(200, 200, 200));

    // Even with easing that might overshoot, values should stay within [100, 200]
    let result = interpolate_color(from, to, 1.0, true);
    if let Some(Color::Rgb(r, g, b)) = result {
        assert!(r >= 100 && r <= 200);
        assert!(g >= 100 && g <= 200);
        assert!(b >= 100 && b <= 200);
    } else {
        panic!("Expected RGB color");
    }
}

#[test]
fn test_fade_handler_interpolate_frame_foreground_fading_in() {
    let handler = FadeHandler;
    let base_fg = Some(Color::Rgb(200, 200, 200));

    // FadingIn: goes from Black to base_fg
    let result_0 = handler.interpolate_frame_foreground(base_fg, AnimationPhase::FadingIn, 0.0);
    assert_eq!(result_0, Some(Color::Rgb(0, 0, 0))); // Black

    let result_1 = handler.interpolate_frame_foreground(base_fg, AnimationPhase::FadingIn, 1.0);
    assert_eq!(result_1, Some(Color::Rgb(200, 200, 200))); // base_fg
}

#[test]
fn test_fade_handler_interpolate_frame_foreground_fading_out() {
    let handler = FadeHandler;
    let base_fg = Some(Color::Rgb(200, 200, 200));

    // FadingOut: goes from base_fg to Black
    let result_0 = handler.interpolate_frame_foreground(base_fg, AnimationPhase::FadingOut, 0.0);
    assert_eq!(result_0, Some(Color::Rgb(200, 200, 200))); // base_fg

    let result_1 = handler.interpolate_frame_foreground(base_fg, AnimationPhase::FadingOut, 1.0);
    assert_eq!(result_1, Some(Color::Rgb(0, 0, 0))); // Black
}

#[test]
fn test_fade_handler_interpolate_frame_foreground_other_phases() {
    let handler = FadeHandler;
    let base_fg = Some(Color::Rgb(200, 200, 200));

    // Dwelling phase should return base color (fully visible)
    let result_dwelling = handler.interpolate_frame_foreground(base_fg, AnimationPhase::Dwelling, 0.5);
    assert_eq!(result_dwelling, base_fg);

    // Pending phase should also return base color
    let result_pending = handler.interpolate_frame_foreground(base_fg, AnimationPhase::Pending, 0.5);
    assert_eq!(result_pending, base_fg);
}

#[test]
fn test_fade_handler_interpolate_frame_foreground_sliding_phases() {
    let handler = FadeHandler;
    let base_fg = Some(Color::Rgb(200, 200, 200));

    // SlidingIn should now interpolate (for slide+fade combined animations)
    // At progress 0.0, should be near black
    let result_0 = handler.interpolate_frame_foreground(base_fg, AnimationPhase::SlidingIn, 0.0);
    assert_eq!(result_0, Some(Color::Rgb(0, 0, 0)));

    // At progress 1.0, should be the base color
    let result_1 = handler.interpolate_frame_foreground(base_fg, AnimationPhase::SlidingIn, 1.0);
    assert_eq!(result_1, base_fg);

    // SlidingOut at progress 1.0 should be near black
    let result_out = handler.interpolate_frame_foreground(base_fg, AnimationPhase::SlidingOut, 1.0);
    assert_eq!(result_out, Some(Color::Rgb(0, 0, 0)));
}

#[test]
fn test_fade_handler_interpolate_content_foreground_fading_in() {
    let handler = FadeHandler;

    // Content fading: Black <-> White
    let result_0 = handler.interpolate_content_foreground(None, AnimationPhase::FadingIn, 0.0);
    assert_eq!(result_0, Some(Color::Rgb(0, 0, 0))); // Black

    let result_1 = handler.interpolate_content_foreground(None, AnimationPhase::FadingIn, 1.0);
    assert_eq!(result_1, Some(Color::Rgb(255, 255, 255))); // White
}

#[test]
fn test_fade_handler_interpolate_content_foreground_fading_out() {
    let handler = FadeHandler;

    // Content fading out: White -> Black
    let result_0 = handler.interpolate_content_foreground(None, AnimationPhase::FadingOut, 0.0);
    assert_eq!(result_0, Some(Color::Rgb(255, 255, 255))); // White

    let result_1 = handler.interpolate_content_foreground(None, AnimationPhase::FadingOut, 1.0);
    assert_eq!(result_1, Some(Color::Rgb(0, 0, 0))); // Black
}

#[test]
fn test_fade_handler_interpolate_content_foreground_other_phases() {
    let handler = FadeHandler;

    // Non-fade phases should return base content color (White)
    let result_dwelling = handler.interpolate_content_foreground(None, AnimationPhase::Dwelling, 0.5);
    // The function returns Some(Color::White) directly, not Some(Color::Rgb(255, 255, 255))
    assert_eq!(result_dwelling, Some(Color::White));
}

// FILE: tests/test_fade_interpolate_color_integration.rs - Integration tests for fade color interpolation
// END OF VERSION: 1.0.0
