#[derive(Default)]
pub struct Editor {
  pub line: Line,
  pub cursor: Cursor,
}

#[derive(Default)]
pub struct Line(pub Vec<char>);

/// The cursor position in `(col, row)` format.
#[derive(Default, Clone, Copy)]
pub struct Cursor(pub usize, pub usize);

impl Editor {
  pub fn push_char(&mut self, char: char) {
    self.line.0.insert(self.cursor.1 as usize, char);
  }
  pub fn remove_char(&mut self) {
    if self.line.0.is_empty() {
      self.cursor.1 = self.cursor.1.saturating_sub(1);
    } else {
      self.line.0.remove(self.cursor.1 as usize);
      self.cursor.1 = self.cursor.1.saturating_sub(1);
    }
  }
}
