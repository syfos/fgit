use icu_segmenter::{LineSegmenter, options::LineBreakOptions};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

fn main() {
  let segmenter = LineSegmenter::new_auto(LineBreakOptions::default());

  let text = "Hello, World! I am a good Guy. I a a very Good Person. Lsps.";

  // Breakpoint are grapheme aware, word aware and sepcially scripto continua aware.
  let breakpoints: Vec<usize> = segmenter.segment_str(text).collect();

  println!("{:?}", breakpoints);
}

// Get the slices at valid break points of strings.
#[allow(dead_code)]
fn get_breakpoint_slices(rope_line: &str, breakpoints: &[usize]) -> Vec<String> {
  breakpoints
    .windows(2)
    .map(|w| rope_line[w[0]..w[1]].to_string())
    .collect()
}

fn break_at_grapheme(slice: &str, viewport_width: &usize) -> Vec<String> {
  //
  let grapheme_aware_break = most_equal(&get_cumulative_widths_of_graphemes(slice), viewport_width);

  let mut wrap = Vec::new();
  wrap.push(slice[..grapheme_aware_break].to_string());

  let remainder = &slice[grapheme_aware_break..];
  // loops for all the cases where the remainer
  // would be wider than viewport_width
  // Note: The last value will be always dropped
  if remainder.width_cjk() > *viewport_width {
    wrap.extend(break_at_grapheme(remainder, viewport_width));
  }
  // This will catch such remainder whose terminal
  // width is lesser than viewport width
  else {
    wrap.push(remainder.to_string());
  }

  wrap
}

// handles cases
fn get_cumulative_widths_of_graphemes(slice: &str) -> Vec<(usize, usize)> {
  let mut cumulative_width_counter_per_grapheme = 0usize;
  let mut byte_idx = 0usize;
  let mut cumulative_widths_of_graphemes = Vec::new();
  for grpaheme in slice.graphemes(true) {
    byte_idx += grpaheme.len();
    cumulative_width_counter_per_grapheme += grpaheme.width_cjk();
    cumulative_widths_of_graphemes.push((byte_idx, cumulative_width_counter_per_grapheme));
  }
  cumulative_widths_of_graphemes
}

/// Returns the element index of the given vector
/// whose value matches the viewport_width such that
/// viewport width is always less than or equal to
/// viewport width.
///
/// Note: `cumulative_width_of_grapheme` is always in sorted form hence, no inaccuracies can be there.
fn most_equal(cumulative_widths_of_grapheme: &[(usize, usize)], viewport_width: &usize) -> usize {
  // The element index already has byte idx and the second value.
  let matched_element_idx = cumulative_widths_of_grapheme
    .partition_point(|grapheme| grapheme.1 <= *viewport_width)
    .saturating_sub(1);
  cumulative_widths_of_grapheme
    .get(matched_element_idx)
    .unwrap()
    .0
}
