// FILE: tests/notifications/functions/test_fnc_slide_offscreen_position.rs - Tests for slide offscreen position calculation
// VERSION: 1.0.0
// WCTX: Implementing slide animation functions with TDD
// CLOG: Initial creation with test cases for offscreen position calculation

#[cfg(test)]
mod tests {
    use ratatui::prelude::Rect;
    use ratatui_notifications::notifications::functions::fnc_slide_offscreen_position::slide_offscreen_position;
    use ratatui_notifications::notifications::types::{Anchor, SlideDirection};

    #[test]
    fn test_from_left_returns_position_left_of_frame() {
        let full_rect = Rect::new(50, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let (x, y) = slide_offscreen_position(
            Anchor::MiddleLeft,
            SlideDirection::FromLeft,
            full_rect,
            frame_area,
        );
        // Should be positioned left of frame: frame_x - width - margin
        // 0 - 20 - 1 = -21
        assert_eq!(x, -21.0);
        assert_eq!(y, full_rect.y as f32); // Y should match full_rect.y
    }

    #[test]
    fn test_from_right_returns_position_right_of_frame() {
        let full_rect = Rect::new(50, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let (x, y) = slide_offscreen_position(
            Anchor::MiddleRight,
            SlideDirection::FromRight,
            full_rect,
            frame_area,
        );
        // Should be positioned right of frame: frame_right + margin
        // 100 + 1 = 101
        assert_eq!(x, 101.0);
        assert_eq!(y, full_rect.y as f32);
    }

    #[test]
    fn test_from_top_returns_position_above_frame() {
        let full_rect = Rect::new(50, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let (x, y) = slide_offscreen_position(
            Anchor::TopCenter,
            SlideDirection::FromTop,
            full_rect,
            frame_area,
        );
        // Should be positioned above frame: frame_y - height - margin
        // 0 - 10 - 1 = -11
        assert_eq!(x, full_rect.x as f32); // X should match full_rect.x
        assert_eq!(y, -11.0);
    }

    #[test]
    fn test_from_bottom_returns_position_below_frame() {
        let full_rect = Rect::new(50, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let (x, y) = slide_offscreen_position(
            Anchor::BottomCenter,
            SlideDirection::FromBottom,
            full_rect,
            frame_area,
        );
        // Should be positioned below frame: frame_bottom + margin
        // 50 + 1 = 51
        assert_eq!(x, full_rect.x as f32);
        assert_eq!(y, 51.0);
    }

    #[test]
    fn test_from_top_left_returns_diagonal_position() {
        let full_rect = Rect::new(50, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let (x, y) = slide_offscreen_position(
            Anchor::TopLeft,
            SlideDirection::FromTopLeft,
            full_rect,
            frame_area,
        );
        // Should be positioned both left and above frame
        assert_eq!(x, -21.0); // 0 - 20 - 1
        assert_eq!(y, -11.0); // 0 - 10 - 1
    }

    #[test]
    fn test_from_top_right_returns_diagonal_position() {
        let full_rect = Rect::new(50, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let (x, y) = slide_offscreen_position(
            Anchor::TopRight,
            SlideDirection::FromTopRight,
            full_rect,
            frame_area,
        );
        // Should be positioned both right and above frame
        assert_eq!(x, 101.0); // 100 + 1
        assert_eq!(y, -11.0); // 0 - 10 - 1
    }

    #[test]
    fn test_from_bottom_left_returns_diagonal_position() {
        let full_rect = Rect::new(50, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let (x, y) = slide_offscreen_position(
            Anchor::BottomLeft,
            SlideDirection::FromBottomLeft,
            full_rect,
            frame_area,
        );
        // Should be positioned both left and below frame
        assert_eq!(x, -21.0); // 0 - 20 - 1
        assert_eq!(y, 51.0); // 50 + 1
    }

    #[test]
    fn test_from_bottom_right_returns_diagonal_position() {
        let full_rect = Rect::new(50, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let (x, y) = slide_offscreen_position(
            Anchor::BottomRight,
            SlideDirection::FromBottomRight,
            full_rect,
            frame_area,
        );
        // Should be positioned both right and below frame
        assert_eq!(x, 101.0); // 100 + 1
        assert_eq!(y, 51.0); // 50 + 1
    }

    #[test]
    fn test_default_direction_returns_full_rect_position() {
        let full_rect = Rect::new(50, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let (x, y) = slide_offscreen_position(
            Anchor::MiddleCenter,
            SlideDirection::Default,
            full_rect,
            frame_area,
        );
        // Default should return the full_rect's position
        assert_eq!(x, full_rect.x as f32);
        assert_eq!(y, full_rect.y as f32);
    }
}

// FILE: tests/notifications/functions/test_fnc_slide_offscreen_position.rs - Tests for slide offscreen position calculation
// END OF VERSION: 1.0.0
