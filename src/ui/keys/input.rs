use crate::{action::IoSignal, ui::Tui};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::error::Error;

impl Tui {
  /// Returns `stdin` translated into [`IoSignal`].
  pub fn handle_input() -> std::result::Result<IoSignal, Box<dyn Error>> {
    if let Event::Key(key) = crossterm::event::read()? {
      match key.code {
        KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => {
          return Ok(IoSignal::Quit);
        }

        KeyCode::Char('w') if key.modifiers == KeyModifiers::CONTROL => {
          if let Event::Key(next) = crossterm::event::read()? {
            match next.code {
              // For vim users Vertical is Horizontal.
              KeyCode::Char('v') => return Ok(IoSignal::Hsplit),
              KeyCode::Char('h') => return Ok(IoSignal::Vsplit),
              _ => {}
            }
          }
        }

        KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
          if let Event::Key(next) = crossterm::event::read()? {
            match next.code {
              KeyCode::Char('v') => return Ok(IoSignal::DelVsplit),
              KeyCode::Char('h') => return Ok(IoSignal::DelHsplit),
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
