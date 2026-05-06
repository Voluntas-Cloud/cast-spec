---
id: 003-draft
role: user
in_reply_to: 002-reply
created: 2026-05-01T00:03:00Z
---

Yes, the second framing — the macro is a spec source, not a foreign
language. Here's the shape I had in mind for the IO edge variant:

```json draft
{
  "intent": "Allow cast::io::continues_in! to point at a macro call site by path, so the assistant graph can name macro invocations the way it names functions.",
  "payload": {
    "macro": "cast::io::continues_in",
    "new_lang_value": "rust_macro",
    "anchor_form": "rust_path naming the macro to match",
    "resolution_rule": "search the cast-watch HIR walker output for macro_call nodes whose resolved path equals anchor; resolved if at least one site matches"
  }
}
```
