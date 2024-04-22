use super::Factory;
use crate::GameEngine;

pub struct BrushFactory {}
impl Factory for BrushFactory {
  fn new(engine: &mut GameEngine) -> Result<u32, String> {
    let brush_id = engine.entities.add("brush".to_string());

    engine.components.translations.register(brush_id);
    engine.components.visible.register(brush_id);

    Ok(brush_id)
  }
}