/// Whether a point is included in a rectangle or on its border
pub fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    left <= x && x <= right && bottom <= y && y <= top
}

/// Linear interpolation between a and b.
/// Return a when t is 0 and b when t is 1.
pub fn lerp(t: f32, a: f32, b: f32) -> f32 {
    (1.0 - t) * a + t * b
}
