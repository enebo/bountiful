use crate::resources::Point;
use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Position {
    pub map_id: String,
    pub loc: Point,
}

impl Position {
    pub fn new(map_id: String, loc: Point) -> Self {
        Self {
            map_id,
            loc
        }
    }
}