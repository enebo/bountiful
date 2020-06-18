use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{Camera, SpriteRender};
use amethyst_window::ScreenDimensions;
use winit::MouseButton;

use crate::components::{Player, Pointer, ProposedMove, ProposedMoveType};
use crate::bountiful::{HEIGHT, WIDTH, TILE_WIDTH, TILE_HEIGHT, POINTER_Z};
use nalgebra::{Point3, Vector2};

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
        ReadStorage<'s, Pointer>,
        ReadExpect<'s, ScreenDimensions>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Camera>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    // FIXME: pointer should probably just be a resource?  There is only one
    fn run(&mut self, (mut moves, mut transforms, players, pointers, dimensions, mut renders, cameras, time, input): Self::SystemData) {
        let mut mouse_pressed = false;
        let mut pointer: Option<Point3<f32>> = None;

        for (camera, camera_transform) in (&cameras, &transforms).join() {
            for player in (&players).join() {
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

                moves.insert(player.entity, ProposedMove {
                    move_type,
                    entity,
                    dx,
                    dy
                }).unwrap();

                if let Some((x, y)) = input.mouse_position() {
                    pointer = Some(camera
                        .projection()
                        .screen_to_world_point(Point3::new(x, y, 0.),
                                               Vector2::new(dimensions.width(), dimensions.height()),
                                               camera_transform));
                    if input.mouse_button_is_down(MouseButton::Left) {
                        mouse_pressed = true;
                    }
                }
            }
        }

        if let Some(pos) = pointer {
            for (_pointer, render, transform) in (&pointers, &mut renders, &mut transforms).join() {
                render.sprite_number = if mouse_pressed { 1 } else { 0 };
                let (cx, cy) = center_of_tile(pos.x, pos.y);
                transform.set_translation_xyz(cx, cy, POINTER_Z);
            }
        }
    }

}

fn center_of_tile(x: f32, y: f32) -> (f32, f32) {
    (((x / TILE_WIDTH).floor() * TILE_WIDTH + TILE_WIDTH / 2.),
     ((y / TILE_HEIGHT).floor() * TILE_HEIGHT + TILE_HEIGHT / 2.));
}