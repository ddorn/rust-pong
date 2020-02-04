use std::f32::consts::PI;

use amethyst::{
    core::{math::Vector2, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::*,
};

use crate::audio::{Sound, SoundQueue};
use crate::components::{HitBox, Paddle, Side, StraightMover};
use crate::config::BallConfig;
use crate::math::lerp;

#[derive(SystemDesc)]
pub struct PaddleBounceSystem;

impl<'s> System<'s> for PaddleBounceSystem {
    type SystemData = (
        Read<'s, BallConfig>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, StraightMover>,
        ReadStorage<'s, HitBox>,
        ReadStorage<'s, Paddle>,
        Write<'s, SoundQueue>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            balls_config,
            transforms,
            mut velocities,
            hitboxes,
            paddles,
            mut sound_queue,
        ) = data;

        for (vel, transform, hitbox) in
            (&mut velocities, &transforms, &hitboxes).join()
        {
            let x: f32 = transform.translation().x;
            let y: f32 = transform.translation().y;
            let r: f32 = hitbox.radius;

            for (paddle, position) in (&paddles, &transforms).join() {
                let paddle_x: f32 = position.translation().x;
                let paddle_y: f32 = position.translation().y;
                let paddle_pos = (paddle_x, paddle_y);

                if paddle.hit(paddle_pos, (x, y), r)
                    && ((paddle.side == Side::Left && vel.direction[0] < 0.0)
                        || (paddle.side == Side::Right
                            && vel.direction[0] > 0.0))
                {
                    // We make the ball bounce with a different angle depending where it landed
                    let hit_prop = ((y - paddle_y) / paddle.height + 0.5)
                        .min(1.0) // Clamp
                        .max(0.0);

                    // The new angle is between -45 and 45 degrees for each bat
                    let a = PI / 4.0;
                    let slope = match paddle.side {
                        Side::Left => lerp(hit_prop, -a, a),
                        Side::Right => lerp(hit_prop, PI + a, PI - a),
                    };
                    vel.direction = Vector2::new(slope.cos(), slope.sin());

                    // And accelerate the ball linearly
                    vel.speed += balls_config.bounce_acceleration;
                    // Cap the speed though
                    vel.speed = vel.speed.min(balls_config.max_speed);

                    sound_queue.push(Sound::Bounce);
                }
            }
        }
    }
}
