use amethyst::{SimpleState, GameData, StateData};
use amethyst::core::transform::Transform;
use amethyst::renderer::Camera;
use amethyst::prelude::*;

pub struct Bountiful;

impl SimpleState for Bountiful {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
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