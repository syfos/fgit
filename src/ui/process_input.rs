use crate::{action::IoSignal, ui::Tui};
use std::error::Error;
impl Tui {
  pub fn process_input(&mut self) -> std::result::Result<bool, Box<dyn Error>> {
    // Immediate mode tui calculates on every frame.
    // Never call draw logic here.
    match Self::handle_input(self) {
      Ok(IoSignal::Quit) => Ok(true),

      Ok(IoSignal::Vsplit) => {
        self.splits.vertical.increment_count();
        Ok(false)
      }

      Ok(IoSignal::Hsplit) => {
        self.splits.horizontal.increment_count();
        Ok(false)
      }

      Ok(IoSignal::DelVsplit) => {
        self.splits.vertical.decrement_count();
        self.splits.vertical.del_split();
        Ok(false)
      }
      Ok(IoSignal::DelHsplit) => {
        self.splits.horizontal.decrement_count();
        self.splits.horizontal.del_split();
        Ok(false)
      }

      Ok(IoSignal::Up) => {
        self.cursor_row = self.cursor_row.saturating_sub(1);
        Ok(false)
      }
      Ok(IoSignal::Down) => {
        self.cursor_row = self.cursor_row.saturating_add(1);
        Ok(false)
      }
      Ok(IoSignal::Left) => {
        self.cursor_col = self.cursor_col.saturating_sub(1);

        Ok(false)
      }
      Ok(IoSignal::Right) => {
        self.cursor_col = self.cursor_col.saturating_add(1);
        Ok(false)
      }

      // Handle io error
      Err(e) => Err(e),

      // Exhaustiv maych
      Ok(IoSignal::None) => Ok(false),
    }
  }
}
