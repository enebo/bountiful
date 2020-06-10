use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::input::{InputHandler, StringBindings};

use crate::components::Player;

#[derive(SystemDesc)]
pub struct InputSystem;

const VELOCITY: f32 = 160.0;

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, players, time, input): Self::SystemData) {
        for (_player, transform) in (&players, &mut transforms).join() {
            if let Some(true) = input.action_is_down("s") {
                transform.prepend_translation_y(-VELOCITY * time.delta_seconds());
            }
            if let Some(true) = input.action_is_down("n") {
                transform.prepend_translation_y(VELOCITY * time.delta_seconds());
            }
            if let Some(true) = input.action_is_down("e") {
                transform.prepend_translation_x(VELOCITY * time.delta_seconds());
            }
            if let Some(true) = input.action_is_down("w") {
                transform.prepend_translation_x(-VELOCITY * time.delta_seconds());
            }
        }
    }
}