// FILE: src/shared_utils/math/mod.rs - Mathematical utility functions
// VERSION: 1.0.0
// WCTX: OFPF migration
// CLOG: Initial creation

mod fnc_lerp;
mod fnc_ease_in_quad;
mod fnc_ease_out_quad;
mod fnc_color_to_rgb;

pub use fnc_lerp::lerp;
pub use fnc_ease_in_quad::ease_in_quad;
pub use fnc_ease_out_quad::ease_out_quad;
pub use fnc_color_to_rgb::color_to_rgb;

// FILE: src/shared_utils/math/mod.rs - Mathematical utility functions
// END OF VERSION: 1.0.0
