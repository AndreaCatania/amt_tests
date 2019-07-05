mod game_state;
mod render_graph;

use game_state::*;
use render_graph::*;

use amethyst::{
    Logger,
    GameDataBuilder,
    Application,

    config::Config,
    window::{DisplayConfig, WindowBundle},

    core::transform::bundle::TransformBundle,

    renderer::{
        RenderingSystem, types::DefaultBackend,
    }
};

fn main()-> amethyst::Result<()> {

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
        ));


    let mut game = Application::build("./", game_state::GameState)?
        .build(game_data)?;

    game.run();

    Ok(())
}
