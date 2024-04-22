use super::ComponentTrait;

pub struct Visible {
  pub owner: u32,
  pub is_visible: bool,
  pub sprite: char,
}

impl ComponentTrait for Visible {
  fn new(entity_id: u32) -> Self {
    Self {
      owner: entity_id,
      is_visible: false,
      sprite: ' ',
    }
  }
}

