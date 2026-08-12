use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::error::Error;
use crate::{action::IoSignal, app::App};

impl App {
  pub fn handle_input() -> std::result::Result<IoSignal, Box<dyn Error>> {
    if let Event::Key(key) = crossterm::event::read()? {
      match key.code {
        KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => {
          return Ok(IoSignal::Quit);
        }



        _ => {}
      }
    }
    Ok(IoSignal::None)
  }
}
