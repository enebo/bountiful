use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{
        application_root_dir,
        fps_counter::FpsCounterBundle,
    },
};

use amethyst::prelude::{GameDataBuilder, Application};


use amethyst_imgui::RenderImgui;
use bountiful::systems::{CollisionSystem, DebugSystem, InputSystem};
use bountiful::welcome::WelcomeScreen;
use bountiful::setup_bundle::SetupBundle;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = config_dir.join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.008, 0.043, 0.067, 1.0]),
                )
                .with_plugin(RenderImgui::<amethyst::input::StringBindings>::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(SetupBundle)?
        .with(InputSystem, "player_input", &["imgui_input_system"])
        .with(CollisionSystem, "collisions", &["player_input"])
        .with(DebugSystem::new(), "debug", &[]);

    let mut game = Application::new(
        assets_dir,
        WelcomeScreen::default(),
        game_data)?;
    game.run();

    Ok(())
}