use ropey::Rope;

pub mod buffer;
pub mod cursor;
pub mod edit;

#[derive(Default)]
pub struct Editor {
  pub buffer: Buffer,
  pub line: Line,
  pub doc: Rope,
  /// current position of cursor in `(col, row)` format
  pub cursor: Cursor,
}

/// Lines create a [`Buffer`].
#[derive(Default)]
pub struct Line(pub Vec<char>);

/// Buffer is a collection of [`Line`].
#[derive(Default)]
pub struct Buffer(pub Vec<Line>);

/// The cursor position in `(col, row)` format.
#[derive(Default, Clone, Copy)]
pub struct Cursor(pub usize, pub usize);
