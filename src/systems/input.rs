use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entity, Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteExpect, WriteStorage};
use amethyst::ecs::shred::DefaultProvider;
use amethyst::core::timing::Time;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{Camera, SpriteRender};
use amethyst_window::ScreenDimensions;
use winit::MouseButton;

use crate::components::{Player, Pointer, ProposedMove};
use crate::bountiful::{center_of_tile, POINTER_Z};
use nalgebra::{Point3, Vector2};
use crate::resources::Hotbar;

#[derive(Default, SystemDesc)]
pub struct InputSystem {
    mouse_down: bool,
}

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
        let mut pointer: Option<Point3<f32>> = None;

        for (camera, camera_transform) in (&cameras, &transforms).join() {
            for player in (&players).join() {
                let entity = player.entity;
                let shift = input.action_is_down("shift").unwrap_or(false);
                let hotbar_selected = Self::process_hotbar_select(&input);

                // FIXME: This is a tangle of state...
                if let Some(index) = hotbar_selected {
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

                // FIXME: I think this should cancel mouse entered destination if one is in progress (if not stop)
                let the_move = Self::process_keyboard_move(&time, &input, entity,shift);
                moves.insert(entity, the_move).unwrap();
            };

            // When we check mouse position we have several different potential interactions:
            // 1. Move character (left mouse click to destination)
            // 2. Select item to interact with (left mouse click with shift)
            // 3. Drag loose item (left mouse held down with drag)
            if let Some((x, y)) = input.mouse_position() {
                let pos = camera
                    .projection()
                    .screen_to_world_point(Point3::new(x, y, 0.),
                                           Vector2::new(dimensions.width(), dimensions.height()),
                                           camera_transform);
                let mouse_down = input.mouse_button_is_down(MouseButton::Left);
                // FIXME: need Loose item from pointer position (which may be on map OR hotbar OR inventory if displayed).
                // FIXME: On raise will return to original loc if invalid drop loc will drop to map OR hotbar OR inventory.

                if self.mouse_down {
                    if !mouse_down { // mouse button raised
                        self.mouse_down = false;
                    } else {        // possibly dragging?
                    }
                } else {            // mouse button pressed
                    self.mouse_down = mouse_down;
                    pointer = Some(pos);
                }
            }
        }

        if let Some(pos) = pointer {
            for (_pointer, render, transform) in (&pointers, &mut renders, &mut transforms).join() {
                render.sprite_number = if self.mouse_down { 1 } else { 0 };
                transform.set_translation(center_of_tile(&pos, Some(POINTER_Z)));
            }
        }
    }
}


impl InputSystem {
    fn process_keyboard_move(
        time: &Read<Time, DefaultProvider>,
        input: &Read<InputHandler<StringBindings>, DefaultProvider>,
        entity: Entity,
        run: bool) -> ProposedMove {

        let (mut dx, mut dy): (f32, f32) = (0., 0.);

        if input.action_is_down("s").unwrap_or(false) {
            dy += -VELOCITY * time.delta_seconds();
        }
        if input.action_is_down("n").unwrap_or(false) {
            dy += VELOCITY * time.delta_seconds();
        }
        if input.action_is_down("e").unwrap_or(false) {
            dx += VELOCITY * time.delta_seconds();
        }
        if input.action_is_down("w").unwrap_or(false) {
            dx += -VELOCITY * time.delta_seconds();
        }

        ProposedMove::new(entity, dx, dy, run)
    }
}

impl InputSystem {
    fn process_hotbar_select(input: &Read<InputHandler<StringBindings>, DefaultProvider>) -> Option<usize> {
        if input.action_is_down("hotbar_1").unwrap_or(false) {
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
        }
    }
}
