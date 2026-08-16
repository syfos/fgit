use crate::{
  action::EventManager,
  ui::{buffer::Buffer, splits::Splits},
};
use ratatui::layout::Rect;

pub mod buffer;
pub mod renderer;
pub mod splits;
pub mod keys;
pub mod process_input;

/// Tui module of Fgit.
pub struct Tui {
  pub cur_buf: Buffer,
  pub buf_area: Rect,
  pub splits: Splits,
  pub event_manager: EventManager,
}

impl Tui {
  pub fn new() -> Self {
    Self {
      cur_buf: Buffer::GitHealth,
      buf_area: Rect::default(),
      splits: Splits::default(),
      event_manager: EventManager::default(),
    }
  }
}
