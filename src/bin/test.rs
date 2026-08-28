fn main() {
  use unicode_segmentation::UnicodeSegmentation;

  let rope_string = "A 👩‍💻 B é";

  for (byte_idx, grapheme) in rope_string.grapheme_indices(true) {
    println!("{byte_idx}: {grapheme:?}");
  }
}
