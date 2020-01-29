//! Pong Tutorial 1

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    core::transform::TransformBundle,
    utils::application_root_dir,
    input::{InputBundle, StringBindings}
};

mod pong;
mod systems;
mod components;
use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    // Start the default logger to see errors and warnings
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");
    let bindings_path = app_root.join("config").join("bindings.ron");

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
                .with_plugin(RenderUi::default())
        )?
        .with_bundle(
            TransformBundle::new()
        )?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(bindings_path)?
        )?
        .with_bundle(
            UiBundle::<StringBindings>::new()
        )?
        .with(systems::PaddleSystem,
              "paddle_system",
              &["input_system"])
        .with(systems::MoveBallsSystem,
              "move_balls_system",
              &["paddle_system"])
        .with(systems::BounceBallsSystem,
              "bounce_balls_system",
              &["paddle_system", "move_balls_system"])
        .with(systems::WinnerSystem,
              "winner_system",
              &["bounce_balls_system"]);

    let mut game = Application::new(assets_dir,
                                    Pong::default(),
                                    game_data)?;
    game.run();

    Ok(())
}