use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use std::time::Duration;

use amethyst::core::frame_limiter::FrameRateLimitStrategy;

mod state;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;

    //let mut game = Application::new(resources, state::MyState, game_data)?;
    let mut game = Application::build(resources, state::MyState)?.with_frame_limit(
        FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
        25,
    )
    .build(game_data)?;
    game.run();

    Ok(())
}
