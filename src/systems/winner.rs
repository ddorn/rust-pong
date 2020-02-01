use std::ops::Deref;
use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::ui::UiText;
use amethyst::audio::{output::Output, Source};
use amethyst::assets::AssetStorage;
use crate::components::{Ball, Score};
use crate::pong::ScoreText;
use crate::config::ArenaConfig;
use crate::audio::{play_sound, Sounds};


#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        Read<'s, ArenaConfig>,
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        // Score and scoreboard
        Write<'s, Score>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, ScoreText>,
        // Sound
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            arena,
            mut balls,
            transforms,
            mut score,
            mut ui_text,
            score_text,
            storage,
            sounds,
            audio_output,
        ) = data;

        for (ball, transform) in (&mut balls, &transforms).join() {
            if transform.translation().x < 0.0 && ball.direction[0] < 0.0 {
                score.right += 1;

                if let Some(text) = ui_text.get_mut(score_text.right) {
                    text.text = score.right.to_string();
                }
            } else if transform.translation().x > arena.width && ball.direction[0] > 0.0 {
                score.left += 1;

                if let Some(text) = ui_text.get_mut(score_text.left) {
                    text.text = score.left.to_string();
                }
            } else {
                continue
            }

            ball.direction[0] *= -1.0;
            play_sound(&sounds.score_sfx, &storage, audio_output.as_ref().map(|o| o.deref()));
        }

    }
}