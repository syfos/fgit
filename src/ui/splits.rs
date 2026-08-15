use ratatui::{
   Frame,
  layout::{Constraint, Rect},
  widgets::{Block, Borders, Paragraph},
};

use crate::ui::Tui;
/// Contains everything regarding splits.
pub struct Splits {
  pub vsplit_count: u32,
  pub hsplit_count: u32,
  pub is_render_vertical: bool,
  pub is_render_horizontal: bool,
  pub vsplits: Vec<Rect>,
  pub hsplits: Vec<Rect>,
}

impl Splits {
  /// Generates default state of splits for the given `tui.buf_area` i.e `current buffer area`
  ///
  ///```
  /// // returns
  ///Self {
  ///  vsplit_count: 0,
  ///  hsplit_count: 0,
  ///  is_render_vertical: false,
  ///  is_render_horizontal: false,
  ///  vsplits: Vec::new(),
  ///  hsplits: Vec::new(),
  ///}
  ///```
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
    self.vsplits = self.split_vertically(buf_area);
  }

  pub fn update_hsplits(&mut self, buf_area: Rect) {
    self.hsplits = self.split_horizontally(buf_area);
  }
  /// Produces equal splits in `vertical direction`, i.e  `stacked one above another`, by dividing the entire terminal area by `net_vsplits`.
  fn split_vertically(&self, buf_area: Rect) -> Vec<Rect> {
    let mut x = self.vsplit_count;

    // The net_vsplits is just a counter
    x += 1;
    // vec![value; count];
    // the number of chunks you get out of Layout::split() always equals the number of constraints you put in.
    // Hence so the size of vector == net_split
    let constraints = vec![Constraint::Ratio(1, x); x as usize];
    ratatui::layout::Layout::default()
      .direction(ratatui::layout::Direction::Vertical)
      .constraints(constraints)
      .split(buf_area)
      .to_vec()
  }

  /// Produces equal splits in `horizontal direction`, i.e  `side by side`, by dividing the entire terminal area by `net_hsplits`.
  fn split_horizontally(&self, buf_area: Rect) -> Vec<Rect> {
    let mut x = self.hsplit_count;
    x += 1;
    let constraints = vec![Constraint::Ratio(1, x); x as usize];
    ratatui::layout::Layout::default()
      .direction(ratatui::layout::Direction::Horizontal)
      .constraints(constraints)
      .split(buf_area)
      .to_vec()
  }
  // Handles rendering logic for splits.
  pub fn render_vsplits(tui: &Tui, frame: &mut Frame) {
    let v_len = tui.splits.vsplits.len();
    for (i, chunk) in tui.splits.vsplits.iter().enumerate() {
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
  pub fn render_hsplits(tui: &Tui, frame: &mut Frame) {
    let h_len = tui.splits.hsplits.len();
    for (i, chunk) in tui.splits.hsplits.iter().enumerate() {
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
