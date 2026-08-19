use crate::{action::IoSignal, ui::Tui};
use std::error::Error;
impl Tui {
  pub fn process_input(&mut self) -> std::result::Result<bool, Box<dyn Error>> {
    match Self::handle_input() {
      Ok(IoSignal::Quit) => Ok(true),

      Ok(IoSignal::Vsplit) => {
        self.splits.vertical.increment_count();
        self
          .splits
          .vertical
          .split(self.screen_area, ratatui::layout::Direction::Vertical);
        Ok(false)
      }

      Ok(IoSignal::Hsplit) => {
        self.splits.horizontal.increment_count();
        self
          .splits
          .horizontal
          .split(self.screen_area, ratatui::layout::Direction::Horizontal);
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

      // Handle io error
      Err(e) => Err(e),

      // Exhaustiv maych
      Ok(IoSignal::None) => Ok(false),
    }
  }
}
