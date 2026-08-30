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

/// Gives Unicode support to Sycode.
#[allow(dead_code)]
pub struct Unicode {
  pub grapheme: Vec<Grapheme>,
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone)]
pub struct Grapheme {
  /// Range of Unicode aware grapheme between two absolute indicies of the Displayed rope line.
  pub rope_absolute_char_index_range: std::ops::Range<usize>,

  /// Cell width of the Grapheme for cursor movement in column
  pub term_cell_width: usize,

  // Cumulative sum i.e sum of all the previous width all the way to current for the same line. Just for comparison about which grapheme width is most near to viewport width. 
  pub cumulative_net_width: usize,
}

#[allow(dead_code)]
impl Unicode {
  /// Returns unicode aware character
  /// width of each grapheme.
  /// The returned vector's indices map 1:1
  /// with char index of line.
  /// This give you flexibility to preform well
  /// cordinated unicode aware operations.
  pub fn get_grapheme_ranges(line: &str, line_to_char: &usize) -> Vec<Grapheme> {
    let mut boundary = Vec::new();
    let mut offset = 0usize;
    let mut cumulative_net_width = 0usize;
    for grapheme in line.graphemes(true) {
      let grapheme_net_chars = grapheme.chars().count();
      let start = *line_to_char + offset;
      offset += grapheme_net_chars;
      let end = *line_to_char + offset;
      let term_cell_width = grapheme.width();
      cumulative_net_width += term_cell_width;
      boundary.push(Grapheme {
        rope_absolute_char_index_range: start..end,
        term_cell_width,
        cumulative_net_width,
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
