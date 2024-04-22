use std::result::Result;

use crate::engine::GameEngine;

use super::Screen;

impl Screen {
  pub fn can_draw(engine: &GameEngine) -> Result<(), String> {
    let active_camera_error = Self::has_active_camera(engine);
    if let Err(_) = active_camera_error { return active_camera_error; }

    let active_camera_id = engine.active_camera_id.unwrap();
    let camera_valid_error = Self::is_valid_camera(engine, active_camera_id);
    if let Err(_) = camera_valid_error { return camera_valid_error; }
    
    Ok(())
  }

  pub fn has_active_camera(engine: &GameEngine) -> Result<(), String> {
    if let None = engine.active_camera_id {
      return Err("No active camera.".to_string());
    }
    Ok(())
  }

  pub fn is_valid_camera(engine: &GameEngine, camera_id: u32) -> Result<(), String> {
    if let None = engine.components.cameras.get(camera_id) {
      return Err(format!("Invalid camera with id: {}. No camera component", camera_id));
    }

    if let None = engine.components.translations.get(camera_id) {
      return Err(format!("Invalid camera with id: {}. No translation component", camera_id));
    }
    Ok(())
  }
}