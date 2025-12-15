// FILE: src/notifications/orc_render.rs - Orchestrates notification rendering
// VERSION: 1.1.0
// WCTX: Fixing fade animation overshoot bug
// CLOG: Fixed fade discontinuity by applying fade interpolation during Dwelling phase

use crate::notifications::functions::fnc_get_level_icon::get_level_icon;
use crate::notifications::functions::fnc_resolve_styles::resolve_styles;
use crate::notifications::orc_stacking::calculate_stacking_positions;
use crate::notifications::types::{Anchor, AnimationPhase, Level};
use ratatui::{
    layout::Alignment,
    prelude::*,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};
use std::collections::HashMap;

/// Trait for renderable notification state.
///
/// This trait defines the interface for notification states that can be rendered.
/// It extends StackableNotification with additional rendering requirements.
pub trait RenderableNotification: crate::notifications::orc_stacking::StackableNotification {
    fn level(&self) -> Option<Level>;
    fn title(&self) -> Option<Line<'static>>;
    fn content(&self) -> Text<'static>;
    fn border_type(&self) -> BorderType;
    fn fade_effect(&self) -> bool;
    fn animation_type(&self) -> crate::notifications::types::Animation;
    fn animation_progress(&self) -> f32;
    fn block_style(&self) -> Option<Style>;
    fn border_style(&self) -> Option<Style>;
    fn title_style(&self) -> Option<Style>;
    fn padding(&self) -> ratatui::widgets::Padding;
    fn set_full_rect(&mut self, rect: Rect);

    // Animation handler methods - avoid dyn compatibility issues by including them directly
    fn calculate_animation_rect(&self, frame_area: Rect) -> Rect;
    fn apply_animation_block_effect<'a>(&self, block: Block<'a>, frame_area: Rect, base_set: &border::Set) -> Block<'a>;
    fn interpolate_frame_foreground(&self, base_fg: Option<Color>, phase: AnimationPhase, progress: f32) -> Option<Color>;
    fn interpolate_content_foreground(&self, base_fg: Option<Color>, phase: AnimationPhase, progress: f32) -> Option<Color>;
}

/// Renders all notifications to the frame.
///
/// This is the main orchestration function that:
/// 1. Iterates through each anchor's notifications
/// 2. Calls calculate_stacking_positions for each anchor
/// 3. For each stacked notification:
///    - Updates state.full_rect with stacked position
///    - Gets animation handler and calculates current rect
///    - Resolves styles
///    - Applies fade effect if enabled
///    - Builds Block with border, title, icon
///    - Renders Clear at stacked position, then Paragraph at animated position
///
/// # Arguments
///
/// * `notifications` - Mutable HashMap of all notification states
/// * `notifications_by_anchor` - Mapping of anchors to notification IDs
/// * `frame` - The frame to render to
/// * `max_concurrent` - Optional limit on concurrent visible notifications
///
/// # Type Parameters
///
/// * `T` - Any type implementing RenderableNotification trait
pub fn render_notifications<T: RenderableNotification>(
    notifications: &mut HashMap<u64, T>,
    notifications_by_anchor: &HashMap<Anchor, Vec<u64>>,
    frame: &mut Frame<'_>,
    max_concurrent: Option<usize>,
) {
    let frame_area = frame.area();

    for (anchor, ids_at_anchor) in notifications_by_anchor.iter() {
        if ids_at_anchor.is_empty() {
            continue;
        }

        // Calculate stacking positions for this anchor
        let stacked_notifications = calculate_stacking_positions(
            notifications,
            *anchor,
            ids_at_anchor,
            frame_area,
            max_concurrent,
        );

        // Render each stacked notification
        for stacked in stacked_notifications {
            if let Some(state) = notifications.get_mut(&stacked.id) {
                // Update the state's full_rect with stacked position
                state.set_full_rect(stacked.rect);

                // Calculate current rect using animation
                let current_rect = state.calculate_animation_rect(frame_area);

                if current_rect.width == 0 || current_rect.height == 0 {
                    continue;
                }

                // Resolve styles
                let (base_block_style, base_border_style, base_title_style) = resolve_styles(
                    state.level(),
                    state.block_style(),
                    state.border_style(),
                    state.title_style(),
                );

                // Apply fade effect if enabled
                let (final_block_style, final_border_style, final_title_style, final_content_style) =
                    apply_fade_if_needed(
                        state,
                        base_block_style,
                        base_border_style,
                        base_title_style,
                    );

                // Build the block
                let mut block = Block::default()
                    .style(final_block_style)
                    .borders(Borders::ALL)
                    .border_type(state.border_type())
                    .border_style(final_border_style)
                    .padding(state.padding());

                // Add title with icon if present
                if let Some(mut title_line) = state.title() {
                    if let Some(icon_str) = get_level_icon(state.level()) {
                        let icon_span = Span::styled(icon_str, final_border_style);
                        title_line.spans.insert(0, icon_span);
                    }
                    block = block.title(title_line.alignment(Alignment::Center).style(final_title_style));
                }

                // Apply block effect from animation
                let border_set = get_border_set(state.border_type());
                block = state.apply_animation_block_effect(block, frame_area, &border_set);

                // Create the paragraph
                let paragraph = Paragraph::new(state.content())
                    .wrap(Wrap { trim: true })
                    .style(final_content_style)
                    .block(block);

                // Render: Clear at stacked position, then Paragraph at animated position
                if stacked.rect.width > 0 && stacked.rect.height > 0 {
                    frame.render_widget(Clear, stacked.rect.intersection(frame_area));
                }
                frame.render_widget(paragraph, current_rect);
            }
        }
    }
}

/// Helper to apply fade effect if needed
fn apply_fade_if_needed<T: RenderableNotification>(
    state: &T,
    base_block_style: Style,
    base_border_style: Style,
    base_title_style: Style,
) -> (Style, Style, Style, Style) {
    use crate::notifications::types::Animation;

    let apply_fade = state.fade_effect() || matches!(state.animation_type(), Animation::Fade);
    let is_in_anim_phase = matches!(
        state.current_phase(),
        AnimationPhase::FadingIn
            | AnimationPhase::FadingOut
            | AnimationPhase::SlidingIn
            | AnimationPhase::SlidingOut
            | AnimationPhase::Expanding
            | AnimationPhase::Collapsing
    );
    let is_dwelling = matches!(state.current_phase(), AnimationPhase::Dwelling);

    if apply_fade && (is_in_anim_phase || is_dwelling) {
        let phase = state.current_phase();
        // For dwelling phase, use progress=1.0 to get the final interpolated color
        // This prevents a jarring discontinuity when transitioning from FadingIn to Dwelling
        let progress = if is_dwelling { 1.0 } else { state.animation_progress() };
        // Use FadingIn phase for dwelling to get the "fully visible" colors
        let effective_phase = if is_dwelling { AnimationPhase::FadingIn } else { phase };

        let effective_base_frame_fg = base_title_style
            .fg
            .or(base_border_style.fg)
            .or(base_block_style.fg);

        let frame_fg = state.interpolate_frame_foreground(effective_base_frame_fg, effective_phase, progress);
        let content_fg = state.interpolate_content_foreground(None, effective_phase, progress);

        let frame_fade_override = Style::default().fg(frame_fg.unwrap_or(Color::Reset));
        let content_fade_override = Style::default().fg(content_fg.unwrap_or(Color::Reset));

        (
            base_block_style.patch(frame_fade_override),
            base_border_style.patch(frame_fade_override),
            base_title_style.patch(frame_fade_override),
            base_block_style.patch(content_fade_override),
        )
    } else {
        (
            base_block_style,
            base_border_style,
            base_title_style,
            base_block_style,
        )
    }
}

/// Helper to get border set from border type
fn get_border_set(border_type: BorderType) -> border::Set {
    match border_type {
        BorderType::Plain => border::PLAIN,
        BorderType::Rounded => border::ROUNDED,
        BorderType::Double => border::DOUBLE,
        BorderType::Thick => border::THICK,
        _ => border::PLAIN,
    }
}


// FILE: src/notifications/orc_render.rs - Orchestrates notification rendering
// END OF VERSION: 1.1.0
