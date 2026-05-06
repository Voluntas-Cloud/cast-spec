# cast

Compile-time architectural annotations for Rust.

> **Highly experimental.** APIs, macro syntax, and crate boundaries can change at any time. Pin exact versions if you depend on this. Feedback and breakage reports welcome at <https://github.com/Voluntas-Cloud/cast-spec/issues>.

> Published on crates.io as **`cast-spec`** — the short name `cast` is held by an unrelated numeric-cast utility. The library name is still `cast`, so source code uses `use cast::concept;` unchanged.

`cast` is a vocabulary of `macro_rules!` macros that expand to nothing. Their job is to let you write down the *concepts* of your codebase — the rules, anti-patterns, pipelines, and decisions — in source, anchored at real Rust paths. The companion crate [`cast-extract`](https://github.com/Voluntas-Cloud/cast-spec/tree/main/crates/cast-extract) reads the source, validates the anchors with rust-analyzer, and emits a concept graph.

## Install

```toml
[dependencies]
cast = { package = "cast-spec", version = "0.1" }
```

The `package = "cast-spec"` alias keeps `use cast::*;` working in source while pulling the published crate from the registry.

## Quick example

```rust
use cast::{concept, rule, anti_pattern};

cast::concept! {
    name: "request_pipeline",
    summary: "HTTP request goes through auth, rate-limit, then handler. \
              Each stage's failure mode is documented at its anchor.",
    anchors: [
        crate::middleware::authenticate,
        crate::middleware::rate_limit,
        crate::handlers::dispatch,
    ],
    tags: ["http"],
}

cast::rule! {
    rule: "Handlers never call the database directly — repositories own \
           every query so we can swap the backend without touching HTTP code.",
    why:  "The database used to be Postgres-shaped; it now isn't. \
           Concentrating queries in repositories paid off when we migrated.",
    governs: [
        crate::handlers::dispatch,
    ],
    tags: ["http"],
}
```

The macros are no-ops at compile time: `cargo build` is unaffected. To check that the anchors still resolve, run `cast-extract` against the crate.

## Macros

| Macro                  | What it names                                                              |
|------------------------|----------------------------------------------------------------------------|
| `concept!`             | A named idea that anchors at one or more Rust paths.                       |
| `rule!`                | A "do this" with a `why:` and a `governs:` list.                           |
| `anti_pattern!`        | A "don't do this" with `instead:` and `instead_at:` pointing at the right way. |
| `pipeline!`            | A staged flow whose stages are anchored functions.                         |
| `tier!`                | An ordered set of stages along a named axis (cost, latency, trust...).     |
| `compare!`, `matrix!`, `gut_check!`         | Smaller-grained markers — see `GRAMMAR.md`. |
| `continues_in!`, `cast::io::continues_in!` | Cross-file / cross-language references. The `io::` form points at non-Rust targets (TypeScript, Kotlin, YAML, RFCs, …). |

The full grammar lives in [`GRAMMAR.md`](GRAMMAR.md).

## Companion tooling

- [`cast-extract`](https://crates.io/crates/cast-extract) — one-shot CLI: walks source, validates, emits a report.
- [`cast-watch`](https://crates.io/crates/cast-watch) — resident daemon: live concept graph + JSON-lines query socket for LLMs and editors.
- [`cast-lsp`](https://crates.io/crates/cast-lsp) — Language Server Protocol server: exposes unresolved anchors and pipeline errors as inline LSP diagnostics in any LSP-aware editor.
- [`cast-web`](https://crates.io/crates/cast-web) — small browser UI + LLM-mailbox shim over a single cast-watch socket. The prescribed front door for both humans and LLMs interacting with a live cast graph.

## License

MIT. See [`LICENSE`](LICENSE).
