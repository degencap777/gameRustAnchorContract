use std::collections::{HashMap, hash_map::Iter};

use self::{
  camera::Camera,
  controllable::Controllable,
  translation::Translation,
  visible::Visible,
};

pub mod camera;
mod controllable;
pub mod translation;
mod visible;

pub trait ComponentTrait {
  fn new(entity_id: u32) -> Self;
}


pub struct ComponentContainer<T> {
  components: HashMap<u32, T>,
  keys: Vec<u32>,
}

impl<T: ComponentTrait> ComponentContainer<T> {
  pub fn new() -> Self {
    Self {
      components: HashMap::new(),
      keys: Vec::new(),
    }
  }

  /// Register a new component for a given entity ID
  pub fn register(&mut self, entity_id: u32) -> Result<(), &str> {
    if self.components.contains_key(&entity_id) {
      return Err("Entity already exists");
    }

    let new_component: T = T::new(entity_id);
    self.components.insert(entity_id, new_component);
    self.keys.push(entity_id);
    return Ok(())
  }

  pub fn has(&self, entity_id: u32) -> bool {
    self.components.contains_key(&entity_id)
  }

  pub fn get(&self, entity_id: u32) -> Option<&T> {
    self.components.get(&entity_id)
  }

  pub fn get_mut(&mut self, entity_id: u32) -> Option<&mut T> {
    self.components.get_mut(&entity_id)
  }

  pub fn iter(&mut self) -> Iter<u32, T> {
    self.components.iter()
  }

  /// Gets a list of all keys for entities used within this component
  /// 
  /// The vector clone is used to prevent issues from lifetimes
  /// TODO - possibly optimize this?
  pub fn keys(&self) -> Vec<u32> {
    self.keys.clone()
  }
}



pub struct ComponentManager {
  pub cameras: ComponentContainer<Camera>,
  pub controllables: ComponentContainer<Controllable>,
  pub translations: ComponentContainer<Translation>,
  pub visible: ComponentContainer<Visible>,
}



impl ComponentManager {
  pub fn new() -> Self {
    Self {
      cameras: ComponentContainer::new(),
      controllables: ComponentContainer::new(),
      translations: ComponentContainer::new(),
      visible: ComponentContainer::new(),
    } 
  }
}

