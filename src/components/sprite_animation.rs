use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct SpriteAnimation {
    pub first_frame: usize,
    pub current_frame: usize,
    pub length: usize,
    pub time_per_frame: f32,
    pub elapsed_time: f32,
}

impl SpriteAnimation {
    pub fn new(first_frame: usize, length: usize, time_per_frame: f32) -> SpriteAnimation {
        SpriteAnimation {
            first_frame,
            length,
            time_per_frame,
            current_frame: 0,
            elapsed_time: 0.0,
        }
    }
}

impl Component for SpriteAnimation {
    type Storage = DenseVecStorage<Self>;
}