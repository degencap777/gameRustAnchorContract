use super::ComponentTrait;

pub struct Controllable {
  owner: u32,
  is_active: bool,
}

impl ComponentTrait for Controllable {
  fn new(entity_id: u32) -> Self {
    Self {
      owner: entity_id,
      is_active: true,
    }
  }
}