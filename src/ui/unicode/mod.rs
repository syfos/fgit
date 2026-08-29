use ropey::Rope;
use unicode_normalization::{is_nfc, is_nfd};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub enum CanonicalType {
  /// String contains `NFC` along `NFD`.
  Mix,
  /// String contains only `NFC`
  Nfc,
  /// String contains only `NFD`
  Nfd,
  /// String conatins neither of `NFC` or `NFD`
  None,
}

#[allow(dead_code)]
pub struct Unicode;

#[allow(dead_code)]
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
  /// This function returns the `[CanonicalType]` of Normalization form of the given string.
  /// ```
  /// use unicode_normalization::{is_nfc, is_nfd};
  ///
  /// match (is_nfc(query), is_nfd(query)) {
  ///   // Means there is no NFC and NFD
  ///   (true, true) => CanonicalType::None,
  ///   // Means there is only NFC
  ///   (true, false) => CanonicalType::Nfc,
  ///   // Means there is only NFD
  ///   (false, true) => CanonicalType::Nfd,
  ///   // Means there is both
  ///   (false, false) => CanonicalType::Mix,
  /// }
  /// ```
  ///
  pub fn check_canonical_form(query: &str) -> CanonicalType {
    match (is_nfc(query), is_nfd(query)) {
      // Means there is no NFC and NFD
      (true, true) => CanonicalType::None,
      // Means there is only NFC
      (true, false) => CanonicalType::Nfc,
      // Means there is only NFD
      (false, true) => CanonicalType::Nfd,
      // Means there is both
      (false, false) => CanonicalType::Mix,
    }
  }

}
