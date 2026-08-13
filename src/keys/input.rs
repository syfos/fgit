use crate::{action::IoSignal, app::App};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::error::Error;

impl App {
  pub fn handle_input(&mut self) -> std::result::Result<IoSignal, Box<dyn Error>> {
    if let Event::Key(key) = crossterm::event::read()? {
      match key.code {
        KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => {
          return Ok(IoSignal::Quit);
        }

        KeyCode::Char('w') if key.modifiers == KeyModifiers::CONTROL => {
          if let Event::Key(next) = crossterm::event::read()? {
            match next.code {
              KeyCode::Char('s') => return Ok(IoSignal::Split),
              _ => {}
            }
          }
        }

        _ => {}
      }
    }
    Ok(IoSignal::None)
  }
}
