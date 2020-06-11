use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct MakeMove {
    pub dx: f32,
    pub dy: f32,
}

impl MakeMove {
    pub(crate) fn new(dx: f32, dy: f32) -> Self {
        MakeMove {
            dx,
            dy,
        }
    }
}