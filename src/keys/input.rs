use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::error::Error;
use crate::{action::InputSignal, app::App};

impl App {
  pub fn handle_input() -> std::result::Result<InputSignal, Box<dyn Error>> {
    if let Event::Key(key) = crossterm::event::read()? {
      match key.code {
        KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => {
          return Ok(InputSignal::Quit);
        }

        _ => {}
      }
    }
    Ok(InputSignal::None)
  }
}
