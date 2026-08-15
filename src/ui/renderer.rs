use ratatui::DefaultTerminal;
use crate::{
  action::IoSignal,
  ui::{Tui, splits::Splits},
};
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
        let area = frame.area();
        self.buf_area = area;

        if self.splits.is_render_vertical {
          Splits::render_vsplits(self, frame);
        }

        if self.splits.is_render_horizontal {
          Splits::render_hsplits(self, frame);
        }
      })?;

      match Self::handle_input() {
        Ok(IoSignal::Quit) => break Ok(()),

        Ok(IoSignal::Vsplit) => {
          self.splits.increment_vsplit_count();
          self.splits.update_vsplits(self.buf_area);
          self.splits.is_render_vertical = true;
        }

        Ok(IoSignal::Hsplit) => {
          self.splits.increment_hsplit_count();
          self.splits.update_hsplits(self.buf_area);
          self.splits.is_render_horizontal = true;
        }

        // Handle io error
        Err(e) => break Err(e),

        // Exhaustiv maych
        Ok(IoSignal::None) => {}
      }
    }
  }
}
