
use amethyst::core::{SystemDesc, timing::Time, math::Vector3};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::assets::Handle;
use amethyst::renderer::{SpriteRender, SpriteSheet};
use crate::components::{StraightMover, WallBouncer, Side, Buff, BuffType, HitBox};
use crate::config::ArenaConfig;
use crate::math::random_direction;
use std::f32::consts::PI;


#[derive(SystemDesc, Default)]
pub struct BuffSpawnSystem {
    last_spwan: f32,
    delay: f32,
}

impl BuffSpawnSystem {
    pub fn new(delay: f32) -> BuffSpawnSystem {
        BuffSpawnSystem {
            last_spwan: 0.0,
            delay: delay,
        }
    }
}


impl <'s> System<'s> for BuffSpawnSystem {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, ArenaConfig>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
        ReadExpect<'s, Handle<SpriteSheet>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            time,
            arena,
            entities,
            updater,
            sprite_sheet,
        ) = data;

        self.last_spwan += time.delta_seconds();

        if self.last_spwan >= self.delay {
            self.last_spwan -= self.delay;

            let velocity = StraightMover{
                direction: random_direction(PI / 3.0),
                speed: 50.0,
            };

            let (side, sprite_id) = if rand::random() {
                (Side::Left, 6)
            } else {
                (Side::Left, 4)
            };

            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: sprite_id,
            };

            let buff_size = 4.0;
            let scale = buff_size / 9.0;

            let mut transform = arena.center();
            transform.set_scale(Vector3::new(1., 1., 1.) * scale);

            updater.create_entity(&entities)
                .with(transform)
                .with(velocity)
                .with(sprite_render)
                .with(Buff { side, buff: BuffType::Speed })
                .with(WallBouncer::all())
                .with(HitBox::new(2.))
                .build();
        }
    }
}