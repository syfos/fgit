#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum LineBreakChar {
  LF,
  FF,
  CR,
  CRFL,
  VT,
  PS,
  LS,
  NEL,
  None,
}
fn detect_trailing_linebreak_char(line: &str) -> LineBreakChar {
  if line.ends_with("\r\n") {
    return LineBreakChar::CRFL;
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
fn main() {
  let bk = detect_trailing_linebreak_char("Hello, World!\r\n");
  println!("{:?}", bk);
}
