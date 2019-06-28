mod game_state;
mod render_graph;

use amethyst::{
    assets::Processor,
    core::{Float, frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::{Application, Config, GameDataBuilder},
    renderer::{sprite::SpriteSheet, types::DefaultBackend, RenderingSystem},
    ui::{DrawUiDesc, UiBundle},
    utils::application_root_dir,
    window::{DisplayConfig, WindowBundle},
    phythyst::{PhysicsBundle, PhysicsTime},
    amethyst_nphysics
};

use render_graph::MyRenderGraphCreator;
use std::string::String;

fn main() -> amethyst::Result<()> {
    // Configure Amethyst the Logger
    amethyst::Logger::from_config(Default::default())
        .level_for("amethyst_phythyst", amethyst::LogLevelFilter::Debug)
        .level_for("amethyst_rendy", amethyst::LogLevelFilter::Warn)
        .level_for("gfx_backend_vulkan", amethyst::LogLevelFilter::Warn)
        .level_for("rendy_factory::factory", amethyst::LogLevelFilter::Warn)
        .level_for(
            "rendy_memory::allocator::dynamic",
            amethyst::LogLevelFilter::Warn,
        )
        .level_for(
            "rendy_graph::node::render::pass",
            amethyst::LogLevelFilter::Warn,
        )
        .level_for("rendy_graph::node::present", amethyst::LogLevelFilter::Warn)
        .level_for("rendy_graph::graph", amethyst::LogLevelFilter::Warn)
        .level_for(
            "rendy_memory::allocator::linear",
            amethyst::LogLevelFilter::Warn,
        )
        .level_for("rendy_wsi", amethyst::LogLevelFilter::Warn)
        .start();

    let game_data = GameDataBuilder::default();
    let game_data = setup_physics(game_data);
    let game_data = setup_window(game_data);
    let game_data = setup_render_graph_constructor(game_data);
    let game_data = setup_transforms(game_data);

    let mut game = Application::build("./", game_state::CubeGameState::new())?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 1000)
        .with_physics(amethyst_nphysics::create_physics::<f32>())
        .with_resource(PhysicsTime::default().set_frames_per_second(240)) // optional
        .build(game_data)?;

    game.run();

    Ok(())
}

const MAIN_DIR: &str = "./game_directory";

#[inline]
fn get_dir_path(path: &str) -> String {
    String::from(MAIN_DIR) + path
}


#[inline]
fn setup_physics<'a, 'b>(gdb: GameDataBuilder<'a, 'b>) -> GameDataBuilder<'a, 'b>{
    gdb.with_bundle(PhysicsBundle::new()).unwrap()
}

#[inline]
fn setup_window<'a, 'b>(gdb: GameDataBuilder<'a, 'b>) -> GameDataBuilder<'a, 'b> {
    let display_config = DisplayConfig::load(get_dir_path("/configs/display_conf.ron"));
    gdb.with_bundle(WindowBundle::from_config(display_config))
        .unwrap()
}

#[inline]
fn setup_render_graph_constructor<'a, 'b>(gdb: GameDataBuilder<'a, 'b>) -> GameDataBuilder<'a, 'b> {
    // Creating this system using the thread local to make it sync in the main thread
    gdb.with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
        MyRenderGraphCreator::default(),
    ))
}

#[inline]
fn setup_transforms<'a, 'b>(gdb: GameDataBuilder<'a, 'b>) -> GameDataBuilder<'a, 'b> {
    gdb.with_bundle(TransformBundle::new()).unwrap()
}
