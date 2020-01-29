use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Write, System, SystemData, World, WriteStorage, ReadStorage, ReadExpect};
use amethyst::ui::UiText;
use crate::components::{Ball, Score};
use crate::pong::{ARENA_WIDTH, ScoreText};


#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        Write<'s, Score>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, (
        mut balls,
        transforms,
        mut score,
        mut ui_text,
        score_text): Self::SystemData) {

        for (ball, transform) in (&mut balls, &transforms).join() {
            if transform.translation().x < 0.0 && ball.velocity[0] < 0.0 {
                score.right += 1;
                ball.velocity[0] *= -1.0;

                if let Some(text) = ui_text.get_mut(score_text.right) {
                    text.text = score.right.to_string();
                }
            } else if transform.translation().x > ARENA_WIDTH && ball.velocity[0] > 0.0 {
                score.left += 1;
                ball.velocity[0] *= -1.0;


                if let Some(text) = ui_text.get_mut(score_text.left) {
                    text.text = score.left.to_string();
                }
            }
        }

    }
}