use unicode_bidi::BidiInfo;
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
  /// Viewport lines into Grapheme aware lines
  pub viewport_grapheme_lines: Vec<Vec<Graphemes>>,
  /// Viewport lines into Bidirection aware lines
  pub viewport_bidirectional_lines: Vec<BidiAwareLine>,
}

/// Data regarding the bidirectional line for rendering
#[allow(dead_code)]
pub struct BidiAwareLine {
  pub level_number: u8,
  pub is_rtl: bool,
  pub reordered_line: String,
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone)]
pub struct Graphemes {
  /// The range of `Unicode` aware Grapheme between two absolute character indicies of a rope line.
  pub rope_absolute_char_index_range: std::ops::Range<usize>,

  /// The width in `terminal cell` the Grapheme is occupying.
  /// Used for cursor movement in terminal column.
  pub term_cell_width: usize,

  // The cumulative width i.e sum of width of terminal cells occupied by very first `Grapheme` all the way to current `Grapheme`.
  // The cumulative_term_cell_width of very first Grapheme is always equal to `term_cell_width` of the Grapheme.
  // Derived by `cumulative_term_cell_width += term_cell_width` for each grapheme.
  pub cumulative_term_cell_width: usize,
}

#[allow(dead_code)]
impl Unicode {
  /// Returns [`Vec<Graphemes>`] which contains all
  /// `Graphemes` of given string along its `cell width` and its `cumulative_term_cell_width`.
  /// ```
  ///pub struct Graphemes {
  ///  /// The range of `Unicode` aware Grapheme between two absolute character indicies of a rope line.
  ///  pub rope_absolute_char_index_range: std::ops::Range<usize>,
  ///
  ///  /// The width in `terminal cell` the Grapheme is occupying.
  ///  /// Used for cursor movement in terminal column.
  ///  pub term_cell_width: usize,
  ///
  ///  // The cumulative width i.e sum of width of terminal cells occupied by very first `Grapheme` all the way to current `Grapheme`.
  ///  // The cumulative_term_cell_width of very first Grapheme is always equal to `term_cell_width` of the Grapheme.
  ///  // Derived by `cumulative_term_cell_width += term_cell_width` for each grapheme.
  ///  pub cumulative_term_cell_width: usize,
  ///}
  ///```
  pub fn into_grapheme_line(line: &str, line_to_char: &usize) -> Vec<Graphemes> {
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
      boundary.push(Graphemes {
        rope_absolute_char_index_range: start..end,
        term_cell_width,
        cumulative_term_cell_width: cumulative_net_width,
      });
    }

    // Each Vec<Graphemes> is a Line
    boundary
  }

  /// Flips chars of `RTL` language (e.g `Arabic`, `Persian`, `Hebrew`) words of given line into `RTL` logical sequence words keeping non RTL words intact.
  pub fn into_bidirectional_line(line: &str) -> BidiAwareLine {
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

    BidiAwareLine {
      level_number,
      is_rtl,
      reordered_line,
    }
  }

  /// This function returns the `[CanonicalType]` of Normalization form of the given string.
  ///
  /// This function is purely for search/replace command.
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
