use amethyst::{
    ecs::prelude::Entity,
    prelude::*,
    ui::{Anchor, UiText, UiTransform},
};

use crate::game::resource_loaders::load_font;


#[derive(Default)]
pub struct Scoreboard {
    pub left_score: i32,
    pub right_score: i32,
}

pub struct ScoreText {
    pub left_score: Entity,
    pub right_score: Entity,
}


pub fn initialise_scoreboard(world: &mut World) {
    let font = load_font(world);
    world.register::<UiTransform>();
    let p_left_trans = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle,
        -50., -50., 1., 200., 50., 0
    );
    let p_right_trans = UiTransform::new(
        "P2".to_string(), Anchor::TopMiddle,
        50., -50., 1., 200., 50., 0
    );

    let left_score = world
        .create_entity()
        .with(p_left_trans)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.,1.,1.,1.],
            50.,
        )).build();

    let right_score = world
        .create_entity()
        .with(p_right_trans)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.,1.,1.,1.],
            50.,
        )).build();

    world.add_resource(ScoreText{ left_score, right_score});
}
