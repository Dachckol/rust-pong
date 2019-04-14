extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
    core::transform::TransformBundle,
    input::InputBundle,
    ui::{DrawUi, UiBundle},
};

mod game;
use crate::game::Pong;
use crate::game::systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let root_dir = application_root_dir();

    let path = format!("{}/resources/display_config.ron", root_dir,);
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new()),
    );

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(format!(
            "{}/resources/bindings_config.ron",
            root_dir,
        ))?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(input_bundle)?
        .with(systems::paddle::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::ball::MoveBallsSystem, "ball_system", &[])
        .with(systems::winner::WinnerSystem, "winner_system", &["ball_system"])
        .with(systems::bounce::BounceSystem, "collision_system", &["ball_system", "paddle_system"]);

    let mut game = Application::new("./", Pong, game_data)?;

    game.run();

    Ok(())
}
