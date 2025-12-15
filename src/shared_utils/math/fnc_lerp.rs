// FILE: src/shared_utils/math/fnc_lerp.rs - Linear interpolation function
// VERSION: 1.0.0
// WCTX: OFPF migration
// CLOG: Extracted from multiple animation files (slide.rs, expand_collapse.rs, fade.rs)

/// Performs linear interpolation between two values.
///
/// # Arguments
///
/// * `start` - The starting value
/// * `end` - The ending value
/// * `t` - The interpolation parameter (typically 0.0 to 1.0)
///
/// # Returns
///
/// The interpolated value at parameter `t`
///
/// # Examples
///
/// ```ignore
/// // Internal function
/// let result = lerp(0.0, 10.0, 0.5);
/// assert_eq!(result, 5.0);
/// ```
#[inline]
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
}

// FILE: src/shared_utils/math/fnc_lerp.rs - Linear interpolation function
// END OF VERSION: 1.0.0
