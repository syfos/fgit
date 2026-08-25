use crate::ui::editor::Editor;
use ratatui::{
  Frame,
  layout::{Position, Rect},
  text::{Line, Text},
  widgets::Paragraph,
};
impl Editor {
  pub fn render_rope(&mut self, frame: &mut Frame, area: Rect) {
    let net_lines = self.rope.len_lines();
    let start_line = self.scroll_offset;
    let end_line = (start_line + area.height as usize).min(net_lines);

    let lines: Vec<ratatui::text::Line> = (start_line..end_line)
      .map(|i| {
        let rope_slice = self.rope.line(i).to_string();
        let line = rope_slice.trim_end_matches('\n');
        Line::raw(line.to_string())
      })
      .collect();

    let paragraph = Paragraph::new(Text::from(lines));
    frame.render_widget(paragraph, area);

    let screen_row = area.y + (self.cursor.1 - start_line) as u16;
    let screen_col = area.x + self.cursor.0 as u16;

    frame.set_cursor_position(Position::new(screen_col, screen_row));
  }
}
