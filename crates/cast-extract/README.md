# cast-extract

Static-analysis tool for [`cast`](../cast-spec) annotations. Walks Rust source, validates every anchor against rust-analyzer, and emits a concept graph as text, JSON, or Markdown.

## What it does

Cast macros are compile-time no-ops; their value is in being *checked*. `cast-extract` is the checker:

1. Loads the workspace (or several workspaces) into rust-analyzer.
2. Finds every `cast::*!` invocation in source.
3. Parses each macro body against the cast grammar.
4. Resolves every Rust-path anchor (`crate::module::function`, `sample::pki::sign_csr`, …) against the right ra-analyzer database.
5. Builds the concept graph and emits a report.

A clean run with zero unresolved anchors is the signal that your annotations and your code agree. The unresolved list is your TODO list.

## Install

```bash
cargo install cast-extract
```

## Usage

```bash
# Analyse a single workspace
cast-extract path/to/crate

# Multi-workspace analysis (concepts can span workspaces)
cast-extract path/to/workspace-a path/to/workspace-b

# JSON output for CI
cast-extract --format json path/to/crate
```

## Output

```
loading path/to/crate ...
found 34 cast::*! invocations
  ok    cast::concept     at src/lib.rs:27   (5/5 paths resolved)
  ok    cast::rule        at src/lib.rs:81   (2/2 paths resolved)
  ...

summary: 34 parsed, 0 parse errors, 0 unimplemented
paths:   82 resolved, 0 unresolved
io:      0 ok, 0 unresolved
pipeline:  0 structural error(s)

concept graph: 15 concepts, 15 declarations, 0 edges
```

Anything other than `0 unresolved` is the file:line list of broken anchors.

## Library use

The analysis library lives in [`cast-spec`](../cast-spec) under the `analysis` feature, not in this crate. To call it from your own tooling:

```toml
[dependencies]
cast = { package = "cast-spec", version = "0.1", features = ["analysis"] }
```

```rust
use cast::{load_projects, run_multi_root_analysis, find_repo_root};

let multi = load_projects(&[project_root])?;
let repo_root = find_repo_root(&project_root).unwrap_or(project_root);
let report = run_multi_root_analysis(&multi, &repo_root);
```

The `Report` is `serde::Serialize`, so you can stream it into your own tooling. `cast-extract` is a thin CLI wrapper over exactly these calls.

## Companion crates

- [`cast-spec`](../cast-spec) — vocabulary (always) + analyzer (feature-gated). Published as `cast-spec` on crates.io, imports as `cast`.
- [`cast-watch`](../cast-watch) — peer daemon. Same `cast` feature, file-watching mode + JSON-lines query socket.
- [`cast-lsp`](../cast-lsp) — peer LSP server. Same `cast` feature, surfaces analyzer output as inline editor diagnostics.
- [`cast-web`](../cast-web) — browser UI + LLM-mailbox shim over a `cast-watch` socket. Vocabulary-only dep on `cast`; reaches the analyzer through the watcher.

## License

MIT. See the workspace `LICENSE`.
