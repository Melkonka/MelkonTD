use amethyst::{core::transform::Transform, ecs::prelude::*};

use crate::minion::Minion;
use crate::projectile::Projectile;

pub trait TowerTrait: Send + Sync {
  fn update<'a>(
    &mut self,
    entities: &Entities<'a>,
    tower_transform: &Transform,
    minions: &ReadStorage<'a, Minion>,
    transforms: &ReadStorage<'a, Transform>,
    projectiles: &mut WriteStorage<'a, Projectile>,
    updater: &Read<'a, LazyUpdate>,
    elapsed: f32,
  );
  
  fn sprite_sheet_name(&self) -> &'static str;
  fn transform(&self) -> &Transform;
  fn transform_mut(&mut self) -> &mut Transform;
}
