use ratatui::{
  DefaultTerminal, Frame,
  layout::{Constraint, Rect},
  widgets::{Block, Borders, Paragraph},
};
use std::{error::Error, result::Result};

use crate::action::{EventManager, IoSignal};

#[allow(dead_code)]
pub enum Buffer {
  GitHealth,
  Help,
  Menu,
}

/// The core that handles `Tui` of `Fgit`.
#[allow(dead_code)]
pub struct Tui {
  pub cur_buf: Buffer,
  pub buf_area: Rect,
  pub splits: Splits,
  pub event_manager: EventManager,
}

pub struct Splits {
  pub vsplit_count: u32,
  pub hsplit_count: u32,
  pub is_render_vertical: bool,
  pub is_render_horizontal: bool,
  pub vsplits: Vec<Rect>,
  pub hsplits: Vec<Rect>,
}

impl Splits {
  pub fn new() -> Self {
    Self {
      vsplit_count: 0,
      hsplit_count: 0,
      is_render_vertical: false,
      is_render_horizontal: false,
      vsplits: Vec::new(),
      hsplits: Vec::new(),
    }
  }

  pub fn increment_vsplit_count(&mut self) {
    self.vsplit_count = self.vsplit_count.saturating_add(1);
  }

  pub fn increment_hsplit_count(&mut self) {
    self.hsplit_count = self.hsplit_count.saturating_add(1);
  }

  pub fn update_vsplits(&mut self, buf_area: Rect) {
    self.vsplits = Tui::split_vertically(buf_area, self.vsplit_count);
  }

  pub fn update_hsplits(&mut self, buf_area: Rect) {
    self.hsplits = Tui::split_horizontally(buf_area, self.hsplit_count);
  }
}

#[allow(dead_code)]
impl Tui {
  pub fn new() -> Self {
    Self {
      cur_buf: Buffer::GitHealth,
      buf_area: Rect::default(),
      splits: Splits::new(),
      event_manager: EventManager::default(),
    }
  }

  /// Wrapper over [`ratatui::run`].
  pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
    ratatui::run(|terminal| self.manager(terminal))?;
    Ok(())
  }

  /// Produces equal splits in `vertical direction`, i.e  `stacked one above another`, by dividing the entire terminal area by `net_vsplits`.
  fn split_vertically(area: Rect, net_vsplits: u32) -> Vec<Rect> {
    let mut x = net_vsplits;

    // The net_vsplits is just a counter
    x += 1;
    // vec![value; count];
    // the number of chunks you get out of Layout::split() always equals the number of constraints you put in.
    // Hence so the size of vector == net_split
    let constraints = vec![Constraint::Ratio(1, x); x as usize];
    ratatui::layout::Layout::default()
      .direction(ratatui::layout::Direction::Vertical)
      .constraints(constraints)
      .split(area)
      .to_vec()
  }

  /// Produces equal splits in `horizontal direction`, i.e  `side by side`, by dividing the entire terminal area by `net_hsplits`.
  fn split_horizontally(area: Rect, net_hsplits: u32) -> Vec<Rect> {
    let mut x = net_hsplits;
    x += 1;
    let constraints = vec![Constraint::Ratio(1, x); x as usize];
    ratatui::layout::Layout::default()
      .direction(ratatui::layout::Direction::Horizontal)
      .constraints(constraints)
      .split(area)
      .to_vec()
  }

  /// Loop that `draws ui` and handles `input keys`.
  fn manager(&mut self, terminal: &mut DefaultTerminal) -> std::result::Result<(), Box<dyn Error>> {
    loop {
      terminal.draw(|frame| {
        let area = frame.area();
        self.buf_area = area;

        if self.splits.is_render_vertical {
          self.render_vsplits(frame);
        }

        if self.splits.is_render_horizontal {
          self.render_hsplits(frame);
        }
      })?;

      match Self::handle_input() {
        Ok(IoSignal::Quit) => break Ok(()),

        Ok(IoSignal::Vsplit) => {
          self.splits.increment_vsplit_count();
          self.splits.update_vsplits(self.buf_area);
          self.splits.is_render_vertical = true;
        }

        Ok(IoSignal::Hsplit) => {
          self.splits.increment_hsplit_count();
          self.splits.update_hsplits(self.buf_area);
          self.splits.is_render_horizontal = true;
        }

        // Handle io error
        Err(e) => break Err(e),

        // Exhaustiv maych
        Ok(IoSignal::None) => {}
      }
    }
  }

  // Handles rendering logic for splits.
  fn render_vsplits(&self, frame: &mut Frame) {
    let v_len = self.splits.vsplits.len();
    for (i, chunk) in self.splits.vsplits.iter().enumerate() {
      let borders = if i + 1 < v_len {
        Borders::BOTTOM
      } else {
        Borders::NONE
      };
      let block = Block::default().borders(borders);
      let paragraph = Paragraph::new(format!("Panel: {i}")).block(block);
      frame.render_widget(paragraph, *chunk);
    }
  }
  fn render_hsplits(&self, frame: &mut Frame) {
    let h_len = self.splits.hsplits.len();
    for (i, chunk) in self.splits.hsplits.iter().enumerate() {
      let borders = if i + 1 < h_len {
        Borders::RIGHT
      } else {
        Borders::NONE
      };
      let block = Block::default().borders(borders);
      let paragraph = Paragraph::new(format!("Panel: {i}")).block(block);
      frame.render_widget(paragraph, *chunk);
    }
  }
}
