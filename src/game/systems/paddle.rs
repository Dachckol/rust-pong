use amethyst::{
    core::Transform,
    ecs::{
        Join,
        Read,
        ReadStorage,
        System,
        WriteStorage,
    },
    input::InputHandler,
};

use crate::game::entities::paddle::{
    Paddle,
    Side,
    PADDLE_HEIGHT,
};

use crate::game::generators::ARENA_HEIGHT;

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem { 
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(mv_amount) = movement {
                let paddle_y = transform.translation().y;
                let delta = 1.5 * mv_amount as f32;
                transform.set_y((paddle_y + delta)
                    .min(ARENA_HEIGHT-PADDLE_HEIGHT*0.5)
                    .max(PADDLE_HEIGHT*0.5),
                );
            }
        }
    }
}
