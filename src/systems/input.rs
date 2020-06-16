use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::input::{InputHandler, StringBindings};
use winit::MouseButton;

use crate::components::{Player, ProposedMove, ProposedMoveType};

#[derive(SystemDesc)]
pub struct InputSystem;

const VELOCITY: f32 = 200.0;

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
            let entity = player.entity;
            let (mut dx, mut dy): (f32, f32) = (0.0, 0.0);

            let velocity = match input.action_is_down("shift") {
                Some(true) => VELOCITY * 3.,
                _ => VELOCITY,
            };

            if let Some(true) = input.action_is_down("s") {
                dy += -velocity * time.delta_seconds();
            }
            if let Some(true) = input.action_is_down("n") {
                dy += velocity * time.delta_seconds();
            }
            if let Some(true) = input.action_is_down("e") {
                dx += velocity * time.delta_seconds();
            }
            if let Some(true) = input.action_is_down("w") {
                dx += -velocity * time.delta_seconds();
            }

            let move_type = if dx != 0.0 || dy != 0.0 {
                ProposedMoveType::Walk
            } else {
                ProposedMoveType::Stop
            };

            moves.insert(player.entity,ProposedMove {
                move_type,
                entity,
                dx,
                dy
            }).unwrap();

            // FIXME: How do I click detect
            if input.mouse_button_is_down(MouseButton::Left) {
                let mouse_position = input.mouse_position();
                println!("MP: {:?}", mouse_position);
            }

        }
    }
}