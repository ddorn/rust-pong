use crate::config::PaddleConfig;
use crate::math::point_in_rect;
use amethyst::{core::math::Vector2, ecs::prelude::*};
use specs_derive::Component;

#[derive(Eq, PartialEq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Component)]
pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct StraightMover {
    pub direction: Vector2<f32>,
    pub speed: f32,
}

#[derive(Component)]
pub struct WallBouncer {
    pub vertical: bool,
    pub horizontal: bool,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Ball;

#[derive(Component)]
pub struct HitBox {
    pub radius: f32,
}

#[derive(Default)]
pub struct Score {
    pub left: i32,
    pub right: i32,
}

#[derive(Eq, PartialEq)]
pub enum BuffType {
    Size,
    Speed,
}

#[derive(Component)]
pub struct Buff {
    pub side: Side,
    pub buff: BuffType,
}

#[derive(Component)]
pub struct TrailGenerator {
    pub duration: f32,
    pub delay: f32,
    elapsed: f32,
}

#[derive(Component)]
pub struct Trail {
    pub duration: f32,
    elapsed: f32,
}

impl Paddle {
    pub fn new(side: Side, config: &PaddleConfig) -> Paddle {
        Paddle {
            side,
            width: config.width,
            height: config.height,
            speed: config.speed,
        }
    }
    pub fn hit(
        &self,
        pos: Vector2<f32>,
        point: Vector2<f32>,
        radius: f32,
    ) -> bool {
        // To determine whether the ball has collided with a paddle, we create a larger
        // rectangle around the current one, by subtracting the ball radius from the
        // lowest coordinates, and adding the ball radius to the highest ones. The ball
        // is then within the paddle if its center is within the larger wrapper
        // rectangle.

        point_in_rect(
            point.x,
            point.y,
            pos.x - self.width / 2.0 - radius,
            pos.y - self.height / 2.0 - radius,
            pos.x + self.width / 2.0 + radius,
            pos.y + self.height / 2.0 + radius,
        )
    }
}

impl WallBouncer {
    pub fn all() -> WallBouncer {
        WallBouncer {
            vertical: true,
            horizontal: true,
        }
    }

    pub fn horizontal() -> WallBouncer {
        WallBouncer {
            vertical: false,
            horizontal: true,
        }
    }
}

impl HitBox {
    pub fn new(radius: f32) -> HitBox {
        HitBox {
            radius: radius.max(0.0),
        }
    }
}

impl TrailGenerator {
    pub fn new(duration: f32, delay: f32) -> TrailGenerator {
        TrailGenerator {
            duration,
            delay,
            elapsed: 0.0,
        }
    }

    pub fn need_trail_creation(&mut self, delta_seconds: f32) -> bool {
        self.elapsed += delta_seconds;
        if self.elapsed >= self.delay {
            self.elapsed -= self.delay;
            return true;
        }
        false
    }
}

impl Trail {
    pub fn new(duration: f32) -> Trail {
        Trail {
            duration,
            elapsed: 0.0
        }
    }

    /// True if the trail still has to live
    pub fn update(&mut self, delta_seconds: f32) -> bool {
        self.elapsed += delta_seconds;
        self.elapsed < self.duration
    }

    /// Proportion of the time elapsed just started is zero
    /// and one is the end of the trail
    pub fn prop(&self) -> f32 {
        self.elapsed / self.duration
    }
}