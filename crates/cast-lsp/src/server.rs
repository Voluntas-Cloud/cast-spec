//! LSP server implementation.
//!
//! Single backend struct holds the loaded `cast::Report` behind a
//! tokio mutex. `initialize` captures the workspace root; the heavy
//! `load_projects` + `run_multi_root_analysis` chain runs on the
//! blocking pool. Diagnostics publish once the analysis lands and
//! again on every `didSave` of a Rust file under the workspace.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result as RpcResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::diagnostics::report_to_diagnostics;

cast::concept! {
    name: "stdio_server_loop",
    summary: "tower-lsp owns the JSON-RPC framing; main hands stdin and \
              stdout to `Server::new` and the trait dispatch happens \
              inside that loop. cast-lsp's LanguageServer impl is the \
              entire request surface.",
    anchors: [
        crate::server::serve_stdio,
        crate::server::CastLspBackend,
    ],
    tags: ["cast_lsp"],
}

cast::concept! {
    name: "lazy_initial_analysis",
    summary: "After `initialize` returns, the server kicks the heavy \
              load on a `spawn_blocking` task. The editor sees a \
              responsive server immediately; the first batch of \
              diagnostics arrives whenever the load finishes. Subsequent \
              `didSave` requests trigger a re-analysis on the same \
              blocking-pool path.",
    anchors: [
        crate::server::initial_analysis,
        crate::server::reanalyze,
    ],
    tags: ["cast_lsp"],
}

cast::continues_in! {
    target:  cast_stdlib::performance::lazy_evaluation::LazyEvaluation,
    concept: "lazy_initial_analysis",
    why:     "`initialize` returns immediately; the heavy `load_projects`+\
              `run_multi_root_analysis` chain is deferred to a \
              spawn_blocking task. If the editor disconnects before \
              the task runs, the work is never done. Classic lazy \
              evaluation applied to expensive workspace bootstrap.",
    tags:    ["cast_lsp"],
}

cast::concept! {
    name: "lsp_edit_loop",
    summary: "On `did_save` of a Rust file under the workspace root, the \
              server `tokio::spawn`s `reanalyze` and returns immediately \
              so the LSP request loop stays responsive — `initialized` \
              works the same way for the initial load. Inside each \
              spawned task, the heavy RA load runs on `spawn_blocking` \
              so it doesn't tie up the async runtime's worker thread. \
              `CastLspBackend::analysis` is held for the whole duration \
              of both functions, so concurrent saves linearize: the \
              second save's analysis waits for the first to finish + \
              publish before starting. An in-flight analysis isn't \
              cancelled — slow under heavy churn but never inconsistent, \
              never out-of-order.",
    anchors: [
        crate::server::CastLspBackend,
        crate::server::reanalyze,
        crate::server::initial_analysis,
    ],
    tags: ["cast_lsp"],
}

cast::concept! {
    name: "lsp_diagnostic_lifecycle",
    summary: "Each publish round computes the next per-file diagnostic \
              set, publishes one batch per file via \
              textDocument/publishDiagnostics, then clears any file that \
              appeared in the previous round but not this one by \
              publishing an empty diagnostic list. The `last_published` \
              map on ServerState is the bookkeeping for that clear pass: \
              without it, a fixed anchor would leave stale squiggles in \
              the editor forever.",
    anchors: [
        crate::server::ServerState,
        crate::server::initial_analysis,
        crate::server::reanalyze,
    ],
    tags: ["cast_lsp"],
}

cast::continues_in! {
    target:  cast_stdlib::messaging::differential_publish::DifferentialPublish,
    concept: "lsp_diagnostic_lifecycle",
    why:     "Each publish round emits one batch per file with the \
              new diagnostics PLUS an explicit empty-list publish for \
              every file that appeared last round but not this one. \
              The `last_published` map is the diff bookkeeping; \
              without it, a fixed anchor would leave stale squiggles \
              in the editor forever.",
    tags:    ["cast_lsp"],
}

cast::rule! {
    rule: "Workspace root precedence is: `--root` CLI override, then \
           `workspace_folders[0]`, then `root_uri`, then `root_path`, \
           then the process cwd. The first source that resolves to a \
           filesystem path wins; later sources are not consulted.",
    why:  "Editors send some subset of these fields and LSP marks \
           `root_uri` / `root_path` deprecated, but enough clients still \
           use them that ignoring them silently breaks real users. \
           Pinning a precedence keeps cast-lsp's behavior predictable \
           across editors and explicit in code review.",
    governs: [
        crate::server::CastLspBackend,
    ],
    tags: ["cast_lsp"],
}

cast::rule! {
    rule: "Path-to-URL conversion is fallible — `Url::from_file_path` \
           rejects relative paths and some non-UTF-8 paths. On failure, \
           log a warning and skip the file rather than crashing the \
           publish loop or emitting a malformed URI.",
    why:  "A single un-convertible path must not block diagnostics for \
           every other file in the report. The LSP transport keeps \
           running; the editor sees diagnostics for every path that did \
           convert, and the warning lands on stderr for the operator.",
    governs: [
        crate::server::ServerState,
    ],
    tags: ["cast_lsp"],
}

/// Holds the loaded report and the workspace roots. Cloneable behind
/// Arc so async handlers can hand fragments to spawn_blocking without
/// locking the whole backend.
#[derive(Default)]
pub struct ServerState {
    pub roots: Vec<PathBuf>,
    pub repo_root: Option<PathBuf>,
    pub last_published: HashMap<Url, ()>,
}

pub struct CastLspBackend {
    pub client: Client,
    pub root_override: Option<PathBuf>,
    pub state: Arc<Mutex<ServerState>>,
    /// Held for the full duration of `initial_analysis` and `reanalyze`
    /// so that concurrent saves linearize: a second analysis waits for
    /// the in-flight one to finish + publish before starting. Without
    /// this, two saves could spawn two `run_analysis_blocking` tasks
    /// and the older one's publish could land after the newer one's,
    /// leaving the editor showing stale diagnostics.
    pub analysis: Arc<Mutex<()>>,
}

impl CastLspBackend {
    pub fn new(client: Client, root_override: Option<PathBuf>) -> Self {
        Self {
            client,
            root_override,
            state: Arc::new(Mutex::new(ServerState::default())),
            analysis: Arc::new(Mutex::new(())),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for CastLspBackend {
    async fn initialize(&self, params: InitializeParams) -> RpcResult<InitializeResult> {
        let root = self
            .root_override
            .clone()
            .or_else(|| derive_root_from_init(&params))
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        {
            let mut state = self.state.lock().await;
            state.roots = vec![root.clone()];
            state.repo_root = Some(cast::find_repo_root(&root).unwrap_or_else(|| root.clone()));
        }

        tracing::info!(root = %root.display(), "cast-lsp initialize");

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "cast-lsp".into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        tracing::info!("cast-lsp initialized handler entered");
        self.client
            .log_message(MessageType::INFO, "cast-lsp ready; loading workspace…")
            .await;
        tracing::info!("cast-lsp ready; spawning initial_analysis");
        // Detach the heavy initial load — tower-lsp processes requests
        // serially, so awaiting here would freeze the server until the
        // RA reload finishes (rejecting shutdown, didSave, everything).
        // The spawned task still serializes against `did_save` via
        // `analysis_lock`, so a save arriving during the initial load
        // queues correctly.
        tokio::spawn(initial_analysis(
            self.client.clone(),
            self.state.clone(),
            self.analysis.clone(),
        ));
    }

    async fn shutdown(&self) -> RpcResult<()> {
        Ok(())
    }

    async fn did_open(&self, _: DidOpenTextDocumentParams) {}
    async fn did_change(&self, _: DidChangeTextDocumentParams) {}

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let path = params.text_document.uri.to_file_path().ok();
        let in_workspace = match (&path, &self.state.lock().await.roots.first()) {
            (Some(p), Some(root)) => p.starts_with(root),
            _ => false,
        };
        if !in_workspace {
            return;
        }
        // Detach reanalysis for the same reason as `initialized` — keep
        // the LSP request loop responsive while RA reloads.
        tokio::spawn(reanalyze(
            self.client.clone(),
            self.state.clone(),
            self.analysis.clone(),
        ));
    }
}

/// Look at `workspace_folders` first (modern editors), then `root_uri`
/// (older), then `root_path`. Returns `None` if the editor sent only an
/// empty request, in which case the caller falls back to `cwd`.
fn derive_root_from_init(params: &InitializeParams) -> Option<PathBuf> {
    if let Some(folders) = params.workspace_folders.as_ref() {
        if let Some(first) = folders.first() {
            if let Ok(p) = first.uri.to_file_path() {
                return Some(p);
            }
        }
    }
    #[allow(deprecated)]
    if let Some(uri) = params.root_uri.as_ref() {
        if let Ok(p) = uri.to_file_path() {
            return Some(p);
        }
    }
    #[allow(deprecated)]
    if let Some(p) = params.root_path.as_ref() {
        return Some(PathBuf::from(p));
    }
    None
}

// Both `initial_analysis` and `reanalyze` follow this exact flow.
// load_projects + run_analysis run on `spawn_blocking`; the rest run
// on the async runtime. acquire_lock is the `analysis_lock.lock()`
// at the top of each fn — the serialization point that makes
// concurrent saves linearize instead of racing.
cast::pipeline! {
    stages: {
        acquire_lock     @ crate::server::CastLspBackend,
        load_projects    @ cast::load_projects,
        run_analysis     @ cast::run_multi_root_analysis,
        map_diagnostics  @ crate::diagnostics::report_to_diagnostics,
        publish_per_file @ crate::server::publish_all,
        clear_stale      @ crate::server::publish_all,
    },
    flow: [
        acquire_lock     -> load_projects,
        load_projects    -> run_analysis,
        run_analysis     -> map_diagnostics,
        map_diagnostics  -> publish_per_file,
        publish_per_file -> clear_stale,
    ],
    cyclic: false,
    entry: acquire_lock,
    tags: ["cast_lsp"],
    note: "publish_per_file and clear_stale share an anchor: \
           publish_all does both — it issues one publishDiagnostics \
           per file in the new report, then publishes empty diagnostic \
           lists for every file that appeared in the previous round \
           but not this one (the `last_published` bookkeeping).",
}

// Two strategies for keeping editor diagnostics fresh under churn.
// cast-lsp picks `monolithic` because the LSP request rate is human-
// scale (one save per ten seconds at peak); cast-watch picks `tiered`
// because its broadcast rate is machine-scale (every keystroke if
// the editor saves continuously). The reanalyze docstring below
// names this gap as a planned port — the `compare!` here pins it as
// a design decision rather than a TODO comment.
cast::compare! {
    monolithic @ crate::server::reanalyze:
        "Full RA reload on every did_save. Simple and correct, but \
         scales poorly: a save under a Voluntas-sized workspace ties \
         up the blocking pool for seconds. Acceptable for editor use \
         where saves are rare and explicit.",
    tiered @ cast_watch::classifier::classify:
        "Classifies an edit as macro-only or implementation; \
         macro-only commits a `syn`-based snapshot in milliseconds, \
         implementation marks anchors stale and defers the heavy RA \
         reload (eager mode) or skips it entirely (lazy mode, the \
         default). Keeps cast-watch's JSON-line socket responsive \
         under high churn.",
    tags: ["cast_lsp"],
    note: "Cross-workspace anchor: the `tiered` arm reaches into the \
           cast-watch crate, resolved via the validator's \
           cross-workspace anchor fallback.",
}

/// Run the initial multi-root analysis on the blocking pool, then
/// publish diagnostics for every cast-bearing file in the workspace.
///
/// Holds `analysis_lock` for the entire run so that a save arriving
/// during the initial load queues behind it instead of racing.
pub async fn initial_analysis(
    client: Client,
    state: Arc<Mutex<ServerState>>,
    analysis_lock: Arc<Mutex<()>>,
) {
    let _guard = analysis_lock.lock().await;
    let (roots, repo_root) = {
        let s = state.lock().await;
        (s.roots.clone(), s.repo_root.clone())
    };
    if roots.is_empty() {
        return;
    }
    let repo_root = repo_root.unwrap_or_else(|| roots[0].clone());

    let report = match run_analysis_blocking(roots.clone(), repo_root.clone()).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!(error = %e, "initial analysis failed");
            client
                .log_message(
                    MessageType::ERROR,
                    format!("cast-lsp initial analysis failed: {e}"),
                )
                .await;
            return;
        }
    };

    publish_all(&client, &report, state).await;
    client
        .log_message(MessageType::INFO, "cast-lsp initial analysis complete")
        .await;
}

/// Re-run the analysis after a save. Same path as the initial load —
/// the analyzer doesn't currently expose a per-file incremental mode,
/// so a save means a full reload. Cast-watch's tiered incremental
/// design is a future port for cast-lsp.
///
/// Holds `analysis_lock` for the entire run so that concurrent saves
/// linearize: the second save waits for the first to finish + publish
/// before its own analysis starts. Without this, the older analysis's
/// publish could land after the newer one's, leaving stale diagnostics
/// in the editor.
pub async fn reanalyze(
    client: Client,
    state: Arc<Mutex<ServerState>>,
    analysis_lock: Arc<Mutex<()>>,
) {
    let _guard = analysis_lock.lock().await;
    let (roots, repo_root) = {
        let s = state.lock().await;
        (s.roots.clone(), s.repo_root.clone())
    };
    if roots.is_empty() {
        return;
    }
    let repo_root = repo_root.unwrap_or_else(|| roots[0].clone());

    let report = match run_analysis_blocking(roots, repo_root).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!(error = %e, "reanalysis failed");
            return;
        }
    };
    publish_all(&client, &report, state).await;
}

async fn run_analysis_blocking(
    roots: Vec<PathBuf>,
    repo_root: PathBuf,
) -> anyhow::Result<cast::Report> {
    tokio::task::spawn_blocking(move || -> anyhow::Result<cast::Report> {
        let multi = cast::load_projects(&roots)?;
        let report = cast::run_multi_root_analysis(&multi, &repo_root);
        Ok(report)
    })
    .await?
}

/// Publish diagnostics per file; clear any previously-published file
/// whose diagnostics this report no longer mentions (so a fixed anchor
/// goes back to a clean editor state).
async fn publish_all(client: &Client, report: &cast::Report, state: Arc<Mutex<ServerState>>) {
    let by_file = report_to_diagnostics(report);
    let mut next_published: HashMap<Url, ()> = HashMap::new();

    for (path, diagnostics) in &by_file {
        let url = match Url::from_file_path(path) {
            Ok(u) => u,
            Err(_) => {
                tracing::warn!(path = %path.display(), "cannot convert path to file URL");
                continue;
            }
        };
        client
            .publish_diagnostics(url.clone(), diagnostics.clone(), None)
            .await;
        next_published.insert(url, ());
    }

    let prior_published = {
        let mut s = state.lock().await;
        std::mem::replace(&mut s.last_published, next_published.clone())
    };
    for url in prior_published.keys() {
        if !next_published.contains_key(url) {
            client
                .publish_diagnostics(url.clone(), Vec::new(), None)
                .await;
        }
    }
}

/// Bind the LSP server to stdio and run until the editor disconnects.
pub async fn serve_stdio(root_override: Option<PathBuf>) -> anyhow::Result<()> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) =
        LspService::new(|client| CastLspBackend::new(client, root_override));
    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}

cast::concept! {
    name: "lsp_backend_handle",
    summary: "Server state holding the LSP client, the workspace root, \
              the loaded analysis cache. Mutable shared state behind a \
              tokio Mutex.",
    anchors: [
        crate::server::ServerState,
        crate::server::CastLspBackend,
    ],
    tags: ["cast_lsp_server"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::resource_handle,
    concept: "lsp_backend_handle",
    why: "Holds an LSP Client + tokio Mutex over mutable analysis \
          cache; identity is the live LSP connection.",
}

cast::concept! {
    name: "lsp_root_resolution",
    summary: "Pure helper that derives the workspace root from \
              InitializeParams.rootUri or workspaceFolders.",
    anchors: [
        crate::server::derive_root_from_init,
    ],
    tags: ["cast_lsp_server"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "lsp_root_resolution",
    why: "Output is a function of InitializeParams alone.",
}

cast::continues_in! {
    target: cast_stdlib::messaging::request_response,
    concept: "lsp_backend_handle",
    why: lazy,
}
