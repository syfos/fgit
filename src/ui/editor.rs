#[derive(Default)]
pub struct Editor {
  pub line: Line,
  /// current position of cursor in `(col, row)` format
  pub cursor: Cursor,
}

#[derive(Default)]
pub struct Line(pub Vec<char>);

/// The cursor position in `(col, row)` format.
#[derive(Default, Clone, Copy)]
pub struct Cursor(pub usize, pub usize);

impl Editor {
  pub fn push_char(&mut self, char: char) {
    self.line.0.insert(self.cursor.0, char);
  }
  pub fn remove_char(&mut self) {
    if !self.line.0.is_empty() {
      self.line.0.remove(self.cursor.0);
    }
  }
}
