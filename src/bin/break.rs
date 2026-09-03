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

