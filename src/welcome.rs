use amethyst::{
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down},
    prelude::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans, WorldExt},
    ui::UiCreator,
    winit::VirtualKeyCode,
};

#[derive(Default, Debug)]
pub struct WelcomeScreen {
    splash_screen: Option<Entity>,
}

impl SimpleState for WelcomeScreen {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.splash_screen =
            Some(data.world.exec(|mut creator: UiCreator<'_>| creator.create("ui/welcome.ron", ())));
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root_entity) = self.splash_screen {
            data.world.delete_entity(root_entity).expect("Failed to delete WelcomeScreen");
        }

        self.splash_screen = None;
    }

    fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent, ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Space) ||
                is_key_down(&event, VirtualKeyCode::Tab) {
                Trans::Switch(Box::new(crate::bountiful::Bountiful::default()))
            } else if is_close_requested(&event) ||
                is_key_down(&event, VirtualKeyCode::Q) ||
                is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}