# Tree-sitter integration

Foreign-language anchor resolution in cast.

## What's built

Cast resolves foreign-language anchors (the `target:` + `anchor:` of
`cast::io::continues_in!`) through one of two backends:

- `crates/cast-spec/src/language_backend/foreign_grep.rs` —
  declaration-prefix grep + word-boundary check. The original
  implementation. Conservative by design; lossy by nature.
- `crates/cast-spec/src/language_backend/tree_sitter.rs` —
  tree-sitter-based AST query. Holds one `Grammar` per supported
  `Lang`; each grammar bundles a `tree_sitter::Language` with a
  precompiled `.scm` query naming the declaration nodes considered
  anchorable.

Dispatch lives in `validator/io_target.rs::anchor_found`:
tree-sitter first when `TreeSitterBackend::shared().supports(lang)`
returns true; grep fallback otherwise.

Today's coverage: **C, Kotlin, TypeScript, Swift, Bash/Shell, YAML**.
Each grammar registration lives in `tree_sitter.rs` as a
`<lang>_grammar()` function with its `.scm` query.

| `Lang` | Crate | Captures |
|---|---|---|
| `C` | `tree-sitter-c` | functions (regular + pointer-return), struct/union/enum, `#define` (with and without args) |
| `Kotlin` | `tree-sitter-kotlin-ng` | function_declaration, class_declaration, object_declaration |
| `TypeScript` | `tree-sitter-typescript` | function, class, interface, type_alias, enum, lexical_declaration (const/let) |
| `Swift` | `tree-sitter-swift` | function_declaration, class_declaration, protocol_declaration |
| `Shell` | `tree-sitter-bash` | function_definition |
| `Yaml` | `tree-sitter-yaml` | block_mapping_pair keys |

Languages without grammars (Vue, Sql, External, Rfc) continue to
use the grep fallback — Vue's SFC structure makes a single grammar
unsuitable; Sql has multiple competing grammar crates that need a
deliberate pick; External and Rfc are not anchor problems
(file-existence and regex-shape checks).

## The dispatcher contract

Two `cast::rule!` blocks pin the contract in source. Find them in
`crates/cast-spec/src/language_backend/tree_sitter.rs`:

1. **Grammar registration is the only edit-site for new languages.**
   No edits to `Lang`, `anchor_found`'s match arms, or
   `validate_io_annotation`. Adding a grammar without a `.scm`
   query is a no-op — the query is what names declarations as
   anchorable.

2. **Tree-sitter is the floor, never the ceiling.** When a
   grammar is registered for a `Lang`, dispatch goes through it
   deterministically; grep is the fallback for variants without a
   grammar. Two backends with overlapping responsibility would
   invite silent disagreement.

These rules are validated by cast itself — `governs:` anchors at
`TreeSitterBackend`, `Grammar`, and `validator::io_target`.

## Adding a new language

The recipe (Kotlin shown; same shape for any tree-sitter grammar):

1. `cargo add tree-sitter-kotlin` (gated behind the `analysis`
   feature in `crates/cast-spec/Cargo.toml`).
2. Write a `.scm` query string for Kotlin's anchorable declaration
   nodes — in tree-sitter-kotlin's grammar, that's
   `function_declaration`, `class_declaration`,
   `object_declaration`, `property_declaration`, etc. Each pattern
   should capture the name node as `@name`.
3. Add a `kotlin_grammar()` function alongside `c_grammar()` in
   `tree_sitter.rs`.
4. Push the grammar onto `TreeSitterBackend::new()`'s `grammars`
   vec.
5. Add a sub-concept in `crates/cast-spec/src/examples.rs`
   (`kotlin_examples`, anchored at `crate::examples`), drop one
   illustrative file under `examples/kotlin/`, and add a few
   `cast::io::continues_in!` blocks pointing at its anchorable
   items.

The umbrella `examples` concept reaches it automatically — its
`continues_in` edge through `crate::examples` lands on every
sibling concept anchored at the same module.

## The Rust question — why not tree-sitter all the way?

Tree-sitter for Rust *would* simplify cast architecturally — one
mechanism for all languages. It is **deliberately not the path
chosen**. The reasoning:

### Re-export resolution is load-bearing

The reason `crates/cast-spec/src/finder.rs` uses rust-analyzer's
HIR (rather than `syn` or tree-sitter) is recorded in its top
docstring. Cast users can write:

- `use cast::compare;` then call `compare!(...)`,
- `use cast::compare as cmp;` then call `cmp!(...)`,
- `use cast::*;` then call `concept!(...)` bare.

HIR resolves all three back to the *defining crate* (`cast`),
which is the ground truth for "is this one of ours." Tree-sitter
sees the literal call-site text only — `cmp!` would just be
`cmp!`, no link to `cast::compare`.

The same applies to anchor resolution. `crate::foo::Bar` walks
through whatever `pub use` chains exist in `foo`; HIR follows
them. Tree-sitter would stop at the textual path.

### Cross-workspace fallback would also break

`validate_annotation_multi` resolves anchors against multiple
`ProjectHandle`s (one per Cargo workspace). When a `.cast` file
declares `cast::foo::Bar` and the originating workspace doesn't
have `Bar` but a sibling workspace does, HIR finds it. Tree-sitter
parses each file in isolation and has no concept of "loaded
crates" — you'd have to reinvent the dep-graph walk on top of
CSTs.

### Symmetry tempts more than it earns

"Use tree-sitter for everything" looks clean — same `Grammar` +
`.scm` shape across languages. But Rust is the language we are
*most* exact about, intentionally. The other languages trade
accuracy for portability because we have no other option in
cast's environment. Forcing Rust down to that bar would erase the
original value prop.

### Where tree-sitter-rust *could* help

These are real wins available without compromising the HIR path:

1. **Cold-start triage.** cast-watch could use tree-sitter-rust
   as a cheap first pass to skip `.rs` files that don't even
   contain the textual substring `cast`, before loading the
   rust-analyzer workspace at all. RA load drops from "every
   `.rs`" to "every `.rs` that mentions cast." Pure latency win;
   no semantic loss because HIR still does anchor resolution on
   the surviving files.

2. **Lite/portable mode.** A hypothetical environment where
   Cargo or rust-analyzer isn't available — web playground,
   partial trees, language-agnostic SaaS. Trade accuracy for
   portability. Real, but a separate product surface.

3. **Second backend, opt-in.** `RustTreeSitterBackend` as a peer
   to `RustHirBackend`, selected by config (`tree_sitter_rust =
   true` in `Cast.cast`). Default stays HIR; opt in for speed at
   the cost of re-export blindness. Honest about the trade-off.

None of these are implemented. None should be without an explicit
use case driving them.

## Pointers into source

| What | Where |
|---|---|
| `TreeSitterBackend` struct + `Grammar` + `find_anchor` | `crates/cast-spec/src/language_backend/tree_sitter.rs` |
| Dispatcher (tree-sitter first, grep fallback) | `crates/cast-spec/src/validator/io_target.rs::anchor_found` |
| C grammar registration + `.scm` query | `tree_sitter.rs::c_grammar()` + `C_QUERY` |
| Foreign-grep fallback matchers | `crates/cast-spec/src/validator/io_target.rs::c_anchor_found` and `match_with_prefixes` |
| Rule: grammar registration is the only edit-site | `tree_sitter.rs` (`cast::rule!` block) |
| Rule: tree-sitter is the floor, never the ceiling | `tree_sitter.rs` (`cast::rule!` block) |
| Rust HIR backend + the re-export rationale | `crates/cast-spec/src/finder.rs` (top docstring) and `crates/cast-spec/src/validator/resolver.rs` |
| Examples corpus + walkable C demonstration | `examples/c/hello.c`, `crates/cast-spec/src/examples.rs` |
