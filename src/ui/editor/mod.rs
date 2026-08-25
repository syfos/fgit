use ropey::Rope;

pub mod cursor;
pub mod edit;

#[derive(Default)]
pub struct Editor {
  pub scroll_offset: usize,
  pub rope: Rope,
  /// current position of cursor in `(col, row)` format
  pub cursor: Cursor,
}
/// The cursor position in `(col, row)` format.
#[derive(Default, Clone, Copy)]
pub struct Cursor(pub usize, pub usize);
