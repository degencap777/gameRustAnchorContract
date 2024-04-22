use crate::engine::GameEngine;

/// Handles translation of an entity
pub struct Translate {}
impl Translate {
  pub fn translate(engine: &mut GameEngine, entity_id: u32, x: f32, y: f32) {
    let translation = engine.components.translations.get_mut(entity_id);
    if let None = translation { return; }

    let translation = translation.unwrap();
    translation.position.x = translation.position.x + x;
    translation.position.y = translation.position.y + y;
  }
}