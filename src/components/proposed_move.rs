use amethyst::ecs::{Component, DenseVecStorage, Entity};

#[derive(Debug, PartialEq)]
pub enum ProposedMoveType {
    Stop,
    Walk,
    Run,
}

impl ProposedMoveType {
    pub fn move_type_from_velocity(dx: f32, dy: f32, run: bool) -> Self {
        match (dx != 0. || dy != 0., run) {
            (true, true) => ProposedMoveType::Run,
            (true, false) => ProposedMoveType::Walk,
            _ => ProposedMoveType::Stop
        }
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct ProposedMove {
    pub move_type: ProposedMoveType,
    pub entity: Entity,
    pub dx: f32,
    pub dy: f32,
}

impl ProposedMove {
    pub fn new(entity: Entity, dx: f32, dy: f32, run: bool) -> Self {
        Self {
            move_type: ProposedMoveType::move_type_from_velocity(dx, dy, run),
            entity,
            dx,
            dy,
        }
    }
}