use amethyst::ecs::{Component, DenseVecStorage, Entity};

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct ProposedMove {
    pub entity: Entity,
    pub dx: f32,
    pub dy: f32,
}