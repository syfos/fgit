use crate::{
  action::EventManager,
  ui::{buffer::ParentBuf, splits::Splits},
};
use ratatui::layout::Rect;

pub mod buffer;
pub mod keys;
pub mod process_input;
pub mod renderer;
pub mod splits;

/// Tui module of Fgit.
pub struct Tui {
  pub parent_buf: ParentBuf,
  pub screen_area: Rect,
  pub splits: Splits,
  pub event_manager: EventManager,
  pub cursor_col: u16,
  pub cursor_row: u16,
}

impl Tui {
  pub fn new() -> Self {
    Self {
      parent_buf: ParentBuf::GitHealth,
      screen_area: Rect::default(),
      splits: Splits::default(),
      event_manager: EventManager::default(),
      cursor_col: 0,
      cursor_row: 0,
    }
  }
}
