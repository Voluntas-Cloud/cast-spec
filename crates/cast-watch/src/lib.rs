//! cast-watch — live, file-watching counterpart to cast-extract.
//!
//! Where cast-extract is one-shot ("extract once, exit"), cast-watch
//! is resident: it watches the workspace, maintains the concept graph
//! incrementally, and exposes a query interface for LLMs and tools to
//! consult while code is being written.
//!
//! Two layers of analysis. The macro layer (cheap, syntactic) handles
//! edits confined to bytes inside `cast::*!` invocations — those don't
//! need rust-analyzer at all, and a full Rust rebuild would be wildly
//! disproportionate to the work. The full layer (heavy, RA-backed) is
//! reserved for changes that touch implementation code referenced by
//! anchors, and only when explicitly requested or under `--eager`.
//!
//! See `crates/cast-spec/GRAMMAR.md` for the macro vocabulary used below;
//! the cast::*! blocks in this file *are* the design — the modules
//! and types they anchor at are the implementation that lands under
//! the spec.

pub mod classifier;
pub mod socket;
pub mod state;
pub mod watcher;

// ─── Concepts ────────────────────────────────────────────────────────────

cast::concept! {
    name: "cast_watch",
    summary: "Per-crate umbrella for the cast-watch daemon. Pulls every \
              cast-watch internal concept underneath this node by \
              longest-prefix-match on the four submodule anchors below \
              — `cast_watch_daemon` is the spec for what the daemon \
              is, but the deeper concepts (`tiered_analysis`, \
              `arc_swap_commit`, `query_language`, `notify_subscription`, \
              etc.) anchor at items inside `classifier`, `socket`, \
              `state`, and `watcher`, so they nest cleanly under this \
              umbrella in the canonical concept tree.",
    anchors: [
        crate::classifier,
        crate::socket,
        crate::state,
        crate::watcher,
        CAST::AS_PRIMITIVE::crate::start,
        CAST::AS_PRIMITIVE::crate::socket::QueryServer,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  crate::start,
    concept: "cast_watch",
    why:     "cast_watch continues into the daemon implementation at start()",
}

cast::continues_in! {
    target:  crate::socket::QueryServer,
    concept: "cast_watch",
    why:     "cast_watch continues into the query interface at QueryServer",
}

cast::concept! {
    name: "cast_watch_daemon",
    summary: "Resident cast analyzer: watches the workspace, maintains an \
              incremental concept graph, exposes a query socket so LLMs \
              and other tools can consult the live model while code is \
              being written. The interactive counterpart to cast-extract's \
              one-shot CLI.",
    anchors: [
        crate::start,
        crate::watcher::run,
        CAST::AS_PRIMITIVE::crate::watcher::FileWatcher,
        CAST::AS_PRIMITIVE::crate::state::LiveState,
        CAST::AS_PRIMITIVE::crate::socket::QueryServer,
        CAST::AS_PRIMITIVE::crate::classifier::classify,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  crate::classifier::classify,
    concept: "cast_watch_daemon",
    why:     "cast_watch_daemon continues into tiered analysis at classify()",
}

cast::continues_in! {
    target:  crate::state::LiveState,
    concept: "cast_watch_daemon",
    why:     "cast_watch_daemon continues into the live state layer at LiveState",
}

cast::continues_in! {
    target:  cast_stdlib::lifecycle::incremental_rebuild::IncrementalRebuild,
    concept: "cast_watch_daemon",
    why:     "Watching the workspace and maintaining an incremental \
              concept graph IS incremental rebuild: filesystem events \
              are the changed-input signal, the classifier decides what \
              part of the graph the event invalidates, and only that \
              part is recomputed. Whole-corpus rebuild is the fallback \
              behind the `rebuild` query, not the steady state.",
    tags:    ["cast_watch"],
}

cast::concept! {
    name: "tiered_analysis",
    summary: "Two layers of analysis with very different costs. The \
              macro layer reparses cast::*! bodies in-process when only \
              annotation bytes change — milliseconds, no rust-analyzer \
              involvement. The full layer reloads the rust-analyzer DB \
              and re-runs the per-handle pipeline — seconds-to-minutes \
              on Voluntas-sized trees. The classifier decides which \
              layer an event belongs to; the daemon runs the cheap path \
              by default and only escalates on demand.",
    anchors: [
        crate::classifier::classify,
        crate::watcher::handle_macro_only,
        crate::watcher::handle_implementation,
        CAST::AS_PRIMITIVE::crate::classifier::ChangeKind,
        CAST::AS_PRIMITIVE::crate::watcher::run,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  crate::watcher::run,
    concept: "tiered_analysis",
    why:     "tiered_analysis continues into tier dispatch at watcher::run",
}

cast::continues_in! {
    target:  cast_stdlib::performance::lazy_evaluation::LazyEvaluation,
    concept: "tiered_analysis",
    why:     "The implementation tier is lazily evaluated: in the \
              default lazy mode the heavy RA reload only fires on a \
              `rebuild` query or under `--eager`. Until the user \
              actually asks for a fresh resolution, the cost is never \
              paid.",
    tags:    ["cast_watch"],
}

cast::tier! {
    axis: handler_cost,
    direction: increasing,
    stages: {
        macro_handler @ crate::watcher::handle_macro_only:
            "Cheap branch: syn reparse of changed cast::*! invocations, \
             snapshot swap, broadcast. Milliseconds. No rust-analyzer.",
        implementation_handler @ crate::watcher::handle_implementation:
            "Expensive branch: full rust-analyzer reload + per-handle \
             pipeline rerun. Seconds-to-minutes. Lazy by default — \
             runs only on `rebuild` query or under `--eager`.",
    },
    tags: ["cast_watch"],
    note: "Structural counterpart to the tiered_analysis concept. The \
           classifier (classifier::classify) decides which stage an \
           event belongs to; the daemon picks the cheap stage by \
           default and only escalates on demand.",
}

cast::tier! {
    axis: analysis_residency,
    direction: increasing,
    stages: {
        empty_state @ crate::state::from_loading:
            "Initial LiveState before any analysis runs. Empty Report, \
             empty file_bytes cache, no RA databases. Tens of KB \
             resident — basically just the socket, the broadcast \
             channel, and the bookkeeping for `phase=loading`.",
        handle_dbs @ cast::load_projects:
            "After load_projects returns, every project root has been \
             loaded as an independent RootDatabase + Vfs and stuffed \
             into MultiProject.handles. For voluntas-shaped trees \
             with N project paths this is the dominant stage and the \
             one that OOMs without containment — each DB sits at \
             1-2 GB minimum, and they all coexist. Single-workspace \
             targets land here at one DB.",
        per_handle_reports @ cast::run_per_handle_analysis:
            "Each per-handle Report is built under attach_db and \
             accumulated into a Vec<Report> before the merge step. \
             Adds Σ(per-handle invocation count × tag fan-out) on \
             top of the handle DBs — small fraction of the DB cost, \
             but allocates per build_report run. The handle DBs are \
             still resident — none are dropped during this stage.",
        merged_report @ cast::merge_reports:
            "merge_reports folds the per-handle Reports plus the .cast \
             pass into a single Report. Per-handle Reports become \
             unreferenced and eligible for drop, but MultiProject \
             (and therefore every handle DB) is still alive — drop \
             happens only after the merged Report has been built and \
             the snapshot committed.",
        snapshot @ crate::state::commit:
            "Snapshot containing Arc<Report>, the file_bytes pre-warm \
             cache, and the stale_files set replaces the empty initial \
             Snapshot via atomic swap. After this point the daemon's \
             steady-state residency is dominated by the Snapshot — \
             the one object the query interface and the broadcast \
             subscribers actually read. Handle DBs and MultiProject \
             can drop; in practice they stay resident under `--eager` \
             so impl_branch reanalysis can run without re-loading.",
    },
    tags: ["cast_watch", "residency"],
    note: "The cliff is between empty_state and handle_dbs: under \
           N-root invocation, that step alone goes from a few KB to \
           N × GB. Single-root invocation (preferred per the \
           single_workspace_invocation rule below) keeps it to one \
           DB; multi-root invocation against voluntas-shaped trees \
           needs cgroup containment per the cold_load_address_space \
           rule. The structural fix that flattens this curve is the \
           cast_spec/project.rs anti_pattern about \
           handle-lifetime — drain handles one at a time, drop after \
           Report is built, instead of holding all of them through \
           the whole run.",
}

cast::concept! {
    name: "query_interface",
    summary: "JSON-lines protocol over a Unix domain socket. One request \
              per line, one response per line. Subscribers receive \
              broadcasts when the live state changes. Modeled on the \
              JSON-lines control-socket pattern common to long-running \
              daemons.",
    anchors: [
        crate::socket::QueryServer,
        crate::socket::handle_rebuild,
        CAST::AS_PRIMITIVE::crate::socket::Request,
        CAST::AS_PRIMITIVE::crate::socket::Response,
        CAST::AS_PRIMITIVE::crate::socket::handle_connection,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  crate::socket::Request,
    concept: "query_interface",
    why:     "query_interface continues into the wire protocol at Request",
}

cast::continues_in! {
    target:  crate::socket::handle_connection,
    concept: "query_interface",
    why:     "query_interface continues into connection handling at handle_connection",
}

cast::continues_in! {
    target:  cast_stdlib::messaging::line_oriented_protocol::LineOrientedProtocol,
    concept: "query_interface",
    why:     "JSON Lines: one request per `\\n`-terminated line, one \
              response per line. The newline IS the framing — no \
              length prefix, no header. Trivially `tail -f`-able and \
              greppable, which matters for an LLM-driven control \
              socket where the human operator wants to read live \
              traffic.",
    tags:    ["cast_watch"],
}

// ─── Rules ───────────────────────────────────────────────────────────────

cast::rule! {
    rule: "Macro-only changes must never trigger a rust-analyzer reload \
           or any rustc invocation. The daemon reparses the changed \
           cast::*! invocation in-process with syn, updates the live \
           snapshot, and broadcasts to subscribers — and that is the \
           entire fast-path.",
    why:  "rust-analyzer's project load on a Voluntas-sized tree is \
           seconds-to-minutes; a Rust rebuild of an interior crate is \
           comparable. Edits to cast annotations happen continuously \
           during development — every keystroke if an LLM is drafting. \
           Forcing the heavy chain on every annotation tweak makes the \
           tool unusable for its purpose. Annotations are syntactic; \
           treat them syntactically.",
    governs: [
        crate::classifier::classify,
        crate::watcher::handle_macro_only,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "Full rust-analyzer reload is opt-in. It runs only when (a) \
           an implementation file referenced by an anchor changes AND \
           the daemon was started with `--eager`, or (b) a client \
           explicitly issues `query: rebuild`. The default is \
           `lazy`: implementation changes mark anchor diagnostics \
           stale but do not reload until rebuild is requested.",
    why:  "The whole point of tiered analysis is that the user — or \
           the LLM — decides when to pay the heavy cost. A daemon that \
           silently kicks off RA reloads is a daemon that thrashes on \
           save-on-keystroke editors. Make the cost visible and chosen.",
    governs: [
        crate::watcher::handle_implementation,
        crate::socket::handle_rebuild,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "Reanalysis runs into a private snapshot; the live state is \
           updated by atomic swap. Reads on the query socket never \
           observe a partially-rebuilt graph and never block on \
           in-flight reanalysis.",
    why:  "If an LLM is asking 'what concepts does X participate in?' \
           while a rebuild is happening, it must get a coherent answer \
           from before-the-rebuild — not a stall, and not a half-built \
           graph. Snapshot-and-swap is the standard fix; explicit here \
           so we don't slip into a write-locked design later.",
    governs: [
        crate::state::LiveState,
        crate::state::commit,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "Daemon liveness is decoupled from the pipeline task. The \
           process exits on socket failure or ctrl_c — never because \
           the initial analysis failed, panicked, or the file watcher \
           stopped. A failed load transitions phase to `Failed` and the \
           socket continues serving `status` so the client can see the \
           error message. The watcher exiting leaves the prior snapshot \
           queryable.",
    why:  "An LLM connecting to cast-watch right after a load panic \
           must get an actionable `phase: failed` answer with the \
           panic message, not a connection refusal. Exiting the \
           daemon when the pipeline task ends collapses three \
           distinguishable failure modes — load error, RA panic, \
           watcher death — into one indistinguishable 'daemon gone' \
           signal at the socket layer.",
    governs: [
        crate::start,
        crate::state::mark_failed,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "cast-watch consumes cast-extract as a library, never as a \
           subprocess. Parser, validator, and concept-graph types are \
           shared; the daemon and the one-shot CLI cannot disagree \
           about what a cast::*! invocation means.",
    why:  "Two binaries with two parsers is exactly the kind of split \
           SSOT this codebase exists to avoid. Reuse the library so the \
           daemon's view of the world is the same view CI computes.",
    governs: [
        crate::watcher::handle_implementation,
        crate::state::commit,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "The initial load (load_projects + run_multi_root_analysis) \
           must run under a bounded address-space ceiling — a cgroup \
           MemoryMax (preferred, OOM-isolates), an RLIMIT_AS, or \
           equivalent. The tiered_analysis concept's lazy-cost \
           guarantees govern steady-state edits; they say nothing \
           about the cold load, which is unconditionally eager and \
           on a voluntas-sized tree can exhaust host memory. A \
           bounded cold load gives the kernel a local target to \
           OOM-kill instead of reaping whatever happens to be \
           biggest on the host (terminal, editor, the user's \
           session). `crates/cast-web/dev/probe-voluntas.sh` is the \
           canonical wrapper.",
    why:  "An OOM kill that takes down the parent shell or the \
           editor the operator is using is a containment failure. \
           cast-watch is intended to coexist with a developer's \
           editing loop, not to be the daemon that crashes their \
           tmux session. The cost of being wrong about residency \
           must not bleed past cast-watch's own process boundary, \
           and the kernel's only mechanism for that is cgroup or \
           rlimit accounting set up by whoever launches the daemon. \
           The May 2026 voluntas OOM crashed the host because no \
           ceiling was in place — see analysis_working_set in \
           cast-spec/project.rs for the diagnostic.",
    governs: [
        crate::start,
        crate::start_async,
    ],
    tags: ["cast_watch", "residency"],
}

cast::rule! {
    rule: "When the target tree has a unifying Cargo.toml workspace, \
           prefer a single-root invocation (`cast-watch \
           /path/to/root`) over a multi-root invocation \
           (`cast-watch /path/to/sub-a /path/to/sub-b ...`). One \
           path → one RootDatabase → linear residency in the \
           workspace's source size. N paths → N independent \
           RootDatabases → multiplicative residency, with \
           voluntas-shaped trees blowing past available RAM well \
           before the analysis finishes.",
    why:  "load_projects calls `ra_ap_load_cargo::load_workspace_at` \
           once per path. That function builds a complete RA \
           database per call, with no sharing of name-resolution \
           caches, syntax trees, or transitive-dep indexes between \
           calls. Two calls covering the same workspace cost \
           roughly twice as much as one call covering it directly. \
           The voluntas May 2026 OOM was exactly this: cast-watch \
           invoked per Cargo.toml in voluntas held 4-8 independent \
           DBs concurrently and was reaped by the kernel; \
           introducing a toplevel virtual workspace at the voluntas \
           root and pointing cast-watch at that single path \
           collapsed residency from `>8 GB OOM` to `~3.7 GB clean \
           run` for the same source tree.",
    governs: [
        crate::start,
        crate::start_async,
    ],
    tags: ["cast_watch", "residency"],
}

// ─── Anti-patterns ───────────────────────────────────────────────────────

cast::anti_pattern! {
    avoid:      "Spawning the cast-extract binary as a subprocess on \
                 every file change.",
    why:        "Subprocess spawn + RA project load on every edit is \
                 catastrophic latency, and it duplicates the parser/\
                 validator surface across two processes that will drift.",
    instead:    "Call into cast-extract's library API: \
                 `run_multi_root_analysis` for full passes, the per-\
                 handle helpers for scoped reruns.",
    instead_at: cast::run_multi_root_analysis,
    governs: [
        crate::watcher::handle_implementation,
    ],
    tags: ["cast_watch"],
}

cast::anti_pattern! {
    avoid:      "Triggering a rust-analyzer reload (or any rustc run) \
                 when the edit was confined to bytes inside cast::*! \
                 invocations.",
    why:        "Macro-only changes are syntactic — the validator's \
                 input is the macro body, which `syn` parses in \
                 microseconds. A full rebuild is six orders of \
                 magnitude more work for zero additional information.",
    instead:    "Use the macro-only path: classifier flags the event, \
                 watcher reparses just the changed file's cast::*! \
                 invocations, state commits the delta.",
    instead_at: crate::watcher::handle_macro_only,
    governs: [
        crate::classifier::classify,
    ],
    tags: ["cast_watch"],
}

cast::anti_pattern! {
    avoid:      "Holding a write lock across reanalysis so query-socket \
                 reads stall until the rebuild finishes.",
    why:        "An LLM consulting cast-watch mid-draft must not be \
                 blocked by a rebuild it didn't ask for. Stalls become \
                 indistinguishable from hangs at the UX layer.",
    instead:    "Reanalyze into a private snapshot, then atomically swap \
                 a single Arc<Snapshot> pointer. Readers always observe \
                 a coherent prior or new state — never an in-progress one.",
    instead_at: crate::state::commit,
    governs: [
        crate::state::LiveState,
    ],
    tags: ["cast_watch"],
}

// ─── Pipeline ────────────────────────────────────────────────────────────

cast::pipeline! {
    stages: {
        notify_event   @ crate::watcher::FileWatcher,
        classify       @ crate::classifier::classify,
        macro_branch   @ crate::watcher::handle_macro_only,
        impl_branch    @ crate::watcher::handle_implementation,
        commit         @ crate::state::commit,
        broadcast      @ crate::socket::QueryServer,
    },
    flow: [
        notify_event -> classify,
        classify     -> macro_branch,
        classify     -> impl_branch,
        macro_branch -> commit,
        impl_branch  -> commit,
        commit       -> broadcast,
    ],
    tags: ["cast_watch"],
    note: "Two terminal-cost branches diverge at `classify` and rejoin \
           at `commit`. `impl_branch` is a no-op in `--lazy` mode \
           except to mark anchor diagnostics stale; the full RA work \
           lives behind the `rebuild` query, which re-enters the \
           pipeline at `impl_branch`.",
}

// ─── Surface ─────────────────────────────────────────────────────────────

/// Start the daemon. Loads the workspace via cast-extract, opens the
/// query socket, kicks off the file watcher, and runs the event
/// pipeline until shutdown.
///
/// Free function rather than `Daemon::start` because cast anchors
/// resolve module paths but not `impl` traversal — and because Cast
/// concepts name *behavior*, not OO method receivers. The orchestration
/// owns no internal state worth a struct.
pub fn start(
    socket_path: std::path::PathBuf,
    projects: Vec<std::path::PathBuf>,
    eager: bool,
) -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(start_async(socket_path, projects, eager))
}

async fn start_async(
    socket_path: std::path::PathBuf,
    projects: Vec<std::path::PathBuf>,
    eager: bool,
) -> anyhow::Result<()> {
    if projects.is_empty() {
        anyhow::bail!("cast-watch needs at least one project root");
    }
    let repo_root = cast::find_repo_root(&projects[0])
        .unwrap_or_else(|| projects[0].clone());

    let live = std::sync::Arc::new(state::from_loading(
        projects.clone(),
        repo_root.clone(),
    ));

    let socket_state = live.clone();
    let socket_path_for_serve = socket_path.clone();
    let socket_task = tokio::spawn(async move {
        if let Err(e) = socket::serve(&socket_path_for_serve, socket_state).await {
            tracing::error!(error = %e, "query socket terminated");
        }
    });

    tracing::info!(
        roots = ?projects,
        repo_root = %repo_root.display(),
        eager,
        "loading initial multi-root analysis (socket already bound; clients can poll status)"
    );

    let loader_state = live.clone();
    let loader_projects = projects.clone();
    let loader_repo_root = repo_root.clone();
    // Heavy work runs on the blocking pool so the socket accept loop
    // and any in-flight handlers keep flowing during the cold load.
    // The pre-warm walk below runs in the same task because it's I/O
    // against the same files RA just touched — cheap, and it keeps the
    // file_bytes cache atomically aligned with the Report it ships with.
    let load_handle = tokio::task::spawn_blocking(move || -> anyhow::Result<(cast::Report, std::collections::HashMap<std::path::PathBuf, Vec<u8>>)> {
        let multi = cast::load_projects(&loader_projects)?;
        let report = cast::run_multi_root_analysis(&multi, &loader_repo_root);
        drop(multi);
        let file_bytes = pre_warm_file_bytes(&report);
        Ok((report, file_bytes))
    });

    let pipeline_task = tokio::spawn(async move {
        let (report, file_bytes) = match load_handle.await {
            Ok(Ok(r)) => r,
            Ok(Err(e)) => {
                tracing::error!(error = %e, "initial multi-root analysis failed");
                state::mark_failed(&loader_state, e.to_string());
                return;
            }
            Err(e) => {
                tracing::error!(error = %e, "initial analysis task panicked");
                state::mark_failed(&loader_state, e.to_string());
                return;
            }
        };

        let cached_files = file_bytes.len();
        let next = state::Snapshot {
            generation: 0,
            report: std::sync::Arc::new(report),
            file_bytes,
            stale_files: std::collections::HashSet::new(),
        };
        state::commit(&loader_state, next);
        tracing::debug!(cached = cached_files, "pre-warmed file_bytes for cast-bearing files");
        state::mark_ready(&loader_state);
        let new_gen = state::current(&loader_state).generation;
        tracing::info!(generation = new_gen, "initial analysis ready");
        let _ = loader_state.broadcast.send(socket::Broadcast {
            snapshot_generation: new_gen,
            kind: socket::BroadcastKind::SnapshotChanged,
        });

        let mut watcher = watcher::new(eager);
        if let Err(e) = watcher::subscribe(&mut watcher, &loader_state.roots) {
            tracing::error!(error = %e, "file watcher subscribe failed");
            return;
        }
        if let Err(e) = watcher::run(&mut watcher, loader_state).await {
            tracing::error!(error = %e, "file watcher terminated");
        }
    });

    // Daemon liveness is the socket and ctrl_c — never the pipeline task.
    // If the initial analysis fails or panics, `mark_failed` records the
    // error and `pipeline_task` returns; the socket must keep answering
    // `status` so the client can see why. Same for the file watcher
    // exiting later: the prior snapshot remains queryable.
    tokio::select! {
        r = socket_task => { r?; }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("shutdown requested");
        }
    }
    pipeline_task.abort();
    Ok(())
}

cast::concept! {
    name: "daemon_orchestration",
    summary: "Top-level entry point and pre-warm helper. Boots the \
              tokio runtime, builds LiveState, opens the query socket, \
              kicks off the cold load on a blocking task, then drives \
              the file watcher until shutdown.",
    anchors: [
        crate::start,
        crate::start_async,
        crate::pre_warm_file_bytes,
    ],
    tags: ["cast_watch_orchestration"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "daemon_orchestration",
    why: "Filesystem reads, network socket binding, tokio runtime \
          scheduling, and signal handling are all observable external \
          state; same args can produce different runs.",
}

cast::continues_in! {
    target: cast_stdlib::lifecycle::reconciliation_loop,
    concept: "daemon_orchestration",
    why: lazy,
}

/// Read the bytes of every file that contains at least one `cast::*!`
/// invocation, keyed by absolute path — the same key shape the watcher
/// will produce when notify fires. Lets the classifier compute a real
/// before/after diff on the very first edit instead of falling back to
/// `Implementation` (the safe-but-pessimistic default it picks when the
/// pre-edit bytes are missing).
fn pre_warm_file_bytes(report: &cast::Report) -> std::collections::HashMap<std::path::PathBuf, Vec<u8>> {
    let mut paths: std::collections::HashSet<std::path::PathBuf> =
        std::collections::HashSet::new();
    for invs in report.groups.values() {
        for inv in invs {
            paths.insert(std::path::PathBuf::from(&inv.file));
        }
    }
    let mut cache = std::collections::HashMap::with_capacity(paths.len());
    for path in paths {
        match std::fs::read(&path) {
            Ok(bytes) => {
                cache.insert(path, bytes);
            }
            Err(e) => {
                tracing::debug!(path = %path.display(), error = %e, "pre-warm: skip");
            }
        }
    }
    cache
}
