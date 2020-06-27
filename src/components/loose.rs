use amethyst::ecs::{Component, DenseVecStorage};

/// Something which can be collected like a dropped item or an item
/// in your inventory or hotbar.
#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Loose {}