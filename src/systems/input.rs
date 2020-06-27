use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entity, Entities, Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteExpect, WriteStorage};
use amethyst::ecs::shred::DefaultProvider;
use amethyst::core::timing::Time;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{Camera, SpriteRender};
use amethyst_core::transform::components::Parent;
use amethyst_window::ScreenDimensions;
use winit::MouseButton;

use crate::components::{Player, Pointer, ProposedMove, Loose};
use crate::bountiful::{center_of_tile, POINTER_Z, TILE_WIDTH, TILE_HEIGHT, HOTBAR_CONTENTS_Z, HOTBAR_SLOTS};
use nalgebra::{Point3, Vector2};
use crate::resources::Hotbar;

#[derive(SystemDesc)]
pub struct InputSystem {
    mouse_down: bool,
    dragged_item: Option<Entity>,
    original_dragged_location: Option<(f32, f32)>,
}

impl Default for InputSystem {
    fn default() -> Self {
        Self {
            mouse_down: false,
            dragged_item: None,
            original_dragged_location: None,
        }
    }
}

const VELOCITY: f32 = 200.0;
const UNARM: usize = HOTBAR_SLOTS + 1;

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
        ReadStorage<'s, Loose>,
        Entities<'s>,
        ReadStorage<'s, Parent>,
    );

    // FIXME: pointer should probably just be a resource?  There is only one
    fn run(&mut self, (mut moves, mut transforms, players, pointers, dimensions, mut renders,
        cameras, time, input, mut hotbars, loose, entities, parents): Self::SystemData) {
        let mut pointer: Option<Point3<f32>> = None;
        let mut drag_check = false;
        let mut player_pos: (f32, f32) = (0., 0.);
        let mut player_entity: Option<Entity> = None;

        for (camera, camera_transform) in (&cameras, &transforms).join() {
            for (player, player_transform) in (&players, &transforms).join() {
                player_pos = (player_transform.translation().x, player_transform.translation().y);
                player_entity = Some(player.entity);
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
                pointer = Some(camera
                    .projection()
                    .screen_to_world_point(Point3::new(x, y, 0.),
                                           Vector2::new(dimensions.width(), dimensions.height()),
                                           camera_transform));
                let mouse_down = input.mouse_button_is_down(MouseButton::Left);

                if self.mouse_down {
                    if !mouse_down { // mouse button raised
                        self.mouse_down = false;
                    } else {        // possibly dragging?
                        drag_check = true;
                    }
                } else {            // mouse button pressed
                    self.mouse_down = mouse_down;
                }
            }
        }

        // FIXME: Add inventory where picked up items get stuffed.
        // FIXME: on dragging to map it will drop next to or worst case where player is and remove Loose from the item.
        // FIXME: on shift-click an item on map will be picked up.

        // Handle dragging items around between hotbar, iventory, and dropping on the ground.
        // All loose items in hotbar and inventory aer children of the Player (via Parent).
        // This means all their locations are relative to the location of the Player.
        if let Some(pos) = pointer {
            if drag_check {
                if let Some(entity) = &self.dragged_item {
                    let transform = transforms.get_mut(*entity).unwrap();
                    transform.set_translation_xyz(pos.x - player_pos.0, pos.y - player_pos.1, HOTBAR_CONTENTS_Z);
                } else {
                    // Items placed in hotbar or invenctories are in transforms relative to parent and not map.
                    for (_loose, parent, entity, transform) in (&loose, &parents, &entities, &transforms).join() {
                        // Safe-guard against use of Parent for more than just the player.
                        if player_entity.unwrap() != parent.entity {
                            continue;
                        }

                        let (i, j) = (transform.translation().x, transform.translation().y);
                        let (ai, aj) = (player_pos.0 + i, player_pos.1 + j);
                        //println!("drag check items loose: {},{}. item: {},{}. player: {},{}. adj: {},{}.", pos.x, pos.y, i, j, player_pos.0, player_pos.1, ai, aj);
                        if (pos.x - ai).abs() <= TILE_WIDTH && (pos.y - aj).abs() <= TILE_HEIGHT {
                            self.dragged_item = Some(entity);
                            self.original_dragged_location = Some((i, j));
                            break;
                        }
                    }
                }
            } else { // highlight tile
                // Drop item somewhere or return it to where it was.
                if let Some(item) = self.dragged_item {
                    let item_transform = transforms.get(item).unwrap();
                    let item_translation = item_transform.translation();
                    let mut found_loc: Option<(f32, f32)> = None;
                    for hotbar in &hotbars.contents {
                        let slot_translation = transforms.get(hotbar.hotbar_gui).unwrap().translation();

                        if (item_translation.x - slot_translation.x).abs() <= TILE_WIDTH / 2. &&
                            (item_translation.y - slot_translation.y).abs() <= TILE_HEIGHT / 2. {
                            found_loc = Some((slot_translation.x, slot_translation.y));
                            break;
                        }
                    }

                    // return to where it came from
                    let loc = found_loc.or_else(|| Some(self.original_dragged_location.unwrap())).unwrap();
                    let item_transform = transforms.get_mut(item).unwrap();
                    item_transform.set_translation_xyz(loc.0, loc.1, HOTBAR_CONTENTS_Z);

                }
                self.dragged_item = None;

                for (_pointer, render, transform) in (&pointers, &mut renders, &mut transforms).join() {
                    render.sprite_number = if self.mouse_down { 1 } else { 0 };
                    transform.set_translation(center_of_tile(&pos, Some(POINTER_Z)));
                }
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
        } else if input.action_is_down("hotbar_9").unwrap_or(false) {
            Some(8)
        } else if input.action_is_down("unarm").unwrap_or(false) {
            Some(UNARM) // A little weird but add bogus once which means unarm.
        } else {
            None
        }
    }
}
