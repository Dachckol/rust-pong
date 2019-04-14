use amethyst::{
    core::Transform,
    ecs::prelude::{
        Join,
        System,
        WriteStorage,
        ReadExpect,
        Write,
    },
    ui::UiText,
};

use crate::game::{
    entities::ball::Ball,
    generators::{ARENA_WIDTH},
    ui::scoreboard::{ Scoreboard, ScoreText },
};


pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, Scoreboard>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, (
        mut balls,
        mut locals,
        mut ui_text,
        mut scores,
        score_text,
    ): Self::SystemData) {
        for (ball, local) in (&mut balls, &mut locals).join() {
            let ball_x = local.translation().x;

            let hit = if ball_x <= ball.radius {
                scores.right_score = (scores.right_score + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.right_score) {
                    text.text = scores.right_score.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                scores.left_score = (scores.left_score + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.left_score) {
                    text.text = scores.left_score.to_string();
                }
                true
            } else {
                false
            };

            if hit {
                ball.velocity[0] = -ball.velocity[0];
                local.set_x(ARENA_WIDTH/2.0);
            }
        }
    }
}
