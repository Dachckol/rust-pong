use amethyst::{
    ecs::prelude::{
        Component,
        DenseVecStorage,
    },
};


pub const BALL_VELOCITY_X: f32 = 25.0;
pub const BALL_VELOCITY_Y: f32 = 15.0;
pub const BALL_RADIUS: f32 = 2.0;

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
