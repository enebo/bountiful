use amethyst::{
    SimpleState, GameData, StateData,
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Builder, World, WorldExt},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::components::{Player, Position, Solid, Bound};
use crate::resources::{Tile, Point, Map};

pub struct Bountiful;

impl SimpleState for Bountiful {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Position>();

        let map = generate_map();

        initialise_camera(world);
        initialize_map(world, &map);
        initialize_player(world);
    }
}

pub const WIDTH: f32 = 1000.;
pub const HEIGHT: f32 = 1000.;

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(WIDTH * 0.5, HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(WIDTH, HEIGHT))
        .with(transform)
        .build();
}

// FIXME: Placement/Transform should be set how once map is defined?  This will also happen when
// changing maps.
fn initialize_player(world: &mut World) {
    let sprite_sheet_handle = load_sprite_sheet(world, "texture/player");
    let mut transform = Transform::default();
    transform.set_translation_xyz(64. + 32., HEIGHT - 64. - 32., 0.0);

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

    world.write_component().insert(entity, Player{ entity});
}

fn generate_map() -> Map {
    let map_string = "################\n\
                            #..#......#....#\n\
                            #...##.#.......#\n\
                            #..##...#.#....#\n\
                            #..######.#....#\n\
                            #..............#\n\
                            #..#......#....#\n\
                            #...##.#.......#\n\
                            #..##...#.#....#\n\
                            #..######.#....#\n\
                            #..............#\n\
                            #..............#\n\
                            #..............#\n\
                            ################";
    crate::resources::map::generate_ascii_map(map_string).unwrap()
}

fn initialize_map(world: &mut World, map: &Map) {
    let id = "top";
    let sprite_sheet_handle = load_sprite_sheet(world, "texture/obj_stoneblock001");

    // FIXME: I need to IntoIter Map...
    for (point, tile) in map.iter().collect::<Vec<(Point, Tile)>>() {

        if tile.id == '#' {
            let mut transform = Transform::default();

            transform.set_translation_xyz(point.x as f32 * 64. + 32., HEIGHT - point.y as f32 * 64. - 32., 0.0);

            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet_handle.clone(),
                sprite_number: 0, // stationary
            };
            
            world
                .create_entity()
                .with(Position::new(id.to_string(), point))
                .with(Solid {})
                .with(Bound::new(64., 64.))
                .with(sprite_render)
                .with(transform)
                .build();
        }
    }
}

fn load_sprite_sheet(world: &mut World, name: &str) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            name.to_string() + ".png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        name.to_string() + ".ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}