mod game_state;
mod render_graph;
mod systems;

use crate::systems::*;
use game_state::*;
use render_graph::*;

use amethyst::{
    config::Config,
    core::transform::bundle::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        bundle::RenderingBundle, types::DefaultBackend, visibility::VisibilitySortingSystem,
    },
    window::{DisplayConfig, WindowBundle},
    Application, GameDataBuilder, Logger,
};

fn main() -> amethyst::Result<()> {
    let display_config = DisplayConfig::load("./configs/display_conf.ron");

    //-------------

    Logger::from_config(Default::default())
        .level_for("amethyst_rendy", amethyst::LogLevelFilter::Warn)
        .start();

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file("configs/input_bindings.ron")
                .unwrap(),
        )?
        .with(MotionSystem::default(), "MotionSystem", &[])
        .with(ToolSystem::default(), "ToolSystem", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(WindowBundle::from_config(display_config))?
        .with(
            VisibilitySortingSystem::new(),
            "VisibilitySortingSystem",
            &[],
        )
        .with_bundle(RenderingBundle::<DefaultBackend, _>::new(
            MyRenderGraphCreator::default(),
        ))?;

    let mut game =
        Application::build("./", game_state::LoadingState::default())?.build(game_data)?;

    game.run();

    Ok(())
}
