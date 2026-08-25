// use crate::ui::editor::Editor;
//
// #[allow(clippy::needless_return)]
// impl Editor {
//   /// Push the typed `char` to `current col` of current line at qwhich cursor is.
//   pub fn push_char(&mut self, char: char) {
//     // Insert char at index 0 if line is empty
//     if self.ref_cursor_line().is_empty() {
//       self.mut_ref_cursor_line().insert(0, char);
//       self.increment_cursor_col_by(1);
//       return;
//     }
//
//     // Insert char at cursor position
//     if self.ref_cursor_line().len() == 1 {
//       self.insert(self.cursor.0, char);
//       self.increment_cursor_col_by(1);
//       return;
//     }
//
//     // Insert char at cursor position when cursor is inside line width.
//     if self.ref_cursor_line().len() > self.cursor.0 {
//       self.insert(self.cursor.0, char);
//       self.increment_cursor_col_by(1);
//       return;
//     }
//
//     // if curosr is at the full width of line i.e the last column of line then push to create new index.
//     if self.mut_ref_cursor_line().len() == self.cursor.0 {
//       self.mut_ref_cursor_line().push(char);
//       self.increment_cursor_col_by(1);
//       return;
//     }
//   }
//
//   /// Remove character which is one coloumn behind cursor.
//   /// For empty line if and only if [`Buffer`] is not empty nor it contains only one line then -> Move to the end of previous line joining the previous line with current one.
//   // pub fn remove_char(&mut self) {
//   //   // Don't trigger when the line is empty
//   //   if self.cursor.0 != 0 {
//   //     self.remove(self.cursor.0.saturating_sub(1));
//   //     self.decrement_cursor_col_by(1);
//   //     return;
//   //   }
//   // }
//
//   /// Increment cursor column by the given number but clamp it at the length of line.
//   pub fn increment_cursor_col_by(&mut self, col: usize) {
//     self.cursor.0 = self
//       .cursor
//       .0
//       .saturating_add(col)
//       .min(self.ref_cursor_line().len());
//   }
//
//   /// Decrement cursor col by given number, clamps at 0.
//   pub fn decrement_cursor_col_by(&mut self, col: usize) {
//     self.cursor.0 = self.cursor.0.saturating_sub(col);
//   }
//
//   /// Increment cursor row by given number, clamps at max length of Buffer i.e at the last line.
//   pub fn increment_cursor_row_by(&mut self, row: usize) {
//     self.cursor.1 = self.cursor.1.saturating_add(row).min(self.buffer.0.len()-1);
//   }
//
//   /// Decrement cursor row by given number, clamp at zero.
//   pub fn decrement_cursor_row_by(&mut self, row: usize) {
//     self.cursor.1 = self.cursor.1.saturating_sub(row);
//   }
// }
