extern crate amethyst;

mod state;

use amethyst::{
    core::transform::TransformBundle,
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

use state::Init;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let display_config = DisplayConfig::load("./config/display_config.ron");
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1., 1., 1., 1.], 1.)
            .with_pass(DrawFlat::<PosTex>::new()),
    );
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?;

    Application::new("./", Init, game_data)?
        .run();

    Ok(())
}
