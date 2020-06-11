use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};


use amethyst_imgui::RenderImgui;
use bountiful::bountiful::Bountiful;
use bountiful::systems::{InputSystem, MoveSystem, CollisionSystem};

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
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.008, 0.043, 0.067, 1.0]),
                )
                .with_plugin(RenderImgui::<amethyst::input::StringBindings>::default())
                .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with(InputSystem, "player_input", &["imgui_input_system"])
        .with(CollisionSystem, "collisions", &["player_input"])
        .with(MoveSystem, "moves", &["collisions"]);

    let mut game = Application::new(assets_dir, Bountiful, game_data)?;
    game.run();

    Ok(())
}