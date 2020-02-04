use crate::audio::initialise_audio;
use crate::components::{
    Ball, Buff, HitBox, Paddle, Side, StraightMover, WallBouncer,
};
use crate::config::{ArenaConfig, BallConfig, PaddleConfig};
use crate::math::random_direction;
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{timing::Time, transform::Transform},
    ecs::prelude::Entity,
    prelude::*,
    renderer::{
        Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat,
        Texture,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};
use std::f32::consts::PI;

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
        let sprite_sheet_handle = load_sprite_sheet(world);
        self.sprite_sheet_handle
            .replace(sprite_sheet_handle.clone());

        world.insert(sprite_sheet_handle);
        world.register::<Buff>();

        initialise_camera(world);
        initialise_paddles(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_scoreboard(world);
        initialise_audio(world);
    }

    fn update(
        &mut self,
        data: &mut StateData<'_, GameData<'_, '_>>,
    ) -> SimpleTrans {
        if let Some(mut timer) = self.ball_spawn_timer.take() {
            // Subtract the elapsed time
            timer -= data.world.fetch::<Time>().delta_seconds();

            if timer <= 0.0 {
                initialise_ball(
                    data.world,
                    self.sprite_sheet_handle.clone().unwrap(),
                );
            } else {
                self.ball_spawn_timer.replace(timer);
            }
        }

        Trans::None
    }
}

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_paddles(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let arena = world.fetch::<ArenaConfig>();
    let paddles = world.fetch::<PaddleConfig>();

    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    let y = arena.height / 2.0;
    left_transform.set_translation_xyz(paddles.width * 0.5, y, 0.0);
    right_transform.set_translation_xyz(
        arena.width - paddles.width * 0.5,
        y,
        0.0,
    );

    // Assign the sprites for the paddles
    let left_sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };
    let right_sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 1, // paddle is the first sprite in the sprite_sheet
    };

    let left_paddle = Paddle::new(Side::Left, &paddles);
    let right_paddle = Paddle::new(Side::Right, &paddles);

    // So we can borrow the world mutably to create the entity
    drop(arena);
    drop(paddles);

    // Create a left plank entity.
    world
        .create_entity()
        .with(left_paddle)
        .with(left_transform)
        .with(left_sprite_render)
        .build();

    // Create right plank entity.
    world
        .create_entity()
        .with(right_paddle)
        .with(right_transform)
        .with(right_sprite_render)
        .build();
}

fn initialise_camera(world: &mut World) {
    let (width, height) = {
        let arena = world.fetch::<ArenaConfig>();
        (arena.width, arena.height)
    };

    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
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
    let arena = world.fetch::<ArenaConfig>();
    let ball_config = world.fetch::<BallConfig>();

    let ball = StraightMover {
        direction: random_direction(PI / 4.0),
        speed: ball_config.speed,
    };

    let hit_box = HitBox {
        radius: ball_config.radius,
    };

    let transform = arena.center();
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 2,
    };

    // So we can borrow the world mutably to create the entity
    drop(arena);
    drop(ball_config);

    world
        .create_entity()
        .with(Ball)
        .with(ball)
        .with(hit_box)
        .with(transform)
        .with(sprite_render)
        .with(WallBouncer::horizontal())
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
        "P1".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -50.,
        -50.,
        1.,
        200.,
        50.,
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        50.,
        -50.,
        1.,
        200.,
        50.,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font_handle.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font_handle,
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    world.insert(ScoreText {
        left: p1_score,
        right: p2_score,
    });
}
