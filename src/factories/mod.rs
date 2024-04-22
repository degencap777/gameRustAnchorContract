use crate::{engine::GameEngine, components::translation::{Coordinates, Translation}, systems::translate};

pub mod brush;
pub mod camera;
pub mod player;

pub trait Factory {
  fn new(engine: &mut GameEngine) -> Result<u32, String>;
}

// pub struct GenericFactory<'a> {
//   pub engine: &'a mut GameEngine,
//   pub entity_id: u32,
// }

// impl<'a> GenericFactory<'a> {
//   pub fn new(engine: &'a mut GameEngine, name: String) -> Self {
//     let entity_id = engine.entities.add(name);
//     Self {
//       engine,
//       entity_id
//     }
//   }

//   /// Finishes the build
//   pub fn build(&'a mut self) -> &'a mut GameEngine {
//     self.engine
//   }
// }

// impl <'a> GenericFactory<'a> {
//   pub fn translation(&'a mut self) -> TranslationFactory {
//     if !self.engine.components.translations.has(self.entity_id) {
//       self.engine.components.translations.register(self.entity_id);
//     }
    
//     TranslationFactory::new(self)
//   }
// }

// pub trait ComponentFactory<'a> {
//   fn new(factory: &'a mut GenericFactory<'a>) -> Self;
//   fn close(&mut self) -> &'a GenericFactory;
// }

// pub struct TranslationFactory<'a> {
//   pub factory: &'a mut GenericFactory<'a>
// }

// impl<'a> TranslationFactory<'a> {
//   fn get(&'a mut self) -> &'a Translation {
    
//   }

//   pub fn position(&'a mut self, position: Coordinates) -> &'a Self {
//     if let None = self.translation { return self; }
//     if let Some(translation) = self.translation {

//     }
//     self.translation.unwrap().position = position;
//     self
//   }

//   pub fn scale(&'a mut self, scale: Coordinates) -> &'a Self {
//     if let None = self.translation { return self; }
//     self.translation.unwrap().scale = scale;
//     self
//   }

//   pub fn origin(&'a mut self, origin: Coordinates) -> &'a Self {
//     if let None = self.translation { return self; }
//     self.translation.unwrap().origin = origin;
//     self
//   }
// }


// impl<'a> ComponentFactory<'a> for TranslationFactory<'a> {
//   fn new(factory: &'a mut GenericFactory<'a>) -> Self {
//     let translation = factory.engine.components.translations.get_mut(factory.entity_id);

//     Self {
//       factory
//     }
//   }

//   fn close(&mut self) -> &'a GenericFactory {
//     return self.factory;
//   }
// }