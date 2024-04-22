use std::io::{self, Stdout, Write};

use crate::components::camera::Camera;
use crate::engine::GameEngine;
use crossterm::style::Stylize;
use crossterm::{QueueableCommand, style};
use crossterm::{execute, {
  style::ResetColor,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
  cursor::{self, Show, Hide},
}};


mod validation;

pub struct Screen {}
impl Screen {
  pub fn setup() -> io::Result<()> {
    execute!(
      io::stdout(),
      EnterAlternateScreen,
      Hide,
    )?;
    enable_raw_mode()
  }

  pub fn teardown() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
      io::stdout(),
      Show,
      ResetColor,
      LeaveAlternateScreen
    )
  }

  pub fn draw(engine: &mut GameEngine) -> io::Result<()> {
    if let Err(error_message) = Self::can_draw(engine) {
      eprintln!("{}", error_message);
      return Ok(());
    }

    let ref active_camera_id = engine.active_camera_id.unwrap();
    let camera_camera = engine.components.cameras.get_mut(*active_camera_id).unwrap();

    let mut stdout = io::stdout();
    Self::draw_background(&mut stdout, camera_camera)?;
    Self::draw_entities(&mut stdout, engine)?;
    stdout.flush()?;

    Ok(())
  }

  fn draw_background(stdout: &mut Stdout, camera: &Camera) -> io::Result<()> {
    for y_position in 0..camera.height {
      for x_position in 0..camera.width {
        Self::draw_at(stdout, x_position, y_position, '_')?;
      }
    }
    Ok(())
  }

  fn draw_entities(stdout: &mut Stdout, engine: &mut GameEngine) -> io::Result<()> {
    let mut iterator = engine.components.visible.iter();
    let camera = engine.components.cameras.get(engine.active_camera_id.unwrap()).unwrap();
    loop {
      let visible = iterator.next();
      if let None = visible { break Ok(()); }

      let (entity_id, visible) = visible.unwrap();
      if !visible.is_visible { continue; }
      let translation = engine.components.translations.get(*entity_id);
      if let None = translation { continue; }
      let translation = translation.unwrap();

      // TODO - bounding system
      if translation.position.x < 0.0 || translation.position.x > (camera.width - 1) as f32 { continue; }
      else if translation.position.y < 0.0 || translation.position.y > (camera.height - 1) as f32 { continue; }

      let x = translation.position.x.round() as u32;
      let y = translation.position.y.round() as u32;

      Self::draw_at(stdout, x, y, visible.sprite)?;
    }
  }

  fn draw_at(stdout: &mut Stdout, x: u32, y: u32, sprite: char) -> io::Result<()> {
    stdout.queue(cursor::MoveTo(x as u16, y as u16))?;
    stdout.queue(style::PrintStyledContent(sprite.white()))?;
    Ok(())
  }
}

