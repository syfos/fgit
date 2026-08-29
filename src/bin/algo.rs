use unicode_normalization::{is_nfc, is_nfd};

enum CanonicalType {
  /// String contains `NFC` along `NFD`.
  Mix,
  /// String contains only `NFC`
  Nfc,
  /// String contains only `NFD`
  Nfd,
  /// String conatins neither of `NFC` or `NFD`
  None,
}

fn main() {
  // no nfd, no nfd (true, true) -- contains none
  // is nfc, is nfd (false, false) -- contains both
  // is nfc, no nfd (true, false) -- only nfc
  // no nfc, is nfd (false, true) -- only nfd
  let s1 = "café e\u{301}";
  let s2 = "café";
  let s3 = "cafe\u{301}";
  let s4 = "Plain";

  let s = s4;

  let a = match (is_nfc(s), is_nfd(s)) {
    (true, true) => CanonicalType::None,
    (true, false) => CanonicalType::Nfc,
    (false, true) => CanonicalType::Nfd,
    (false, false) => CanonicalType::Mix,
  };

  match a {
    CanonicalType::Mix => println!("MIX"),
    CanonicalType::Nfc => println!("NFC"),
    CanonicalType::Nfd => println!("NFD"),
    CanonicalType::None => println!("NONE"),
  }
}
