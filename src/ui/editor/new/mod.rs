use crate::ui::editor::Editor;

pub mod word_motion;
pub mod softwrap;

#[allow(dead_code)]
pub struct ViewportLine {
  pub string: String,
  pub line_break_char: LineBreakChar,
}

#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
#[allow(nonstandard_style)]
pub enum LineBreakChar {
  LineFeed,
  FromFeed,
  CarriageReturn,
  CarriageReturn_LineFeed,
  VerticalTab,
  ParagraphSeperator,
  LineSeperator,
  NextLine,
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
      return LineBreakChar::CarriageReturn_LineFeed;
    }

    match line.chars().last() {
      Some('\n') => LineBreakChar::LineFeed,
      Some('\r') => LineBreakChar::CarriageReturn,
      Some('\u{0B}') => LineBreakChar::VerticalTab,
      Some('\u{0C}') => LineBreakChar::FromFeed,
      Some('\u{85}') => LineBreakChar::NextLine,
      Some('\u{2028}') => LineBreakChar::LineSeperator,
      Some('\u{2029}') => LineBreakChar::ParagraphSeperator,
      _ => LineBreakChar::None,
    }
  }
}
