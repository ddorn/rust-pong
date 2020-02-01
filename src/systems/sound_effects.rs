use std::ops::Deref;
use amethyst::{
    ecs::prelude::*,
    audio::{output::Output, Source},
    assets::AssetStorage,
    core::SystemDesc,
    derive::SystemDesc,
};

use crate::audio::{SoundQueue, Sounds, play_sound};


#[derive(SystemDesc)]
pub struct SoundEffectsSystem;


impl <'s> System<'s> for SoundEffectsSystem {
    type SystemData = (
        Write<'s, SoundQueue>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut sound_effects,
            storage,
            sounds,
            audio_output,
        ) = data;

        for &sound in &sound_effects.to_play {
            let sound = sounds.get(sound);
            play_sound(&sound,
                       &storage,
                       audio_output.as_ref().map(|o| o.deref()));
        }

        sound_effects.to_play.clear();
    }
}