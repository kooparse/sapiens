#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate amethyst;
extern crate genmesh;

mod utils;
mod game_state;

use amethyst::prelude::*;
use amethyst::assets::HotReloadBundle;
use amethyst::core::transform::bundle::TransformBundle;
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::renderer::{DisplayConfig, DrawShaded, Pipeline, PosNormTex,
                         RenderBundle, RenderSystem, Stage};

use game_state::GameState;

const BACKGROUND_COLOUR: [f32; 4] = [0., 0., 0., 0.];

fn run() -> Result<(), amethyst::Error> {
    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target(BACKGROUND_COLOUR, 1.0)
            .with_pass(DrawShaded::<PosNormTex>::new()),
    );

    let mut game = Application::build("./", GameState)?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 60)
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new())?
        .with_local(RenderSystem::build(pipe, Some(config))?)
        .with_bundle(HotReloadBundle::default())?
        .build()
        .expect("Fatal error");

    game.run();
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    }
}
