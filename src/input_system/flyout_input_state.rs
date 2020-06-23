use amethyst::{
  core::math::Point3,
  core::transform::Transform,
  ecs::{
    prelude::*,
    prelude::{Entities, LazyUpdate, Read},
  },
  input::{InputHandler, StringBindings},
  renderer::Camera,
  window::ScreenDimensions,
  winit::MouseButton,
};

use super::input_state_trait::{EventType, InputState, Transition};
use crate::flyout_actions::{FlyoutAction, FlyoutActionStorage};
use crate::tile_map::TileMap;
use crate::z_layer::z_layer_to_coordinate;
use crate::z_layer::ZLayer;
use utils::coord::Coord;
use utils::rect::Rect;

pub struct FlyoutInputState {
  pub clicked_tile_index: i32,
  pub clicked_tile_rect: Rect,
  pub flyout_entity: Entity,
  pub flyout_rect: Rect,
  pub flyout_actions: Vec<FlyoutAction>,
}

impl FlyoutInputState {
  pub fn new<'a>(
    entities: &Entities<'a>,
    updater: &Read<'a, LazyUpdate>,
    clicked_tile_index: i32,
    clicked_tile_rect: Rect,
    flyout_actions: Vec<FlyoutAction>,
  ) -> Self {
    let mut transform = Transform::default();

    let tile_dimension = clicked_tile_rect.width as f32;
    let flyout_dimension = 32.;
    let dimension_diff = tile_dimension - flyout_dimension;

    transform.set_translation_xyz(
      clicked_tile_rect.bottom_left.x as f32 + tile_dimension / 2.,
      clicked_tile_rect.bottom_left.y as f32 + tile_dimension * 3. / 2. - dimension_diff / 2.,
      z_layer_to_coordinate(ZLayer::UiFlyout),
    );

    //TODO: Generalize for N action
    let flyout_action = &flyout_actions[0];

    transform.set_scale(flyout_action.icon.default_scale);

    let flyout_entity = updater
      .create_entity(&entities)
      .with(flyout_action.icon.sprite_render.clone())
      .with(transform)
      .build();
    let flyout_rect = Rect::new(
      Coord::new(
        clicked_tile_rect.bottom_left.x + (dimension_diff / 2.) as i32,
        clicked_tile_rect.bottom_left.y + tile_dimension as i32,
      ),
      clicked_tile_rect.width - dimension_diff as i32,
      clicked_tile_rect.height - dimension_diff as i32,
    );

    FlyoutInputState {
      clicked_tile_index,
      clicked_tile_rect,
      flyout_entity,
      flyout_rect,
      flyout_actions,
    }
  }
}

impl<'b> InputState for FlyoutInputState {
  fn process_event<'a>(
    &mut self,
    event: &EventType,
    input_handler: &Read<'a, InputHandler<StringBindings>>,
    _tile_map: &mut WriteExpect<'a, TileMap>,
    entities: &Entities<'a>,
    updater: &Read<'a, LazyUpdate>,
    cameras: &ReadStorage<'a, Camera>,
    transforms: &ReadStorage<'a, Transform>,
    screen_dimensions: &ReadExpect<'a, ScreenDimensions>,
    _flyout_action_storage: &Read<'a, FlyoutActionStorage>,
  ) -> Transition {
    match event {
      EventType::MouseButtonPressed(MouseButton::Left) => {
        if let Some((x, y)) = input_handler.mouse_position() {
          let world_point = {
            if let Some((camera, transform)) = (cameras, transforms).join().next() {
              let center_screen = Point3::new(x, y, 0.0);
              Some(camera.projection().screen_to_world_point(
                center_screen,
                screen_dimensions.diagonal(),
                &transform,
              ))
            } else {
              None
            }
          };
          if let Some(world_point) = world_point {
            if self.flyout_rect.is_in(world_point.x, world_point.y) {
              let clicked_tile_index = self.clicked_tile_index.clone();
              let clicked_tile_rect = self.clicked_tile_rect.clone();
              let flyout_action = self.flyout_actions[0].clone();
              updater.exec_mut(move |world| {
                (flyout_action.action)(world, clicked_tile_index, clicked_tile_rect);
              });

              entities
                .delete(self.flyout_entity)
                .expect("failed to delete flyout");

              return Transition::PopState;
            } else {
              entities
                .delete(self.flyout_entity)
                .expect("failed to delete flyout");
              return Transition::PopState;
            }
          }
        }
      }
      _ => (),
    }
    Transition::KeepState
  }
}
