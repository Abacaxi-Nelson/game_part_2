use amethyst::{
    prelude::*,
    core::{transform::TransformBundle},
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    ui::{RenderUi, UiBundle}
};

mod state;
mod component;
mod sprite;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    
    let assets_dir = app_root.join("assets");
    let display_config_path = assets_dir.join("display.ron");
    let binding_path = assets_dir.join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?   
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?    
                    .with_clear([252.0, 242.0, 240.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;
    

    let mut game = Application::new(assets_dir, state::gameplay::GameplayState, game_data)?;
    
    game.run();

    Ok(())
}
