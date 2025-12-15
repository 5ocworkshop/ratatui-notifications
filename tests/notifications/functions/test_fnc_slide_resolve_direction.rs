// FILE: tests/notifications/functions/test_fnc_slide_resolve_direction.rs - Tests for slide direction resolution function
// VERSION: 1.0.0
// WCTX: Implementing slide animation functions with TDD
// CLOG: Initial creation with test cases for direction resolution

#[cfg(test)]
mod tests {
    use ratatui_notifications::notifications::functions::fnc_slide_resolve_direction::resolve_slide_direction;
    use ratatui_notifications::notifications::types::{Anchor, SlideDirection};

    #[test]
    fn test_non_default_direction_returns_unchanged() {
        // When a specific direction is set (not Default), it should return unchanged
        assert_eq!(
            resolve_slide_direction(SlideDirection::FromLeft, Anchor::TopRight),
            SlideDirection::FromLeft
        );
        assert_eq!(
            resolve_slide_direction(SlideDirection::FromRight, Anchor::TopLeft),
            SlideDirection::FromRight
        );
        assert_eq!(
            resolve_slide_direction(SlideDirection::FromTop, Anchor::BottomCenter),
            SlideDirection::FromTop
        );
        assert_eq!(
            resolve_slide_direction(SlideDirection::FromBottom, Anchor::TopCenter),
            SlideDirection::FromBottom
        );
        assert_eq!(
            resolve_slide_direction(SlideDirection::FromTopLeft, Anchor::BottomRight),
            SlideDirection::FromTopLeft
        );
        assert_eq!(
            resolve_slide_direction(SlideDirection::FromTopRight, Anchor::BottomLeft),
            SlideDirection::FromTopRight
        );
        assert_eq!(
            resolve_slide_direction(SlideDirection::FromBottomLeft, Anchor::TopRight),
            SlideDirection::FromBottomLeft
        );
        assert_eq!(
            resolve_slide_direction(SlideDirection::FromBottomRight, Anchor::TopLeft),
            SlideDirection::FromBottomRight
        );
    }

    #[test]
    fn test_default_direction_for_top_left_anchor() {
        assert_eq!(
            resolve_slide_direction(SlideDirection::Default, Anchor::TopLeft),
            SlideDirection::FromTopLeft
        );
    }

    #[test]
    fn test_default_direction_for_top_center_anchor() {
        assert_eq!(
            resolve_slide_direction(SlideDirection::Default, Anchor::TopCenter),
            SlideDirection::FromTop
        );
    }

    #[test]
    fn test_default_direction_for_top_right_anchor() {
        assert_eq!(
            resolve_slide_direction(SlideDirection::Default, Anchor::TopRight),
            SlideDirection::FromTopRight
        );
    }

    #[test]
    fn test_default_direction_for_middle_left_anchor() {
        assert_eq!(
            resolve_slide_direction(SlideDirection::Default, Anchor::MiddleLeft),
            SlideDirection::FromLeft
        );
    }

    #[test]
    fn test_default_direction_for_middle_center_anchor() {
        assert_eq!(
            resolve_slide_direction(SlideDirection::Default, Anchor::MiddleCenter),
            SlideDirection::FromLeft
        );
    }

    #[test]
    fn test_default_direction_for_middle_right_anchor() {
        assert_eq!(
            resolve_slide_direction(SlideDirection::Default, Anchor::MiddleRight),
            SlideDirection::FromRight
        );
    }

    #[test]
    fn test_default_direction_for_bottom_left_anchor() {
        assert_eq!(
            resolve_slide_direction(SlideDirection::Default, Anchor::BottomLeft),
            SlideDirection::FromBottomLeft
        );
    }

    #[test]
    fn test_default_direction_for_bottom_center_anchor() {
        assert_eq!(
            resolve_slide_direction(SlideDirection::Default, Anchor::BottomCenter),
            SlideDirection::FromBottom
        );
    }

    #[test]
    fn test_default_direction_for_bottom_right_anchor() {
        assert_eq!(
            resolve_slide_direction(SlideDirection::Default, Anchor::BottomRight),
            SlideDirection::FromBottomRight
        );
    }
}

// FILE: tests/notifications/functions/test_fnc_slide_resolve_direction.rs - Tests for slide direction resolution function
// END OF VERSION: 1.0.0
