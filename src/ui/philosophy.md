# Softwrap

> Break only at: ASCII spaces, commas, or after/before such special symbols where it feels right, so the user never sees corrupted soft-wrap.

# Cursor movement philosophy

> There is no need for a GraphemeCursor or persistent word-boundary structure. Just parse the current line and directly jump to the word start/word end. Reparse the line whenever it changes.

# Rope and Render 1:1 on line break.

By default Ropey crate supports the following line break characters :

U + 000A -- LF (Line Feed) -- \n -- Move to next line.
U + 000B -- VT (Vertical Tab) -- \v --Move vertically down.
U + 000C -- Form Feed -- \f -- Advance to the next page/form
U + 000D -- Cariage Return -- \r -- Return to the beginning of the line
U + 0085 -- Next Line NEL -- Go to next line
U + 2028 -- Line Seperator -- Explicit Unicode line seperator 
U + 2029 -- Paragraph Seperator PS -- Explict Unicode Paragraoh Seperator.


+

CRLF -> Carriage return + Line Feed.
VT is the counter part of HT(Horizontal Tabbing).
Form Feed -- Switch to next page but since in text editor there is no page concept the way similar to typewriters hence ropey instead take it as a line break.

NEL -> for terminal terminology

LS & PS are modern concepts hence so worth taking seriously more than the LF.

> The viewport renderer must be 1:1 with rope recognized newline sequences.

> LF must be the top priority as whole modern ecosystem depends on it. Hence so Enter must give insert "\n".

> Ropey crate treats all the above listed unicodes as real line breaks. If you stacks them all in a single line through python cmd :

```sh
python3 -c 'w=["apple","river","mountain","window","garden","computer","language"]; b=["\u000A","\u000B","\u000C","\u000D","\u0085","\u2028","\u2029"]; open("txt.txt","w",encoding="utf-8",newline="").write("".join(x+"".join(b[j] for j in range(7) if j!=i) for i,x in enumerate(w)))'
```

```sh
python3 -c 'w=["apple","river","mountain","window","garden","computer","language"]; b=["\u000A","\u000B","\u000C","\u000D","\u0085","\u2028","\u2029"]; open("txt.txt","w",encoding="utf-8",newline="").write("".join(x+b[i]+b[(i+1)%7] for i,x in enumerate(w)))'
```

and run the following :

```rust
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
  println!("chars: {}", rope.len_chars());
  println!("lines: {}", rope.len_lines());

  for i in 0..rope.len_lines() {
    println!("line {i}: {:?}", rope.line(i));
  }
  Ok(())
}
```

> You will clearly see that each line break unicode is take seriously.

Hence, iterate for each of theme and create the next line for each of their occurences.
