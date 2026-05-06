---
id: 002-reply
role: assistant
in_reply_to: 001-prompt
created: 2026-05-01T00:01:30Z
---

Two natural homes:

- `crates/cast-spec/src/language_backend/` — a new `rust_macro` backend
  alongside the tree-sitter ones. It would walk macro-call sites and
  match on the macro path.
- `crates/cast-spec/src/spec_source/` — the macro is *already* a
  spec source (rust_inline). What you're describing might be better
  framed as letting `cast::io::continues_in!` target a macro call
  site with `lang: rust_macro, anchor: "cast::concept"`.

Want me to draft either as a `json draft`?
