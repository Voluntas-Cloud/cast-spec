# Hello

A minimal markdown fixture used to exercise the tree-sitter-md
anchor resolver. Heading paths are joined with `::`; inline emphasis
markers are stripped from the anchor surface.

## Architecture

A nested H2. Addressable as `Hello::Architecture` — the path-walk
joins the H1 and H2 with `::`.

### Mailbox

A deeper H3. Addressable as `Hello::Architecture::Mailbox` — verifies
the level-keyed stack extends past two segments.

## *Emphasized*

The inline emphasis markers `*` are part of the markdown but not the
anchor surface. Addressable as `Hello::Emphasized`, not
`Hello::*Emphasized*` — the inline-render rule strips the delimiters
before path comparison.
