use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
struct Player {}