// FILE: src/shared_utils/math/fnc_ease_out_quad.rs - Quadratic ease-out easing function
// VERSION: 1.0.0
// WCTX: OFPF migration
// CLOG: Extracted from fade.rs

/// Applies quadratic ease-out easing to a linear progress value.
///
/// The ease-out function starts quickly and decelerates toward the end.
///
/// # Arguments
///
/// * `t` - The linear progress value (typically 0.0 to 1.0)
///
/// # Returns
///
/// The eased progress value
///
/// # Examples
///
/// ```ignore
/// // Internal function
/// let result = ease_out_quad(0.5);
/// assert_eq!(result, 0.75);
/// ```
#[inline]
pub fn ease_out_quad(t: f32) -> f32 {
    t * (2.0 - t)
}

// FILE: src/shared_utils/math/fnc_ease_out_quad.rs - Quadratic ease-out easing function
// END OF VERSION: 1.0.0
