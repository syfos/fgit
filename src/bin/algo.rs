use unicode_bidi::BidiInfo;

fn main() {
  // Stored in logical order: م ر ح ب ا (typed order)
  let text = "Arabic مرحبا with english EMBEDDED then مزيد arabic";
  println!("{text}");

  // Resolve embedding levels. `None` = auto-detect paragraph direction.
  let bidi_info = BidiInfo::new(text, None);

  let para = &bidi_info.paragraphs[0];
  println!("Paragraph level: {}", para.level.number()); // odd = RTL
  println!("Is RTL: {}", para.level.is_rtl()); // true

  // Get the *visually* reordered line for display
  let line = para.range.clone();
  let display = bidi_info.reorder_line(para, line);
  println!("Visual order string: {}", display);
}
