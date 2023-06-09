use amethyst::core::math::Vector2;
use amethyst::core::Transform;
use rand::Rng;
use std::f32::consts::PI;

/// Whether a point is included in a rectangle or on its border
pub fn point_in_rect(
    x: f32,
    y: f32,
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
) -> bool {
    left <= x && x <= right && bottom <= y && y <= top
}

/// Linear interpolation between a and b.
/// Return a when t is 0 and b when t is 1.
pub fn lerp(t: f32, a: f32, b: f32) -> f32 {
    (1.0 - t) * a + t * b
}

/// Return a 2D direction vector with a angle from
/// the x axis caped by angle_max
pub fn random_direction(angle_max: f32) -> Vector2<f32> {
    let angle = angle_max.min(PI / 2.0).max(0.0);
    let mut angle = rand::thread_rng().gen_range(-angle, angle);
    if rand::random() {
        // random starting player
        angle += PI;
    }

    Vector2::new(angle.cos(), angle.sin())
}

pub fn pos2d(transform: &Transform) -> Vector2<f32> {
    let trans = transform.translation();
    Vector2::new(trans.x, trans.y)
}
