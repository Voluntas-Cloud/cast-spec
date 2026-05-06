# cast-lsp

> **Highly experimental.** APIs and crate boundaries can change at any time. Pin exact versions if you depend on this. Feedback and breakage reports welcome at <https://github.com/Voluntas-Cloud/cast-spec/issues>.

Language Server Protocol implementation for [`cast`](https://crates.io/crates/cast-spec) annotations. Surfaces unresolved anchors, parse errors, and pipeline errors as inline diagnostics in any LSP-aware editor.

Peer to [`cast-extract`](https://crates.io/crates/cast-extract) and [`cast-watch`](https://crates.io/crates/cast-watch): all three depend on `cast` with `features = ["analysis"]` and share the analyzer library, so an LSP diagnostic, a CLI run, and a daemon snapshot can never disagree about what a `cast::*!` invocation means.

## What it does

`cast-extract` is right for CI; `cast-watch` is right for sub-second iteration loops driven by an LLM or script. `cast-lsp` is the third shape — analyzer output as **inline editor decoration**, surfaced through whatever LSP-aware editor you already use:

- On `initialize`, captures the workspace root from the editor (or `--root` if passed) and runs `load_projects` + `run_multi_root_analysis` on a blocking task.
- Maps each analyzer outcome to LSP `Diagnostic` objects, one per unresolved anchor — a five-anchor block where one path is broken becomes one diagnostic, not five.
- Publishes diagnostics via `textDocument/publishDiagnostics` once the first analysis completes.

## Install

```bash
cargo install cast-lsp
```

## Usage

cast-lsp speaks LSP over stdio. Configure your editor's LSP client to launch it for Rust files:

```bash
cast-lsp                    # use the editor's rootUri
cast-lsp --root path/to/ws  # pin a workspace root
```

Stdout is the LSP transport — every byte written there is framed JSON-RPC. All tracing and error reporting goes to stderr (set `RUST_LOG=debug` to see analyzer activity).

### Editor configuration sketch

Helix (`.helix/languages.toml`):

```toml
[language-server.cast-lsp]
command = "cast-lsp"

[[language]]
name = "rust"
language-servers = [{ name = "rust-analyzer" }, { name = "cast-lsp" }]
```

The same shape works for any LSP client — Neovim's `lspconfig`, VS Code via a thin extension, etc.

## Library use

The analysis library lives in [`cast-spec`](https://crates.io/crates/cast-spec) under the `analysis` feature, not in this crate. cast-lsp itself is a thin LSP wrapper over `cast::run_multi_root_analysis` + a translation layer in `diagnostics.rs`.

## Companion crates

- [`cast-spec`](https://crates.io/crates/cast-spec) — vocabulary (always) + analyzer (feature-gated). Published as `cast-spec`, imports as `cast`.
- [`cast-extract`](https://crates.io/crates/cast-extract) — peer one-shot CLI over the same analyzer.
- [`cast-watch`](https://crates.io/crates/cast-watch) — peer resident daemon with a JSON-lines query socket.
- [`cast-web`](https://crates.io/crates/cast-web) — small browser UI + LLM-mailbox shim that fronts a single cast-watch socket.

## License

MIT. See [`LICENSE`](LICENSE).
