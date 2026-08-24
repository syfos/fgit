use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::{Frame, layout::Rect};

use crate::ui::editor::{Editor, Line};

#[allow(dead_code)]
impl Editor {
  /// Creates new line in the buffer.
  pub fn create_new_line(&mut self) {
    self.buffer.0.push(Line::default());
    self.increment_cursor_row_by(1);
  }

  /// Inserts [`char`] at specific index of cursor line.
  pub fn insert(&mut self, idx: usize, char: char) {
    let line = self.mut_ref_cursor_line();
    line.insert(idx, char);
  }

  /// Removes specific index of cursor line.
  pub fn remove(&mut self, idx: usize) {
    let line = self.mut_ref_cursor_line();
    line.remove(idx);
  }

  /// Returns the refrence of cursor line vector.
  pub fn ref_cursor_line(&mut self) -> &Vec<char> {
    if self.buffer.0.is_empty() {
      self.buffer.0.insert(0, Line::default());
      return &self.buffer.0.first().unwrap().0;
    }
    &self.buffer.0.get(self.cursor.1).unwrap().0
  }

  /// Returns the `mut` refrence of cursor line vector.
  pub fn mut_ref_cursor_line(&mut self) -> &mut Vec<char> {
    if self.buffer.0.is_empty() {
      // self.buffer.0.insert(0, Line::default());
      return &mut self.buffer.0.get_mut(0).unwrap().0;
    }
    &mut self.buffer.0.get_mut(self.cursor.1).unwrap().0
  }

  pub fn render_buffer(&self, frame: &mut Frame, area: Rect) {
    let start_row = 0; // later: calculate from scroll offset
    let end_row = (start_row + area.height as usize).min(self.buffer.0.len());

    for (screen_row, line) in self.buffer.0[start_row..end_row].iter().enumerate() {
      let y = area.y + screen_row as u16;

      for (col, ch) in line.0.iter().enumerate() {
        if col >= area.width as usize {
          break;
        }

        let x = area.x + col as u16;

        frame.buffer_mut()[(x, y)].set_char(*ch);
      }
    }

    // Cursor
    let cx = area.x + self.cursor.0 as u16;
    let cy = area.y + (self.cursor.1 - start_row) as u16;

    if cx < area.x + area.width && cy < area.y + area.height {
      frame.buffer_mut()[(cx, cy)].set_style(Style::default().bg(Color::White).fg(Color::Black));
    }
  }
}
