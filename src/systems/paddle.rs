
use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::pong::{ARENA_HEIGHT, PADDLE_HEIGHT};
use crate::components::{Paddle, Side};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };

            if let Some(mv_amount) = movement {
                let mv_amount = 3.0 * mv_amount;
                if mv_amount != 0.0 {
                    let y = transform.translation().y;

                    transform.set_translation_y(
                        (y + mv_amount)
                            .max(PADDLE_HEIGHT * 0.5)
                            .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                    );
                }
            }
        }
    }
}