use amethyst::ecs::{Component, DenseVecStorage, Entity};

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Hotbar {
    pub entity: Option<Entity>,
}