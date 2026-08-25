use crate::ui::editor::Editor;

#[allow(dead_code)]
impl Editor {
  pub fn insert_char(&mut self, char: char) {
    let cursor_idx = self.rope.line_to_char(self.cursor.1) + self.cursor.0;
    self.rope.insert_char(cursor_idx, char);
    self.increment_cursor_col(1);
  }

  pub fn new_line(&mut self) {
    self
      .rope
      .insert_char(self.rope.line_to_char(self.cursor.1) + self.cursor.0, '\n');
    self.increment_cursor_row(1);
    self.cursor.0 = 0;
  }

  pub fn remove_char(&mut self) {
    let line = self.rope.line(self.cursor.1).to_string();
    if line.is_empty() {
      return;
    }

    let cursor_idx = self.rope.line_to_char(self.cursor.1) + self.cursor.0;
    self.rope.remove(cursor_idx.saturating_sub(1)..cursor_idx);
    self.decrement_cursor_row(1);
  }
}
