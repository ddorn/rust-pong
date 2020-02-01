
use amethyst::core::{Transform, SystemDesc, timing::Time};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, World, WriteStorage};
use crate::components::Ball;


#[derive(SystemDesc)]
pub struct  MoveBallsSystem;


impl <'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Ball>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut transforms, mut balls, time) : Self::SystemData) {
        for (transform, ball) in (&mut transforms, &mut balls).join() {
            transform.prepend_translation_x(ball.direction[0] * ball.speed * time.delta_seconds());
            transform.prepend_translation_y(ball.direction[1] * ball.speed * time.delta_seconds());
        }
    }
}