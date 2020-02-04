use crate::components::StraightMover;
use amethyst::core::{timing::Time, SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;

#[derive(SystemDesc)]
pub struct MoveStraightSystem;

impl<'s> System<'s> for MoveStraightSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, StraightMover>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut transforms, mut straight_mover, time) = data;
        for (transform, velocity) in
            (&mut transforms, &mut straight_mover).join()
        {
            transform.prepend_translation_x(
                velocity.direction[0] * velocity.speed * time.delta_seconds(),
            );
            transform.prepend_translation_y(
                velocity.direction[1] * velocity.speed * time.delta_seconds(),
            );
        }
    }
}
