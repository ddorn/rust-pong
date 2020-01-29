use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};

use crate::pong::{PADDLE_WIDTH, PADDLE_HEIGHT};


#[derive(Eq, PartialEq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

impl Paddle {
    pub fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}


pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

