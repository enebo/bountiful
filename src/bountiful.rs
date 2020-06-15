use amethyst::{
    SimpleState, GameData, StateData,
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Builder, World, WorldExt},
    renderer::{Camera, ImageFormat, Sprite, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::components::{Player, Position, Solid, Bound};
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use tiled::{parse_with_path, Tileset, Map};

pub struct Bountiful;

impl SimpleState for Bountiful {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Position>();

        initialize_map(world);
        let (x, y) = initialize_player(world);
        initialise_camera(world, (x, y));
    }
}

pub const WIDTH: f32 = 1000.;
pub const HEIGHT: f32 = 1000.;

fn initialise_camera(world: &mut World, (x, y): (f32, f32)) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(WIDTH, HEIGHT))
        .with(transform)
        .build();
}

// FIXME: Placement/Transform should be set how once map is defined?  This will also happen when
// changing maps.
fn initialize_player(world: &mut World) -> (f32, f32) {
    let sprite_sheet_handle = load_sprite_sheet(world, "texture/player");
    let mut transform = Transform::default();
    let (x, y) = (64. + 32., 64. + 32.);// HEIGHT - 64. - 32.);
    transform.set_translation_xyz(x, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // stationary
    };

    let entity = world
        .create_entity()
        .with(sprite_render)
        .with(Bound::new(28., 54.))
        .with(transform)
        .build();

    world.write_component().insert(entity, Player{ entity}).unwrap();
    (x, y)
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
                tile_transform.set_translation_xyz(x, y, -1.0 + layer_i as f32);

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