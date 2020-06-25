use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteExpect, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{Camera, SpriteRender};
use amethyst_window::ScreenDimensions;
use winit::MouseButton;

use crate::components::{Player, Pointer, ProposedMove, ProposedMoveType};
use crate::bountiful::{center_of_tile, POINTER_Z};
use nalgebra::{Point3, Vector2};
use crate::resources::Hotbar;

#[derive(SystemDesc)]
pub struct InputSystem;

const VELOCITY: f32 = 200.0;
const UNARM: usize = 8;

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
        WriteExpect<'s, Hotbar>,
    );

    // FIXME: pointer should probably just be a resource?  There is only one
    fn run(&mut self, (mut moves, mut transforms, players, pointers, dimensions, mut renders,
        cameras, time, input, mut hotbars): Self::SystemData) {
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

                if input.action_is_down("s").unwrap_or(false) {
                    dy += -velocity * time.delta_seconds();
                }
                if input.action_is_down("n").unwrap_or(false) {
                    dy += velocity * time.delta_seconds();
                }
                if input.action_is_down("e").unwrap_or(false) {
                    dx += velocity * time.delta_seconds();
                }
                if input.action_is_down("w").unwrap_or(false) {
                    dx += -velocity * time.delta_seconds();
                }

                let selected = if input.action_is_down("hotbar_1").unwrap_or(false) {
                    Some(0)
                } else if input.action_is_down("hotbar_2").unwrap_or(false) {
                    Some(1)
                } else if input.action_is_down("hotbar_3").unwrap_or(false) {
                    Some(2)
                } else if input.action_is_down("hotbar_4").unwrap_or(false) {
                    Some(3)
                } else if input.action_is_down("hotbar_5").unwrap_or(false) {
                    Some(4)
                } else if input.action_is_down("hotbar_6").unwrap_or(false) {
                    Some(5)
                } else if input.action_is_down("hotbar_7").unwrap_or(false) {
                    Some(6)
                } else if input.action_is_down("hotbar_8").unwrap_or(false) {
                    Some(7)
                } else if input.action_is_down("unarm").unwrap_or(false) {
                    Some(UNARM) // A little weird but add bogus once which means unarm.
                } else {
                    None
                };

                // FIXME: This is a tangle of state...
                if let Some(index) = selected {
                    let selected = hotbars.selected;
                    let unarm = index == UNARM && selected.is_some();
                    let index = if unarm { selected.unwrap() } else { index };
                    let gui = hotbars.contents.get(index).unwrap().hotbar_gui;
                    renders.get_mut(gui).unwrap().sprite_number = 1;
                    if let Some(selected_index) = hotbars.selected {
                        if selected_index != index || unarm {
                            let gui = hotbars.contents.get(selected_index).unwrap().hotbar_gui;
                            renders.get_mut(gui).unwrap().sprite_number = 0;
                        }
                    }
                    hotbars.selected = Some(index);
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
                transform.set_translation(center_of_tile(&pos, Some(POINTER_Z)));
            }
        }
    }
}