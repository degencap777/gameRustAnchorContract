use crate::entities::EntityManager;
use crate::components::ComponentManager;
use crate::systems::screen::Screen;

pub struct GameEngine {
  pub entities: EntityManager,
  pub components: ComponentManager,

  pub active_camera_id: Option<u32>,
}

impl GameEngine {
  pub fn new() -> Self {
    Self {
      entities: EntityManager::new(),
      components: ComponentManager::new(),

      active_camera_id: None
    }
  }

  pub fn set_active_camera(&mut self, camera_id: u32) -> Result<(), String> {
    let camera_error = Screen::is_valid_camera(self, camera_id);
    if let Err(error_message) = camera_error {
      return Err(error_message);
    }

    self.active_camera_id = Some(camera_id);
    Ok(())
  }
}
