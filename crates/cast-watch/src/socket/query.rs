use super::filter::{concept_matches, invocation_matches, path_matches};
use super::protocol::{Dim, IoBridgeHit, PathHit, QueryPredicate, ResponseBody, Stream, WalkEdge};
use super::walk::walk_io_into_foreign_target;
use crate::state::Snapshot;

cast::concept! {
    name: "query_language",
    summary: "A composable query surface for the snapshot with three \
              orthogonal axes. SELECTION filters the typed records \
              (invocations, concepts, rules, paths) by predicate. WALK \
              traverses the concept graph from a starting set along \
              typed edges (anchors, governs, target, instead_at, \
              pipeline flow), optionally crossing `continues_in!` and \
              `cast::io::continues_in!` bridges — the only verb that \
              traces a concept end-to-end through Rust → Kotlin/Vue/\
              YAML. FLASHLIGHT, after each hit, widens the response \
              with the incident neighborhood: `radius` (hops out from \
              the hit), `via` (which edge kinds to widen along — \
              independent of walk's `follow:`), and `dim` (per-node \
              brightness: name_only | summary | full). Today's bespoke \
              verbs (`unresolved_in_file`, `concepts_for_path`, \
              `rules_for_path`, `graph_for_tag`) are zero-hop \
              degenerate cases this language subsumes. Design forks \
              still open: per-result vs per-result-set flashlight; \
              `dim:` granularity (coarse three-step vs per-node-kind \
              map); `until:` as a stop predicate alongside `hops:`. \
              Lean today: per-result flashlight with optional dedup; \
              coarse `dim:`; both `hops:` and `until:` accepted, \
              default `hops: 1`.",
    anchors: [
        crate::socket::Request,
        crate::socket::ResponseBody,
        crate::socket::handle_query_select,
        crate::socket::handle_query_walk,
        crate::socket::handle_query_flashlight,
        crate::socket::handle_query_serialize,
        CAST::AS_PRIMITIVE::crate::socket::render_with_format,
        CAST::AS_PRIMITIVE::crate::socket::walk_io_into_foreign_target,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  crate::socket::render_with_format,
    concept: "query_language",
    why:     "query_language continues into wire-format negotiation at render_with_format()",
}

cast::continues_in! {
    target:  crate::socket::walk_io_into_foreign_target,
    concept: "query_language",
    why:     "query_language continues into foreign-target walk at walk_io_into_foreign_target()",
}

cast::continues_in! {
    target:  cast_stdlib::api::composable_query::ComposableQuery,
    concept: "query_language",
    why:     "Three orthogonal axes (select / walk / flashlight) the \
              caller composes per request. Adding a use case = a new \
              composition of existing axes, not a new endpoint. \
              Today's bespoke verbs (unresolved_in_file etc.) are \
              zero-hop degenerate cases of the same DSL.",
    tags:    ["cast_watch"],
}

cast::pipeline! {
    stages: {
        select     @ crate::socket::handle_query_select,
        walk       @ crate::socket::handle_query_walk,
        flashlight @ crate::socket::handle_query_flashlight,
        serialize  @ crate::socket::handle_query_serialize,
    },
    flow: [
        select     -> walk,
        walk       -> flashlight,
        flashlight -> serialize,
    ],
    tags: ["cast_watch"],
    note: "select produces the candidate set from the snapshot's typed \
           records; walk extends it asymmetrically along directed \
           edges (and may cross IO bridges, where serialize will tag \
           non-Rust hits with their `lang:`); flashlight widens each \
           hit symmetrically with its incident neighborhood per its \
           own radius/via/dim — independent edge set from walk's \
           follow; serialize emits the same `cast::emit::model` shapes \
           the rest of the daemon already returns (per the rule at \
           socket.rs that detail bodies reuse model types, not \
           pre-formatted strings). All four stage anchors are \
           unresolved-by-design until the impl lands — that is the \
           TODO list.",
}

cast::rule! {
    rule: "Walk and flashlight are separate primitives, not one knob. \
           Walk decides which nodes are *hits* — directed, edge-typed, \
           may cross `continues_in!` and `cast::io::continues_in!` \
           bridges. Flashlight decides what surrounds each hit in the \
           response — symmetric, edge-typed independently of walk, \
           with its own radius and brightness.",
    why:  "Conflating them — one `radius:` parameter that both \
           extends reachability and widens the response — forces the \
           caller to choose between 'reach further into the graph' and \
           'see more around what I found' when the LLM use case \
           routinely wants both at once with different edge sets per \
           axis. Asymmetric walk + symmetric flashlight is what makes \
           'walk continues_in.target only, light up governs around \
           each landing site' expressible in one query rather than two \
           round-trips with client-side join.",
    governs: [
        crate::socket::handle_query_walk,
        crate::socket::handle_query_flashlight,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "Flashlight has two dimensions, not one: `radius` (how many \
           hops out from a hit) AND `dim` (per-node brightness: \
           name_only | summary | full). Both must be expressible \
           independently — `radius: 3, dim: name_only` is a wide \
           overview; `radius: 1, dim: full` is a deep zoom; both are \
           valid LLM-context shapes.",
    why:  "The response shape itself is the budget for an LLM client. \
           A radius-only flashlight cannot trade coverage for detail — \
           `radius: 3` with full ConceptReports is ~10× the bytes of \
           `radius: 3` with name_only. Without a brightness knob the \
           caller has to choose between 'see far' and 'see clearly' \
           when both are legitimate questions; with `dim:`, the same \
           predicate returns either shape without the caller \
           rewriting the query.",
    governs: [
        crate::socket::handle_query_flashlight,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "`radius` is a fractional graph distance, not an integer \
           count. The integer part adds full rings of symmetric \
           incident neighbors; the fractional part adds an \
           alphabetically-first prefix of the next ring (`0.5` = \
           first half of ring 1 by concept name). The sort \
           tiebreaker MUST be deterministic so nested-radius \
           requests are strict subsets: `radius: 0.3` ⊂ `radius: \
           0.4` ⊂ `radius: 1.0` for the same seed, query, and \
           snapshot generation.",
    why:  "Without fractional radius, the only paging knob would be \
           a separate `limit:` field that's ring-blind — capable of \
           returning a stray hop-3 concept while skipping closer \
           ones, breaking the 'response is a coherent neighborhood' \
           guarantee that makes the graph view legible. With a \
           fractional radius, every response is still a connected \
           sub-graph in distance order: ring 0 (the seed), then ring \
           1 fully or in part, then ring 2 fully or in part, never \
           ring 3 before ring 2. The deterministic-prefix property \
           is what lets a caller stream a thread of incrementally \
           larger requests (`0.3 → 0.6 → 1.0 → 1.5`) and trust that \
           each response builds on the previous, never replaces it.",
    governs: [
        crate::socket::widen_concept_set,
    ],
    tags: ["cast_watch"],
}

cast::tier! {
    axis: flashlight_readiness,
    direction: increasing,
    of: crate::socket::handle_query_flashlight,
    stages: {
        designed: "Concept and rules pin the contract — `query_language` \
                   declares the four-stage pipeline; rules pin `walk vs \
                   flashlight` separation and the three-axis `radius` + \
                   `dim` + `via` shape. Reached.",
        wired @ crate::socket::handle_query_flashlight: "Pipeline stage \
                   exists and is reachable from `dispatch_request`. \
                   The hookpoint exists so radius/via implementations \
                   drop in without touching `dispatch_request`. \
                   Reached.",
        exposed @ crate::socket::Request: "Wire schema accepts `dim:`, \
                   `radius:`, and `via:` on `Walk` and `Query`. \
                   Callers can write the call sites the design \
                   prescribes; the daemon accepts them. Reached.",
        implemented: "Handler honors all three flashlight axes for \
                   the Concepts-stream: `dim` trims ConceptReport \
                   bodies (name_only / summary / full), `radius` \
                   widens each hit with N hops of symmetric incident \
                   neighbors, `via` filters which edge kinds widening \
                   follows. Order of operations is widen-then-trim so \
                   widened entries get the same brightness budget as \
                   seed hits. Reached.",
        complete @ crate::socket::widen_concept_set: "Handler also \
                   honors per-stream `Dim` for invocations and paths \
                   (today they pass through), plus a stop predicate \
                   `until:` alongside hops/radius for the \
                   `cast::concept!` `query_language` open fork. \
                   Future.",
    },
    tags: ["cast_watch"],
    note: "Each stage's anchor is the load-bearing item that proves \
           the stage is reached. Climbing the ladder is what makes a \
           pinned design actually reachable — the handler being a \
           no-op stops mattering once the schema accepts the field, \
           because callers can already exercise the contract; without \
           the schema fields, the design is invisible from the wire \
           regardless of how rich the rules are. exposed → \
           implemented is therefore a smaller step than wired → \
           exposed: once the field is published, any caller's request \
           is the test case.",
}


cast::rule! {
    rule: "`dim` (flashlight content selection) and `format` (wire \
           serialization) are orthogonal axes. They MUST be \
           expressible independently — a caller can ask for \
           `dim: name_only, format: json` (structured names list) or \
           `dim: full, format: yaml` (full data, YAML-encoded) and \
           every cell in the 2×3 matrix is a legitimate response \
           shape. Folding either axis into the other (e.g., a \
           `format: names_only` that bypasses `dim`) collapses the \
           matrix and forces callers to choose between content and \
           encoding when they should be choosing both.",
    why:  "Dim and format act at different stages — flashlight \
           (stage 3) picks fields; serialize (stage 4) encodes them. \
           Mixing them would either teach flashlight about wire \
           formats (it'd need to know JSON vs YAML to decide whether \
           anchors are 'cheap to keep') or teach serialize about \
           field selection (every renderer would need its own \
           field-trimming logic). Keeping them at separate stages \
           with their own request fields is what makes the matrix \
           expressible in one query rather than N round-trips.",
    governs: [
        crate::socket::handle_query_flashlight,
        crate::socket::render_with_format,
    ],
    tags: ["cast_watch"],
}

cast::tier! {
    axis: format_readiness,
    direction: increasing,
    of: crate::socket::render_with_format,
    stages: {
        designed: "Concept and compare blocks pin the contract — \
                   `output_format` declares the orthogonal axis to \
                   `dim`; `cast_emit_format` vs `socket_output_format` \
                   names the precedent and the divergence. Today.",
        wired @ crate::socket::render_with_format: "Pipeline gains a \
                   stage-4b render step reachable from \
                   `dispatch_request`. Today.",
        exposed @ crate::socket::Request: "Wire schema accepts \
                   `format:` on `Walk` and `Query`. Callers can ask \
                   for any supported format. Today.",
        implemented: "Handler honors `Json` (passthrough) and `Yaml` \
                   (serde_yaml). Today.",
        complete: "Handler honors at least one query-shaped text \
                   form (e.g. `Names`, `Terse`, `Markdown`) so \
                   pipe-line consumers and PR-comment consumers can \
                   skip parsing. Future.",
    },
    tags: ["cast_watch"],
    note: "Mirrors `flashlight_readiness`. Both ladders embody the \
           same insight: the wire schema is the publishing step, \
           separate from the implementation. With `format` published \
           and `Json | Yaml` implemented today, adding a new format \
           is a one-arm match against `OutputFormat` plus a renderer \
           — the dispatch graph and the schema do not move.",
}

cast::anti_pattern! {
    avoid:   "Pin a feature's contract via cast::concept! and \
              cast::rule!, wire a no-op handler into the dispatch \
              graph, but leave the request enum without the \
              parameters the design specifies. The contract is \
              documented and reachable in code yet un-callable \
              from the wire — every caller silently keeps getting \
              the unbounded default response and no one can pressure-\
              test the design.",
    why:     "cast-watch sat in this exact shape for `flashlight` — \
              the cast::concept! `query_language` block, the \
              cast::pipeline!, and the no-op handler all existed \
              but Request::Walk and Request::Query did not accept \
              `dim:` or `radius:`, so the contract was un-callable. \
              Worse, since the manual emission already advertised \
              the new fields, casual readers (and the LLMs we want \
              to call cast-watch) believed the feature was usable \
              and silently failed to discover it wasn't.",
    instead: "Publish the wire schema as soon as the design is \
              pinned, even when the handler is a no-op — that's \
              the `exposed` stage of the `flashlight_readiness` \
              ladder. Schema publication is its own step between \
              `wired` and `implemented`, not folded into either: \
              once the field is on the wire, callers can write the \
              call sites they want and any caller's request \
              becomes a test case for the design.",
    instead_at: crate::socket::Request,
    governs: [
        crate::socket::Request,
        crate::socket::handle_query_flashlight,
    ],
    tags: ["cast_watch"],
}

/// Records selected from the snapshot by `select` or reached by `walk`.
/// Internal to the query pipeline; not serialized directly — the
/// serialize stage projects it into the public `ResponseBody` shape.
pub enum SelectedRecords {
    Invocations(Vec<cast::emit::model::InvocationReport>),
    Concepts(std::collections::BTreeMap<String, cast::emit::model::ConceptReport>),
    Paths(Vec<PathHit>),
}

/// Walk traversal output. Carries enough metadata to round-trip into
/// `ResponseBody::WalkResult`.
pub struct WalkOutcome {
    pub starting: Vec<String>,
    pub follow: Vec<WalkEdge>,
    pub hops_requested: usize,
    pub hops_used: usize,
    pub visited: std::collections::BTreeMap<String, cast::emit::model::ConceptReport>,
    pub io_bridges: Vec<IoBridgeHit>,
}

/// Output of a query pipeline stage. Either branch produces the same
/// type so `flashlight` and `serialize` can be single fns shared
/// between Query and Walk requests.
pub enum QueryStage {
    Select {
        stream: Stream,
        records: SelectedRecords,
    },
    Walk(WalkOutcome),
}

/// Stage 1 (Query path): filter the snapshot's typed records by
/// `from:` (which stream) and an optional `where:` predicate. v1
/// supports the four streams listed on `Stream`. Records are *cloned*
/// out of the snapshot so the response body owns them across await
/// points, satisfying the `socket.rs:154` rule that detail bodies
/// reuse `cast::emit::model` shapes verbatim.
pub fn handle_query_select(
    snap: &Snapshot,
    stream: Stream,
    filter: Option<&QueryPredicate>,
) -> QueryStage {
    use cast::emit::model::AnnotationKind;
    let records = match stream {
        Stream::Invocations => {
            let mut hits = Vec::new();
            for invs in snap.report.groups.values() {
                for inv in invs {
                    if invocation_matches(inv, filter) {
                        hits.push(inv.clone());
                    }
                }
            }
            SelectedRecords::Invocations(hits)
        }
        Stream::Rules => {
            let mut hits = Vec::new();
            for invs in snap.report.groups.values() {
                for inv in invs {
                    if !matches!(inv.kind, AnnotationKind::Rule) {
                        continue;
                    }
                    if invocation_matches(inv, filter) {
                        hits.push(inv.clone());
                    }
                }
            }
            SelectedRecords::Invocations(hits)
        }
        Stream::Concepts => {
            let mut hits = std::collections::BTreeMap::new();
            for (name, c) in &snap.report.concept_graph.concepts {
                if concept_matches(c, filter) {
                    hits.insert(name.clone(), c.clone());
                }
            }
            SelectedRecords::Concepts(hits)
        }
        Stream::Paths => {
            let mut hits = Vec::new();
            for invs in snap.report.groups.values() {
                for inv in invs {
                    for p in &inv.paths {
                        if path_matches(inv, p, filter) {
                            hits.push(PathHit {
                                file: inv.file.clone(),
                                line: inv.line,
                                macro_path: inv.macro_path.clone(),
                                kind: inv.kind,
                                tags: inv.tags.clone(),
                                field: p.field.clone(),
                                text: p.text.clone(),
                                outcome: p.outcome,
                                error: p.error.clone(),
                            });
                        }
                    }
                }
            }
            SelectedRecords::Paths(hits)
        }
    };
    QueryStage::Select { stream, records }
}

/// Stage 2 (Walk path): traverse the concept graph from `from` along
/// the edge kinds in `follow` (empty = all). v1 traverses
/// `continues_in` (Rust target → concept-by-anchor lookup) and
/// records `io_continues_in` bridges without traversing further.
/// `hops` bounds depth — `hops: 0` returns just the starting set.
pub fn handle_query_walk(
    snap: &Snapshot,
    from: &[String],
    follow: &[WalkEdge],
    hops: usize,
) -> QueryStage {
    use cast::emit::model::EdgeKindRepr;
    use std::collections::{BTreeMap, HashMap, HashSet};

    let allowed: HashSet<EdgeKindRepr> = if follow.is_empty() {
        [EdgeKindRepr::ContinuesIn, EdgeKindRepr::IoContinuesIn]
            .into_iter()
            .collect()
    } else {
        follow
            .iter()
            .map(|e| match e {
                WalkEdge::ContinuesIn => EdgeKindRepr::ContinuesIn,
                WalkEdge::IoContinuesIn => EdgeKindRepr::IoContinuesIn,
            })
            .collect()
    };

    let concepts = &snap.report.concept_graph.concepts;
    let mut by_anchor: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut by_file: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut by_io_target: HashMap<&str, Vec<(&str, Option<&str>)>> = HashMap::new();
    for (name, c) in concepts {
        for d in &c.declarations {
            for a in &d.anchors {
                by_anchor.entry(a.path.as_str()).or_default().push(name.as_str());
            }
            by_file.entry(d.file.as_str()).or_default().push(name.as_str());
        }
        for e in &c.edges {
            if matches!(e.kind, EdgeKindRepr::IoContinuesIn) {
                by_io_target
                    .entry(e.target.as_str())
                    .or_default()
                    .push((name.as_str(), e.lang.as_deref()));
            }
        }
    }

    let mut visited: BTreeMap<String, cast::emit::model::ConceptReport> = BTreeMap::new();
    let mut io_bridges: Vec<IoBridgeHit> = Vec::new();
    let mut hops_used = 0usize;
    let mut frontier: Vec<String> = from.to_vec();

    for hop in 0..=hops {
        let mut next: Vec<String> = Vec::new();
        let mut added_this_hop = false;
        for name in &frontier {
            if visited.contains_key(name) {
                continue;
            }
            let Some(c) = concepts.get(name) else {
                continue;
            };
            visited.insert(name.clone(), c.clone());
            added_this_hop = true;
            if hop == hops {
                continue;
            }
            for e in &c.edges {
                if !allowed.contains(&e.kind) {
                    continue;
                }
                match e.kind {
                    EdgeKindRepr::ContinuesIn => {
                        if let Some(targets) = by_anchor.get(e.target.as_str()) {
                            for t in targets {
                                if !visited.contains_key(*t) {
                                    next.push((*t).to_string());
                                }
                            }
                        }
                    }
                    EdgeKindRepr::IoContinuesIn => {
                        // Two paths into a foreign-target file:
                        // - by_file: concept(s) declared at the target
                        //   (Rust source the analyzer indexes).
                        // - by_io_target: sibling concepts bridging
                        //   into the same target (foreign-file hub).
                        // `external` lang short-circuits the second
                        // path — it can't anchor.
                        let by_file_targets = by_file.get(e.target.as_str());
                        let foreign_neighbors: Vec<&str> = walk_io_into_foreign_target(
                            &by_io_target,
                            e.target.as_str(),
                            e.lang.as_deref(),
                            name,
                        );
                        let traversed = by_file_targets.is_some()
                            || !foreign_neighbors.is_empty();
                        io_bridges.push(IoBridgeHit {
                            from_concept: name.clone(),
                            target: e.target.clone(),
                            lang: e.lang.clone(),
                            traversed,
                        });
                        if let Some(targets) = by_file_targets {
                            for t in targets {
                                if !visited.contains_key(*t) {
                                    next.push((*t).to_string());
                                }
                            }
                        }
                        for t in foreign_neighbors {
                            if !visited.contains_key(t) {
                                next.push(t.to_string());
                            }
                        }
                    }
                }
            }
        }
        if added_this_hop && hop > 0 {
            hops_used = hop;
        }
        if hop >= hops {
            break;
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }

    QueryStage::Walk(WalkOutcome {
        starting: from.to_vec(),
        follow: follow.to_vec(),
        hops_requested: hops,
        hops_used,
        visited,
        io_bridges,
    })
}

/// Stage 3: widen each hit per `radius` (incident neighborhood) and
/// trim each result per `dim` (brightness). Order is widen-then-trim
/// so the widened entries get the same brightness as the seed hits;
/// applying dim first would force the widened entries to use the
/// full ConceptReport bodies and defeat the budget.
///
/// `radius` is fractional. Integer part = full rings of symmetric
/// incident neighbors. Fractional part = sorted-by-name prefix of
/// the next ring (`0.5` = first half). See the rule pinning
/// fractional ring-coherent semantics for the rationale.
///
/// Honors `dim` and `radius` for `Concepts`-bearing stages (Walk's
/// `visited` and `Query{from: concepts}`); flat streams
/// (`Invocations`, `Paths`) pass through unchanged in v1.
pub fn handle_query_flashlight(
    snap: &Snapshot,
    stage: QueryStage,
    dim: Dim,
    radius: f64,
    via: &[WalkEdge],
) -> QueryStage {
    let stage = if radius > 0.0 {
        widen_with_radius(snap, stage, radius, via)
    } else {
        stage
    };
    apply_dim(stage, dim)
}

fn apply_dim(stage: QueryStage, dim: Dim) -> QueryStage {
    if matches!(dim, Dim::Full) {
        return stage;
    }
    match stage {
        QueryStage::Select { stream, records } => QueryStage::Select {
            stream,
            records: trim_records(records, dim),
        },
        QueryStage::Walk(mut w) => {
            for c in w.visited.values_mut() {
                trim_concept(c, dim);
            }
            QueryStage::Walk(w)
        }
    }
}

fn widen_with_radius(
    snap: &Snapshot,
    stage: QueryStage,
    radius: f64,
    via: &[WalkEdge],
) -> QueryStage {
    match stage {
        QueryStage::Select { stream, records } => match records {
            SelectedRecords::Concepts(seed) => {
                let widened = widen_concept_set(snap, seed, radius, via);
                QueryStage::Select {
                    stream,
                    records: SelectedRecords::Concepts(widened),
                }
            }
            other => QueryStage::Select { stream, records: other },
        },
        QueryStage::Walk(mut w) => {
            w.visited = widen_concept_set(snap, w.visited, radius, via);
            QueryStage::Walk(w)
        }
    }
}

/// Symmetric BFS from each concept in `visited`. At each hop, add
/// every neighbor reachable via an outgoing edge (concept's own
/// edges) OR an incoming edge (another concept whose edges target
/// THIS concept's anchors/files), filtered by the `via` edge-kind
/// set. New entries are merged into `visited` with their full
/// `ConceptReport`; a later `apply_dim` pass trims them.
///
/// Fractional radius: `floor(radius)` full rings are added, then
/// the next ring is computed but only the alphabetically-first
/// `ceil(ring_size * fract)` items are kept. The sort tiebreaker
/// is concept name — see the rule pinning ring-coherent semantics.
pub fn widen_concept_set(
    snap: &Snapshot,
    mut visited: std::collections::BTreeMap<String, cast::emit::model::ConceptReport>,
    radius: f64,
    via: &[WalkEdge],
) -> std::collections::BTreeMap<String, cast::emit::model::ConceptReport> {
    use cast::emit::model::EdgeKindRepr;
    use std::collections::{HashMap, HashSet};

    let allowed: HashSet<EdgeKindRepr> = if via.is_empty() {
        [EdgeKindRepr::ContinuesIn, EdgeKindRepr::IoContinuesIn]
            .into_iter()
            .collect()
    } else {
        via.iter()
            .map(|e| match e {
                WalkEdge::ContinuesIn => EdgeKindRepr::ContinuesIn,
                WalkEdge::IoContinuesIn => EdgeKindRepr::IoContinuesIn,
            })
            .collect()
    };

    let concepts = &snap.report.concept_graph.concepts;

    // Outgoing index: anchor → concepts owning that anchor;
    //                 file → concepts declared in that file.
    let mut by_anchor: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut by_file: HashMap<&str, Vec<&str>> = HashMap::new();
    for (name, c) in concepts {
        for d in &c.declarations {
            for a in &d.anchors {
                by_anchor.entry(a.path.as_str()).or_default().push(name.as_str());
            }
            by_file.entry(d.file.as_str()).or_default().push(name.as_str());
        }
    }

    // Incoming index: for every concept, which other concepts have
    // edges (filtered to `allowed` kinds) pointing INTO this concept's
    // anchors/files. Built once across all concepts so each BFS step
    // costs O(frontier × small per-concept lookup).
    let mut incoming: HashMap<&str, Vec<&str>> = HashMap::new();
    for (other_name, other) in concepts {
        for e in &other.edges {
            if !allowed.contains(&e.kind) {
                continue;
            }
            let landings: Option<&Vec<&str>> = match e.kind {
                EdgeKindRepr::ContinuesIn => by_anchor.get(e.target.as_str()),
                EdgeKindRepr::IoContinuesIn => by_file.get(e.target.as_str()),
            };
            if let Some(targets) = landings {
                for t in targets {
                    incoming
                        .entry(t)
                        .or_default()
                        .push(other_name.as_str());
                }
            }
        }
    }

    // Compute one ring's worth of neighbors from `frontier`, given
    // current `visited`. Returns alphabetically-sorted names so the
    // fractional-radius prefix is deterministic.
    let compute_ring = |frontier: &[String],
                        visited: &std::collections::BTreeMap<
                            String,
                            cast::emit::model::ConceptReport,
                        >|
     -> Vec<String> {
        let mut next: HashSet<String> = HashSet::new();
        for name in frontier {
            if let Some(c) = concepts.get(name) {
                for e in &c.edges {
                    if !allowed.contains(&e.kind) {
                        continue;
                    }
                    let landings: Option<&Vec<&str>> = match e.kind {
                        EdgeKindRepr::ContinuesIn => by_anchor.get(e.target.as_str()),
                        EdgeKindRepr::IoContinuesIn => by_file.get(e.target.as_str()),
                    };
                    if let Some(ts) = landings {
                        for t in ts {
                            if !visited.contains_key(*t) {
                                next.insert((*t).to_string());
                            }
                        }
                    }
                }
            }
            if let Some(srcs) = incoming.get(name.as_str()) {
                for src in srcs {
                    if !visited.contains_key(*src) {
                        next.insert((*src).to_string());
                    }
                }
            }
        }
        let mut sorted: Vec<String> = next.into_iter().collect();
        sorted.sort();
        sorted
    };

    let full_hops = radius.floor() as usize;
    let frac = radius.fract();
    let mut frontier: Vec<String> = {
        let mut s: Vec<String> = visited.keys().cloned().collect();
        s.sort();
        s
    };

    // Phase 1: `full_hops` complete rings.
    for _ in 0..full_hops {
        let ring = compute_ring(&frontier, &visited);
        if ring.is_empty() {
            return visited;
        }
        for name in &ring {
            if let Some(c) = concepts.get(name) {
                visited.insert(name.clone(), c.clone());
            }
        }
        frontier = ring;
    }

    // Phase 2: fractional prefix of the next ring. The ring is sorted
    // alphabetically by concept name so `0.3` and `0.4` are nested —
    // the smaller is always a subset of the larger.
    if frac > 0.0 {
        let ring = compute_ring(&frontier, &visited);
        if !ring.is_empty() {
            let take = ((ring.len() as f64) * frac).ceil() as usize;
            let take = take.min(ring.len());
            for name in ring.iter().take(take) {
                if let Some(c) = concepts.get(name) {
                    visited.insert(name.clone(), c.clone());
                }
            }
        }
    }

    visited
}

fn trim_records(records: SelectedRecords, dim: Dim) -> SelectedRecords {
    match records {
        SelectedRecords::Concepts(mut m) => {
            for c in m.values_mut() {
                trim_concept(c, dim);
            }
            SelectedRecords::Concepts(m)
        }
        // Invocations and Paths are flat record types; per-stream
        // `Dim` mappings (drop `paths[]` / `pipeline[]` from
        // invocations, drop `error` on resolved paths, etc.) land in a
        // follow-up rather than guessing the projection now.
        other => other,
    }
}

pub fn trim_concept(c: &mut cast::emit::model::ConceptReport, dim: Dim) {
    match dim {
        Dim::Full => {}
        Dim::NameOnly => {
            c.declarations.clear();
            c.edges.clear();
        }
        Dim::Summary => {
            // Keep the first declaration with its summary line so a
            // human reader still has context, but drop anchor lists
            // and edges — those are the structural axis, surfaced
            // separately by walk traversal when actually needed.
            if c.declarations.len() > 1 {
                c.declarations.truncate(1);
            }
            for d in &mut c.declarations {
                d.anchors.clear();
            }
            c.edges.clear();
        }
    }
}

/// Stage 4: project the pipeline output into the public response
/// body. The serialize stage is what guarantees the wire shape obeys
/// the `socket.rs:154` rule — any data shape massaging happens here,
/// at one well-known seam, rather than scattered through the earlier
/// stages.
pub fn handle_query_serialize(stage: QueryStage) -> ResponseBody {
    match stage {
        QueryStage::Select { stream, records } => match records {
            SelectedRecords::Invocations(hits) => ResponseBody::QueryResult {
                stream,
                count: hits.len(),
                invocations: Some(hits),
                concepts: None,
                paths: None,
            },
            SelectedRecords::Concepts(hits) => ResponseBody::QueryResult {
                stream,
                count: hits.len(),
                invocations: None,
                concepts: Some(hits),
                paths: None,
            },
            SelectedRecords::Paths(hits) => ResponseBody::QueryResult {
                stream,
                count: hits.len(),
                invocations: None,
                concepts: None,
                paths: Some(hits),
            },
        },
        QueryStage::Walk(w) => ResponseBody::WalkResult {
            starting: w.starting,
            follow: w.follow,
            hops_requested: w.hops_requested,
            hops_used: w.hops_used,
            visited: w.visited,
            io_bridges: w.io_bridges,
        },
    }
}

cast::concept! {
    name: "query_pipeline_handlers",
    summary: "Pure handlers for the generic `query` request shape: \
              select-records, walk-graph, flashlight, dim/radius \
              widening, serialise. Each takes Snapshot + request \
              args, returns ResponseBody (or an intermediate stage).",
    anchors: [
        crate::socket::query::handle_query_select,
        crate::socket::query::handle_query_walk,
        crate::socket::query::handle_query_flashlight,
        crate::socket::query::handle_query_serialize,
        crate::socket::query::widen_concept_set,
        crate::socket::query::trim_concept,
        crate::socket::query::apply_dim,
        crate::socket::query::widen_with_radius,
        crate::socket::query::trim_records,
    ],
    tags: ["cast_watch_socket"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "query_pipeline_handlers",
    why: "Output is a function of (&Snapshot, args) alone; no I/O, no \
          mutation, no time/RNG.",
}

cast::concept! {
    name: "query_pipeline_stages",
    summary: "Sum-typed intermediate stages of the query pipeline: \
              SelectedRecords (post-filter), QueryStage (post-widen), \
              and the WalkOutcome record carried along.",
    anchors: [
        crate::socket::query::SelectedRecords,
        crate::socket::query::QueryStage,
        crate::socket::query::WalkOutcome,
    ],
    tags: ["cast_watch_socket"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::sum_type,
    concept: "query_pipeline_stages",
    why: "SelectedRecords and QueryStage are enums; WalkOutcome is a \
          struct that flows through the pipeline.",
}

cast::continues_in! {
    target: cast_stdlib::messaging::request_response,
    concept: "query_pipeline_handlers",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::architecture::classifier_dispatch,
    concept: "query_pipeline_stages",
    why: lazy,
}
