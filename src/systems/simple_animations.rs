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
        for (_move, sprite_render, anim) in (&moves, &mut sprite_renders, &mut sprite_animations).join() {
            anim.elapsed_time += time.delta_seconds();
            let i = anim.first_frame + (anim.elapsed_time / anim.time_per_frame) as usize % anim.length;
            if i != anim.current_frame {
                anim.current_frame = i;
                sprite_render.sprite_number = i;
            }
        }
    }
}