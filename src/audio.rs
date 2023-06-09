use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{output::Output, AudioSink, OggFormat, Source, SourceHandle},
    ecs::{World, WorldExt},
};
use std::{iter::Cycle, vec::IntoIter};

const BOUNCE_SOUND: &str = "audio/bounce.ogg";
const SCORE_SOUND: &str = "audio/score.ogg";

const MUSIC_TRACKS: &[&str] =
    &["audio/wheres_my_jetpack.ogg", "audio/albatross.ogg"];

#[derive(Debug, Copy, Clone)]
pub enum Sound {
    Score,
    Bounce,
}

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle,
}

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

#[derive(Default)]
pub struct SoundQueue {
    pub to_play: Vec<Sound>,
}

impl<'a> Sounds {
    pub fn get(&'a self, sound: Sound) -> &'a SourceHandle {
        match sound {
            Sound::Score => &self.score_sfx,
            Sound::Bounce => &self.bounce_sfx,
        }
    }
}

impl SoundQueue {
    pub fn push(&mut self, sound: Sound) {
        self.to_play.push(sound);
    }
}

/// Loads an ogg audio track.
fn load_audio_track(
    loader: &Loader,
    world: &World,
    file: &str,
) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub fn play_sound(
    sound: &SourceHandle,
    storage: &AssetStorage<Source>,
    output: Option<&Output>,
) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sound) {
            output.play_once(sound, 1.0);
        }
    }
}

pub fn initialise_audio(world: &mut World) {
    let (sound_effects, music) = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.4); // Music is a bit loud, reduce the volume.

        let music = MUSIC_TRACKS
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };

        let sound = Sounds {
            bounce_sfx: load_audio_track(&loader, &world, BOUNCE_SOUND),
            score_sfx: load_audio_track(&loader, &world, SCORE_SOUND),
        };

        (sound, music)
    };

    // Add sound effects to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.insert(SoundQueue { to_play: vec![] });
    world.insert(sound_effects);
    world.insert(music);
}
