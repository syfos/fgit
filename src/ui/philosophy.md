# Softwrap

> Break only at: ASCII spaces, commas, or after/before such special symbols where it feels right, so the user never sees corrupted soft-wrap.

# Cursor movement philosophy

> There is no need for a GraphemeCursor or persistent word-boundary structure. Just parse the current line and directly jump to the word start/word end. Reparse the line whenever it changes.
