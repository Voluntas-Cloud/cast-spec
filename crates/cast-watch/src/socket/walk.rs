use super::protocol::ResponseBody;
use crate::state::Snapshot;

cast::rule! {
    rule: "Walk traverses an `io_continues_in` edge along two \
           paths: (1) the by-file index — if any concept is declared \
           in the target file, hop to those concepts (today: `.rs` \
           and `.cast` files the analyzer indexes); (2) the \
           by-io-target index — concepts that bridge into the same \
           foreign target file are neighbours, and walk steps across \
           the foreign file as a hub. `external` lang is the only \
           dead-end: by definition it can't anchor, so there's \
           nothing on the other side to land on. Each `IoBridgeHit` \
           records whether walk traversed it.",
    why:  "A `lang:`-based test is unreliable: the parser tags any \
           file-path target as 'External' regardless of the file's \
           actual language, so a Rust-target io_continues_in and a \
           Vue-target io_continues_in can both appear with \
           lang=External. The data-driven tests (does by-file \
           resolve? does by-io-target have other bridgers?) are \
           robust. The two paths cover both kinds of foreign-target \
           graph: 'foreign file with concepts declared at it via a \
           spec.cast' (by-file) and 'foreign file used as a shared \
           hub by sibling concepts' (by-io-target). True dead-ends \
           — `external` lang or a target nobody else bridges into \
           — stay marked `traversed: false` in `io_bridges` so the \
           caller can spot real cul-de-sacs.",
    governs: [
        crate::socket::handle_query_walk,
        crate::socket::walk_io_into_foreign_target,
    ],
    tags: ["cast_watch"],
}

cast::concept! {
    name: "walk_into_indexed_foreign_targets",
    summary: "Walk traverses `io_continues_in` into foreign-target \
              files via the `by_io_target` hub: any two concepts \
              that bridge into the same target are neighbours, and \
              walk steps across the foreign file the same way it \
              steps across a Rust target via `by_anchor`. \
              `walk_io_into_foreign_target` (called from \
              `handle_query_walk`) is the implementation site; \
              `external` lang is the only short-circuit because it \
              can't anchor. `IoBridgeHit` carries `traversed: bool` \
              so the caller can tell which bridges walk crossed \
              versus which dead-ended (true cul-de-sacs: external, \
              or targets with no other bridgers and no in-file \
              concept declarations). Composes with `jump` — walk \
              fans out across all foreign bridges of an allowed \
              kind; jump commits to a single named bridge.",
    anchors: [
        crate::socket::handle_query_walk,
        crate::socket::IoBridgeHit,
        crate::socket::walk_io_into_foreign_target,
        CAST::AS_PRIMITIVE::crate::socket::handle_jump,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  crate::socket::handle_jump,
    concept: "walk_into_indexed_foreign_targets",
    why:     "walk_into_indexed_foreign_targets continues into the single-edge primitive at handle_jump()",
}

cast::continues_in! {
    target:  cast_stdlib::storage::mutable_index_over_immutable_data::MutableIndexOverImmutableData,
    concept: "walk_into_indexed_foreign_targets",
    why:     "The `by_io_target` hub IS a mutable index over immutable \
              snapshot data: the snapshot's `io_continues_in` edges \
              are append-only per generation, but the secondary index \
              that lets walk pivot from a foreign target to every \
              concept bridging into it is rebuilt at commit time. \
              Walk costs an index probe instead of a full edge scan.",
    tags:    ["cast_watch"],
}

cast::concept! {
    name: "jump",
    summary: "Single-edge primitive for foreign-language bridges. \
              Where `walk` fans out across every `io_continues_in` \
              edge of every visited concept, `jump` picks one \
              specific bridge — caller names the source concept and \
              the target file (plus an optional `anchor:` to \
              disambiguate when multiple bridges share a target) — \
              and the response carries every other concept whose \
              `io_continues_in!` lands at the same target. Use when \
              walk's fan-out is too wide and the bridge of interest \
              is already known. Composes with walk: walk to find \
              candidate bridges, jump to commit to one. \
              `handle_jump` is the implementation; `Request::Jump` \
              and `ResponseBody::JumpResult` are the wire shapes.",
    anchors: [
        crate::socket::Request,
        crate::socket::ResponseBody,
        crate::socket::handle_jump,
    ],
    tags: ["cast_watch"],
}

cast::compare! {
    walk @ crate::socket::handle_query_walk: "Graph-shape, multi-edge, \
        hops-bounded. Answers 'starting from {C}, following {edge \
        kinds}, where can I reach within N hops?' — every edge of an \
        allowed kind from every visited concept fans into the next \
        frontier. Selectivity is statistical: bound the depth, choose \
        the edge kinds. Output is a connected sub-graph (visited set \
        + edges traversed).",
    jump @ crate::socket::handle_jump: "Single-edge, single-step. \
        Answers 'this concept has a bridge — follow exactly that one, \
        what's on the other side?' Selectivity is structural: the \
        request names the specific bridge to take. Output is the \
        landing — concepts/invocations at the bridge target — not a \
        sub-graph. Composes with walk: walk to find candidate \
        bridges; jump to commit to one.",
    tags: ["cast_watch"],
    note: "Walk handles every graph-shape traversal: `continues_in` \
           PLUS `io_continues_in` along two indices — by_file (Rust \
           targets indexed by the analyzer) and by_io_target \
           (foreign targets used as a hub by sibling bridgers). \
           Both tests are data-driven; `lang:` is never the \
           dispatch key because the parser tags any file-path \
           target as 'External'. Jump remains the right tool when \
           the caller already knows the specific bridge they want \
           to follow — its selectivity is structural, not \
           statistical. The only true dead-ends now are `external` \
           lang (can't anchor) and a target nobody else bridges \
           into; both surface as `IoBridgeHit { traversed: false }`.",
}

/// Find next-hop concepts via a foreign-language `io_continues_in!`
/// bridge. Two concepts that bridge into the same target file are
/// neighbours: walk treats the foreign file as a hub and steps across
/// it, the same way it steps across a Rust target via the `by_anchor`
/// index. `external` lang is skipped because it can't anchor — there's
/// nothing to land on.
///
/// `by_io_target` is keyed on the bridge target (a file path) and maps
/// to `(concept_name, lang)` pairs; `skip_concept` is the source we're
/// stepping out of (don't return self as a neighbour).
pub fn walk_io_into_foreign_target<'a>(
    by_io_target: &'a std::collections::HashMap<&'a str, Vec<(&'a str, Option<&'a str>)>>,
    target: &str,
    lang: Option<&str>,
    skip_concept: &str,
) -> Vec<&'a str> {
    if is_external_lang(lang) {
        return Vec::new();
    }
    let Some(neighbours) = by_io_target.get(target) else {
        return Vec::new();
    };
    neighbours
        .iter()
        .filter(|(name, neighbour_lang)| {
            *name != skip_concept && !is_external_lang(*neighbour_lang)
        })
        .map(|(name, _)| *name)
        .collect()
}

pub fn is_external_lang(lang: Option<&str>) -> bool {
    matches!(lang, Some(s) if s.eq_ignore_ascii_case("external"))
}

/// Single-bridge primitive for foreign-language continuations. Where
/// `walk` discovers ALL reachable neighbors via every io_continues_in
/// edge of every visited concept, `jump` picks one specific bridge
/// (source concept + target file [+ optional anchor]) and reports who
/// else lands at the same target. Use when walk's fan-out is too wide
/// and you've already identified the bridge you care about.
///
/// The "landing" is every other concept whose `io_continues_in!`
/// targets the same file. Same-target-different-anchor still lands —
/// disambiguate with the optional `anchor:` field on the request.
pub fn handle_jump(snap: &Snapshot, from: &str, target: &str, anchor: Option<&str>) -> ResponseBody {
    use cast::emit::model::EdgeKindRepr;
    let concepts = &snap.report.concept_graph.concepts;
    let bridge_lang = concepts.get(from).and_then(|c| {
        c.edges
            .iter()
            .find(|e| matches!(e.kind, EdgeKindRepr::IoContinuesIn) && e.target == target)
            .and_then(|e| e.lang.clone())
    });
    let mut landing: Vec<String> = Vec::new();
    for (name, c) in concepts {
        if name == from {
            continue;
        }
        let matches_target = c.edges.iter().any(|e| {
            matches!(e.kind, EdgeKindRepr::IoContinuesIn) && e.target == target
        });
        if matches_target {
            landing.push(name.clone());
        }
    }
    landing.sort();
    ResponseBody::JumpResult {
        from: from.to_string(),
        target: target.to_string(),
        lang: bridge_lang,
        anchor: anchor.map(String::from),
        landing,
    }
}

cast::concept! {
    name: "graph_walk_helpers",
    summary: "Pure graph-traversal helpers for the walk and jump \
              query kinds: find IO-bridge landings, classify external \
              langs, build a JumpResult.",
    anchors: [
        crate::socket::walk::walk_io_into_foreign_target,
        crate::socket::walk::is_external_lang,
        crate::socket::walk::handle_jump,
    ],
    tags: ["cast_watch_socket"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "graph_walk_helpers",
    why: "Read-only on the snapshot graph; output is a function of \
          the inputs alone.",
}
