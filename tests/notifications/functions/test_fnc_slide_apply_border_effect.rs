// FILE: tests/notifications/functions/test_fnc_slide_apply_border_effect.rs - Tests for slide border effect application
// VERSION: 1.0.0
// WCTX: Implementing slide animation functions with TDD
// CLOG: Initial creation with test cases for border effect application

#[cfg(test)]
mod tests {
    use ratatui::prelude::*;
    use ratatui::symbols::border;
    use ratatui::widgets::{Block, BorderType, Borders};
    use ratatui_notifications::notifications::functions::fnc_slide_apply_border_effect::slide_apply_border_effect;
    use ratatui_notifications::notifications::types::{Anchor, AnimationPhase, SlideDirection};

    #[test]
    fn test_no_effect_during_dwelling() {
        let full_rect = Rect::new(50, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let base_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let base_set = border::ROUNDED;

        let result_block = slide_apply_border_effect(
            base_block.clone(),
            Anchor::MiddleRight,
            SlideDirection::FromRight,
            0.5,
            AnimationPhase::Dwelling,
            full_rect,
            None,
            None,
            frame_area,
            &base_set,
        );

        // During dwelling, no effect should be applied
        assert_eq!(result_block.border_symbols, base_set);
    }

    #[test]
    fn test_no_effect_when_fully_onscreen() {
        let full_rect = Rect::new(50, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let base_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let base_set = border::ROUNDED;

        // Custom path that stays fully within frame
        let custom_start = Some((40.0, 25.0));
        let custom_end = Some((60.0, 25.0));

        let result_block = slide_apply_border_effect(
            base_block.clone(),
            Anchor::MiddleCenter,
            SlideDirection::Default,
            0.5,
            AnimationPhase::SlidingOut,
            full_rect,
            custom_start,
            custom_end,
            frame_area,
            &base_set,
        );

        // When path stays onscreen, no effect should be applied
        assert_eq!(result_block.border_symbols, base_set);
    }

    #[test]
    fn test_effect_applied_when_sliding_from_right() {
        let full_rect = Rect::new(90, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let base_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let base_set = border::ROUNDED;

        let result_block = slide_apply_border_effect(
            base_block.clone(),
            Anchor::MiddleRight,
            SlideDirection::FromRight,
            0.9, // Late in the animation when edge crosses
            AnimationPhase::SlidingOut,
            full_rect,
            None,
            None,
            frame_area,
            &base_set,
        );

        // Effect should modify right edge symbols
        // vertical_right should become " "
        assert_eq!(result_block.border_symbols.vertical_right, " ");
        // top_right should become horizontal_top
        assert_eq!(
            result_block.border_symbols.top_right,
            base_set.horizontal_top
        );
        // bottom_right should become horizontal_bottom
        assert_eq!(
            result_block.border_symbols.bottom_right,
            base_set.horizontal_bottom
        );
    }

    #[test]
    fn test_effect_applied_when_sliding_from_left() {
        let full_rect = Rect::new(5, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let base_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let base_set = border::ROUNDED;

        let result_block = slide_apply_border_effect(
            base_block.clone(),
            Anchor::MiddleLeft,
            SlideDirection::FromLeft,
            0.9, // Late in the animation
            AnimationPhase::SlidingOut,
            full_rect,
            None,
            None,
            frame_area,
            &base_set,
        );

        // Effect should modify left edge symbols
        // vertical_left should become " "
        assert_eq!(result_block.border_symbols.vertical_left, " ");
        // top_left should become horizontal_top
        assert_eq!(
            result_block.border_symbols.top_left,
            base_set.horizontal_top
        );
        // bottom_left should become horizontal_bottom
        assert_eq!(
            result_block.border_symbols.bottom_left,
            base_set.horizontal_bottom
        );
    }

    #[test]
    fn test_no_effect_early_in_slide_out() {
        let full_rect = Rect::new(90, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let base_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let base_set = border::ROUNDED;

        let result_block = slide_apply_border_effect(
            base_block.clone(),
            Anchor::MiddleRight,
            SlideDirection::FromRight,
            0.1, // Early in the animation, before edge crosses
            AnimationPhase::SlidingOut,
            full_rect,
            None,
            None,
            frame_area,
            &base_set,
        );

        // Early in slide-out, no effect yet
        assert_eq!(result_block.border_symbols, base_set);
    }

    #[test]
    fn test_effect_applied_early_in_slide_in_from_right() {
        let full_rect = Rect::new(90, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let base_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let base_set = border::ROUNDED;

        let result_block = slide_apply_border_effect(
            base_block.clone(),
            Anchor::MiddleRight,
            SlideDirection::FromRight,
            0.05, // Early in slide-in, effect should be active
            AnimationPhase::SlidingIn,
            full_rect,
            None,
            None,
            frame_area,
            &base_set,
        );

        // Early in slide-in, effect should be applied
        assert_eq!(result_block.border_symbols.vertical_right, " ");
    }

    #[test]
    fn test_no_effect_late_in_slide_in() {
        let full_rect = Rect::new(90, 25, 20, 10);
        let frame_area = Rect::new(0, 0, 100, 50);
        let base_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let base_set = border::ROUNDED;

        let result_block = slide_apply_border_effect(
            base_block.clone(),
            Anchor::MiddleRight,
            SlideDirection::FromRight,
            0.9, // Late in slide-in, effect should be gone
            AnimationPhase::SlidingIn,
            full_rect,
            None,
            None,
            frame_area,
            &base_set,
        );

        // Late in slide-in, no effect
        assert_eq!(result_block.border_symbols, base_set);
    }
}

// FILE: tests/notifications/functions/test_fnc_slide_apply_border_effect.rs - Tests for slide border effect application
// END OF VERSION: 1.0.0
