use crate::components::Trail;
use amethyst::core::{timing::Time, math::Vector3, SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;

#[derive(SystemDesc, Default)]
pub struct TrailsSystem;

impl<'s> System<'s> for TrailsSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        WriteStorage<'s, Trail>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, time, mut trails, mut transforms) = data;

        for (entity, trail, transform) in
            (&entities, &mut trails, &mut transforms).join()
        {

            if trail.update(time.delta_seconds()) {
                transform.set_scale(Vector3::new(1.0, 1.0, 1.0) * (1.0 - trail.prop()));
            } else {
                entities.delete(entity).ok();
            }
        }
    }
}
