use crate::resources::Point;
use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
struct Position {
    map_id: String,
    loc: Point,
}

impl Position {
    fn new(map_id: String, loc: Point) -> Self {
        Self {
            map_id,
            loc
        }
    }
}