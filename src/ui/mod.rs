use crate::ui::{editor::Editor, splits::Splits};
use ratatui::layout::Rect;

pub mod buffer;
pub mod editor;
pub mod keys;
pub mod process_input;
pub mod renderer;
pub mod splits;

/// Tui module of Fgit.
pub struct Tui {
  pub screen_area: Rect,
  pub splits: Splits,
  pub editor: Editor,
}

impl Tui {
  pub fn new() -> Self {
    Self {
      screen_area: Rect::default(),
      splits: Splits::default(),
      editor: Editor::default(),
    }
  }
}
