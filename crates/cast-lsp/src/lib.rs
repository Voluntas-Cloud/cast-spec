//! cast-lsp — LSP server surfacing cast::*! analysis as editor diagnostics.
//!
//! Peer to cast-extract (one-shot CLI) and cast-watch (resident daemon
//! with a JSON-lines query socket). All three depend on the cast crate
//! with `features = ["analysis"]` and share the analyzer library, so
//! an LSP diagnostic, a CLI run, and a daemon snapshot can never
//! disagree about what a cast::*! invocation means.
//!
//! See `crates/cast-spec/GRAMMAR.md` for the macro vocabulary; the cast::*!
//! blocks below *are* the design — the modules and functions they
//! anchor at are the implementation that lands under the spec.

pub mod diagnostics;
pub mod server;

// ─── Concepts ────────────────────────────────────────────────────────────

cast::concept! {
    name: "cast_lsp",
    summary: "Per-crate umbrella for the cast-lsp server. Pulls every \
              cast-lsp internal concept underneath this node by \
              longest-prefix-match on the two submodule anchors below — \
              `cast_lsp_server` is the spec for what the server is, but \
              the deeper concepts (`lsp_workspace_load`, \
              `lsp_diagnostic_lifecycle`, `lsp_edit_loop`, \
              `report_to_diagnostics`) anchor at items inside \
              `diagnostics` and `server`, so they nest cleanly under \
              this umbrella in the canonical concept tree.",
    anchors: [
        crate::diagnostics,
        crate::server,
    ],
    tags: ["cast_lsp"],
}

cast::concept! {
    name: "cast_lsp_server",
    summary: "LSP server that surfaces cast::*! analysis as diagnostics \
              in any LSP-aware editor. Loads the workspace once on \
              `initialize`, runs the same multi-root analysis pipeline \
              cast-extract uses, maps each invocation's outcome to LSP \
              Diagnostic objects, and publishes them via \
              textDocument/publishDiagnostics. The interactive editor \
              counterpart to cast-extract's CLI report.",
    anchors: [
        crate::run,
        crate::server::CastLspBackend,
        crate::diagnostics::report_to_diagnostics,
    ],
    tags: ["cast_lsp"],
}

cast::concept! {
    name: "lsp_workspace_load",
    summary: "On `initialize`, the server captures the workspace root \
              from the editor and spawns the heavy `load_projects` + \
              `run_multi_root_analysis` chain on the blocking pool. \
              Until the load finishes, the server reports an empty \
              snapshot — the same shape cast-watch uses for its \
              Phase::Loading state. Diagnostics publish as soon as the \
              first analysis completes.",
    anchors: [
        crate::server::CastLspBackend,
        crate::server::initial_analysis,
    ],
    tags: ["cast_lsp"],
}

// ─── Rules ───────────────────────────────────────────────────────────────

cast::rule! {
    rule: "cast-lsp consumes the cast crate as a library, never spawns \
           cast-extract or cast-watch as a subprocess. The per-handle \
           analysis pipeline runs in-process via \
           `cast::run_multi_root_analysis`.",
    why:  "Three peer binaries over one shared analyzer is the explicit \
           design: a diagnostic the editor surfaces must match what CI \
           computes and what cast-watch broadcasts, byte-for-byte. \
           Subprocessing reintroduces the split-SSOT failure mode the \
           library design exists to prevent.",
    governs: [
        crate::server::initial_analysis,
    ],
    tags: ["cast_lsp"],
}

cast::rule! {
    rule: "Stdout is the LSP transport — every byte written there is \
           framed JSON-RPC. All tracing, error reporting, and ad-hoc \
           debug output goes to stderr.",
    why:  "An LSP server that prints anything outside the framed \
           protocol corrupts its own transport: the editor sees \
           garbage, the connection drops, the user sees nothing. \
           Stdout is sacred — the only writer is the JSON-RPC encoder.",
    governs: [
        crate::run,
    ],
    tags: ["cast_lsp"],
}

cast::rule! {
    rule: "Diagnostics carry the source location reported by the \
           analyzer's `Location` (file path + 1-based line), translated \
           into LSP zero-based ranges. When the analyzer produces a \
           per-anchor outcome, each unresolved anchor becomes its own \
           Diagnostic so the editor can highlight individual paths \
           rather than the whole macro block.",
    why:  "An invocation with five anchors where one is unresolved is a \
           one-anchor problem, not a five-anchor problem. Collapsing \
           the per-anchor detail into a block-level diagnostic costs \
           the user the information they need to fix it — which path \
           is the broken one.",
    governs: [
        crate::diagnostics::report_to_diagnostics,
    ],
    tags: ["cast_lsp"],
}

// ─── Anti-patterns ───────────────────────────────────────────────────────

cast::anti_pattern! {
    avoid:      "Re-running `load_projects` + `run_multi_root_analysis` \
                 inline on the LSP request thread.",
    why:        "The RA project load is seconds-to-minutes on Voluntas-\
                 sized trees. Running it on the request thread freezes \
                 the editor for the whole duration: keystrokes pile up, \
                 the LSP transport blocks, and the user assumes the \
                 server has hung.",
    instead:    "Run the heavy analysis on `tokio::task::spawn_blocking`. \
                 The request handler returns quickly; the publish step \
                 fires when the blocking task completes.",
    instead_at: crate::server::initial_analysis,
    governs: [
        crate::server::CastLspBackend,
    ],
    tags: ["cast_lsp"],
}

cast::anti_pattern! {
    avoid:      "Writing log lines or print! output to stdout from \
                 anywhere in the cast-lsp process.",
    why:        "Stdout is the LSP transport. Any non-JSON-RPC byte on \
                 stdout corrupts the framing and the editor disconnects.",
    instead:    "Initialize tracing with `with_writer(std::io::stderr)` \
                 in main; never call `println!`/`print!` from library \
                 code.",
    instead_at: crate::run,
    governs: [
        crate::run,
    ],
    tags: ["cast_lsp"],
}

// ─── Surface ─────────────────────────────────────────────────────────────

/// Run the LSP server on stdio.
///
/// `root_override` lets the user pin a workspace root from the command
/// line; otherwise the server uses the `rootUri` from `initialize`.
pub fn run(root_override: Option<std::path::PathBuf>) -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(server::serve_stdio(root_override))
}

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "cast_lsp_server",
    why: "stdio I/O, async scheduling, and rust-analyzer reload all \
          read observable external state.",
}

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "lsp_workspace_load",
    why: "Filesystem reads, cargo subprocess, async task scheduling.",
}

cast::continues_in! {
    target: cast_stdlib::messaging::request_response,
    concept: "cast_lsp_server",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::lifecycle::reconciliation_loop,
    concept: "lsp_workspace_load",
    why: lazy,
}
