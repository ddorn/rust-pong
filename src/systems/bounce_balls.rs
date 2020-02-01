use amethyst::core::{Transform, SystemDesc, math::Vector2};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use crate::components::{Ball, Paddle, Side};
use crate::config::{ArenaConfig, BallConfig};
use std::f32::consts::PI;

#[derive(SystemDesc)]
pub struct BounceBallsSystem;

impl<'s> System<'s> for BounceBallsSystem {
    type SystemData = (
        Read<'s, ArenaConfig>,
        Read<'s, BallConfig>,
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Paddle>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (arena,
            balls_config,
            mut balls,
            transforms,
            paddles) = data;

        for (ball, transform) in (&mut balls, &transforms).join() {
            let x: f32 = transform.translation().x;
            let y: f32 = transform.translation().y;
            let r: f32 = ball.radius;

            // Bounce against top and bottom walls
            if (y < r && ball.direction[1] < 0.0)
                || (y > arena.height - r && ball.direction[1] > 0.0) {
                ball.direction[1] *= -1.0;
            }

            for (paddle, position) in (&paddles, &transforms).join() {
                let paddle_x: f32 = position.translation().x;
                let paddle_y: f32 = position.translation().y;

                // To determine whether the ball has collided with a paddle, we create a larger
                // rectangle around the current one, by subtracting the ball radius from the
                // lowest coordinates, and adding the ball radius to the highest ones. The ball
                // is then within the paddle if its center is within the larger wrapper
                // rectangle.
                if point_in_rect(
                    x, y,
                    paddle_x - paddle.width / 2.0 - r,
                    paddle_y - paddle.height / 2.0 - r,
                    paddle_x + paddle.width / 2.0 + r,
                    paddle_y + paddle.height / 2.0 + r,
                ) {
                    if (paddle.side == Side::Left && ball.direction[0] < 0.0)
                        || (paddle.side == Side::Right && ball.direction[0] > 0.0) {

                        // We make the ball bounce with a different angle depending where it landed
                        let hit_prop = ((y - paddle_y) / paddle.height + 0.5)
                            .min(1.0)  // Clamp
                            .max(0.0);

                        // The new angle is between -45 and 45 degrees for each bat
                        let a = PI / 4.0;
                        let slope = match paddle.side {
                            Side::Left => lerp(hit_prop, -a, a),
                            Side::Right => lerp(hit_prop, PI + a, PI - a),
                        };
                        ball.direction = Vector2::new(slope.cos(), slope.sin());

                        // And accelerate the ball linearly
                        ball.speed += balls_config.bounce_acceleration;
                        // Cap the speed though
                        ball.speed = ball.speed.min(balls_config.max_speed);
                    }
                }
            }
        }
    }
}

/// Whether a point is included in a rectangle or on its border
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    left <= x && x <= right && bottom <= y && y <= top
}

/// Linear interpolation between a and b.
/// Return a when t is 0 and b when t is 1.
fn lerp(t: f32, a: f32, b: f32) -> f32 {
    (1.0 - t) * a + t * b
}
