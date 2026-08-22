use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::{Frame, layout::Rect};

#[derive(Default)]
pub struct Editor {
  pub line: Line,
  /// current position of cursor in `(col, row)` format
  pub cursor: Cursor,
}

#[derive(Default)]
pub struct Line(pub Vec<char>);

/// The cursor position in `(col, row)` format.
#[derive(Default, Clone, Copy)]
pub struct Cursor(pub usize, pub usize);

impl Editor {
  pub fn push_char(&mut self, char: char) {
    if self.line.0.is_empty() {
      self.cursor.0 = 0;
      self.line.0.insert(0, char);
      self.cursor.0 = self.cursor.0.saturating_add(1);
    } else if self.cursor.0 > self.line.0.len() - 1 {
      self.line.0.push(char);
      self.cursor.0 = self.line.0.len();
    } else if self.cursor.0 <= self.line.0.len() - 1 {
      self.line.0.push(char);
      self.cursor.0 = self.line.0.len();
    }
  }
  pub fn remove_char(&mut self) {
    if !self.line.0.is_empty() {
      self.line.0.remove(self.cursor.0);
    }
  }

  pub fn render(&self, frame: &mut Frame, area: Rect) {
    // draw each character
    for (i, ch) in self.line.0.iter().enumerate() {
      let x = area.x + i as u16;
      let y = area.y + self.cursor.1 as u16;
      frame.buffer_mut()[(x, y)].set_char(*ch);
    }

    // draw cursor on top — note: if cursor is past the last char,
    // this cell just shows a blank space with inverted colors
    let cx = area.x + self.cursor.0 as u16;
    let cy = area.y + self.cursor.1 as u16;
    frame.buffer_mut()[(cx, cy)].set_style(Style::default().bg(Color::White).fg(Color::Black));
  }
}
