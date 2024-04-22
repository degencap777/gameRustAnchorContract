use crossterm::event::{read, Event, KeyCode, KeyEvent};

use crate::enums::actions::Action;
/// Describes taking inputs and converting that to actions
pub struct Input {}
impl Input {
  /// Gets an input while blocking program progress
  /// 
  /// Returns a filtered crossterm::event::Event object for keypress events
  pub fn get_input_blocking() -> Event {
    loop {
      let input_event: Result<Event, std::io::Error> = read();
      if let Err(_) = input_event { continue; }

      let input_event: Event = input_event.unwrap();
      match input_event {
        Event::Key(_) => return input_event,
        _ => continue,
      }
    }
  }

  /// Converts an input into an Action enum
  /// 
  /// This can potentially be modified for mouse events if we want it
  pub fn input_to_action(input_event: Event) -> Option<Action> {
    match input_event {
      Event::Key(key_event) => return Self::keypress_to_action(key_event),
      _ => return None,
    }
  }

  /// Converts a keypress into an action
  /// 
  /// TODO - allow this to read in setting information so that inputs can be remapped
  fn keypress_to_action(key_event: KeyEvent) -> Option<Action> {
    let action = match key_event.code {
      KeyCode::Up => Action::MoveUp,
      KeyCode::Down => Action::MoveDown,
      KeyCode::Left => Action::MoveLeft,
      KeyCode::Right => Action::MoveRight,
      KeyCode::Char('q') => Action::Quit,
      _ => return None,
    };
    Some(action)
  }
}