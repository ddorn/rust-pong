use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
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
    pub velocity: [f32; 2],
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
