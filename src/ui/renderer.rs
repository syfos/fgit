use crate::ui::{Tui, splits::Splits};
use ratatui::{DefaultTerminal, style::Style};
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
        self.screen_area = frame.area();
        Splits::render(&mut self.splits, frame);
        frame.buffer_mut()[(0, 0)].set_style(Style::default().bg(ratatui::style::Color::White).fg(ratatui::style::Color::Black));
      })?;
      if self.process_input()? {
        break Ok(());
      } else {
        continue;
      }
    }
  }
}
