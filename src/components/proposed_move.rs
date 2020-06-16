use amethyst::ecs::{Component, DenseVecStorage, Entity};

#[derive(Debug, PartialEq)]
pub enum ProposedMoveType {
    Stop,
    Walk,
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct ProposedMove {
    pub move_type: ProposedMoveType,
    pub entity: Entity,
    pub dx: f32,
    pub dy: f32,
}