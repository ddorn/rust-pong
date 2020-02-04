//! Pong Tutorial 1

use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod audio;
mod components;
mod config;
mod math;
mod pong;
mod systems;
use crate::audio::Music;
use crate::config::PongConfig;
use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    // Start the default logger to see errors and warnings
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_dir = app_root.join("config");
    let assets_dir = app_root.join("assets");
    let display_config_path = config_dir.join("display.ron");
    let bindings_path = config_dir.join("bindings.ron");
    let config_path = config_dir.join("config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(bindings_path)?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(
            systems::MoveStraightSystem,
            "move_straight",
            &["paddle_system"],
        )
        .with(
            systems::PaddleBounceSystem,
            "bounce_balls_system",
            &["paddle_system", "move_straight"],
        )
        .with(systems::WinnerSystem, "winner_system", &["move_straight"])
        .with(systems::SoundEffectsSystem, "sound_effects_system", &[])
        .with(systems::WallBounceSystem, "wall_bounce", &["move_straight"])
        .with(systems::BuffSpawnSystem::new(2.0), "buff_spawn", &[]);

    let config = PongConfig::load(config_path);

    let mut game = Application::build(assets_dir, Pong::default())?
        .with_resource(config.arena)
        .with_resource(config.ball)
        .with_resource(config.paddles)
        .build(game_data)?;

    game.run();

    Ok(())
}
