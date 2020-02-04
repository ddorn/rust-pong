use amethyst::{
    ecs::prelude::*,
    core::{Transform, SystemDesc},
    derive::SystemDesc,
};

use crate::components::{StraightMover, WallBouncer, HitBox};
use crate::config::{ArenaConfig};

#[derive(SystemDesc)]
pub struct WallBounceSystem;

impl<'s> System<'s> for WallBounceSystem {
    type SystemData = (
        Read<'s, ArenaConfig>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, StraightMover>,
        ReadStorage<'s, WallBouncer>,
        ReadStorage<'s, HitBox>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            arena,
            transforms,
            mut velocities,
            wall_bouncers,
            hitboxes,
        ) = data;

        for (vel, transform, walls, hitbox) in (&mut velocities, &transforms, &wall_bouncers, &hitboxes).join() {
            let x: f32 = transform.translation().x;
            let y: f32 = transform.translation().y;
            let r = hitbox.radius;

            // Bounce against top and bottom walls
            if walls.horizontal {
                if (y < r && vel.direction[1] < 0.0)
                    || (y > arena.height - r && vel.direction[1] > 0.0) {
                    vel.direction[1] *= -1.0;
                }
            }

            // Bounce against left and right walls
            if walls.vertical {
                if (x < r && vel.direction[0] < 0.0)
                    || (x > arena.width - r && vel.direction[0] > 0.0) {
                    vel.direction[0] *= -1.0;
                }
            }
        }
    }
}
