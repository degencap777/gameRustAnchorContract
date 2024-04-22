use crate::engine::GameEngine;

use super::translate::Translate;

/// Controller is code that runs a control command for an entity
pub struct Controller {}
impl Controller {
  /// Moves an entity up
  pub fn move_up(engine: &mut GameEngine, entity_id: u32) -> Result<(), ()> {
    Translate::translate(engine, entity_id, 0.0, -1.0);
    Ok(())
  }

  /// Moves an entity down
  pub fn move_down(engine: &mut GameEngine, entity_id: u32) -> Result<(), ()> {
    Translate::translate(engine, entity_id, 0.0, 1.0);
    Ok(())
  }

  /// Moves an entity left
  pub fn move_left(engine: &mut GameEngine, entity_id: u32) -> Result<(), ()> {
    Translate::translate(engine, entity_id, -1.0, 0.0);
    Ok(())
  }

  /// Moves an entity right
  pub fn move_right(engine: &mut GameEngine, entity_id: u32) -> Result<(), ()> {
    Translate::translate(engine, entity_id, 1.0, 0.0);
    Ok(())
  }

  /// Runs an action for each of a controllable element
  /// TODO - make this something for 
  pub fn for_all(engine: &mut GameEngine, some_action: fn(& mut GameEngine, u32) -> Result<(), ()>) {
    let keys = engine.components.controllables.keys();

    // let mut controllable;
    for key in keys {
      some_action(engine, key);
    }
  }
}