use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    core::math::Vector2,
};


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

pub struct Ball {
    pub direction: Vector2<f32>,
    pub speed: f32,
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}


#[derive(Default)]
pub struct Score {
    pub left: i32,
    pub right: i32
}

impl Component for Score {
    type Storage = DenseVecStorage<Self>;
}
