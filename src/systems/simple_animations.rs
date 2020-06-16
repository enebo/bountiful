use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::renderer::SpriteRender;

use crate::components::{SpriteAnimation, MakeMove};

#[derive(SystemDesc)]
pub struct SimpleAnimationsSystem;

impl<'s> System<'s> for SimpleAnimationsSystem {
    type SystemData = (
        ReadStorage<'s, MakeMove>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, SpriteAnimation>,
        Read<'s, Time>,
    );

    fn run(&mut self, (moves, mut sprite_renders, mut sprite_animations, time): Self::SystemData) {
        for (mmove, sprite_render, anim) in (&moves, &mut sprite_renders, &mut sprite_animations).join() {
            sprite_render.sprite_number = anim.update(time.delta_seconds(), (mmove.dx, mmove.dy));
        }
    }
}
