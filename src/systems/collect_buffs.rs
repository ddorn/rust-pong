use crate::components::{Buff, BuffType, HitBox, Paddle};
use crate::math::pos2d;
use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;

#[derive(SystemDesc)]
pub struct CollectBuffSystem;

impl<'s> System<'s> for CollectBuffSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Paddle>,
        ReadStorage<'s, Buff>,
        ReadStorage<'s, HitBox>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut paddles, buffs, hitboxes, transforms) = data;

        for (entity, buff, hitbox, buff_pos) in
            (&entities, &buffs, &hitboxes, &transforms).join()
        {
            for (paddle, pad_pos) in (&mut paddles, &transforms).join() {
                // Collision
                if paddle.hit(pos2d(pad_pos), pos2d(buff_pos), hitbox.radius) {
                    if buff.side == paddle.side {
                        match buff.buff {
                            BuffType::Speed => paddle.speed += 10.,
                            _ => (),
                        }
                    } else {
                        match buff.buff {
                            BuffType::Speed => paddle.speed -= 5.,
                            _ => (),
                        }
                    }

                    entities.delete(entity).ok();
                }
            }
        }
    }
}
