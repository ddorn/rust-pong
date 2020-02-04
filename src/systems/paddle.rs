use amethyst::core::{timing::Time, SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    Join, Read, ReadStorage, System, SystemData, World, WriteStorage,
};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::{Paddle, Side};
use crate::config::{ArenaConfig};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        Read<'s, ArenaConfig>,
        Read<'s, Time>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (arena, time, mut transforms, paddles, input) = data;

        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };

            if let Some(mv_amount) = movement {
                let mv_amount = mv_amount * paddle.speed * time.delta_seconds();
                if mv_amount != 0.0 {
                    let y = transform.translation().y;

                    transform.set_translation_y(
                        (y + mv_amount)
                            .max(paddle.height * 0.5)
                            .min(arena.height - paddle.height * 0.5),
                    );
                }
            }
        }
    }
}
