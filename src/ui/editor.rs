#[derive(Default)]
pub struct Editor {
  pub text: Vec<Line>,
  pub cursor_col: u16,
  pub cursor_row: u16,
}

#[derive(Default)]
pub struct Line {
  pub line: Vec<char>,
}

