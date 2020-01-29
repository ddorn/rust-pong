use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{transform::Transform, timing::Time},
    prelude::*,
    ecs::prelude::{Entity},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use crate::components::{Paddle, Side, Ball, Score};


pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub const BALL_RADIUS: f32 = 2.0;
pub const BALL_VELOCITY: [f32; 2] = [150.0, 90.0];

#[derive(Default)]
pub struct Pong {
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

/// ScoreText contains the ui text components that display the score
pub struct ScoreText {
    pub left: Entity,
    pub right: Entity,
}


impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Wait one second before spawning the ball.
        self.ball_spawn_timer.replace(1.0);
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        initialise_camera(world);
        initialise_paddles(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_scoreboard(world)
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {

        if let Some(mut timer) = self.ball_spawn_timer.take() {
            // Subtract the elapsed time
            timer -= data.world.fetch::<Time>().delta_seconds();

            if timer <= 0.0 {
                initialise_ball(data.world, self.sprite_sheet_handle.clone().unwrap());
            } else {
                self.ball_spawn_timer.replace(timer);
            }
        }

        Trans::None
    }
}


/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_paddles(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    // Create a left plank entity.
    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .with(sprite_render.clone())
        .build();

    // Create right plank entity.
    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .with(sprite_render.clone())
        .build();
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world.create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let loader = world.read_resource::<Loader>();
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialise_ball(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {

    let ball = Ball {
        velocity: BALL_VELOCITY,
        radius: BALL_RADIUS,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 1
    };

    world
        .create_entity()
        .with(ball)
        .with(transform)
        .with(sprite_render)
        .build();
}

fn initialise_scoreboard(world: &mut World) {
    let font_handle = world.read_resource::<Loader>().load(
        "fonts/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        -50., -50., 1., 200., 50.,
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        50., -50., 1., 200., 50.,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font_handle.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        )).build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font_handle.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        )).build();

    world.insert(ScoreText { left: p1_score, right: p2_score });

}