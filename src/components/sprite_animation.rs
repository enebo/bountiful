use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::cmp::Ordering::{Less, Equal, Greater};

#[derive(Debug, PartialOrd, PartialEq)]
pub enum SpriteAnimationDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct SpriteAnimation {
    direction: SpriteAnimationDirection,
    up: usize,
    down: usize, // default rendered sprite without specified direction.
    left: usize,
    right: usize,
    first_frame: usize, // FIXME: put in methods
    current_frame: usize,
    length: usize,
    time_per_frame: f32,
    elapsed_time: f32,
}

impl SpriteAnimation {
    pub fn new(first_frame: usize, length: usize, time_per_frame: f32) -> Self {
        Self {
            direction: SpriteAnimationDirection::Down,
            up: 0,
            down: first_frame,
            left: 0,
            right: 0,
            first_frame,
            current_frame: first_frame,
            length,
            time_per_frame,
            elapsed_time: 0.,
        }
    }

    pub fn new_directional(up: usize, down: usize, left: usize, right: usize, length: usize, time_per_frame: f32) -> Self {
        Self {
            direction: SpriteAnimationDirection::Down,
            up,
            down,
            left,
            right,
            length,
            time_per_frame,
            first_frame: down,
            current_frame: down,
            elapsed_time: 0.
        }
    }

    /// Given a delta time since last frame and a velocity component (dx, dy) calculate
    /// the next animation sprite index to render.
    pub fn update(&mut self, delta_seconds: f32, (dx, dy): (f32, f32)) -> usize {
        self.elapsed_time += delta_seconds;
        let direction = Self::calculate_direction(dx, dy);

        // We changed directions swap out index and reset back to 0.
        if direction != self.direction {
            self.direction = direction;
            self.current_frame = match self.direction {
                SpriteAnimationDirection::Up => self.up,
                SpriteAnimationDirection::Down => self.down,
                SpriteAnimationDirection::Left => self.left,
                SpriteAnimationDirection::Right => self.right,
            };
            self.first_frame = self.current_frame;
            self.elapsed_time = 0.;
            self.current_frame
        } else {
            let i = self.first_frame + (self.elapsed_time / self.time_per_frame) as usize % self.length;
            if i != self.current_frame {
                self.current_frame = i;
            }
            i
        }
    }

    pub fn stop(&mut self) -> usize {
        self.current_frame = self.down;
        self.first_frame = self.current_frame;
        self.elapsed_time = 0.;
        self.current_frame
    }

    pub fn calculate_direction(dx: f32, dy: f32) -> SpriteAnimationDirection {
        let zero = 0. as f32;
        if dx == 0. {
            match dy.partial_cmp(&zero) {
                Some(Less) => SpriteAnimationDirection::Down,
                Some(Greater) => SpriteAnimationDirection::Up,
                Some(Equal) => SpriteAnimationDirection::Down, // default direction
                None => panic!("dy is NaN or something weird"),
            }
        } else if dy == 0. {
            match dx.partial_cmp(&zero) {
                Some(Less) => SpriteAnimationDirection::Left,
                Some(Greater) => SpriteAnimationDirection::Right,
                _ => panic!("dx is NaN or something weird"),
            }
        } else if dx < 0. {
            SpriteAnimationDirection::Left
        } else { // dx > 0
            SpriteAnimationDirection::Right
        }
    }
}

impl Component for SpriteAnimation {
    type Storage = DenseVecStorage<Self>;
}