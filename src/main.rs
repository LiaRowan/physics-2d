extern crate amethyst;

mod systems;
mod components;
mod state;

use amethyst::{
    core::{
        frame_limiter::FrameRateLimitStrategy,
        transform::TransformBundle,
    },
    input::InputBundle,
    prelude::{ Application, Config, GameDataBuilder },
    renderer::{
        DisplayConfig,
        DrawFlat,
        Pipeline,
        PosTex,
        RenderBundle,
        Stage,
    },
};
use std::time::Duration;

use state::Init;


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file("./config/input_bindings.ron")?;

    let display_config = DisplayConfig::load("./config/display_config.ron");

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1., 1., 1., 1.], 1.)
            .with_pass(DrawFlat::<PosTex>::new()),
    );
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?
        .with_bundle(input_bundle)?
        .with(
            systems::PlayerMovement,
            "player_movement_system",
            &["input_system"],
        );

    let mut game = Application::build("./", Init)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
