//! Live state — the snapshot a query reads from.
//!
//! Holds an `Arc<cast::Report>` plus the file/byte cache the
//! classifier reads. Updated by atomic swap so readers never observe
//! a partially-rebuilt graph.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Instant;

cast::concept! {
    name: "live_state_snapshot",
    summary: "The queryable view of the world. Wraps an \
              Arc<cast::Report> (the same struct cast-extract \
              emits from one-shot runs) and caches the file bytes \
              used by the classifier. Updated only via `commit`, which \
              swaps a private new snapshot into place atomically — \
              readers always observe a coherent prior or new state.",
    anchors: [
        crate::state::LiveState,
        crate::state::Snapshot,
        crate::state::commit,
        crate::state::current,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  crate::state::commit,
    concept: "live_state_snapshot",
    why:     "live_state_snapshot continues into the swap mechanism at commit()",
}

cast::continues_in! {
    target:  cast_stdlib::storage::snapshot_storage::SnapshotStorage,
    concept: "live_state_snapshot",
    why:     "live_state_snapshot is a `snapshot_storage` instance — \
              `commit` produces a coherent point-in-time capture; \
              readers see a snapshot that committed before their query \
              and never a partially-written view.",
    tags:    ["cast_watch"],
}

cast::rule! {
    rule: "Snapshot must be cheap to clone — readers (query handlers, \
           subscribers) get an Arc<Snapshot> and the lock is released \
           before any rendering or serialization happens.",
    why:  "A query that fetches the concept graph for a tag should not \
           hold the state lock while it walks the graph and serializes \
           it to JSON. Lock-free reads are the only way to keep the \
           query latency uncorrelated with rebuild duration.",
    governs: [
        crate::state::current,
    ],
    tags: ["cast_watch"],
}

cast::concept! {
    name: "arc_swap_commit",
    summary: "The atomic-swap mechanism. `commit` takes the write lock \
              just long enough to replace a single Arc<Snapshot> \
              pointer, then releases. `current` clones the Arc out \
              under a brief read lock. Read latency is uncoupled from \
              commit work: a query in flight when commit runs observes \
              the prior snapshot to completion, then subsequent queries \
              observe the new one. There is never a 'partial snapshot' \
              state visible to readers.",
    anchors: [
        crate::state::LiveState,
        crate::state::commit,
        crate::state::current,
        CAST::AS_PRIMITIVE::crate::state::Snapshot,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  crate::state::Snapshot,
    concept: "arc_swap_commit",
    why:     "arc_swap_commit continues into generation tracking at Snapshot",
}

cast::continues_in! {
    target:  cast_stdlib::consistency::compare_and_swap::CompareAndSwap,
    concept: "arc_swap_commit",
    why:     "The pointer-replacement step is exactly compare_and_swap: \
              `commit` replaces the Arc<Snapshot> as a single atomic \
              update under a brief write lock; readers either see the \
              old or the new pointer, never an in-between.",
    tags:    ["cast_watch"],
}

cast::continues_in! {
    target:  cast_stdlib::consistency::optimistic_concurrency_control::OptimisticConcurrencyControl,
    concept: "arc_swap_commit",
    why:     "Read-side is OCC: `current` returns the current Arc \
              without coordinating with writers. A reader holding a \
              cloned Arc can finish its work even while a newer \
              snapshot is committed; no retry is needed because the \
              reader's view is internally consistent.",
    tags:    ["cast_watch"],
}

cast::continues_in! {
    target:  cast_stdlib::lifecycle::hot_swap::HotSwap,
    concept: "arc_swap_commit",
    why:     "The architectural pattern that compare_and_swap \
              implements here: live in-process state replaced \
              atomically without restart. Every reader sees either \
              the prior snapshot to completion or the new one going \
              forward — never a partial in-between. cast-watch can \
              ingest an annotation edit and serve queries against \
              the new graph without dropping any in-flight requests.",
    tags:    ["cast_watch"],
}

cast::concept! {
    name: "snapshot_generation",
    summary: "Each Snapshot carries a monotonically-increasing \
              generation number, assigned at commit time. Responses \
              and Broadcasts on the query socket include the \
              generation so a client can dedupe (a broadcast with \
              generation N + 1 supersedes a response with generation \
              N) and detect that its current view is stale (its \
              generation < daemon's current).",
    anchors: [
        crate::state::Snapshot,
        crate::state::next_generation,
        CAST::AS_PRIMITIVE::crate::state::record_file_bytes,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  crate::state::record_file_bytes,
    concept: "snapshot_generation",
    why:     "snapshot_generation continues into the byte cache at record_file_bytes()",
}

cast::continues_in! {
    target:  cast_stdlib::identity::monotonic_sequence_id::MonotonicSequenceId,
    concept: "snapshot_generation",
    why:     "Generations are issued by the daemon in strictly \
              increasing order; clients dedupe by comparing generation \
              numbers and detect their view is stale when their \
              generation < daemon's current. Classic sequence-ID role.",
    tags:    ["cast_watch"],
}

cast::concept! {
    name: "write_boundary",
    summary: "The single architectural seam every mutation crosses: a \
              call to `state::commit` followed by a `state.broadcast.send` \
              of a Broadcast describing what just changed. Both are done \
              together at every site that mutates the snapshot — \
              `watcher::handle_macro_only`, `watcher::handle_implementation`, \
              `socket::dispatch::handle_rebuild`. The pair is a \
              convention enforced by review, not by the type system; \
              this concept names it so the convention has somewhere to \
              live.",
    anchors: [
        crate::state::commit,
        CAST::AS_PRIMITIVE::crate::state::LiveState,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "Every call to state::commit must be paired with a \
           state.broadcast.send(...) on the same code path, before \
           that path returns control. Commit-without-broadcast leaves \
           subscribers stale; broadcast-without-commit advertises a \
           snapshot generation that doesn't exist.",
    why:  "Subscribers dedupe on snapshot_generation. If the daemon \
           commits a new snapshot but doesn't fan out a broadcast, \
           connected LLM clients keep operating on the prior \
           snapshot until they next issue a query — defeating the \
           push design. If the daemon broadcasts without committing, \
           subscribers chase a generation that the daemon's own \
           subsequent reads contradict. The pairing is the contract; \
           keeping them lockstep is what makes 'live state' coherent.",
    governs: [
        crate::state::commit,
        crate::watcher::handle_macro_only,
        crate::watcher::handle_implementation,
        crate::socket::handle_rebuild,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "Generation numbers are strictly monotonic across the \
           daemon's lifetime. Even an idempotent commit (one that \
           swaps in a Snapshot byte-equal to the prior) bumps the \
           generation.",
    why:  "Clients dedupe on generation. If two distinct commits could \
           share a generation — even when 'nothing meaningful changed' \
           — a client's dedupe logic would silently drop the second \
           one. Strict monotonicity is cheaper than 'change detection' \
           and makes the dedupe contract trivial to reason about.",
    governs: [
        crate::state::commit,
        crate::state::next_generation,
    ],
    tags: ["cast_watch"],
}

cast::concept! {
    name: "file_byte_cache",
    summary: "Per-file byte cache the classifier reads to compute \
              before/after diffs without an extra disk roundtrip. The \
              cache lives inside Snapshot so it commits atomically \
              with the report it describes — there is never a Snapshot \
              whose report and file_bytes disagree about which content \
              they were computed from.",
    anchors: [
        crate::state::Snapshot,
        crate::state::record_file_bytes,
    ],
    tags: ["cast_watch"],
}

/// One immutable view of the world. Cheap to clone (held behind Arc).
pub struct Snapshot {
    pub generation: u64,
    pub report: Arc<cast::Report>,
    pub file_bytes: HashMap<PathBuf, Vec<u8>>,
    /// Files modified since the last full rebuild — their anchor
    /// resolutions may have drifted.
    pub stale_files: HashSet<PathBuf>,
}

/// Daemon lifecycle phase. Visible on the `status` query so a client
/// connecting during the multi-second cold load can tell the difference
/// between "graph is empty" and "graph hasn't been computed yet."
#[derive(Debug, Clone)]
pub enum Phase {
    Loading,
    Ready,
    Failed(String),
}

/// Owns the current snapshot, the broadcast channel, and the loaded
/// project roots. Constructed once at daemon startup.
pub struct LiveState {
    current: RwLock<Arc<Snapshot>>,
    next_gen: AtomicU64,
    phase: RwLock<Phase>,
    pub started_at: Instant,
    pub broadcast: tokio::sync::broadcast::Sender<crate::socket::Broadcast>,
    pub roots: Vec<PathBuf>,
    pub repo_root: PathBuf,
}

fn empty_report() -> cast::Report {
    cast::Report {
        summary: cast::emit::model::Summary::default(),
        groups: std::collections::BTreeMap::new(),
        concept_graph: cast::emit::model::ConceptGraphReport {
            concepts: std::collections::BTreeMap::new(),
            warnings: vec![],
        },
        policies: vec![],
        policy_warnings: vec![],
    }
}

/// Build a LiveState in `Loading` phase with an empty Report. Lets the
/// query socket bind immediately; the heavy `load_projects` /
/// `run_multi_root_analysis` chain runs on a blocking worker and
/// commits the real snapshot when it finishes.
pub fn from_loading(roots: Vec<PathBuf>, repo_root: PathBuf) -> LiveState {
    let (tx, _rx) = tokio::sync::broadcast::channel(64);
    let snap = Snapshot {
        generation: 0,
        report: Arc::new(empty_report()),
        file_bytes: HashMap::new(),
        stale_files: HashSet::new(),
    };
    LiveState {
        current: RwLock::new(Arc::new(snap)),
        next_gen: AtomicU64::new(1),
        phase: RwLock::new(Phase::Loading),
        started_at: Instant::now(),
        broadcast: tx,
        roots,
        repo_root,
    }
}

/// Build a LiveState already in `Ready` phase with the given Report.
/// Convenience for tests and for callers that already have a finished
/// analysis in hand.
pub fn from_initial(
    report: cast::Report,
    roots: Vec<PathBuf>,
    repo_root: PathBuf,
) -> LiveState {
    let live = from_loading(roots, repo_root);
    let next = Snapshot {
        generation: 0,
        report: Arc::new(report),
        file_bytes: HashMap::new(),
        stale_files: HashSet::new(),
    };
    commit(&live, next);
    mark_ready(&live);
    live
}

/// Read the current daemon phase. Cheap.
pub fn phase(state: &LiveState) -> Phase {
    state.phase.read().expect("LiveState phase lock poisoned").clone()
}

/// Transition the daemon to `Ready`. Called once by the loader task
/// after the initial commit lands.
pub fn mark_ready(state: &LiveState) {
    *state.phase.write().expect("LiveState phase lock poisoned") = Phase::Ready;
}

/// Transition the daemon to `Failed`. Called by the loader task if
/// `load_projects` or `run_multi_root_analysis` errored out — the
/// status query will surface the message.
pub fn mark_failed(state: &LiveState, error: String) {
    *state.phase.write().expect("LiveState phase lock poisoned") = Phase::Failed(error);
}

/// Read the current snapshot. Cheap; never blocks long.
pub fn current(state: &LiveState) -> Arc<Snapshot> {
    state.current.read().expect("LiveState lock poisoned").clone()
}

/// Atomically replace the current snapshot. Bumps generation
/// regardless of content equality (see the `snapshot_generation` rule).
pub fn commit(state: &LiveState, mut next: Snapshot) {
    next.generation = next_generation(state);
    let mut guard = state.current.write().expect("LiveState lock poisoned");
    *guard = Arc::new(next);
}

/// Allocate the next generation number. Strictly monotonic across the
/// daemon's lifetime.
pub fn next_generation(state: &LiveState) -> u64 {
    state.next_gen.fetch_add(1, Ordering::Relaxed)
}

/// Update Snapshot's per-file byte cache for a single file. Called by
/// the watcher whenever a file is read so the next classify() has the
/// pre-edit bytes available without an extra disk roundtrip.
pub fn record_file_bytes(snapshot: &mut Snapshot, path: PathBuf, bytes: Vec<u8>) {
    snapshot.file_bytes.insert(path, bytes);
}

cast::concept! {
    name: "live_state_handle",
    summary: "Daemon state handle. Wraps an ArcSwap<Snapshot> + a \
              broadcast channel; commit swaps the snapshot pointer \
              atomically and notifies subscribers.",
    anchors: [
        crate::state::LiveState,
        crate::state::from_loading,
        crate::state::from_initial,
        crate::state::commit,
        crate::state::current,
        crate::state::phase,
        crate::state::mark_ready,
        crate::state::mark_failed,
        crate::state::next_generation,
        crate::state::record_file_bytes,
    ],
    tags: ["cast_watch_state"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::resource_handle,
    concept: "live_state_handle",
    why: "Wraps mutable shared state behind ArcSwap + a tokio broadcast \
          channel; identity is the channel and the swap slot, not the \
          field shape.",
}

cast::concept! {
    name: "snapshot_value",
    summary: "Immutable snapshot of a single point-in-time analysis: \
              generation, Arc<Report>, file_bytes cache, stale_files \
              set. The unit of atomic swap.",
    anchors: [
        crate::state::Snapshot,
    ],
    tags: ["cast_watch_state"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::product_type,
    concept: "snapshot_value",
    why: "Struct with all four fields simultaneously inhabited.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::value_type,
    concept: "snapshot_value",
    why: "Cloneable, structurally compared via Arc shares; the snapshot \
          itself is immutable post-commit.",
}

cast::concept! {
    name: "phase_category",
    summary: "Daemon lifecycle phase: Loading, Ready, Failed.",
    anchors: [
        crate::state::Phase,
    ],
    tags: ["cast_watch_state"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::sum_type,
    concept: "phase_category",
    why: "Three-variant enum; exactly one phase inhabited at a time.",
}

cast::continues_in! {
    target: cast_stdlib::consistency::compare_and_swap,
    concept: "live_state_handle",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::architecture::typed_handle,
    concept: "snapshot_value",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::architecture::classifier_dispatch,
    concept: "phase_category",
    why: lazy,
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn generation_is_strictly_monotonic() {
        let state = from_initial(empty_report(), vec![], PathBuf::new());
        let g1 = next_generation(&state);
        let g2 = next_generation(&state);
        let g3 = next_generation(&state);
        assert!(g1 < g2 && g2 < g3);
    }

    #[test]
    fn commit_bumps_generation_even_for_equivalent_snapshot() {
        let state = from_initial(empty_report(), vec![], PathBuf::new());
        let g_before = current(&state).generation;
        commit(
            &state,
            Snapshot {
                generation: 0,
                report: Arc::new(empty_report()),
                file_bytes: HashMap::new(),
                stale_files: HashSet::new(),
            },
        );
        let g_after = current(&state).generation;
        assert!(g_after > g_before);
    }

    #[test]
    fn current_returns_arc_after_commit() {
        let state = from_initial(empty_report(), vec![], PathBuf::new());
        let a = current(&state);
        commit(
            &state,
            Snapshot {
                generation: 0,
                report: Arc::new(empty_report()),
                file_bytes: HashMap::new(),
                stale_files: HashSet::new(),
            },
        );
        let b = current(&state);
        assert!(a.generation < b.generation);
    }
}
