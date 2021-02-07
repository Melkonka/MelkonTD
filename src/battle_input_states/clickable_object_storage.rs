use sfml::graphics::IntRect;

use generational_arena::Index;

pub enum ClickableObjectRectType {
  Slot,
  Tower(Index),
}

pub struct ClickableObject {
  pub rect: IntRect,
  pub rect_type: ClickableObjectRectType,
}

pub struct ClickableObjectStorage {
  clickable_objects: Vec<ClickableObject>,
}

impl ClickableObjectStorage {
  pub fn new() -> ClickableObjectStorage {
    ClickableObjectStorage {
      clickable_objects: Vec::new(),
    }
  }

  pub fn insert(&mut self, rect: IntRect, rect_type: ClickableObjectRectType) {
    self
      .clickable_objects
      .push(ClickableObject { rect, rect_type });
  }

  pub fn find_object(
    &mut self,
    position_x: i32,
    position_y: i32,
  ) -> Option<&mut ClickableObject> {
    self.clickable_objects.iter_mut().find(|object| {
      position_x >= object.rect.left
        && position_x <= object.rect.left + object.rect.width
        && position_y >= object.rect.top
        && position_y <= object.rect.top + object.rect.height
    })
  }
}