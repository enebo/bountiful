use amethyst::{
    SimpleState, GameData, StateData,
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Builder, Entity, World, WorldExt},
    renderer::{Camera, ImageFormat, Sprite, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use amethyst_core::transform::components::Parent;
use amethyst_window::ScreenDimensions;
use nalgebra::{Point3, Vector2, Vector3};

use crate::components::{Player, Pointer, Position, Solid, Bound, SpriteAnimation, HotbarGui, Loose};
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use tiled::{parse_with_path, Tileset, Map};
use crate::resources::hotbar::HotbarSlot;
use crate::resources::{Hotbar, Items};

#[derive(Default)]
pub struct Bountiful;

impl SimpleState for Bountiful {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Position>();

        initialize_map(world);
        let (player, player_transform) = initialize_player(world);
        let camera= initialise_camera(world, player);
        initialize_pointer(world);
        let hotbar = Hotbar { selected: None, contents: initialize_hotbar(world, &camera, player, &player_transform) };
        let items = load_items(world);

        world.insert(items);
        world.insert(hotbar);

        equip_player(world, player);
    }
}

pub const TILE_WIDTH: f32 = 64.;
pub const TILE_HEIGHT: f32 = 64.;
pub const WIDTH: f32 = 1000.;
pub const HEIGHT: f32 = 1000.;
pub const HOTBAR_SLOTS: usize = 9;

pub const CAMERA_Z: f32 = 1.0;
pub const HOTBAR_CONTENTS_Z: f32 = 0.15;
pub const HOTBAR_Z: f32 = 0.1;
pub const POINTER_Z: f32 = 0.05;
pub const PLAYERS_Z: f32 = 0.0;
pub const MAP_LAYERS_Z: [f32; 3] = [-0.3, -0.2, -0.1]; // base, solid, iso

// FIXME: Lots wrong here but this is just temporary to work in item interaction.
fn equip_player(world: &mut World, player: Entity) {
    let (textures, texture_id) = {
        let items = world.read_resource::<Items>();
        (items.textures.clone(), items.items.iter().find(|e| e.name == "Pick Axe").unwrap().texture_id)
    };

    let slot_translation = world.read_resource::<Hotbar>().translation_of(world, 0).unwrap();
    let mut transform= Transform::default();
    transform.set_translation_xyz(slot_translation.x, slot_translation.y,HOTBAR_CONTENTS_Z);

    let sprite_render = SpriteRender {
        sprite_sheet: textures,
        sprite_number: texture_id, // stationary
    };

    //println!("AX AT: {}, {}", x, y);
    let _ax = world
        .create_entity()
        .with(sprite_render)
        .with(Loose {})
        .with(Parent { entity: player })
        .with(transform)
        .build();
}

fn initialise_camera(world: &mut World, player: Entity) -> Camera {
    let mut transform= Transform::default();
    transform.set_translation_xyz(0., 0., 1.);

    let camera = Camera::standard_2d(WIDTH, HEIGHT);

    world
        .create_entity()
        .with(camera.clone())
        .with(Parent { entity: player })
        .with(transform)
        .build();

    camera
}

fn initialize_pointer(world: &mut World) {
    let sprite_sheet_handle = load_sprite_sheet(world, "texture/pointer");

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0, // stationary
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, POINTER_Z);

    let entity = world
        .create_entity()
        .with(sprite_render)
        .with(transform)
        .build();

    world.write_component().insert(entity, Pointer {}).unwrap();
}

fn initialize_hotbar(world: &mut World, camera: &Camera, player: Entity, player_transform: &Transform) -> Vec<HotbarSlot> {
    let dims = {
        let sd = world.read_resource::<ScreenDimensions>();
        Vector2::new(sd.width(), sd.height())
    };

    let sprite_sheet_handle = load_sprite_sheet(world, "texture/hotbar");
    let width = dims.x / 2. - HOTBAR_SLOTS as f32 / 2. * TILE_WIDTH;
    let point = Point3::new(width, dims.y - TILE_HEIGHT / 2., 0.);
    let pos = camera.projection().screen_to_world_point(point, dims, player_transform);
    let mut hotbars= Vec::<HotbarSlot>::with_capacity(HOTBAR_SLOTS);

    world.register::<HotbarGui>();
    for slot in 0..HOTBAR_SLOTS {
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 0, // stationary
        };

        let mut transform = Transform::default();
        transform.set_translation_xyz(pos.x + slot as f32 * TILE_WIDTH, pos.y, HOTBAR_Z);

        let entity = world
            .create_entity()
            .with(sprite_render)
            .with(Parent { entity: player })
            .with(transform)
            .build();

        hotbars.push(HotbarSlot { hotbar_gui: entity, contents: None });

        world.write_component().insert(entity, HotbarGui { slot }).unwrap();
    }

    hotbars
}

// FIXME: Placement/Transform should be set how once map is defined?  This will also happen when
// changing maps.
fn initialize_player(world: &mut World) -> (Entity, Transform) {
    let sprite_sheet_handle = load_sprite_sheet(world, "texture/player");
    let mut transform = Transform::default();
    // FIXME: Should be a position from map to start.
    transform.set_translation(center_of_tile(&Point3::new(TILE_WIDTH, TILE_HEIGHT, PLAYERS_Z), None));

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // stationary
    };

    // FIXME: need to define bound of things better than this.
    let entity = world
        .create_entity()
        .with(sprite_render)
        .with(SpriteAnimation::new_directional(1,17,9, 25, 8, 0.05))
        .with(Bound::new(28., 54.))
        .with(transform.clone())
        .build();

    world.write_component().insert(entity, Player{ entity}).unwrap();
    (entity, transform)
}

fn load_items(world: &mut World) -> Items {
    let sprite_sheet = load_sprite_sheet(world, "texture/items");
    let mut items = Items::new(sprite_sheet);

    items.add("Pick Axe".to_string(), 0);

    items
}

fn initialize_map(world: &mut World) {
    let texture_handle = load_texture_handle(world, "texture/pathetic");
    let map = load_tiled_map();
    let map_tileset = map.get_tileset_by_gid(1).expect("Missing first tileset in tiled map");
    let (tile_width, tile_height) = (map_tileset.tile_width, map_tileset.tile_height);
    let tile_sprites = load_sprites(map_tileset, tile_width, tile_height);

    let sprite_sheet = SpriteSheet {
        texture: texture_handle,
        sprites: tile_sprites
    };

    let sprite_sheet_handle = {
        let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        world.read_resource::<Loader>().load_from_data(sprite_sheet, (), &sprite_sheet_storage)
    };

    // Now that all the tile sprites/textures are loaded in
    // we can start drawing the tiles for our viewing pleasure
    // NOTE: Only rendering the first layer
    for layer_i in 0..2 {
        let solid = layer_i == 1;

        // Reverse because screen y is bottom at 0 and tile is 0 at top.
        for (j, row) in map.layers[layer_i].tiles.iter().rev().enumerate() {
            for (i, tile) in row.iter().enumerate() {
                if tile.gid == 0 { continue; } // gids 1-based. 0 means nothing.

                let tile_sprite = SpriteRender {
                    sprite_sheet: sprite_sheet_handle.clone(),
                    sprite_number: (tile.gid - 1) as usize, // sprites are 0-based.
                };

                // adjust x/y to account for sprites being centered in amethyst.
                let center_x_offset = tile_width as f32 / 2.0;
                let center_y_offset = -(tile_height as i32) as f32 / 2.0;
                let x = (i * tile_width as usize) as f32 + center_x_offset;
                let y = (j as f32 * tile_height as f32) + tile_height as f32 + center_y_offset;

                let mut tile_transform = Transform::default();
                tile_transform.set_translation_xyz(x, y, MAP_LAYERS_Z[layer_i] as f32);

                let mut tile = world
                    .create_entity()
                    .with(tile_transform)
                    .with(tile_sprite);

                if solid {
                    tile = tile
                        .with(Solid {})
                        .with(Bound::new(tile_width as f32, tile_height as f32));
                }

                tile.build();
            }
        }
    }
}

fn load_sprites(map_tileset: &Tileset, sprite_w: u32, sprite_h: u32) -> Vec<Sprite> {
    let mut tile_sprites = Vec::new();
    let image = &map_tileset.images[0];
    let (tileset_width, tileset_height) = (image.width, image.height);
    let columns = (tileset_width / sprite_w as i32) as u32;
    let rows = (tileset_height / sprite_h as i32) as u32;

    for x in 0..rows {
        for y in 0..columns {
            // For some reason rows are columns???
            let (pixel_top, pixel_left) = ((x * sprite_w), (y * sprite_h));
            let offsets = [0.0; 2];

            tile_sprites.push(Sprite::from_pixel_values(
                tileset_width as u32,
                tileset_height as u32,
                sprite_w,
                sprite_h,
                pixel_left,
                pixel_top,
                offsets,
                false,
                false
            ));
        }
    }

    tile_sprites
}

fn load_texture_handle(world: &mut World, prefix: &str) -> Handle<Texture> {
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let name = prefix.to_string() + ".png";

    world.read_resource::<Loader>().load(name, ImageFormat::default(), (), &texture_storage)
}

fn load_sprite_sheet(world: &mut World, prefix: &str) -> Handle<SpriteSheet> {
    let texture_handle= load_texture_handle(world, prefix);
    let name = prefix.to_string() + ".ron";
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    world.read_resource::<Loader>().load(name, SpriteSheetFormat(texture_handle), (), &sprite_sheet_store)
}

fn load_tiled_map() -> Map {
    let file = File::open(&Path::new("assets/texture/bountiful.tmx")).unwrap();
    let path = Path::new("assets/texture/pathetic.tsx");

    parse_with_path(BufReader::new(file), path)
        .expect("Assets missing while loading tmx")
}

pub fn center_of_tile(pos: &Point3<f32>, alternate_z: Option<f32>) -> Vector3<f32> {
    Vector3::new((pos.x / TILE_WIDTH).floor() * TILE_WIDTH + TILE_WIDTH / 2.,
                 (pos.y / TILE_HEIGHT).floor() * TILE_HEIGHT + TILE_HEIGHT / 2.,
                 alternate_z.or_else(|| Some(pos.z)).unwrap())
}