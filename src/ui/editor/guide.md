# Insertion

Api :
```rust
Rope::insert()
```

Usage:  
```rust
let char_idx = rope.line_to_char(line_idx) + cursor_col;
rope.insert(char_idx, char);
println!("{rope}");
```

# Backspace based single character deletion.

Api: 
```rust
Rope::remove()
```

Usage:
```rust
let cursor_idx = rope.line_to_char(line_idx) + cursor_col;
rope.remove(cursor_idx - 1..cursor_idx);
```
Here cursor_idx is the absolute char index of line. On backspace we delete the previous character to the visible cursor hence `cursor_idx - 1`.

Rust's syntax ->
```rust
cursor_idx - 1 .. cursor_idx
```

this literally means -> in range 8 all the way behind 9

