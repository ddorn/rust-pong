use crate::components::{Trail, TrailGenerator};
use amethyst::core::{timing::Time, SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc, Default)]
pub struct TrailGeneratorSystem;

impl<'s> System<'s> for TrailGeneratorSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, Time>,
        WriteStorage<'s, TrailGenerator>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, SpriteRender>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, updater, time, mut trails, transforms, sprites) = data;

        for (trail, tranform, sprite) in
            (&mut trails, &transforms, &sprites).join()
        {
            if trail.need_trail_creation(time.delta_seconds()) {
                updater
                    .create_entity(&entities)
                    .with(tranform.clone())
                    .with(sprite.clone())
                    .with(Trail::new(trail.duration))
                    .build();
            }
        }
    }
}
