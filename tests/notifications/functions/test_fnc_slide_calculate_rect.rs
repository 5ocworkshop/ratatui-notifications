// FILE: tests/notifications/functions/test_fnc_slide_calculate_rect.rs - Tests for slide rectangle calculation
// VERSION: 1.0.0
// WCTX: Implementing slide animation functions with TDD
// CLOG: Initial creation with test cases for slide rectangle calculation

#[cfg(test)]
mod tests {
    use ratatui::prelude::Rect;
    use ratatui_notifications::notifications::functions::fnc_slide_calculate_rect::slide_calculate_rect;
    use ratatui_notifications::notifications::functions::fnc_slide_offscreen_position::slide_offscreen_position;
    use ratatui_notifications::notifications::functions::fnc_slide_resolve_direction::resolve_slide_direction;
    use ratatui_notifications::notifications::types::{Anchor, AnimationPhase, SlideDirection};
    use ratatui_notifications::shared_utils::math::lerp;

    #[test]
    fn test_sliding_in_progress_zero_returns_offscreen() {
        let full_rect = Rect::new(100, 25, 10, 5);
        let frame_area = Rect::new(0, 0, 120, 50);
        let progress = 0.0;
        let phase = AnimationPhase::SlidingIn;
        let slide_direction = SlideDirection::FromRight;
        let custom_start = None;

        let rect = slide_calculate_rect(
            full_rect,
            frame_area,
            progress,
            phase,
            Anchor::MiddleRight,
            slide_direction,
            custom_start,
            None,
        );

        // At progress 0.0, notification should be fully offscreen
        // Width should be 0 or very small due to clipping
        assert_eq!(rect, Rect::default());
    }

    #[test]
    fn test_sliding_in_progress_one_returns_full_rect() {
        let full_rect = Rect::new(100, 25, 10, 5);
        let frame_area = Rect::new(0, 0, 120, 50);
        let progress = 1.0;
        let phase = AnimationPhase::SlidingIn;
        let slide_direction = SlideDirection::FromRight;

        let rect = slide_calculate_rect(
            full_rect,
            frame_area,
            progress,
            phase,
            Anchor::MiddleRight,
            slide_direction,
            None,
            None,
        );

        // At progress 1.0, notification should be at full_rect
        assert_eq!(rect, full_rect);
    }

    #[test]
    fn test_sliding_in_progress_half_returns_interpolated() {
        let full_rect = Rect::new(50, 25, 10, 5);
        let frame_area = Rect::new(0, 0, 100, 50);
        let progress = 0.5;
        let phase = AnimationPhase::SlidingIn;
        let slide_direction = SlideDirection::FromLeft;

        let rect = slide_calculate_rect(
            full_rect,
            frame_area,
            progress,
            phase,
            Anchor::MiddleLeft,
            slide_direction,
            None,
            None,
        );

        // At progress 0.5, should be halfway between start and end
        // The exact rect depends on clipping logic, but should not be empty or full
        assert!(rect.width > 0);
        assert!(rect.width < full_rect.width || rect.x != full_rect.x);
    }

    #[test]
    fn test_sliding_out_progress_zero_returns_full_rect() {
        let full_rect = Rect::new(100, 25, 10, 5);
        let frame_area = Rect::new(0, 0, 120, 50);
        let progress = 0.0;
        let phase = AnimationPhase::SlidingOut;
        let slide_direction = SlideDirection::FromRight;

        let rect = slide_calculate_rect(
            full_rect,
            frame_area,
            progress,
            phase,
            Anchor::MiddleRight,
            slide_direction,
            None,
            None,
        );

        // At progress 0.0 when sliding out, should be at full_rect
        assert_eq!(rect, full_rect);
    }

    #[test]
    fn test_sliding_out_progress_one_returns_offscreen() {
        let full_rect = Rect::new(100, 25, 10, 5);
        let frame_area = Rect::new(0, 0, 120, 50);
        let progress = 1.0;
        let phase = AnimationPhase::SlidingOut;
        let slide_direction = SlideDirection::FromRight;

        let rect = slide_calculate_rect(
            full_rect,
            frame_area,
            progress,
            phase,
            Anchor::MiddleRight,
            slide_direction,
            None,
            None,
        );

        // At progress 1.0 when sliding out, should be offscreen
        assert_eq!(rect, Rect::default());
    }

    #[test]
    fn test_dwelling_phase_returns_full_rect() {
        let full_rect = Rect::new(100, 25, 10, 5);
        let frame_area = Rect::new(0, 0, 120, 50);
        let progress = 0.5; // Progress irrelevant during dwelling
        let phase = AnimationPhase::Dwelling;
        let slide_direction = SlideDirection::FromRight;

        let rect = slide_calculate_rect(
            full_rect,
            frame_area,
            progress,
            phase,
            Anchor::MiddleRight,
            slide_direction,
            None,
            None,
        );

        // During dwelling, should always return full_rect
        assert_eq!(rect, full_rect);
    }

    #[test]
    fn test_custom_start_position_is_used() {
        let full_rect = Rect::new(50, 25, 10, 5);
        let frame_area = Rect::new(0, 0, 100, 50);
        let progress = 0.0;
        let phase = AnimationPhase::SlidingIn;
        let slide_direction = SlideDirection::FromLeft;
        let custom_start = Some((20.0, 30.0)); // Custom start position

        let rect = slide_calculate_rect(
            full_rect,
            frame_area,
            progress,
            phase,
            Anchor::MiddleLeft,
            slide_direction,
            custom_start,
            None,
        );

        // At progress 0.0 with custom start, should clip to custom position
        // Since custom start (20, 30) is onscreen, we should get a non-empty rect
        assert!(rect.x >= 20 || rect.width > 0);
    }

    #[test]
    fn test_custom_end_position_is_used() {
        let full_rect = Rect::new(50, 25, 10, 5);
        let frame_area = Rect::new(0, 0, 100, 50);
        let progress = 1.0;
        let phase = AnimationPhase::SlidingOut;
        let slide_direction = SlideDirection::FromRight;
        let custom_end = Some((80.0, 30.0)); // Custom end position

        let rect = slide_calculate_rect(
            full_rect,
            frame_area,
            progress,
            phase,
            Anchor::MiddleRight,
            slide_direction,
            None,
            custom_end,
        );

        // At progress 1.0 with custom end, should clip to custom position
        // Custom end at (80, 30) is onscreen, so should have some width
        assert!(rect.width > 0);
    }
}

// FILE: tests/notifications/functions/test_fnc_slide_calculate_rect.rs - Tests for slide rectangle calculation
// END OF VERSION: 1.0.0
