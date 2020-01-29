use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, World, WriteStorage};
use crate::components::{Ball, Paddle, Side};
use crate::pong::{ARENA_HEIGHT};

#[derive(SystemDesc)]
pub struct BounceBallsSystem;

impl<'s> System<'s> for BounceBallsSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Paddle>
    );

    fn run(&mut self, (mut balls, transforms, paddles): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let x: f32 = transform.translation().x;
            let y: f32 = transform.translation().y;
            let r: f32 = ball.radius;

            if (y < r && ball.velocity[1] < 0.0)
                || (y > ARENA_HEIGHT - r && ball.velocity[1] > 0.0)
            {
                ball.velocity[1] *= -1.0;
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
                    if (paddle.side == Side::Left && ball.velocity[0] < 0.0)
                        || (paddle.side == Side::Right && ball.velocity[0] > 0.0) {
                        ball.velocity[0] *= -1.0;
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