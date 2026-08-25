use crate::ui::editor::Editor;
use crate::ui::{Tui, splits::Splits};
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
        self.screen_area = frame.area();
        Splits::render(&mut self.splits, frame);
        Editor::render_rope(&mut self.editor, frame, self.screen_area);

        // buffer_mut gives mut ref of buffer
        // [(self.cursor_col, self.cursor_row)] means access the given cell.
        // Note that every (cursor_col, cursor_row) is a cell
        // frame.buffer_mut()[(self.editor.cursor.0 as u16, self.editor.cursor.1 as u16)]
        //   .set_style(Style::default().bg(Color::White).fg(Color::Black));
      })?;

      if self.process_input()? {
        break Ok(());
      } else {
        continue;
      }
    }
  }
}
