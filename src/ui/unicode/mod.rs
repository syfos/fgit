use unicode_segmentation::UnicodeSegmentation;

#[allow(dead_code)]
pub struct Unicode;
#[allow(dead_code)]
impl Unicode {
  /// Returns unicode aware character
  /// width of each grapheme.
  /// The returned vector's indices map 1:1
  /// with char index of line.
  /// This give you flexibility to preform well
  /// cordinated unicode aware operations.
  pub fn get_grapheme_width(line: &str) -> Vec<usize> {
    let mut boundary = Vec::new();
    for grapheme in line.graphemes(true) {
      let grapheme_width = grapheme.chars().count();
      boundary.push(grapheme_width);
    }
    boundary
  }
}
