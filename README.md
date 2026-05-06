# Cast

Compile-time architectural annotations for Rust, plus the static-analysis tooling that reads them.

Cast is a small vocabulary of macros that let you name the *concepts* in a codebase — the rules, anti-patterns, and pipelines that survive across refactors. The macros expand to nothing; the tools in this workspace are what give them meaning.

## Install

```sh
cargo add cast-spec
```

```rust
use cast::concept;
```

## Workspace

| Crate          | Registry name | What it is                                                                                                          |
|----------------|---------------|---------------------------------------------------------------------------------------------------------------------|
| `cast`           | `cast-spec`      | Vocabulary + analyzer. Macros (`concept!`, `rule!`, …) are always compiled; the static analyzer is opt-in via the `analysis` feature. |
| `cast-extract`   | `cast-extract`   | One-shot CLI binary over `cast`'s `analysis` feature. Walks Rust source, validates anchors with rust-analyzer, emits a graph. |
| `cast-watch`     | `cast-watch`     | Resident daemon binary over `cast`'s `analysis` feature. Watches the workspace, serves a JSON-lines query socket for LLMs and editor tooling. |
| `cast-lsp`       | `cast-lsp`       | Language Server Protocol server (stdio JSON-RPC) over `cast`'s `analysis` feature. Surfaces unresolved anchors, parse errors, and pipeline errors as inline LSP diagnostics. |
| `cast-web`       | `cast-web`       | Small browser UI + LLM-mailbox shim over a single cast-watch socket. Talks to `cast-watch` via JSON-lines only — does not import the analyzer. |
| `cast-stdlib`    | `cast-stdlib`    | Experimental. Curated library of reusable architectural concepts (`api`, `errors`, `lifecycle`, `security`, …) that downstream projects reference via `cast::continues_in!` instead of re-deriving inline. |
| `cast-os-stdlib` | `cast-os-stdlib` | Experimental. Same idea for OS-level concepts (`scheduling`, `kernel_user_boundary`, `ipc`, `virtualization`, …). |

> The macro crate publishes to crates.io as `cast-spec` (the short name `cast` is taken by an unrelated numeric-cast utility), but its library name is `cast`, so source code uses `use cast::concept;` regardless.

`cast-extract`, `cast-watch`, and `cast-lsp` are *peer* binaries over the analyzer: each depends on `cast` with `features = ["analysis"]`, none depend on each other. They share the analyzer library so they can never disagree about what an annotation means. `cast-web` is a peer at one remove — it depends on `cast` with vocabulary-only features and reaches the analyzer through `cast-watch`'s socket, which is the encapsulation seam.

## Why

Comments rot. Documentation drifts. The shared mental model that lets a team work on the same codebase usually lives in heads, then leaks into a wiki, then dies. Cast makes the mental model a first-class artifact in source: a `cast::concept!` block lives next to the code it describes, anchors at real Rust paths, and is checked on every CI run. When the anchor moves, the build tells you.

The companion tools turn that vocabulary into something a human or an LLM can consult interactively — what concepts does this function participate in? what rules govern this anchor? where are the unresolved annotations right now?

## What you'd reach for it for

A `cast::concept!` or `cast::rule!` is worth writing when the idea outlives any single function and the wrong refactor would silently break it. Examples:

- signed-request flow, capability boundary, bootstrap phase
- reconciliation loop, storage lifecycle, state-transition validation
- policy enforcement points, identity verification, kernel/user boundary

A `cast::pipeline!` is worth writing when the *order* matters and getting the steps wrong is a bug. Examples:

- request → verify → apply flows
- parser → validator → executor passes
- bootstrap sequences, signing/verification chains, import/export paths

A `cast::continues_in!` (or `cast::io::continues_in!`) is worth writing when a concept crosses a Rust crate, language, or file-format boundary — server Rust ↔ TypeScript client, Rust ↔ YAML deploy spec, Rust ↔ shell bootstrap, etc.

## What it looks like

```rust
use cast::concept;

pub fn verify_signature() { /* ... */ }

cast::concept! {
    name: "signed request verification",
    summary: "The API verifies a signed intent before applying durable state.",
    anchors: [
        crate::verify_signature,
    ],
    tags: ["security"],
}
```

The `concept!` block compiles to nothing. With the analyzer enabled (the `analysis` feature on `cast-spec`, or any of the workspace tools), Cast checks that `crate::verify_signature` still resolves and reports a diagnostic if it doesn't.

## Status

Published on crates.io as `cast-spec`. The grammar is still stabilising; breaking changes between minor versions should be expected until it settles.

## License

MIT. See `LICENSE`.
