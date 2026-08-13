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
  pub area: Rect,
  pub netsplit: Vec<Rect>,
}

#[allow(dead_code)]
impl Tui {
  pub fn new() -> Self {
    Self {
      current_buffer: Buffer::GitHealth,
      area: Rect::default(),
      netsplit: Vec::new(),
    }
  }

  /// Wrapper over [`ratatui::run`].
  pub fn run(app: &mut App) -> Result<(), Box<dyn Error>> {
    ratatui::run(|terminal| Self::manager(app, terminal))?;
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
  fn manager(
    app: &mut App,
    terminal: &mut DefaultTerminal,
  ) -> std::result::Result<(), Box<dyn Error>> {
    loop {
      terminal.draw(|frame| {
        let area = frame.area();
        app.tui.area = area;
        let chunks = app.tui.netsplit.clone();
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

      match App::handle_input(app) {
        Ok(IoSignal::Quit) => break Ok(()),

        Ok(IoSignal::Split) => {
          let count = app.tui.netsplit.len() + 1;
          let split = Self::split_horizontal(app.tui.area, count as u32);
          app.tui.netsplit = split;
        }

        // Handle io error
        Err(e) => break Err(e),

        // Exhaustiv maych
        Ok(IoSignal::None) => {}
      }
    }
  }
}
