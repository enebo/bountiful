use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entity, Entities, Join, System, SystemData, WriteStorage};

use crate::components::MakeMove;

#[derive(SystemDesc)]
pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, MakeMove>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
    );

    fn run(&mut self, (mut moves, mut transforms, entities): Self::SystemData) {
        let mut to_remove: Vec<Entity> = vec![];

        for (entity, make_move, transform) in (&entities, &mut moves, &mut transforms).join() {
            transform.prepend_translation_x(make_move.dx);
            transform.prepend_translation_y(make_move.dy);

            to_remove.push(entity);
        }

        for entity in to_remove {
            moves.remove(entity);
        }
    }
}