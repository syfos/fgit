use unicode_normalization::{is_nfc, is_nfd};
fn main() {
  // no nfd, no nfd (true, true) -- contains none
  // is nfc, is nfd (false, false) -- contains both
  // is nfc, no nfd (true, false) -- only nfc
  // no nfc, is nfd (false, true) -- only nfd
  let s = "café e\u{301}";

  println!("{}", is_nfc(s));
  println!("{}", is_nfd(s));
}
