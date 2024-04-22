use crate::engine::GameEngine;

use super::Factory;

pub struct CameraFactory {}
impl Factory for CameraFactory {
  /// Creates a new camera for the Game Engine and returns the ID
  /// 
  /// Creates a new camera for the given Game Engine and returns the ID. If the
  /// engine does not have an active camera, this camera is set as the active camera
  fn new(engine: &mut GameEngine) -> Result<u32, String> {
    let camera_id = engine.entities.add("camera".to_string());

    engine.components.translations.register(camera_id);
    engine.components.cameras.register(camera_id);
    
    if let Some(_) = engine.active_camera_id {
      return Ok(camera_id);
    }

    let set_active_camera_result: Result<(), String> = engine.set_active_camera(camera_id);
    if let Err(message) = set_active_camera_result { 
      return Err(message);
    }

    return Ok(camera_id);
  }
}