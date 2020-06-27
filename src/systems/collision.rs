use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entity, Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::renderer::SpriteRender;

use crate::components::{ProposedMove, ProposedMoveType, Solid, Bound, SpriteAnimation};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, ProposedMove>,
        ReadStorage<'s, Solid>,
        ReadStorage<'s, Bound>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, SpriteAnimation>,
        Read<'s, Time>,
        Entities<'s>,
    );

    // FIXME: More complication animations cannot be combined line this...systems? or type of aninmation so it is one component
    fn run(&mut self, (mut transforms, mut moves, solids, bounds, mut renders, mut sprite_animations, time, entities): Self::SystemData) {
        let mut to_move: Vec<(Entity, f32, f32)> = vec![];
        let mut to_remove: Vec<Entity> = vec![];

        for (new_move, transform, sprite_render, anim) in (&mut moves, &transforms, &mut renders, &mut sprite_animations).join() {
            let entity = new_move.entity;

            to_remove.push(entity); // All moves die here.

            // FIXME: If I am running very fast I can teleport through things.  Either make that speed impossible or change collision system.
            // FIXME: If I am running pretty fast I might end up not being able to move at all even though there is plenty of open space between me and an obstacle.
            let (dx, dy) = match new_move.move_type {
                ProposedMoveType::Walk => (new_move.dx, new_move.dy),
                ProposedMoveType::Run => (new_move.dx * 3., new_move.dy * 3.), // FIXME: run multiple should come from join stats of any mover.
                ProposedMoveType::Stop => {
                    sprite_render.sprite_number = anim.stop();
                    continue;
                }
            };

            let (x, y) = (transform.translation().x, transform.translation().y);
            let mover_bound = bounds.get(entity).expect("Something moving which has no bound?");

            // FIXME: Only do this join if within a game point vs all possible solids.

            // A lot of searching but we do not have any idea how many solids exist that we might
            // collide with.  We also can get x and y velocities at the same time and we want to
            // be able to slide along a solid if either x or y dim is invalid but not both.
            for (dx, dy) in &[(dx, dy), (0., dy), (dx, 0.)] {
                let mut should_move = true;
                let (nx, ny) = (x + dx, y + dy);
                for (_solid, bound, entity) in (&solids, &bounds, &entities).join() {
                    let ot = transforms.get(entity).expect("No Solid Bound").translation();
                    if bound.intersects((ot.x, ot.y), (nx, ny), mover_bound) {
                        should_move = false;
                        break;
                    }
                }

                if should_move {
                    sprite_render.sprite_number = anim.update(time.delta_seconds(), (*dx, *dy));
                    to_move.push( (entity, *dx, *dy));
                    break;
                }
            }
        }

        for (entity, dx, dy) in to_move {
            let transform = transforms.get_mut(entity).unwrap();
            transform.prepend_translation_x(dx);
            transform.prepend_translation_y(dy);
        }

        for entity in to_remove {
            moves.remove(entity);
        }
    }
}