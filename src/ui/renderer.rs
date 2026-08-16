use crate::{
  action::IoSignal,
  ui::{Tui, splits::SplitSeperator},
};
use ratatui::DefaultTerminal;
use std::{error::Error, result::Result};

impl Tui {
  /// Wrapper over [`ratatui::run`].
  pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
    ratatui::run(|terminal| self.renderer(terminal))?;
    Ok(())
  }

  /// Loop that `draws ui` and handles `input keys`.
  fn renderer(
    &mut self,
    terminal: &mut DefaultTerminal,
  ) -> std::result::Result<(), Box<dyn Error>> {
    loop {
      terminal.draw(|frame| {
        self.buf_area = frame.area();

        if !self.splits.vertical.splits.is_empty() {
          self.splits.vertical.render(frame, SplitSeperator::Bottom);
        }

        if !self.splits.horizontal.splits.is_empty() {
          self.splits.horizontal.render(frame, SplitSeperator::Right);
        }
      })?;

      match Self::handle_input() {
        Ok(IoSignal::Quit) => break Ok(()),

        Ok(IoSignal::Vsplit) => {
          self.splits.vertical.increment_count();
          self
            .splits
            .vertical
            .split(self.buf_area, ratatui::layout::Direction::Vertical);
        }

        Ok(IoSignal::Hsplit) => {
          self.splits.horizontal.increment_count();
          self
            .splits
            .horizontal
            .split(self.buf_area, ratatui::layout::Direction::Horizontal);
        }

        // Handle io error
        Err(e) => break Err(e),

        // Exhaustiv maych
        Ok(IoSignal::None) => {}
      }
    }
  }
}
