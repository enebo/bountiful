use amethyst::ecs::{Component, DenseVecStorage, Entity};

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Player {
    pub entity: Entity,
}