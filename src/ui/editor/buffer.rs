use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::{Frame, layout::Rect};

use crate::ui::editor::{Editor, Line};

#[allow(dead_code)]
impl Editor {
  /// Creates new line in the buffer.
  pub fn create_new_line(&mut self) {
    self.buffer.0.push(Line::default());
    self.increment_cursor_row_by(1);
  }

  /// Inserts [`char`] at specific index of cursor line.
  pub fn insert(&mut self, idx: usize, char: char) {
    let line = self.mut_ref_cursor_line();
    line.insert(idx, char);
  }

}
