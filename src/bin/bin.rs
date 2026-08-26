use anyhow::Context;
use ropey::Rope;
use std::path::{Path, PathBuf};
use std::{fs, io::BufReader};

fn string_to_path(path_string: &str) -> anyhow::Result<PathBuf> {
  let expanded = shellexpand::full(path_string)
    .with_context(|| format!("failed to expand path: `{path_string}`"))?;

  let canonical = Path::new(expanded.as_ref())
      .canonicalize()
      .with_context(|| format!("The expanded path do not exists or is inaccessible : `{expanded}` probably the `{path_string}` is wrong."))?;
  Ok(canonical)
}

fn main() -> anyhow::Result<()> {
  let mut rope = Rope::from_reader(BufReader::new(fs::File::open(string_to_path(
    "~/impl/rust/fgit/src/bin/txt.txt",
  )?)?))?;
  let prev_line = rope.line(0);
  println!("Line 0: {prev_line}");
  let char_len = prev_line.len_chars();
  println!("Line 0 char len: {char_len}");

  let s = rope.line_to_char(0) + char_len;
  println!("Char idx of last value of line 0: {char_len}");

  rope.remove(s-1..s);
  println!("{rope:?}");
  Ok(())
}
