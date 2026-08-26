use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::{fs, io::BufReader};

use ropey::Rope;

pub mod cursor;
pub mod edit;
pub mod helper;
pub mod render;

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
