use amethyst::core::transform::Transform;
use amethyst::ecs::Entity;
use amethyst::prelude::{World, WorldExt};
use nalgebra::Vector3;

#[derive(Default)]
pub struct Hotbar {
    pub selected: Option<usize>,
    pub contents: Vec<HotbarSlot>,
}

pub struct HotbarSlot {
    pub hotbar_gui: Entity,
    pub contents: Option<Entity>,
}

impl Hotbar {
    pub fn translation_of(&self, world: &World, slot: usize) -> Option<Vector3<f32>>{
        if let Some(hotbar_slot) = self.contents.get(slot) {
            let reader = world.read_component::<Transform>();
            Some(reader.get(hotbar_slot.hotbar_gui).unwrap().translation().clone())
        } else {
            None
        }
    }
}