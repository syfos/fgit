use ratatui::{
  DefaultTerminal,
  layout::{Constraint, Rect},
  widgets::{Block, Borders, Paragraph},
};
use std::{error::Error, result::Result};

use crate::{action::IoSignal, app::App};

#[allow(dead_code)]
pub enum Buffer {
  GitHealth,
  Help,
  Menu,
}

/// The core that handles `Tui` of `Fgit`.
#[allow(dead_code)]
pub struct Tui {
  pub current_buffer: Buffer,
}

#[allow(dead_code)]
impl Tui {
  pub fn new() -> Self {
    Self {
      current_buffer: Buffer::GitHealth,
    }
  }

  /// Wrapper over [`ratatui::run`].
  pub fn run() -> Result<(), Box<dyn Error>> {
    ratatui::run(Self::manager)?;
    Ok(())
  }

  /// Produces equal splits that are `vertically` stacked one above another, by dividing the entire terminal area by `net_splits`.
  fn split_horizontal(area: Rect, net_splits: u32) -> Vec<Rect> {
    // vec![value; count];
    // the number of chunks you get out of Layout::split() always equals the number of constraints you put in.
    // Hence so the size of vector == net_split
    let constraints = vec![Constraint::Ratio(1, net_splits); net_splits as usize];
    ratatui::layout::Layout::default()
      .direction(ratatui::layout::Direction::Vertical)
      .constraints(constraints)
      .split(area)
      .to_vec()
  }

  /// Loop that `draws ui` and handles `input keys`.
  fn manager(terminal: &mut DefaultTerminal) -> std::result::Result<(), Box<dyn Error>> {
    loop {
      terminal.draw(|frame| {
        let chunks = Self::split_horizontal(frame.area(), 4);
        for (i, chunk) in chunks.iter().enumerate() {
          let borders = if i == 0 {
            Borders::TOP | Borders::BOTTOM
          } else {
            Borders::BOTTOM
          };
          let block = Block::default().borders(borders);

          let paragraph = Paragraph::new(format!("Panel: {i}")).block(block);

          frame.render_widget(paragraph, *chunk);
        }
      })?;

      match App::handle_input() {
        Ok(IoSignal::Quit) => break Ok(()),

        // Handle io error
        Err(e) => break Err(e),

        // Exhaustiv maych
        Ok(IoSignal::None) => {}
      }
    }
  }
}
