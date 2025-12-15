// FILE: src/shared_utils/math/fnc_color_to_rgb.rs - Color to RGB conversion function
// VERSION: 1.0.0
// WCTX: OFPF migration
// CLOG: Extracted from fade.rs

use ratatui::style::Color;

/// Converts a ratatui Color to an RGB tuple.
///
/// This function handles common named colors, grayscale values, and RGB colors.
/// For colors that cannot be converted (like indexed colors or Reset), returns None.
///
/// # Arguments
///
/// * `color` - The optional Color to convert
///
/// # Returns
///
/// An optional tuple of (r, g, b) values in the range 0-255, or None if the color
/// cannot be converted to RGB.
///
/// # Examples
///
/// ```ignore
/// // Internal function
/// use ratatui::style::Color;
/// let rgb = color_to_rgb(Some(Color::Red));
/// assert_eq!(rgb, Some((255, 0, 0)));
/// ```
#[inline]
pub fn color_to_rgb(color: Option<Color>) -> Option<(u8, u8, u8)> {
    match color {
        Some(Color::Black) => Some((0, 0, 0)),
        Some(Color::Red) => Some((255, 0, 0)),
        Some(Color::Green) => Some((0, 255, 0)),
        Some(Color::Yellow) => Some((255, 255, 0)),
        Some(Color::Blue) => Some((0, 0, 255)),
        Some(Color::Magenta) => Some((255, 0, 255)),
        Some(Color::Cyan) => Some((0, 255, 255)),
        Some(Color::Gray) => Some((128, 128, 128)),
        Some(Color::DarkGray) => Some((64, 64, 64)),
        Some(Color::LightRed) => Some((255, 128, 128)),
        Some(Color::LightGreen) => Some((128, 255, 128)),
        Some(Color::LightYellow) => Some((255, 255, 128)),
        Some(Color::LightBlue) => Some((128, 128, 255)),
        Some(Color::LightMagenta) => Some((255, 128, 255)),
        Some(Color::LightCyan) => Some((128, 255, 255)),
        Some(Color::White) => Some((255, 255, 255)),
        Some(Color::Rgb(r, g, b)) => Some((r, g, b)),
        _ => None,
    }
}

// FILE: src/shared_utils/math/fnc_color_to_rgb.rs - Color to RGB conversion function
// END OF VERSION: 1.0.0
