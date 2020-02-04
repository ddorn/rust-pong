use crate::audio::{Sound, SoundQueue};
use crate::components::{Ball, Score, StraightMover};
use crate::config::ArenaConfig;
use crate::pong::ScoreText;
use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::ui::UiText;

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        Read<'s, ArenaConfig>,
        WriteStorage<'s, StraightMover>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        // Score and scoreboard
        Write<'s, Score>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, ScoreText>,
        // Sound
        Write<'s, SoundQueue>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            arena,
            mut balls,
            transforms,
            ball_tags,
            mut score,
            mut ui_text,
            score_text,
            mut sound_queue,
        ) = data;

        for (ball, transform, _) in (&mut balls, &transforms, &ball_tags).join()
        {
            if transform.translation().x < 0.0 && ball.direction[0] < 0.0 {
                score.right += 1;

                if let Some(text) = ui_text.get_mut(score_text.right) {
                    text.text = score.right.to_string();
                }
            } else if transform.translation().x > arena.width
                && ball.direction[0] > 0.0
            {
                score.left += 1;

                if let Some(text) = ui_text.get_mut(score_text.left) {
                    text.text = score.left.to_string();
                }
            } else {
                continue;
            }

            // We don't put the ball back to the center
            // Instead we just make it bounce
            // to keep the speed of the game
            ball.direction[0] *= -1.0;
            sound_queue.push(Sound::Score);
        }
    }
}
