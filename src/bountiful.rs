use amethyst::{
    SimpleState, GameData, StateData,
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Builder, World, WorldExt},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::components::Player;

pub struct Bountiful;

impl SimpleState for Bountiful {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
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
    transform.set_translation_xyz(WIDTH * 0.5, HEIGHT * 0.5, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // stationary
    };

    world
        .create_entity()
        .with(Player {})
        .with(sprite_render)
        .with(transform)
        .build();
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