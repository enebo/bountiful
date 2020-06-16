use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entity, Entities, Join, ReadStorage, System, SystemData, WriteStorage};
use amethyst::renderer::SpriteRender;

use crate::components::{MakeMove, ProposedMove, ProposedMoveType, Solid, Bound, SpriteAnimation};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, MakeMove>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, ProposedMove>,
        ReadStorage<'s, Solid>,
        ReadStorage<'s, Bound>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, SpriteAnimation>,
        Entities<'s>,
    );

    // FIXME: More complication animations cannot be combined line this...systems? or type of aninmation so it is one component
    fn run(&mut self, (mut make_move, transforms, mut moves, solids, bounds, mut renders, mut sprite_animations, entities): Self::SystemData) {
        let mut to_remove: Vec<Entity> = vec![];

        for (new_move, transform, sprite_render, anim) in (&mut moves, &transforms, &mut renders, &mut sprite_animations).join() {
            if new_move.move_type == ProposedMoveType::Stop {
                anim.current_frame = 0;
                sprite_render.sprite_number = anim.first_frame;
                continue;
            }
            let entity = new_move.entity;
            let (x, y) = (transform.translation().x, transform.translation().y);
            let (dx, dy) = (new_move.dx, new_move.dy);
            let mover_bound = bounds.get(entity).expect("Something moving which has no bound?");

            // FIXME: Only do this join if within a game point vs all possible solids.

            // A lot of searching but we do not have any idea how many solids exist that we might
            // collide with.  We also can get x and y velocities at the same time and we want to
            // be able to slide along a solid if either x or y dim is invalid but not both.
            for (dx, dy) in &[(dx, dy), (0., dy), (dx, 0.)] {
                let mut should_move = true;
                let new_loc= [x + dx, y + dy, 0.];
                for (_solid, bound, entity) in (&solids, &bounds, &entities).join() {
                    let ot = transforms.get(entity).expect("Solid is missing bound");
                    let a = [ot.translation().x, ot.translation().y, 0.];
                    if bound.intersects(a, new_loc, mover_bound) {
                        should_move = false;
                        break;
                    }
                }

                if should_move {
                    make_move.insert(entity, MakeMove::new(*dx, *dy)).unwrap();
                    to_remove.push(entity);
                    break;
                }
            }
        }

        for entity in to_remove {
            moves.remove(entity);
        }
    }
}