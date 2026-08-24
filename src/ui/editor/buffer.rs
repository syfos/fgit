use crate::ui::editor::{Editor, Line};

#[allow(dead_code)]
impl Editor {
  pub fn create_new_line(&mut self) {
    self.buffer.0.push(Line::default());
    self.increment_row_by(1);
  }
  fn iter(&mut self) {
    for (idx, value) in self.buffer.0.iter().enumerate() {

    }
  }
}
