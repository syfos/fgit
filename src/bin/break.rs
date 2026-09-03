use icu_segmenter::{LineSegmenter, options::LineBreakOptions};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

fn main() {
  let rope_line = "Hello, World! I am a good Guy. I a a very Good Person. Lsps.\n";

  get_breakpoints(rope_line);

  println!("Wrap lines: ");
  for i in wrap(rope_line, &40usize) {
    println!("{i:?}");
  }
}

/// Returns the vector containing breakpoints of given string.
/// Note: the breakpoints are `unicode-aware`, `grapheme-aware` and more specifically `scripto continua-aware`
fn get_breakpoints(rope_line: &str) -> Vec<usize> {
  LineSegmenter::new_auto(LineBreakOptions::default())
    .segment_str(rope_line)
    .collect()
}

/// Get the slices at valid break points of strings.
#[allow(dead_code)]
fn get_breakpoint_slices(rope_line: &str, breakpoints: &[usize]) -> Vec<String> {
  breakpoints
    .windows(2)
    .map(|w| rope_line[w[0]..w[1]].to_string())
    .collect()
}

/// Returns the slices of a line for softwrap.
/// Note: Only the last value will contain a linebreak char/unicode.
#[allow(dead_code)]
fn wrap(rope_line: &str, viewport_width: &usize) -> Vec<String> {
  let breakpoints = &get_breakpoints(rope_line);
  let mut wrap = Vec::new();
  let breakpoint_slices = get_breakpoint_slices(rope_line, breakpoints);

  let mut current_line = String::new();
  let mut current_width = 0usize;

  for str in breakpoint_slices.iter() {
    let slice_width = str.width_cjk();

    if slice_width > *viewport_width {
      if !current_line.is_empty() {
        wrap.push(std::mem::take(&mut current_line));
        current_width = 0;
      }
      wrap.extend(break_at_grapheme(str, viewport_width));
    } else if current_width + slice_width > *viewport_width {
      wrap.push(std::mem::take(&mut current_line));
      current_line.push_str(str);
      current_width = slice_width;
    } else {
      current_line.push_str(str);
      current_width += slice_width;
    }
  }

  if !current_line.is_empty() {
    wrap.push(current_line);
  }

  wrap
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

/// Holds `cumulative`width of a grapheme of a slice string
/// along the byte index the grapheme belongs to.
pub struct SliceData {
  pub byte_idx: usize,
  pub grapheme_cumulative_width: usize,
}

fn get_cumulative_widths_of_graphemes(slice: &str) -> Vec<SliceData> {
  let mut cumulative_width_counter_per_grapheme = 0usize;
  let mut byte_idx = 0usize;
  let mut cumulative_widths_of_graphemes = Vec::new();
  for grpaheme in slice.graphemes(true) {
    byte_idx += grpaheme.len();
    cumulative_width_counter_per_grapheme += grpaheme.width_cjk();
    cumulative_widths_of_graphemes.push(SliceData {
      byte_idx,
      grapheme_cumulative_width: cumulative_width_counter_per_grapheme,
    });
  }
  cumulative_widths_of_graphemes
}

/// Returns the element index of the given vector
/// whose value matches the viewport_width such that
/// viewport width is always less than or equal to
/// viewport width.
///
/// Note: `cumulative_width_of_grapheme` is always in sorted form hence, no inaccuracies can be there.
fn most_equal(cumulative_widths_of_grapheme: &[SliceData], viewport_width: &usize) -> usize {
  // The element index already has byte idx and the second value.
  let matched_element_idx = cumulative_widths_of_grapheme
    .partition_point(|grapheme| grapheme.grapheme_cumulative_width <= *viewport_width)
    .saturating_sub(1);
  cumulative_widths_of_grapheme
    .get(matched_element_idx)
    .unwrap()
    .byte_idx
}
