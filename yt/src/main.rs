mod game_state;
mod render_graph;
mod sphere_system;

use crate::sphere_system::*;
use game_state::*;
use render_graph::*;

use amethyst::{
    config::Config,
    core::transform::bundle::TransformBundle,
    renderer::{types::DefaultBackend, RenderingSystem},
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
        .with_bundle(WindowBundle::from_config(display_config))?
        .with_bundle(TransformBundle::new())?
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            MyRenderGraphCreator::default(),
        ))
        .with(MotionSystem::default(), "MotionSystem", &[]);

    let mut game =
        Application::build("./", game_state::LoadingState::default())?.build(game_data)?;

    game.run();

    Ok(())
}
