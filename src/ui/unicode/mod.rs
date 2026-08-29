use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[allow(dead_code)]
pub struct Unicode;

pub struct GraphemeLine {
  pub range: std::ops::Range<usize>,
  pub width: usize,
}

#[allow(dead_code)]
impl Unicode {
  /// Returns unicode aware character
  /// width of each grapheme.
  /// The returned vector's indices map 1:1
  /// with char index of line.
  /// This give you flexibility to preform well
  /// cordinated unicode aware operations.
  pub fn get_grapheme_ranges(line: &str, line_to_char: &usize) -> Vec<GraphemeLine> {
    let mut boundary = Vec::new();
    let mut offset = 0usize;
    for grapheme in line.graphemes(true) {
      let grapheme_char_count = grapheme.chars().count();
      let start = *line_to_char + offset;
      offset += grapheme_char_count;
      let end = *line_to_char + offset;
      let grapheme_width = grapheme.width();
      boundary.push(GraphemeLine {
        range: start..end,
        width: grapheme_width,
      });
    }
    boundary
  }
}
