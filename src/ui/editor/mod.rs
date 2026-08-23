pub mod cursor;

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
