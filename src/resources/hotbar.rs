use amethyst::ecs::Entity;

pub struct Hotbars {
    pub selected: Option<usize>,
    pub contents: Vec<Hotbar>,
}

pub struct Hotbar {
    pub hotbar_gui: Entity,
    pub contents: Option<Entity>,
}