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

