use crate::ui::editor::Editor;
impl Editor {
  /// increment cursor col by the given number, clamp at length of cursor row.
  pub fn increment_cursor_col(&mut self, col: usize) {
    self.cursor.0 = self
      .cursor
      .0
      .saturating_add(col)
      .min(self.rope.line(self.cursor.1).len_chars());
  }

  /// increment cursor row by the given number, clamp at last line.
  pub fn increment_cursor_row(&mut self, row: usize) {
    if self.rope.len_lines() != 0 && self.rope.len_lines() >= 1 {
      self.cursor.1 = self
        .cursor
        .1
        .saturating_add(row)
        .min(self.rope.len_lines() - 1);
      self.cursor.0 = self.rope.line(self.cursor.1).len_chars();
    }
  }

  /// Decrement cursor column by given number, clamps at 0.
  pub fn decrement_cursor_col(&mut self, col: usize) {
    self.cursor.0 = self.cursor.0.saturating_sub(col);
  }

  /// Decrement cursor row by given number, clamps at 0.
  pub fn decrement_cursor_row(&mut self, row: usize) {
    self.cursor.1 = self.cursor.1.saturating_sub(row);
  }
}
