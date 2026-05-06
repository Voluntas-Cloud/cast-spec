use std::sync::Arc;

use super::help_data::{help_body, manual_body};
use super::protocol::{Broadcast, BroadcastKind, Request, Response, ResponseBody};
use super::query::{
    handle_query_flashlight, handle_query_select, handle_query_serialize, handle_query_walk,
};
use super::render_with_format;
use super::walk::handle_jump;
use crate::state::{self, LiveState, Snapshot};

cast::concept! {
    name: "query_dispatch",
    summary: "Pure dispatch surface — Request variant in, Response out. \
              No I/O happens at this layer; each match arm calls a \
              handle_* function with the current Snapshot. The set of \
              match arms is the audit point for what queries cast-\
              watch supports — adding a query means adding an arm here.",
    anchors: [
        crate::socket::Request,
        crate::socket::dispatch_request,
        CAST::AS_PRIMITIVE::crate::socket::handle_query_select,
        CAST::AS_PRIMITIVE::crate::state::commit,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  crate::socket::handle_query_select,
    concept: "query_dispatch",
    why:     "query_dispatch continues into the query language at handle_query_select",
}

cast::continues_in! {
    target:  crate::state::commit,
    concept: "query_dispatch",
    why:     "query_dispatch::handle_rebuild crosses the write_boundary — commits + broadcasts",
}

cast::continues_in! {
    target:  cast_stdlib::architecture::classifier_dispatch::ClassifierDispatch,
    concept: "query_dispatch",
    why:     "`dispatch_request` is exactly classifier_dispatch: the \
              `Request` enum is the classifier (each variant is one \
              category), the per-variant `handle_*` calls are the \
              handlers. Keeping the classifier visible as a single \
              `match` keeps 'why did this request land in branch X?' \
              answerable as a standalone question.",
    tags:    ["cast_watch"],
}

cast::rule! {
    rule: "The query surface must let consumers (humans and LLMs) \
           express the question they actually have, without first \
           translating it into the daemon's internal vocabulary. \
           When a verb on Request requires the caller to know an \
           internal type name, an indexing strategy, or a stage of \
           the pipeline, that verb is wrong-shaped — refactor it \
           toward the composable query_language surface (select / \
           walk / flashlight / serialize) before adding more \
           bespoke verbs.",
    why:  "Bespoke verbs (`unresolved_in_file`, `concepts_for_path`, \
           `rules_for_path`, `graph_for_tag`, …) accreted because \
           each new use case asked for one. They share their data \
           sources but not their grammar — consumers learn nine \
           verbs to ask one underlying question. The composable \
           grammar collapses that surface into one verb with three \
           orthogonal axes (selection, walk, flashlight), so adding \
           a use case becomes a parameter combination, not a new \
           Request variant.",
    governs: [
        crate::socket::Request,
        crate::socket::ResponseBody,
        crate::socket::dispatch_request,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "Response shapes carry enough context to be self-describing \
           — a consumer reading a response should not need a \
           companion `manual` or `help` request to interpret what's \
           on the wire. Concept names, edge kinds, and warning kinds \
           are spelled out; record bodies are reusable model types \
           the consumer can re-feed into queries.",
    why:  "A response that needs a glossary is a response that fails \
           consumer_fluency. The cost of fatter responses is bytes; \
           the cost of opaque responses is round-trips. Bytes are \
           cheaper than turns.",
    governs: [
        crate::socket::ResponseBody,
        crate::socket::manual_body,
        crate::socket::help_body,
    ],
    tags: ["cast_watch"],
}

/// Handle a `Rebuild` request: drops the existing rust-analyzer state,
/// reloads the project, and re-runs the per-handle pipeline.
///
/// Async + `spawn_blocking` because the load is CPU-heavy and
/// synchronous; running it inline on a `current_thread` runtime would
/// freeze every other socket handler for the duration of the rebuild
/// (~30s on Voluntas-sized trees). Wrapping it sends the work to the
/// blocking pool and lets the accept loop and other connections keep
/// flowing.
///
/// Emits `RebuildStarted` as soon as the request is accepted so any
/// subscribed client knows their other queries are about to wait — and
/// `RebuildCompleted` once the new snapshot has committed.
///
/// This is the single entry point through which a client can pay the
/// RA-reload cost — keeping it concentrated here makes the cost easy
/// to audit (anyone calling cast::run_multi_root_analysis from
/// elsewhere is violating `full_reload_is_opt_in`).
pub async fn handle_rebuild(state: &Arc<LiveState>) -> anyhow::Result<Response> {
    let prior_gen = state::current(state).generation;
    let _ = state.broadcast.send(Broadcast {
        snapshot_generation: prior_gen,
        kind: BroadcastKind::RebuildStarted,
    });

    let roots = state.roots.clone();
    let repo_root = state.repo_root.clone();
    let report = tokio::task::spawn_blocking(move || -> anyhow::Result<cast::Report> {
        let multi = cast::load_projects(&roots)?;
        let report = cast::run_multi_root_analysis(&multi, &repo_root);
        drop(multi);
        Ok(report)
    })
    .await??;

    let paths_resolved = report.summary.paths_resolved as u64;
    let paths_unresolved = report.summary.paths_unresolved as u64;
    let prior = state::current(state);
    let next = Snapshot {
        generation: 0,
        report: Arc::new(report),
        file_bytes: prior.file_bytes.clone(),
        stale_files: std::collections::HashSet::new(),
    };
    state::commit(state, next);
    let new_gen = state::current(state).generation;
    let _ = state.broadcast.send(Broadcast {
        snapshot_generation: new_gen,
        kind: BroadcastKind::RebuildCompleted,
    });
    Ok(Response {
        snapshot_generation: new_gen,
        body: ResponseBody::Rebuilt {
            paths_resolved,
            paths_unresolved,
        },
    })
}

/// Pure dispatch: Request → Response. No I/O at this layer (except
/// `Rebuild`, which is documented as the one heavy entry point — see
/// `handle_rebuild`).
///
/// `Help` and `Status` are always serviced — they're the introspection
/// queries a client uses to find its bearings. Every other query is
/// gated on `Phase::Ready`; during `Loading` or after `Failed` they
/// return an `Error` body pointing the client at `Status`.
///
/// The match arms are the audit point for cast-watch's query surface.
pub async fn dispatch_request(state: &Arc<LiveState>, request: Request) -> Response {
    let snap = state::current(state);
    let gen = snap.generation;

    if matches!(request, Request::Help) {
        return Response {
            snapshot_generation: gen,
            body: help_body(),
        };
    }
    if matches!(request, Request::Status) {
        return Response {
            snapshot_generation: gen,
            body: status_body(state, &snap),
        };
    }
    if matches!(request, Request::Manual) {
        return Response {
            snapshot_generation: gen,
            body: manual_body(),
        };
    }

    let phase = state::phase(state);
    if !matches!(phase, state::Phase::Ready) {
        let message = match phase {
            state::Phase::Loading => "daemon still loading initial analysis; query 'status' to poll".to_string(),
            state::Phase::Failed(e) => format!("daemon failed initial load: {e}; query 'status' for details"),
            state::Phase::Ready => unreachable!(),
        };
        return Response {
            snapshot_generation: gen,
            body: ResponseBody::Error { message },
        };
    }

    let body = match request {
        Request::Help | Request::Status | Request::Manual => unreachable!(),
        Request::Snapshot => snapshot_body(&snap),
        Request::UnresolvedInFile { path } => unresolved_in_file_body(&snap, &path),
        Request::ConceptsForPath { path } => concepts_for_path_body(&snap, &path),
        Request::RulesForPath { path } => rules_for_path_body(&snap, &path),
        Request::GraphForTag { tag } => graph_for_tag_body(&snap, &tag),
        Request::Rebuild => match handle_rebuild(state).await {
            Ok(r) => return r,
            Err(e) => ResponseBody::Error {
                message: e.to_string(),
            },
        },
        Request::Subscribe => ResponseBody::Subscribed,
        Request::Query { from, filter, dim, radius, via, format } => {
            let stage = handle_query_select(&snap, from, filter.as_ref());
            let stage = handle_query_flashlight(&snap, stage, dim, radius, &via);
            render_with_format(handle_query_serialize(stage), format)
        }
        Request::Walk { from, follow, hops, dim, radius, via, format } => {
            let stage = handle_query_walk(&snap, &from, &follow, hops);
            let stage = handle_query_flashlight(&snap, stage, dim, radius, &via);
            render_with_format(handle_query_serialize(stage), format)
        }
        Request::Tree => tree_body(&snap),
        Request::TreeExpand { concept } => tree_expand_body(&snap, &concept),
        Request::Jump { from, target, anchor } => {
            handle_jump(&snap, &from, &target, anchor.as_deref())
        }
    };
    Response {
        snapshot_generation: gen,
        body,
    }
}

pub fn status_body(state: &LiveState, snap: &Snapshot) -> ResponseBody {
    let phase = state::phase(state);
    let (phase_str, error, summary, snapshot_generation) = match &phase {
        state::Phase::Loading => ("loading".to_string(), None, None, None),
        state::Phase::Ready => (
            "ready".to_string(),
            None,
            Some(snap.report.summary.clone()),
            Some(snap.generation),
        ),
        state::Phase::Failed(e) => ("failed".to_string(), Some(e.clone()), None, None),
    };
    ResponseBody::Status {
        phase: phase_str,
        uptime_seconds: state.started_at.elapsed().as_secs_f64(),
        roots: state.roots.iter().map(|p| p.display().to_string()).collect(),
        repo_root: state.repo_root.display().to_string(),
        snapshot_generation,
        summary,
        error,
    }
}


pub fn snapshot_body(snap: &Snapshot) -> ResponseBody {
    let summary = snap.report.summary.clone();
    let stale_files = snap
        .stale_files
        .iter()
        .map(|p| p.display().to_string())
        .collect();
    ResponseBody::Snapshot {
        summary,
        stale_files,
    }
}

pub fn unresolved_in_file_body(snap: &Snapshot, path: &std::path::Path) -> ResponseBody {
    let target = path.to_string_lossy();
    let mut entries = Vec::new();
    for invs in snap.report.groups.values() {
        for inv in invs {
            if inv.file != target {
                continue;
            }
            for p in &inv.paths {
                if matches!(
                    p.outcome,
                    cast::emit::model::PathOutcomeRepr::Unresolved
                ) {
                    entries.push(format!("{}:{} {} = {}", inv.file, inv.line, p.field, p.text));
                }
            }
        }
    }
    ResponseBody::UnresolvedInFile { entries }
}

pub fn concepts_for_path_body(snap: &Snapshot, path: &str) -> ResponseBody {
    let mut concepts = Vec::new();
    for (name, c) in &snap.report.concept_graph.concepts {
        let mentions = c
            .declarations
            .iter()
            .any(|d| d.anchors.iter().any(|a| a.path == path));
        if mentions {
            concepts.push(name.clone());
        }
    }
    ResponseBody::ConceptsForPath { concepts }
}

pub fn rules_for_path_body(snap: &Snapshot, path: &str) -> ResponseBody {
    let mut rules = Vec::new();
    for invs in snap.report.groups.values() {
        for inv in invs {
            if !matches!(inv.kind, cast::emit::model::AnnotationKind::Rule) {
                continue;
            }
            if inv.paths.iter().any(|p| p.text == path) {
                rules.push(format!("{}:{}", inv.file, inv.line));
            }
        }
    }
    ResponseBody::RulesForPath { rules }
}

pub fn graph_for_tag_body(snap: &Snapshot, tag: &str) -> ResponseBody {
    let files_in_tag: std::collections::HashSet<&str> = snap
        .report
        .groups
        .get(tag)
        .into_iter()
        .flatten()
        .map(|inv| inv.file.as_str())
        .collect();
    let graph: std::collections::BTreeMap<String, cast::emit::model::ConceptReport> = snap
        .report
        .concept_graph
        .concepts
        .iter()
        .filter(|(_, c)| {
            c.declarations
                .iter()
                .any(|d| files_in_tag.contains(d.file.as_str()))
        })
        .map(|(n, c)| (n.clone(), c.clone()))
        .collect();
    ResponseBody::GraphForTag { graph }
}

pub fn tree_body(snap: &Snapshot) -> ResponseBody {
    let tree = cast::tree::build(&snap.report);
    ResponseBody::Tree { tree }
}

pub fn tree_expand_body(snap: &Snapshot, concept: &str) -> ResponseBody {
    // Find every non-concept invocation attributed to `concept` via the
    // same per-anchor longest-prefix-match rule the tree builder uses.
    // For v1 we hand back the existing `InvocationReport` shape — the
    // renderer can show kind/file/line/tags/paths now and we'll thread
    // through richer prose fields (`rule:`, `why:`, etc.) in a follow-up.
    let children = cast::tree::expand(&snap.report, concept);
    ResponseBody::TreeExpandResult {
        concept: concept.to_string(),
        children,
    }
}

cast::concept! {
    name: "request_handlers",
    summary: "Pure body-builders for each request kind. Take a \
              Snapshot reference and request arguments, return a \
              ResponseBody. No I/O, no mutation of inputs.",
    anchors: [
        crate::socket::dispatch::status_body,
        crate::socket::dispatch::snapshot_body,
        crate::socket::dispatch::unresolved_in_file_body,
        crate::socket::dispatch::concepts_for_path_body,
        crate::socket::dispatch::rules_for_path_body,
        crate::socket::dispatch::graph_for_tag_body,
        crate::socket::dispatch::tree_body,
        crate::socket::dispatch::tree_expand_body,
    ],
    tags: ["cast_watch_socket"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "request_handlers",
    why: "Output is a function of (&Snapshot, args) alone; no I/O, no \
          mutation, no time/RNG.",
}

cast::continues_in! {
    target: cast_stdlib::messaging::request_response,
    concept: "request_handlers",
    why: lazy,
}
