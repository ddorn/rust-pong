use amethyst::core::Transform;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PongConfig {
    pub arena: ArenaConfig,
    pub ball: BallConfig,
    pub paddles: PaddleConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArenaConfig {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BallConfig {
    pub radius: f32,
    pub speed: f32,
    pub bounce_acceleration: f32,
    pub max_speed: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaddleConfig {
    pub width: f32,
    pub height: f32,
    pub speed: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        ArenaConfig {
            height: 100.0,
            width: 100.0,
        }
    }
}

impl Default for BallConfig {
    fn default() -> Self {
        BallConfig {
            radius: 2.0,
            speed: 90.0,
            bounce_acceleration: 2.0,
            max_speed: 400.0,
        }
    }
}

impl Default for PaddleConfig {
    fn default() -> Self {
        PaddleConfig {
            width: 4.0,
            height: 16.0,
            speed: 40.0,
        }
    }
}

impl ArenaConfig {
    pub fn center(&self) -> Transform {
        let mut trans = Transform::default();
        trans.set_translation_xyz(self.width / 2.0, self.height / 2.0, 0.0);

        trans
    }
}
