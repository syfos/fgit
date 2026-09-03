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

