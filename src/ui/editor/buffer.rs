use crate::ui::editor::{Editor, Line};

#[allow(dead_code)]
impl Editor {
  pub fn create_new_line(&mut self) {
    self.buffer.0.push(Line::default());
    self.increment_cursor_row_by(1);
  }
}
