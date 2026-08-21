#[derive(Default)]
pub struct Editor {
  pub line: Line,
  pub cursor: Cursor,
}

#[derive(Default)]
pub struct Line(pub Vec<char>);

/// The cursor position in `(row, col)` format.
#[derive(Default, Clone, Copy)]
pub struct Cursor(pub usize, pub usize);

impl Editor {
  pub fn push_char(&mut self, char: char) {
    self.line.0.insert(self.cursor.1 as usize, char);
  }
}
