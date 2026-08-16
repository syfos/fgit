use ratatui::{
  Frame,
  layout::{Constraint, Direction, Rect},
  widgets::{Block, Borders, Paragraph},
};

/// Contains splits based on two directions `Horizontal` and `Vertical`.
#[derive(Default)]
pub struct Splits {
  pub horizontal: SplitAxis,
  pub vertical: SplitAxis,
}

/// Pick `Bottom` for rendering `Vertical` splits and `Right` for rendering `Horizontal` splits.
pub enum SplitSeperator {
  Bottom,
  Right,
}

impl SplitSeperator {
  /// Returns the equivalent [`Borders`] bitflag.
  /// ```
  /// match self {
  ///   SplitSeperator::Bottom => Borders::BOTTOM,
  ///   SplitSeperator::Right => Borders::RIGHT,
  /// }
  /// ```
  fn as_borders(&self) -> Borders {
    match self {
      SplitSeperator::Bottom => Borders::BOTTOM,
      SplitSeperator::Right => Borders::RIGHT,
    }
  }
}

/// Contains count and splits for two underlying directions `Horizontal` and `Vertical`.
#[derive(Default)]
pub struct SplitAxis {
  pub count: u32,
  pub splits: Vec<Rect>,
}

impl SplitAxis {
  pub fn increment_count(&mut self) {
    self.count = self.count.saturating_add(1);
  }

  pub fn split(&mut self, buf_area: Rect, direction: Direction) {
    let x = self.count + 1;
    let constraints = vec![Constraint::Ratio(1, x); x as usize];
    self.splits = ratatui::layout::Layout::default()
      .direction(direction)
      .constraints(constraints)
      .split(buf_area)
      .to_vec()
  }

  pub fn render(&self, frame: &mut Frame, seperator: SplitSeperator) {
    let seperator = seperator.as_borders();
    for (i, chunk) in self.splits.iter().enumerate() {
      let borders = if (i + 1) < self.splits.len() {
        seperator
      } else {
        Borders::NONE
      };
      let block = Block::default().borders(borders);
      let paragraph = Paragraph::new(format!("Panel: {i}")).block(block);
      frame.render_widget(paragraph, *chunk);
    }
  }
}
