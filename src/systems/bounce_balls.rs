use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, World, WriteStorage};
use crate::components::{Ball, Paddle};
use crate::pong::{ARENA_HEIGHT, ARENA_WIDTH};

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
//            let transform : Transform = transform;
            let x: f32 = transform.translation().x;
            let y: f32 = transform.translation().y;
            let r: f32 = ball.radius;

            if y < r || y > ARENA_HEIGHT - r{
                ball.velocity[1] *= -1.0;
            }

            if x < r || x > ARENA_WIDTH - r {
                ball.velocity[0] *= -1.0;
            }
        }
    }
}