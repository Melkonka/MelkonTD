use amethyst::{
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

mod flyout_actions;
mod game;
mod game_state;
mod hierarchy_lookup;
mod input_system;
mod load_image;
mod loading_state;
mod minion;
mod minion_spawn_system;
mod projectile;
mod simple_animation;
mod simple_animation_system;
mod texture_lookup;
mod tile_map;
mod tower;
mod z_layer;

fn main() -> amethyst::Result<()> {
  let mut game = game::Game::new();

  game.run();

  amethyst::start_logger(Default::default());

  let app_root = application_root_dir()?;

  let resources = app_root.join("resources");
  let display_config = resources.join("display_config.ron");
  let game_data = GameDataBuilder::default().with_bundle(
    RenderingBundle::<DefaultBackend>::new()
      .with_plugin(
        RenderToWindow::from_config_path(display_config)?.with_clear([0.34, 0.36, 0.52, 1.0]),
      )
      .with_plugin(RenderFlat2D::default()),
  )?;
  let mut game = Application::build(resources, loading_state::LoadingState)?
    .with_frame_limit(
      FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
      25,
    )
    .build(game_data)?;
  game.run();

  Ok(())
}
