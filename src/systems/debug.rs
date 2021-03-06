use amethyst::{
    core::{Time, Transform},
    ecs::prelude::{Join, Read, ReadStorage, System},
    utils::fps_counter::FpsCounter,
};
use amethyst_imgui::{
    imgui,
    imgui::im_str,
};
use crate::bountiful::{TILE_WIDTH, TILE_HEIGHT};
use crate::components::Player;


pub struct DebugSystem {
    elapsed_time: f32,
    last_fps: f32,
}

impl DebugSystem {
    pub fn new() -> Self {
        Self { elapsed_time: 0.0, last_fps: 0.0 }
    }
}

const DISTANCE: f32 = 10.0;

impl<'s> System<'s> for DebugSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, FpsCounter>,
    );

    fn run(&mut self, (players, transforms, time, fps_counter): Self::SystemData) {
        let mut open = true;
        let window_pos = [DISTANCE, DISTANCE];
        let window_pos_pivot = [0.0, 0.0];

        amethyst_imgui::with(|ui| {
            let title = im_str!("Debug");
            let window = imgui::Window::new(&title)
                .bg_alpha(0.35)
                .movable(true)
                .no_decoration()
                .always_auto_resize(true)
                .save_settings(false)
                .focus_on_appearing(false)
                .no_nav()
                .opened(&mut open)
                .position(window_pos, imgui::Condition::Always)
                .position_pivot(window_pos_pivot);

            self.elapsed_time += time.delta_seconds();
            if self.elapsed_time > 2. {
                self.last_fps = fps_counter.sampled_fps();
                self.elapsed_time = 0.;
            }

            window.build(ui, || {
                ui.text(im_str!("FPS: {}", self.last_fps));
                for (_player, transform) in (&players, &transforms).join() {
                    let (i, j) = (transform.translation().x, transform.translation().y);
                    ui.text(im_str!("Loc [{},{}]", i, j));
                    ui.text(im_str!("Point [{},{}]", (i / TILE_WIDTH) as usize, (j / TILE_HEIGHT) as usize));
                }
            });
        });
    }
}