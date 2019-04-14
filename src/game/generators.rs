use amethyst::{
    core::transform::Transform,
    renderer::{Camera, Projection},
    prelude::*,
    renderer::{
        SpriteRender,
        Flipped,
    },
};

use super::entities::paddle::{Paddle, Side, PADDLE_WIDTH};
use super::entities::ball::{Ball,BALL_RADIUS, BALL_VELOCITY_X, BALL_VELOCITY_Y};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}


pub fn initialise_paddles(world: &mut World, sprite_render: SpriteRender) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = ARENA_HEIGHT/2.0;

    left_transform.set_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_xyz(ARENA_WIDTH-PADDLE_WIDTH * 0.5, y, 0.0);


    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(sprite_render.clone())
        .with(right_transform)
        .build();

    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(sprite_render.clone())
        .with(Flipped::Horizontal)
        .with(left_transform)
        .build();
}

pub fn initialise_ball(world: &mut World, sprite_render: SpriteRender) {
    world.register::<Ball>();
    let mut transform = Transform::default();

    let y = ARENA_HEIGHT/2.0;
    let x = ARENA_WIDTH/2.0;
    transform.set_xyz(x,y,0.0);

    world
        .create_entity()
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(transform)
        .with(sprite_render.clone())
        .build();
}
