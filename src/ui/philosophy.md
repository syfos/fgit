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
