use crate::ui::editor::Editor;

#[allow(dead_code)]
pub struct ViewportLine {
  pub string: String,
  pub line_break_char: LineBreakChar,
}

#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
pub enum LineBreakChar {
  LF,
  FF,
  CR,
  CRLF,
  VT,
  PS,
  LS,
  NEL,
  None,
}

#[allow(dead_code)]
impl Editor {
  /// Returns `Vec<String>` that will be displayed in the viewport of editor.
  /// Note that each [`String`] of the vector is by default has only 1 valid line break.
  pub fn get_viewport_lines(&mut self, viewport_height: &usize) -> Vec<ViewportLine> {
    // The line_idx number of very first line to be rendered in viewport.
    let mut scroll_offset = self.scroll_offset;

    // A counter to track row along viewport_height
    let mut row_tracker = 0usize;

    let mut viewport_lines = Vec::new();

    while row_tracker < *viewport_height + 1 {
      let string = self.rope.line(scroll_offset).to_string();
      let line_break_char = Self::detect_trailing_linebreak_char(&string);
      viewport_lines.push(ViewportLine {
        string,
        line_break_char,
      });
      scroll_offset += 1;
      row_tracker += 1;
    }

    viewport_lines
  }

  /// Matches the last most line break unicode character and returns one of variant of [`LineBreakChar`].
  fn detect_trailing_linebreak_char(line: &str) -> LineBreakChar {
    if line.ends_with("\r\n") {
      return LineBreakChar::CRLF;
    }

    match line.chars().last() {
      Some('\n') => LineBreakChar::LF,
      Some('\r') => LineBreakChar::CR,
      Some('\u{0B}') => LineBreakChar::VT,
      Some('\u{0C}') => LineBreakChar::FF,
      Some('\u{85}') => LineBreakChar::NEL,
      Some('\u{2028}') => LineBreakChar::LS,
      Some('\u{2029}') => LineBreakChar::PS,
      _ => LineBreakChar::None,
    }
  }
}
