use crate::{
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

  /// Main renderer that renders Fgit's whole Tui.
  pub fn renderer(
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
      if self.process_input()? {
        break Ok(());
      }
    }
  }
}
