use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::input::{InputHandler, StringBindings};
use winit::MouseButton;

use crate::components::{Player, ProposedMove};

#[derive(SystemDesc)]
pub struct InputSystem;

const VELOCITY: f32 = 400.0;

// Input can generate actions and moves.  Moves are proposed and collision system will decide
// whether they can occur.
impl<'s> System<'s> for InputSystem {
    type SystemData = (
        WriteStorage<'s, ProposedMove>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut moves, mut transforms, players, time, input): Self::SystemData) {
        for (player, _transform) in (&players, &mut transforms).join() {
            let (mut dx, mut dy): (f32, f32) = (0.0, 0.0);

            if let Some(true) = input.action_is_down("s") {
                dy += -VELOCITY * time.delta_seconds();
            }
            if let Some(true) = input.action_is_down("n") {
                dy += VELOCITY * time.delta_seconds();
            }
            if let Some(true) = input.action_is_down("e") {
                dx += VELOCITY * time.delta_seconds();
            }
            if let Some(true) = input.action_is_down("w") {
                dx += -VELOCITY * time.delta_seconds();
            }

            if dx != 0.0 || dy != 0.0 {
                moves.insert(player.entity,ProposedMove {
                    entity: player.entity,
                    dx,
                    dy
                }).unwrap();
            }

            // FIXME: How do I click detect
            if input.mouse_button_is_down(MouseButton::Left) {
                let mouse_position = input.mouse_position();
                println!("MP: {:?}", mouse_position);
            }

        }
    }
}