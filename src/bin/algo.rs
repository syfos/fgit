use unicode_bidi::BidiInfo;

/// Data regarding the bidirectional line for rendering
pub struct BidiLine {
  pub level_number: u8,
  pub is_rtl: bool,
  pub reordered_line: String,
}

/// Flips chars of `RTL` language (e.g `Arabic`, `Persian`, `Hebrew`) words into `RTL` logical sequence, if line contains any of them.
pub fn into_bidirectional_line(line: &str) -> BidiLine {
  let bidi_info = BidiInfo::new(line, None);

  // The internal UBA algorith works on Paragraphs.
  // Hence if your line does contains any Paragraph
  // seperator then this would consider it as
  // (net_paragraph_seperator_count + 1)
  //
  // List of Paragraoh Seperator:
  // [LF, CR, CRLF, NEL, LS, PS]
  let paragraphs = &bidi_info.paragraphs[0];

  // Odd number means RTL word
  // Even number means LTR word
  let level_number = paragraphs.level.number();
  let is_rtl = paragraphs.level.is_rtl();

  // Tells which part belong to what paragraph.
  let paragraph_range = paragraphs.range.clone();

  let reordered_line = bidi_info
    .reorder_line(paragraphs, paragraph_range)
    .to_string();

  BidiLine {
    level_number,
    is_rtl,
    reordered_line,
  }
}

fn main() {
  into_bidirectional_line("مزحبا");
}
