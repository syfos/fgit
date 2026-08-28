use unicode_segmentation::UnicodeSegmentation;

fn main() {
  let range = get_grapheme_ranges("👨‍👨‍👧‍👦", &7);
  println!("{:?}", range);
}

pub fn get_grapheme_ranges(line: &str, line_to_char: &usize) -> Vec<std::ops::Range<usize>> {
  let mut boundary = Vec::new();
  let mut offset = 0usize;
  for grapheme in line.graphemes(true) {
    let grapheme_width = grapheme.chars().count();
    let start = *line_to_char + offset;
    offset += grapheme_width;
    let end = *line_to_char + offset;
    boundary.push(start..end);
  }
  boundary
}
