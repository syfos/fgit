use ratatui::{
  DefaultTerminal, Frame,
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
  pub vsplits_count: usize,
  pub hsplits_counts: usize,
}

#[allow(dead_code)]
impl Tui {
  pub fn new() -> Self {
    Self {
      current_buffer: Buffer::GitHealth,
      area: Rect::default(),
      vsplits_count: 0,
      hsplits_counts: 0,
    }
  }

  /// Wrapper over [`ratatui::run`].
  pub fn run(app: &mut App) -> Result<(), Box<dyn Error>> {
    ratatui::run(|terminal| Self::manager(app, terminal))?;
    Ok(())
  }

  /// Produces equal splits in `vertical direction`, i.e  `stacked one above another`, by dividing the entire terminal area by `net_vsplits`.
  fn split_vertically(area: Rect, net_vsplits: u32) -> Vec<Rect> {
    // vec![value; count];
    // the number of chunks you get out of Layout::split() always equals the number of constraints you put in.
    // Hence so the size of vector == net_split
    let constraints = vec![Constraint::Ratio(1, net_vsplits); net_vsplits as usize];
    ratatui::layout::Layout::default()
      .direction(ratatui::layout::Direction::Vertical)
      .constraints(constraints)
      .split(area)
      .to_vec()
  }

  /// Produces equal splits in `horizontal direction`, i.e  `side by side`, by dividing the entire terminal area by `net_hsplits`.
  fn split_horizontally(area: Rect, net_hsplits: u32) -> Vec<Rect> {
    let constraints = vec![Constraint::Ratio(1, net_hsplits); net_hsplits as usize];
    ratatui::layout::Layout::default()
      .direction(ratatui::layout::Direction::Horizontal)
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
        Self::render_splits(app, frame);
      })?;

      match App::handle_input(app) {
        Ok(IoSignal::Quit) => break Ok(()),

        Ok(IoSignal::Vsplit) => {
          app.tui.vsplits_count += 1;
        }

        Ok(IoSignal::Hsplit) => {
          app.tui.hsplits_counts += 1;
        }

        // Handle io error
        Err(e) => break Err(e),

        // Exhaustiv maych
        Ok(IoSignal::None) => {}
      }
    }
  }

  // Handles rendering logic for splits.
  fn render_splits(app: &mut App, frame: &mut Frame) {
    let vsplits = Tui::split_vertically(frame.area(), app.tui.vsplits_count as u32);
    let hsplits = Tui::split_horizontally(frame.area(), app.tui.hsplits_counts as u32);
    for (i, chunk) in vsplits.iter().enumerate() {
      let borders = Borders::BOTTOM;
      let block = Block::default().borders(borders);

      let paragraph = Paragraph::new(format!("Panel: {i}")).block(block);

      frame.render_widget(paragraph, *chunk);
    }
    for (i, chunk) in hsplits.iter().enumerate() {
      let borders = Borders::RIGHT;
      let block = Block::default().borders(borders);

      let paragraph = Paragraph::new(format!("Panel: {i}")).block(block);

      frame.render_widget(paragraph, *chunk);
    }
  }
}
