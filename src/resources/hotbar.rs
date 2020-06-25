use amethyst::ecs::Entity;

#[derive(Default)]
pub struct Hotbar {
    pub selected: Option<usize>,
    pub contents: Vec<HotbarSlot>,
}

pub struct HotbarSlot {
    pub hotbar_gui: Entity,
    pub contents: Option<Entity>,
}