//! File watcher and tier dispatch.
//!
//! Owns the `notify` event stream, classifies each event via
//! `crate::classifier::classify`, and routes into one of two terminal
//! handlers — `handle_macro_only` (cheap) or `handle_implementation`
//! (escalates to RA reload only when the daemon is `--eager` or a
//! client issued `rebuild`).

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::socket::{Broadcast, BroadcastKind};
use crate::state::{self, LiveState, Snapshot};

cast::concept! {
    name: "tier_dispatch",
    summary: "The branch point of the event pipeline. notify produces a \
              stream of filesystem changes; the watcher pulls each event, \
              looks up the cached file bytes, asks the classifier which \
              tier the change belongs to, and calls the matching handler.",
    anchors: [
        crate::watcher::FileWatcher,
        crate::watcher::run,
        crate::watcher::handle_macro_only,
        crate::watcher::handle_implementation,
        CAST::AS_PRIMITIVE::crate::watcher::debounce,
        CAST::AS_PRIMITIVE::crate::watcher::subscribe,
        CAST::AS_PRIMITIVE::crate::watcher::mark_stale,
        CAST::AS_PRIMITIVE::crate::state::commit,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  crate::watcher::debounce,
    concept: "tier_dispatch",
    why:     "tier_dispatch continues into debounce() for event coalescing",
}

cast::continues_in! {
    target:  crate::watcher::subscribe,
    concept: "tier_dispatch",
    why:     "tier_dispatch continues into subscribe() for notify wiring",
}

cast::continues_in! {
    target:  crate::watcher::mark_stale,
    concept: "tier_dispatch",
    why:     "tier_dispatch continues into mark_stale() for lazy-mode impl changes",
}

cast::continues_in! {
    target:  crate::state::commit,
    concept: "tier_dispatch",
    why:     "tier_dispatch terminates at the write_boundary — every handler commits + broadcasts",
}

cast::continues_in! {
    target:  cast_stdlib::architecture::classifier_dispatch::ClassifierDispatch,
    concept: "tier_dispatch",
    why:     "Classic classify-then-route: `classifier::classify` \
              maps each filesystem event to a ChangeKind, then the \
              dispatcher calls handle_macro_only or \
              handle_implementation based on the kind. The classifier \
              IS the policy (which tier this event belongs to); the \
              handlers are the mechanism.",
    tags:    ["cast_watch"],
}

cast::pipeline! {
    stages: {
        subscribe         @ crate::watcher::subscribe,
        debounce          @ crate::watcher::debounce,
        classify          @ crate::classifier::classify,
        macro_branch      @ crate::watcher::handle_macro_only,
        impl_branch       @ crate::watcher::handle_implementation,
        mark_stale_branch @ crate::watcher::mark_stale,
    },
    flow: [
        subscribe         -> debounce,
        debounce          -> classify,
        classify          -> macro_branch,
        classify          -> impl_branch,
        impl_branch       -> mark_stale_branch,
    ],
    tags: ["cast_watch"],
    note: "Watcher-internal flow inside tier_dispatch: notify events \
           are filtered by subscribe, coalesced by debounce, branched \
           by classify, and either committed cheaply (macro_branch) or \
           marked stale (impl_branch via mark_stale_branch). The \
           daemon-level pipeline in lib.rs picks up at commit and \
           continues to broadcast.",
}

cast::rule! {
    rule: "handle_macro_only must complete entirely without invoking \
           rust-analyzer or any cargo command. Its work is: reparse \
           the changed file's cast::*! invocations with syn, run the \
           grammar-only validation, build a new Snapshot, commit.",
    why:  "This is the rule the user asked for in plain English: \
           'don't fully build if changes are just in the macro layer'. \
           Encoded as an enforced anchor so any future refactor that \
           introduces an RA call here will surface as a governance \
           violation in cast-extract output.",
    governs: [
        crate::watcher::handle_macro_only,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "handle_implementation in `--lazy` mode must not trigger an \
           RA reload — it marks the affected anchors stale in the \
           current snapshot and returns. The reload is deferred until \
           a client issues `rebuild` (which calls into the same handler \
           with eager=true).",
    why:  "Lazy is the default because most edits don't need RA \
           verification immediately — the LLM is mid-draft, the human \
           is mid-edit, and an asynchronous rebuild would compete with \
           the editor for CPU. Surfacing 'stale' is enough; pulling the \
           trigger is the user's decision.",
    governs: [
        crate::watcher::handle_implementation,
    ],
    tags: ["cast_watch"],
}

cast::concept! {
    name: "notify_subscription",
    summary: "The notify-backend wiring. Recursive watch over each \
              project root, with a path filter that drops irrelevant \
              events (target/, .git/, node_modules/, .swp files) before \
              they enter the dispatch loop. The watcher hands events \
              off to an async channel so the notify thread is never \
              blocked by classification or RA work.",
    anchors: [
        crate::watcher::FileWatcher,
        crate::watcher::subscribe,
        crate::watcher::should_watch,
    ],
    tags: ["cast_watch"],
}

cast::concept! {
    name: "event_debounce",
    summary: "Editors emit several FS events per save (write, rename, \
              create, modify) within milliseconds. The debouncer \
              coalesces events on the same path inside a short window \
              into one classification pass — without it, a single Vim \
              :w costs three syn parses.",
    anchors: [
        crate::watcher::debounce,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  cast_stdlib::performance::debounce::Debounce,
    concept: "event_debounce",
    why:     "The debouncer keeps a per-path Instant of the most \
              recent event; a burst of notify events for the same \
              path inside DEBOUNCE_WINDOW reset the timer instead of \
              dispatching, and a classification pass fires once after \
              the path has been quiet for the window. Trailing-edge \
              debounce, in user space.",
    tags:    ["cast_watch"],
}

cast::rule! {
    rule: "The debounce window is bounded — tens of milliseconds, not \
           seconds. The fast path's whole value proposition is sub-\
           perception latency from save to validated annotation; a \
           multi-hundred-ms debounce defeats it.",
    why:  "If an LLM writes a cast::*! invocation and the daemon \
           responds 800ms later, the LLM's next decision proceeds \
           without the validator's feedback. The debounce window is a \
           direct multiplier on the LLM's effective context — keep it \
           tight.",
    governs: [
        crate::watcher::debounce,
    ],
    tags: ["cast_watch"],
}

cast::concept! {
    name: "staleness_marker",
    summary: "Lazy mode's response to an implementation change: record \
              the path in Snapshot.stale_files and commit. Queries still \
              see the last-known resolutions but the response carries \
              the stale set so an LLM can decide whether to trust them \
              or issue `rebuild`.",
    anchors: [
        crate::watcher::handle_implementation,
        crate::watcher::mark_stale,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  cast_stdlib::consistency::eventual_consistency::EventualConsistency,
    concept: "staleness_marker",
    why:     "The lazy-mode contract is eventual consistency: queries \
              see the last-good resolution after an implementation \
              edit and the daemon converges to the new truth on the \
              next `rebuild`. The stale_files set is the explicit \
              divergence marker that lets clients decide whether to \
              wait or proceed.",
    tags:    ["cast_watch"],
}

/// Debounce window for coalescing rapid same-path events. Tight — the
/// `event_debounce` rule explains why.
pub const DEBOUNCE_WINDOW: Duration = Duration::from_millis(50);

/// Owns the notify watcher handle, the event channel, the pending-
/// debounce table, and (in `--eager` mode) the flag that escalates
/// implementation changes immediately.
pub struct FileWatcher {
    pub eager: bool,
    pub events_rx: tokio::sync::mpsc::UnboundedReceiver<notify::Event>,
    pub events_tx: tokio::sync::mpsc::UnboundedSender<notify::Event>,
    pub _notify: Option<notify::RecommendedWatcher>,
    pub pending: HashMap<PathBuf, Instant>,
}

/// Build a fresh FileWatcher with no notify handle yet attached. Call
/// `subscribe` to register the project roots.
pub fn new(eager: bool) -> FileWatcher {
    let (events_tx, events_rx) = tokio::sync::mpsc::unbounded_channel();
    FileWatcher {
        eager,
        events_rx,
        events_tx,
        _notify: None,
        pending: HashMap::new(),
    }
}

/// Run the watch loop until the shutdown signal fires. Each event is
/// debounced, classified and dispatched.
pub async fn run(watcher: &mut FileWatcher, state: Arc<LiveState>) -> anyhow::Result<()> {
    let mut tick = tokio::time::interval(DEBOUNCE_WINDOW);
    loop {
        tokio::select! {
            maybe_event = watcher.events_rx.recv() => {
                let Some(event) = maybe_event else { break };
                for path in event_paths(&event) {
                    if !should_watch(&path) {
                        continue;
                    }
                    watcher.pending.insert(path, Instant::now());
                }
            }
            _ = tick.tick() => {
                let now = Instant::now();
                let ready: Vec<PathBuf> = watcher
                    .pending
                    .iter()
                    .filter(|(_, t)| now.duration_since(**t) >= DEBOUNCE_WINDOW)
                    .map(|(p, _)| p.clone())
                    .collect();
                for path in ready {
                    watcher.pending.remove(&path);
                    if let Err(e) = dispatch(&path, &state, watcher.eager) {
                        tracing::warn!(?path, error = %e, "dispatch failed");
                    }
                }
            }
        }
    }
    Ok(())
}

/// Pull every `PathBuf` out of a notify event.
fn event_paths(event: &notify::Event) -> Vec<PathBuf> {
    event.paths.clone()
}

/// Classify a single ready path and run the matching handler.
fn dispatch(path: &Path, state: &LiveState, eager: bool) -> anyhow::Result<()> {
    let after = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e.into()),
    };
    let snap = state::current(state);
    let before = snap.file_bytes.get(path).cloned();
    let kind = crate::classifier::classify(path, before.as_deref(), &after);
    match kind {
        crate::classifier::ChangeKind::Irrelevant => Ok(()),
        crate::classifier::ChangeKind::MacroOnly => handle_macro_only(path, &after, state),
        crate::classifier::ChangeKind::Implementation => {
            handle_implementation(path, state, eager)
        }
    }
}

/// Macro-only fast path. Update the cached file bytes, commit a new
/// snapshot with bumped generation, and broadcast SnapshotChanged. The
/// concept graph itself does not change here — the macro-only edit
/// shows up the next time a query asks for `unresolved_in_file` (which
/// reparses on demand) or the next `rebuild`.
pub fn handle_macro_only(
    path: &Path,
    new_bytes: &[u8],
    state: &LiveState,
) -> anyhow::Result<()> {
    let prior = state::current(state);
    let mut next = clone_for_commit(&prior);
    state::record_file_bytes(&mut next, path.to_path_buf(), new_bytes.to_vec());
    next.stale_files.remove(path);
    state::commit(state, next);
    let new_gen = state::current(state).generation;
    let _ = state.broadcast.send(Broadcast {
        snapshot_generation: new_gen,
        kind: BroadcastKind::SnapshotChanged,
    });
    Ok(())
}

/// Implementation path. Lazy: mark `path` stale, commit, broadcast.
/// Eager: rebuild the report from scratch via cast-extract and commit
/// the resulting Snapshot.
pub fn handle_implementation(
    path: &Path,
    state: &LiveState,
    eager: bool,
) -> anyhow::Result<()> {
    if !eager {
        let prior = state::current(state);
        let mut next = clone_for_commit(&prior);
        mark_stale(&mut next, path);
        if let Ok(bytes) = std::fs::read(path) {
            state::record_file_bytes(&mut next, path.to_path_buf(), bytes);
        }
        state::commit(state, next);
        let new_gen = state::current(state).generation;
        let _ = state.broadcast.send(Broadcast {
            snapshot_generation: new_gen,
            kind: BroadcastKind::SnapshotChanged,
        });
        return Ok(());
    }
    let _ = state.broadcast.send(Broadcast {
        snapshot_generation: state::current(state).generation,
        kind: BroadcastKind::RebuildStarted,
    });
    let multi = cast::load_projects(&state.roots)?;
    let report = cast::run_multi_root_analysis(&multi, &state.repo_root);
    let prior = state::current(state);
    let next = Snapshot {
        generation: 0,
        report: Arc::new(report),
        file_bytes: prior.file_bytes.clone(),
        stale_files: HashSet::new(),
    };
    state::commit(state, next);
    let new_gen = state::current(state).generation;
    let _ = state.broadcast.send(Broadcast {
        snapshot_generation: new_gen,
        kind: BroadcastKind::RebuildCompleted,
    });
    Ok(())
}

/// Build a new Snapshot that carries forward everything in `prior` —
/// the macro/lazy paths only mutate the cached bytes and stale-file
/// set, so the Report and generation are reused as-is (commit will
/// rewrite the generation).
fn clone_for_commit(prior: &Snapshot) -> Snapshot {
    Snapshot {
        generation: 0,
        report: prior.report.clone(),
        file_bytes: prior.file_bytes.clone(),
        stale_files: prior.stale_files.clone(),
    }
}

/// Wire the notify backend to the event channel. Recursive watch over
/// each project root; events flow to the dispatch loop through an
/// async channel so the notify thread is never blocked by parse or RA
/// work.
pub fn subscribe(
    watcher: &mut FileWatcher,
    roots: &[PathBuf],
) -> anyhow::Result<()> {
    use notify::{RecursiveMode, Watcher};
    let tx = watcher.events_tx.clone();
    let mut backend = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if let Ok(event) = res {
            let _ = tx.send(event);
        }
    })?;
    for root in roots {
        backend.watch(root, RecursiveMode::Recursive)?;
    }
    watcher._notify = Some(backend);
    Ok(())
}

/// Path filter — return false for paths the daemon should ignore
/// (target/, .git/, node_modules/, .swp, vendored deps).
pub fn should_watch(path: &Path) -> bool {
    let s = path.to_string_lossy();
    const IGNORE_SEGMENTS: &[&str] = &[
        "/target/",
        "/.git/",
        "/node_modules/",
        "/.cargo/",
        "/.rustup/",
    ];
    for seg in IGNORE_SEGMENTS {
        if s.contains(seg) {
            return false;
        }
    }
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        if name.ends_with(".swp") || name.ends_with("~") || name.starts_with(".#") {
            return false;
        }
    }
    true
}

/// Coalesce rapid same-path events into one classification pass.
/// Window is bounded — see the `event_debounce` rule. Returns
/// `Some(path)` once `path` has been quiet for the window.
pub fn debounce(
    event: notify::Event,
    pending: &mut HashMap<PathBuf, Instant>,
) -> Option<PathBuf> {
    let now = Instant::now();
    let mut latest: Option<PathBuf> = None;
    for p in event.paths {
        pending.insert(p.clone(), now);
        latest = Some(p);
    }
    let path = latest?;
    let when = pending.get(&path).copied()?;
    if now.duration_since(when) >= DEBOUNCE_WINDOW {
        pending.remove(&path);
        Some(path)
    } else {
        None
    }
}

/// Mark `path` as stale in the Snapshot's stale-file set. Used by
/// `handle_implementation` in `--lazy` mode.
pub fn mark_stale(
    snapshot: &mut Snapshot,
    path: &Path,
) {
    snapshot.stale_files.insert(path.to_path_buf());
}

cast::concept! {
    name: "file_watcher_handle",
    summary: "Wraps notify::RecommendedWatcher + a debouncer + the \
              eager flag. The runtime resource that turns inotify \
              events into ChangeKind dispatches.",
    anchors: [
        crate::watcher::FileWatcher,
        crate::watcher::new,
        crate::watcher::subscribe,
        crate::watcher::run,
    ],
    tags: ["cast_watch_watcher"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::resource_handle,
    concept: "file_watcher_handle",
    why: "Wraps a kernel inotify subscription via notify::Watcher; \
          identity is the inotify fd, not the field shape.",
}

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "file_watcher_handle",
    why: "Driven by filesystem events; same call may produce different \
          dispatches across runs depending on event timing and \
          ordering.",
}

cast::concept! {
    name: "watcher_event_dispatch",
    summary: "Per-event dispatch path: extract paths, run classifier, \
              choose macro-only or implementation branch, commit. \
              Deterministic given inputs (path, before bytes, after \
              bytes), but invoked under non-deterministic filesystem \
              event timing.",
    anchors: [
        crate::watcher::dispatch,
        crate::watcher::handle_macro_only,
        crate::watcher::handle_implementation,
        crate::watcher::event_paths,
        crate::watcher::clone_for_commit,
        crate::watcher::should_watch,
        crate::watcher::debounce,
        crate::watcher::mark_stale,
    ],
    tags: ["cast_watch_watcher"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::deterministic,
    concept: "watcher_event_dispatch",
    why: "Each function's output is decided by its inputs; the \
          non-determinism lives in the event source, not in the \
          dispatch logic.",
}

cast::continues_in! {
    target: cast_stdlib::lifecycle::reconciliation_loop,
    concept: "file_watcher_handle",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::architecture::classifier_dispatch,
    concept: "watcher_event_dispatch",
    why: lazy,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_directory_is_filtered() {
        assert!(!should_watch(Path::new("/repo/target/debug/foo.rs")));
        assert!(!should_watch(Path::new("/repo/.git/HEAD")));
        assert!(!should_watch(Path::new("/repo/node_modules/x/index.js")));
    }

    #[test]
    fn editor_swap_files_filtered() {
        assert!(!should_watch(Path::new("/repo/src/.foo.rs.swp")));
        assert!(!should_watch(Path::new("/repo/src/foo.rs~")));
        assert!(!should_watch(Path::new("/repo/src/.#foo.rs")));
    }

    #[test]
    fn ordinary_rust_file_watched() {
        assert!(should_watch(Path::new("/repo/src/lib.rs")));
        assert!(should_watch(Path::new("/repo/Cargo.toml")));
    }

    #[test]
    fn mark_stale_records_the_path() {
        let mut snap = Snapshot {
            generation: 0,
            report: Arc::new(empty_report()),
            file_bytes: HashMap::new(),
            stale_files: HashSet::new(),
        };
        mark_stale(&mut snap, Path::new("/repo/src/foo.rs"));
        assert!(snap.stale_files.contains(Path::new("/repo/src/foo.rs")));
    }

    fn empty_report() -> cast::Report {
        cast::Report {
            summary: cast::emit::model::Summary {
                invocations: 0,
                parsed: 0,
                parse_errors: 0,
                unimplemented: 0,
                paths_resolved: 0,
                paths_unresolved: 0,
                io_ok: 0,
                io_unresolved: 0,
                pipeline_errors: 0,
                graph_warnings: 0,
            },
            groups: std::collections::BTreeMap::new(),
            concept_graph: cast::emit::model::ConceptGraphReport {
                concepts: std::collections::BTreeMap::new(),
                warnings: vec![],
            },
        }
    }
}
