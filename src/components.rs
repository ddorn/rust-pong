use amethyst::{
    ecs::prelude::*,
    core::math::Vector2,
};
use specs_derive::Component;
use crate::math::point_in_rect;


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
    pub right: i32
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


impl Paddle {
    pub fn hit (&self, pos: (f32, f32), point: (f32, f32), radius: f32) -> bool {
        // To determine whether the ball has collided with a paddle, we create a larger
        // rectangle around the current one, by subtracting the ball radius from the
        // lowest coordinates, and adding the ball radius to the highest ones. The ball
        // is then within the paddle if its center is within the larger wrapper
        // rectangle.

        point_in_rect(
            point.0,
            point.1,
            pos.0 - self.width / 2.0 - radius,
            pos.1 - self.height / 2.0 - radius,
            pos.0 + self.width / 2.0 + radius,
            pos.1 + self.height / 2.0 + radius,
        )
    }
}

impl WallBouncer {
    pub fn all() -> WallBouncer {
        WallBouncer {
            vertical: true,
            horizontal: true
        }
    }

    pub fn horizontal() -> WallBouncer {
        WallBouncer {
            vertical: false,
            horizontal: true
        }
    }
}

impl HitBox {
    pub fn new(radius: f32) -> HitBox {
        HitBox{radius: radius.max(0.0)}
    }
}