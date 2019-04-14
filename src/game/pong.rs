use amethyst::{
    prelude::*,
    renderer::SpriteRender,
};

use super::generators;
use super::resource_loaders;
use super::ui;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let spritesheet_handle = resource_loaders::load_sprite_sheet(world);
        let ball_sprite = SpriteRender {
            sprite_sheet: spritesheet_handle.clone(),
            sprite_number: 1,
        };
        let paddle_sprite = SpriteRender {
            sprite_sheet: spritesheet_handle.clone(),
            sprite_number: 0,
        };


        generators::initialise_ball(world, ball_sprite);
        generators::initialise_paddles(world, paddle_sprite);
        generators::initialise_camera(world);
        ui::scoreboard::initialise_scoreboard(world)
    }
}

