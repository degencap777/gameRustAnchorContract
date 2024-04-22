pub struct Entity {
  id: u32,
  name: String,
}

pub struct EntityManager {
  current_index: u32,
  entities: Vec<Entity>,
}

impl EntityManager {
  pub fn new() -> Self {
    Self {
      current_index: 0,
      entities: Vec::new()
    }
  }

  pub fn add(&mut self, name: String) -> u32 {
    let new_id = self.current_index.clone();
    self.current_index += 1;

    let new_entity = Entity {
      id: new_id.clone(),
      name,
    };
    self.entities.push(new_entity);

    new_id
  }
}