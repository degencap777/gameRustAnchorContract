use enums::actions::Action;
use factories::Factory;
use factories::{
  camera::CameraFactory,
  player::PlayerFactory
};
use systems::controller::Controller;
use systems::input::Input;

// use rand::Rng;
use crate::engine::GameEngine;
use crate::systems::screen::Screen;

mod map;
mod components;
mod entities;
mod enums;
mod engine;
mod factories;
mod tiles;
mod systems;

use std::io;

fn main() -> io::Result<()> {
  if let Err(_) = setup() {
    teardown()?;
    return Ok(())
  }

  run_game()?;

  teardown()?;
  Ok(())
}

fn run_game() -> io::Result<()> {
  let mut engine = match setup_game() {
    Ok(engine) => engine,
    Err(message) => {
      println!("{}", message);
      return Err(std::io::Error::new(io::ErrorKind::Other, message));
    }
  };

  let p2_id = PlayerFactory::new(&mut engine).unwrap();
  {
    let translation = engine.components.translations.get_mut(p2_id).unwrap();
    translation.position.x = 3.0;
    translation.position.y = 3.0;
  }

  loop {
    Screen::draw(&mut engine)?;
    let input_event = Input::get_input_blocking();
    let action = Input::input_to_action(input_event);

    if let None = action { continue; }
    let action = action.unwrap();

    match action {
      Action::MoveUp => Controller::for_all(&mut engine, Controller::move_up),
      Action::MoveDown => Controller::for_all(&mut engine, Controller::move_down),
      Action::MoveLeft => Controller::for_all(&mut engine, Controller::move_left),
      Action::MoveRight => Controller::for_all(&mut engine, Controller::move_right),
      Action::Quit => return Ok(()),
    }

    // TODO - tick?
  }
}

/// Sets up the surrounding environment prior to the game running
fn setup() -> std::io::Result<()> {
  Screen::setup()?;
  Ok(())
}

/// Tears down all changes made to the surrounding environment following the conclusion of the game
fn teardown() -> std::io::Result<()> {
  Screen::teardown()?;
  Ok(())
}

/// Initializes the game space with all required defaults, menus, player, camera, and other entities
fn setup_game() -> Result<GameEngine, String> {
  let mut engine: GameEngine = GameEngine::new();
  if let Err(message) = PlayerFactory::new(&mut engine) { return Err(message); }
  if let Err(message) = CameraFactory::new(&mut engine) { return Err(message); }

  Ok(engine)
}
