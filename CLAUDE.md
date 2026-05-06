# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Workspace shape

Five crates, all in `crates/`:

- **`cast`** — published as `cast-spec` (the short name `cast` is taken). Library name stays `cast`, so source uses `use cast::concept;` regardless. Ships two surfaces: the macro vocabulary (always compiled, expands to nothing) and the static analyzer behind `features = ["analysis"]`.
- **`cast-extract`** — one-shot CLI binary. Thin wrapper over `cast`'s analysis feature: `load_projects` + `run_multi_root_analysis` + render.
- **`cast-watch`** — resident daemon binary with a Unix-socket JSON-lines query interface. Library + binary; `cast_watch::start` is the entry point.
- **`cast-lsp`** — Language Server Protocol server (stdio JSON-RPC). Translates a `cast::Report` into LSP diagnostics so any LSP-aware editor sees unresolved anchors, parse errors, and pipeline errors inline.
- **`cast-web`** — HTTP/WebSocket shim over a single cast-watch socket, plus a filesystem-mailbox surface for LLM collaboration. Default `:8765`; `POST /watcher/query` relays one JSON-line request to the watcher and returns the response, with each turn tee'd to a watcher-log directory. Vocabulary-only dep on `cast` — does NOT import the analyzer; reaches it through cast-watch's socket.

`cast-extract`, `cast-watch`, and `cast-lsp` are **peer binaries** over the analyzer: each depends on `cast` with `features = ["analysis"]`, none depend on each other. `cast-web` is a peer at one remove — it talks to cast-watch over a Unix socket and never imports the analyzer features. This is load-bearing — see `cast::anti_pattern! shell_out_to_extract` in `crates/cast-watch/src/lib.rs`, the equivalent rules in `crates/cast-lsp/src/lib.rs`, and the `cast-web speaks to cast-watch through a unix-socket JSON-line client only` rule in `crates/cast-web/src/lib.rs`.

## Build / test / run

```bash
cargo build                                  # vocabulary-only build of cast, plus the analyzer-side binaries
cargo build -p cast-spec --features analysis # exercise the analyzer surface alone
cargo test                                   # workspace tests (smoke test in crates/cast-spec/tests/smoke.rs)
cargo test -p cast-spec all_macros_parse     # tokeniser smoke test only
cargo run -p cast-extract -- path/to/workspace
cargo run -p cast-watch    -- path/to/workspace          # lazy mode
cargo run -p cast-watch    -- --eager path/to/workspace  # immediate RA reload on impl edits
cargo run -p cast-watch    -- --no-walk-up path/to/sub-crate   # error if root is a workspace member
```

The macro vocabulary compiles with zero deps; only `analysis` pulls in the rust-analyzer crates and `syn`/`serde`/etc. Downstream consumers that just want `use cast::*;` should never see those.

## Architecture

### Two-layer analysis

`cast` macros are no-ops at compile time. The analyzer turns them into a checked concept graph:

1. **Macro layer (cheap, `syn`-based)** — `crates/cast-spec/src/parser/` has one file per macro grammar. Each implements `syn::parse::Parse` and produces a typed value unified by `parser::CastAnnotation`. Dispatch in `parser/mod.rs::parse_macro_body` keys on the macro's *defining* path (so `cast::io::continues_in!` arrives as `cast::continues_in_io`).
2. **Full layer (heavy, RA-backed)** — `crates/cast-spec/src/finder.rs` walks the rust-analyzer DB to discover invocations; `validator/` resolves anchor paths; `graph.rs` builds the concept graph; `emit/` renders text/JSON/markdown.

`run_multi_root_analysis` in `crates/cast-spec/src/lib.rs` is the canonical pipeline; cast-extract, cast-watch, and cast-lsp all call it. Cross-workspace anchor resolution happens via `MultiProject` (`crates/cast-spec/src/project.rs`).

### cast-watch tiered analysis

`crates/cast-watch/src/lib.rs` is itself the design doc: the file's `cast::concept!`, `cast::rule!`, `cast::anti_pattern!`, and `cast::pipeline!` blocks are the spec for the daemon, and they anchor at the modules that implement them. **Read these blocks before editing the daemon** — they are load-bearing rules, not commentary.

Key invariants encoded there:

- `classifier::classify` decides whether an event is **macro-only** or **implementation**. Macro-only edits commit a new snapshot via `syn` in milliseconds; implementation edits mark anchors stale by default and only trigger a full RA reload under `--eager` or on a `rebuild` query. Never trigger an RA reload from a macro-only edit.
- State updates use snapshot-and-swap (`Arc<Snapshot>`, atomic pointer swap in `state::commit`). Reads on the query socket must never observe a partially-rebuilt graph and never block on in-flight reanalysis.
- The query interface (`socket.rs`) speaks JSON-lines: one request per line, one response per line. Subscribers receive broadcasts on snapshot change, each carrying a monotonic `snapshot_generation` for deduping.

### Single-crate analysis lock (`--no-walk-up`)

When you point cast-extract or cast-watch at a sub-crate of a larger workspace, rust-analyzer's loader silently walks up to the enclosing workspace root and loads everything in `members`. That turns `cast-watch path/to/sub-crate` into `cast-watch path/to/whole-workspace` with no visible signal.

`--no-walk-up` is the opt-in gate. It runs `cargo locate-project` twice (with and without `--workspace`); if the package manifest and the workspace manifest disagree, the CLI refuses with an error naming the parent workspace. Use it when you genuinely want analysis isolated to one crate (e.g. iterating on a single member without paying for the rest of the workspace).

Encoded as `cast::concept! block_walk_up` + a `cast::rule!` in `crates/cast-spec/src/project.rs`, enforced by `cast::assert_standalone_root`.

### RA pinning

`ra_ap_*` crates are pinned to `=0.0.330` in `crates/cast-spec/Cargo.toml`. They do not follow SemVer; bumping requires explicit review for API drift.

## Conventions

### Fixture vocabulary

Examples in docstrings, integration tests, and `GRAMMAR.md` use **only** the fictional namespaces `sample::*`, `sample_agent::*`, and `samples/external/*`. Never introduce real crate names from sister projects. The rule and its rationale are pinned in `crates/cast-spec/src/parser/mod.rs` as a `cast::rule!` block — when adding a new example, copy an existing one's namespace. Note: `docs/ARCHITECTURE_PROSE_STYLE.md` is the conceptual ancestor of cast and is exempt — its voluntas-flavored example is part of its provenance, not drift.

### Dogfooding

`cast::*!` blocks in `crates/cast-spec/src/**/*.rs`, `crates/cast-watch/src/lib.rs`, `crates/cast-lsp/src/lib.rs`, and `crates/cast-web/src/lib.rs` are real annotations the analyzer validates against itself. `extern crate self as cast;` in `crates/cast-spec/src/lib.rs` is what makes `cast::concept!` resolve from inside the cast-spec crate. When you change a function name or move a module, the `cast::*!` anchors that pointed at it will fail validation — fix the anchors in the same change.

### Grammar reference

`crates/cast-spec/GRAMMAR.md` is the in-repo grammar spec, kept aligned with the parser. The **live** source of truth is `cast-watch`'s `{"kind":"manual"}` query, which returns macros, fields, languages, warning kinds, and recommended workflows from the running daemon — query it (via `crates/cast-web/dev/cwq.sh` against a running cast-web, or directly over the watcher socket) when you need the current shape of a macro and don't trust the on-disk doc.
