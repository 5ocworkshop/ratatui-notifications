// FILE: src/notifications/functions/fnc_fade_interpolate_color.rs - Fade animation color interpolation
// VERSION: 1.1.0
// WCTX: Fixing slide+fade combined animation
// CLOG: Handle SlidingIn/SlidingOut/Expanding/Collapsing phases for combined animations

use crate::notifications::types::AnimationPhase;
use crate::shared_utils::math::{color_to_rgb, ease_in_quad, ease_out_quad, lerp};
use ratatui::style::Color;

// Target color when fully faded out
const FADED_OUT_COLOR: Option<Color> = Some(Color::Black);
// Base color assumed for content text
const BASE_CONTENT_COLOR: Option<Color> = Some(Color::White);

/// Interpolates between two colors using eased RGB lerp if possible, otherwise snaps at midpoint.
///
/// # Arguments
///
/// * `from` - The starting color
/// * `to` - The ending color
/// * `progress` - Linear progress value (0.0 to 1.0)
/// * `is_fading_in` - True if fading in (uses ease_out_quad), false if fading out (uses ease_in_quad)
///
/// # Returns
///
/// The interpolated color, or a snapped color if RGB interpolation is not possible
///
/// # Examples
///
/// ```
/// use ratatui::style::Color;
/// use ratatui_notifications::notifications::functions::fnc_fade_interpolate_color::interpolate_color;
///
/// let result = interpolate_color(Some(Color::Black), Some(Color::White), 0.5, true);
/// // Returns an interpolated gray color based on eased progress
/// ```
pub fn interpolate_color(
    from: Option<Color>,
    to: Option<Color>,
    progress: f32,
    is_fading_in: bool,
) -> Option<Color> {
    let linear_progress = progress.clamp(0.0, 1.0);

    if let (Some((r1, g1, b1)), Some((r2, g2, b2))) = (color_to_rgb(from), color_to_rgb(to)) {
        let eased_progress = if is_fading_in {
            ease_out_quad(linear_progress)
        } else {
            ease_in_quad(linear_progress)
        };

        // Interpolate RGB components using eased progress
        let r_f = lerp(r1 as f32, r2 as f32, eased_progress);
        let g_f = lerp(g1 as f32, g2 as f32, eased_progress);
        let b_f = lerp(b1 as f32, b2 as f32, eased_progress);

        // Clamp results to the min/max range of the start/end points
        let min_r = r1.min(r2);
        let max_r = r1.max(r2);
        let min_g = g1.min(g2);
        let max_g = g1.max(g2);
        let min_b = b1.min(b2);
        let max_b = b1.max(b2);

        let r = (r_f.round() as u8).clamp(min_r, max_r);
        let g = (g_f.round() as u8).clamp(min_g, max_g);
        let b = (b_f.round() as u8).clamp(min_b, max_b);

        Some(Color::Rgb(r, g, b))
    } else {
        // Fall back to snapping at the midpoint
        if linear_progress < 0.5 {
            from
        } else {
            to
        }
    }
}

/// Handler struct for fade color interpolation operations.
///
/// This struct provides methods for interpolating frame foreground and content foreground
/// colors during fade animations.
#[derive(Debug, Clone, Copy)]
pub struct FadeHandler;

impl FadeHandler {
    /// Calculates the interpolated foreground color for frame elements (borders, titles).
    ///
    /// # Arguments
    ///
    /// * `base_fg` - The base foreground color
    /// * `phase` - The current animation phase
    /// * `progress` - Animation progress (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// The interpolated color for the current animation state
    pub fn interpolate_frame_foreground(
        &self,
        base_fg: Option<Color>,
        phase: AnimationPhase,
        progress: f32,
    ) -> Option<Color> {
        let is_fading_in = matches!(phase, AnimationPhase::FadingIn | AnimationPhase::SlidingIn | AnimationPhase::Expanding);
        let (start_fg, end_fg) = match phase {
            AnimationPhase::FadingIn | AnimationPhase::SlidingIn | AnimationPhase::Expanding => {
                (FADED_OUT_COLOR, base_fg)
            }
            AnimationPhase::FadingOut | AnimationPhase::SlidingOut | AnimationPhase::Collapsing => {
                (base_fg, FADED_OUT_COLOR)
            }
            _ => return base_fg,
        };
        interpolate_color(start_fg, end_fg, progress, is_fading_in)
    }

    /// Calculates the interpolated foreground color for content text (White <-> Black).
    ///
    /// # Arguments
    ///
    /// * `_base_fg` - The base foreground color (ignored for content, which uses White)
    /// * `phase` - The current animation phase
    /// * `progress` - Animation progress (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// The interpolated color for content text
    pub fn interpolate_content_foreground(
        &self,
        _base_fg: Option<Color>,
        phase: AnimationPhase,
        progress: f32,
    ) -> Option<Color> {
        let is_fading_in = matches!(phase, AnimationPhase::FadingIn | AnimationPhase::SlidingIn | AnimationPhase::Expanding);
        let (start_fg, end_fg) = match phase {
            AnimationPhase::FadingIn | AnimationPhase::SlidingIn | AnimationPhase::Expanding => {
                (FADED_OUT_COLOR, BASE_CONTENT_COLOR)
            }
            AnimationPhase::FadingOut | AnimationPhase::SlidingOut | AnimationPhase::Collapsing => {
                (BASE_CONTENT_COLOR, FADED_OUT_COLOR)
            }
            _ => return BASE_CONTENT_COLOR,
        };
        interpolate_color(start_fg, end_fg, progress, is_fading_in)
    }
}

// FILE: src/notifications/functions/fnc_fade_interpolate_color.rs - Fade animation color interpolation
// END OF VERSION: 1.1.0
