use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entity, Entities, Join, ReadStorage, System, SystemData, WriteStorage};
use amethyst::renderer::Camera;

use crate::components::{MakeMove, Player};

#[derive(SystemDesc)]
pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, MakeMove>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Camera>,
        ReadStorage<'s, Player>,
        Entities<'s>,
    );

    fn run(&mut self, (mut moves, mut transforms, mut cameras, players, entities): Self::SystemData) {
        let mut to_remove: Vec<(Entity, f32, f32)> = vec![];

        for (entity, make_move, transform) in (&entities, &mut moves, &mut transforms).join() {
            transform.prepend_translation_x(make_move.dx);
            transform.prepend_translation_y(make_move.dy);

            to_remove.push((entity, transform.translation().x, transform.translation().y));
        }

        for (entity, x, y) in to_remove {
            if players.get(entity).is_some() {
                for (_camera, transform) in (&cameras, &mut transforms).join() {
                    transform.set_translation_x(x);
                    transform.set_translation_y(y);
                }
            }
            moves.remove(entity);
        }
    }
}