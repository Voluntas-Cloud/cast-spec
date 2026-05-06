# Cast

Compile-time architectural annotations for Rust, plus the static-analysis tooling that reads them.

Cast is a small vocabulary of macros that let you name the *concepts* in a codebase — the rules, anti-patterns, and pipelines that survive across refactors. The macros expand to nothing; the tools in this workspace are what give them meaning.

## Workspace

| Crate          | Registry name | What it is                                                                                                          |
|----------------|---------------|---------------------------------------------------------------------------------------------------------------------|
| `cast`         | `cast-spec`   | Vocabulary + analyzer. Macros (`concept!`, `rule!`, …) are always compiled; the static analyzer is opt-in via the `analysis` feature. |
| `cast-extract` | `cast-extract`| One-shot CLI binary over `cast`'s `analysis` feature. Walks Rust source, validates anchors with rust-analyzer, emits a graph. |
| `cast-watch`   | `cast-watch`  | Resident daemon binary over `cast`'s `analysis` feature. Watches the workspace, serves a JSON-lines query socket for LLMs and editor tooling. |
| `cast-lsp`     | `cast-lsp`    | Language Server Protocol server (stdio JSON-RPC) over `cast`'s `analysis` feature. Surfaces unresolved anchors, parse errors, and pipeline errors as inline LSP diagnostics. **Standalone Cargo workspace.** |
| `cast-web`     | `cast-web`    | Small browser UI + LLM-mailbox shim over a single cast-watch socket. Talks to `cast-watch` via JSON-lines only — does not import the analyzer. **Standalone Cargo workspace.** |

> The macro crate publishes to crates.io as `cast-spec` (the short name `cast` is taken by an unrelated numeric-cast utility), but its library name is `cast`, so source code uses `use cast::concept;` regardless.

`cast-extract`, `cast-watch`, and `cast-lsp` are *peer* binaries over the analyzer: each depends on `cast` with `features = ["analysis"]`, none depend on each other. They share the analyzer library so they can never disagree about what an annotation means. `cast-web` is a peer at one remove — it depends on `cast` with vocabulary-only features and reaches the analyzer through `cast-watch`'s socket, which is the encapsulation seam.

## Why

Comments rot. Documentation drifts. The shared mental model that lets a team work on the same codebase usually lives in heads, then leaks into a wiki, then dies. Cast makes the mental model a first-class artifact in source: a `cast::concept!` block lives next to the code it describes, anchors at real Rust paths, and is checked on every CI run. When the anchor moves, the build tells you.

The companion tools turn that vocabulary into something a human or an LLM can consult interactively — what concepts does this function participate in? what rules govern this anchor? where are the unresolved annotations right now?

## Status

Pre-release. The grammar is stabilising; expect breaking changes before `0.1.0` lands on crates.io.

## License

MIT. See `LICENSE`.
