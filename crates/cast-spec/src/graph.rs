//! Concept graph — joins annotations across files by their `concept:`
//! string.
//!
//! Three kinds of input contribute:
//!
//! - `cast::concept!` declarations: provide a node with a summary plus a
//!   set of declared anchor paths (the canonical participants).
//! - `cast::continues_in!` invocations: one edge each, target is a Rust
//!   path on a sibling concept anchor.
//! - `cast::io::continues_in!` invocations: one edge each, target is an
//!   out-of-Rust resource (Kotlin file, RFC, URL, ...).
//!
//! The graph emits three categories of warning (per `GRAMMAR.md` §5):
//!
//! - **Orphan** — concept appears exactly once across the entire program.
//! - **Undeclared** — edges exist for the concept but no `concept!`
//!   declaration. Acceptable but worth flagging.
//! - **TargetNotInAnchors** — a `continues_in!` edge points at a Rust
//!   path that the matching `concept!` declaration didn't anchor. The
//!   typical cause is a renamed item; sometimes intentional (the new
//!   anchor wasn't added to `concept!` yet).
//!
//! `io::continues_in!` edges never trigger TargetNotInAnchors — their
//! targets are external strings, not Rust paths.

use crate::model::Location;
use crate::parser::common::WhyValue;
use crate::parser::io_continues_in::Lang;
use crate::parser::CastAnnotation;
use crate::validator::paths::syn_path_to_string;
use std::collections::BTreeMap;

// ─── concept aggregation by name — the property that makes multi-root work
//
// build_graph keys ConceptNode entries by the concept's `name:` string.
// That means N declarations of the same name from N different source
// files (whether they live in the same Cargo workspace or not) collapse
// into one node, with their declarations and edges merged. No special-
// case logic is needed for cross-workspace concepts — the same code path
// that handles "two declarations of `pki_trust_root` in the same
// workspace" also handles "three declarations of `pki_trust_root` across
// three workspaces." This is why multi-root support reduces to: load
// each workspace, run the pipeline, hand all (loc, ann) pairs to one
// build_graph call.

cast::rule! {
    rule: "ConceptNode entries are keyed by the concept's `name` \
           string only. Declarations and edges with matching names \
           are merged regardless of which file or workspace they came \
           from. Two unrelated concepts must NEVER share a name — \
           name collision silently merges their graphs.",
    why:  "This naming-as-identity rule is what lets cast-extract \
           model cross-workspace concepts without any special path \
           machinery. Each workspace contributes its slice; \
           build_graph stitches them. The cost is that name collisions \
           are not detectable here — they look identical to intentional \
           cross-workspace participation. Concept names are \
           therefore a small shared namespace and should be chosen \
           carefully (the `tag:` field is the conventional way to \
           keep related concepts grouped without colliding names).",
    governs: [
        crate::graph::build_graph,
        crate::graph::ConceptNode,
        crate::graph::ConceptGraph,
    ],
    tags: ["cast_extract_pipeline"],
}

#[derive(Debug, Clone)]
pub struct ConceptGraph {
    pub concepts: BTreeMap<String, ConceptNode>,
}

#[derive(Debug, Clone, Default)]
pub struct ConceptNode {
    pub declarations: Vec<DeclarationRef>,
    pub edges: Vec<ConceptEdge>,
}

#[derive(Debug, Clone)]
pub struct AnchorRef {
    pub path: String,
    pub role: crate::parser::concept::AnchorRole,
}

#[derive(Debug, Clone)]
pub struct DeclarationRef {
    pub location: Location,
    pub summary: String,
    pub anchors: Vec<AnchorRef>,
    /// `tags:` (or legacy single `tag:`) on the originating
    /// `cast::concept!` block. Carried through so concept-stream
    /// queries can filter on tags without re-traversing the
    /// invocation index. Empty Vec = no tags.
    pub tags: Vec<String>,
    /// Optional explicit parent concept name from `parent:` on the
    /// `cast::concept!` block. Read by the tree placement engine as
    /// a hard parent override; `None` means the strict-prefix-on-
    /// anchors rule applies as usual.
    pub parent: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConceptEdge {
    pub kind: EdgeKind,
    pub location: Location,
    pub target: EdgeTarget,
    /// `why:` field as it was written in source — `Some(Prose { text })`,
    /// `Some(Lazy)` for the deferred-explanation marker, or `None` if
    /// the writer omitted the field entirely.
    pub why: Option<WhyValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    ContinuesIn,
    IoContinuesIn,
}

#[derive(Debug, Clone)]
pub enum EdgeTarget {
    Rust(String),
    External { target: String, lang: Lang },
}

#[derive(Debug, Clone)]
pub struct GraphWarning {
    pub concept: String,
    pub kind: WarningKind,
}

#[derive(Debug, Clone)]
pub enum WarningKind {
    /// Concept appears in only one annotation total.
    Orphan,
    /// Edges exist for this concept but no `concept!` declared it.
    Undeclared,
    /// A `continues_in!` edge's Rust target isn't among the declared anchors.
    TargetNotInAnchors {
        location: Location,
        target: String,
    },
    /// `parent:` field on a `cast::concept!` block names a concept
    /// that doesn't exist in the graph. Tree placement falls back to
    /// the strict-prefix-on-anchors rule when this fires.
    UnresolvedParent {
        location: Location,
        parent: String,
    },
    /// A non-umbrella concept has no `continues_in!` edge to a
    /// `cast_stdlib::*` target. Stdlib grounding is the convention
    /// for tying a local concept to a shared primitive; without it
    /// the concept rests only on its own anchors and can't be
    /// matched against other concepts that use the same primitive.
    /// Umbrella-shaped concepts (≥2 embodied anchors at module
    /// depth ≤ 2) are exempt — they're per-crate roots, not leaves.
    MissingStdlibGrounding,
}

impl std::fmt::Display for WarningKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WarningKind::Orphan => write!(f, "orphan (only one reference)"),
            WarningKind::Undeclared => write!(f, "no concept! declaration"),
            WarningKind::TargetNotInAnchors { location, target } => write!(
                f,
                "target `{target}` at {}:{} not in declared anchors",
                location.file.display(),
                location.line
            ),
            WarningKind::UnresolvedParent { location, parent } => write!(
                f,
                "parent `{parent}` at {}:{} doesn't name an existing concept",
                location.file.display(),
                location.line
            ),
            WarningKind::MissingStdlibGrounding => write!(
                f,
                "no continues_in! to a cast_stdlib::* primitive"
            ),
        }
    }
}

/// Build the concept graph from a sequence of (location, annotation)
/// pairs. The location identifies *where the annotation appeared* — used
/// to attach edges and declarations to source positions.
///
/// Concepts are keyed by `name` alone EXCEPT when the same name is
/// declared with two or more different primary tags (`tags[0]`). In
/// that case the colliding declarations are namespaced by primary tag
/// — `consistency` declared with `tags[0]="cast_stdlib"` and
/// `consistency` declared with `tags[0]="cast_os_stdlib"` become two
/// distinct nodes keyed `consistency@cast_stdlib` and
/// `consistency@cast_os_stdlib`. Singletons keep their bare name so
/// cross-workspace participation (the deliberate same-name merge
/// pattern) still works for non-colliding cases.
pub fn build_graph<'a>(
    annotations: impl IntoIterator<Item = (&'a Location, &'a CastAnnotation)>,
) -> ConceptGraph {
    let pairs: Vec<(&Location, &CastAnnotation)> = annotations.into_iter().collect();

    let colliding_names = find_colliding_names(&pairs);

    let mut concepts: BTreeMap<String, ConceptNode> = BTreeMap::new();

    for (loc, ann) in &pairs {
        match ann {
            CastAnnotation::Concept(c) => {
                let key = compose_key(&c.name, &c.common.tags, &colliding_names);
                let node = concepts.entry(key).or_default();
                node.declarations.push(DeclarationRef {
                    location: (*loc).clone(),
                    summary: c.summary.clone(),
                    anchors: c
                        .anchors
                        .iter()
                        .map(|a| AnchorRef {
                            path: syn_path_to_string(&a.path),
                            role: a.role,
                        })
                        .collect(),
                    tags: c.common.tags.clone(),
                    parent: c.parent.clone(),
                });
            }
            CastAnnotation::ContinuesIn(c) => {
                let key = compose_key(&c.concept, &c.common.tags, &colliding_names);
                let node = concepts.entry(key).or_default();
                node.edges.push(ConceptEdge {
                    kind: EdgeKind::ContinuesIn,
                    location: (*loc).clone(),
                    target: EdgeTarget::Rust(syn_path_to_string(&c.target)),
                    why: c.why.clone(),
                });
            }
            CastAnnotation::IoContinuesIn(c) => {
                let key = compose_key(&c.concept, &c.common.tags, &colliding_names);
                let node = concepts.entry(key).or_default();
                node.edges.push(ConceptEdge {
                    kind: EdgeKind::IoContinuesIn,
                    location: (*loc).clone(),
                    target: EdgeTarget::External {
                        target: c.target.clone(),
                        lang: c.lang,
                    },
                    why: c.why.clone(),
                });
            }
            // Other annotation kinds don't carry a `concept:` field, so
            // they don't participate in the graph at this level.
            _ => {}
        }
    }

    ConceptGraph { concepts }
}

/// Pre-pass over all declarations: for each name, collect every
/// distinct full tag-tuple that ever declared it. Names with two or
/// more distinct tag-tuples are colliding. The map's value carries
/// the smallest *prefix length* `k` such that `tags[0..k]` is unique
/// for every declaration of that name — that prefix is what
/// `compose_key` uses as the discriminator. Cross-crate cases
/// (`cast_stdlib` vs `cast_os_stdlib`) discriminate at k=1; cases
/// where two declarations share the crate-tag but differ in
/// category-tag (e.g. `cast_os_stdlib::observability` umbrella vs
/// `cast_os_stdlib::design_qualities::observability` leaf)
/// discriminate at k=2.
fn find_colliding_names(
    pairs: &[(&Location, &CastAnnotation)],
) -> std::collections::BTreeMap<String, usize> {
    let mut tag_tuples_by_name: std::collections::BTreeMap<
        String,
        std::collections::BTreeSet<Vec<String>>,
    > = std::collections::BTreeMap::new();
    for (_, ann) in pairs {
        if let CastAnnotation::Concept(c) = ann {
            tag_tuples_by_name
                .entry(c.name.clone())
                .or_default()
                .insert(c.common.tags.clone());
        }
    }
    let mut out = std::collections::BTreeMap::new();
    for (name, tuples) in tag_tuples_by_name {
        if tuples.len() < 2 {
            continue;
        }
        let max_len = tuples.iter().map(|t| t.len()).max().unwrap_or(0);
        let mut found: Option<usize> = None;
        for k in 1..=max_len {
            let prefixes: std::collections::BTreeSet<Vec<&str>> = tuples
                .iter()
                .map(|t| t.iter().take(k).map(|s| s.as_str()).collect())
                .collect();
            if prefixes.len() == tuples.len() {
                found = Some(k);
                break;
            }
        }
        if let Some(k) = found {
            out.insert(name, k);
        }
    }
    out
}

fn compose_key(
    name: &str,
    tags: &[String],
    colliding: &std::collections::BTreeMap<String, usize>,
) -> String {
    let Some(&k) = colliding.get(name) else {
        return name.to_string();
    };
    let suffix = tags
        .iter()
        .take(k)
        .cloned()
        .collect::<Vec<_>>()
        .join("-");
    if suffix.is_empty() {
        name.to_string()
    } else {
        format!("{name}@{suffix}")
    }
}

/// Compute warnings over the graph. Order: by concept name, then by
/// warning kind discriminator — so output is deterministic.
pub fn warnings(graph: &ConceptGraph) -> Vec<GraphWarning> {
    let mut out = Vec::new();
    let known_concept_names: std::collections::BTreeSet<&str> =
        graph.concepts.keys().map(|s| s.as_str()).collect();
    for (name, node) in &graph.concepts {
        let total_refs = node.declarations.len() + node.edges.len();
        if total_refs <= 1 {
            out.push(GraphWarning {
                concept: name.clone(),
                kind: WarningKind::Orphan,
            });
            continue; // orphan implies undeclared, no need to repeat
        }
        if node.declarations.is_empty() {
            out.push(GraphWarning {
                concept: name.clone(),
                kind: WarningKind::Undeclared,
            });
        } else {
            // Build the set of declared anchors across all declarations.
            let mut declared: Vec<&str> = Vec::new();
            for decl in &node.declarations {
                for a in &decl.anchors {
                    declared.push(a.path.as_str());
                }
            }
            // Primitive concepts (0 declared anchors across all
            // declarations) are by-design open to merge-by-name
            // extension via continues_in!. There is no closed anchor
            // set to check against, so suppress TargetNotInAnchors —
            // every legitimate primitive use would otherwise warn.
            if !declared.is_empty() {
                for edge in &node.edges {
                    if let (EdgeKind::ContinuesIn, EdgeTarget::Rust(target)) =
                        (&edge.kind, &edge.target)
                    {
                        if !declared.contains(&target.as_str()) {
                            out.push(GraphWarning {
                                concept: name.clone(),
                                kind: WarningKind::TargetNotInAnchors {
                                    location: edge.location.clone(),
                                    target: target.clone(),
                                },
                            });
                        }
                    }
                }
            }

            // UnresolvedParent: `parent:` field names a concept that
            // doesn't exist in the graph. One warning per declaration
            // that sets parent (lets the user see which file/line
            // needs the fix when the concept is declared in multiple
            // places).
            for decl in &node.declarations {
                let Some(parent_name) = decl.parent.as_deref() else { continue };
                if known_concept_names.contains(parent_name) {
                    continue;
                }
                out.push(GraphWarning {
                    concept: name.clone(),
                    kind: WarningKind::UnresolvedParent {
                        location: decl.location.clone(),
                        parent: parent_name.to_string(),
                    },
                });
            }

            // MissingStdlibGrounding: a non-umbrella concept must
            // have at least one continues_in! to a cast_stdlib::*
            // target. Umbrellas (≥2 embodied anchors at depth ≤ 2)
            // are exempt — they're per-crate roots, not leaves that
            // realise a primitive. The depth check matches the
            // tree.rs heuristic for the same shape.
            let mut max_embodied_depth: usize = 0;
            let mut embodied_count: usize = 0;
            for decl in &node.declarations {
                for a in &decl.anchors {
                    if matches!(a.role, crate::parser::concept::AnchorRole::Embodied) {
                        embodied_count += 1;
                        let depth = a.path.split("::").count();
                        if depth > max_embodied_depth {
                            max_embodied_depth = depth;
                        }
                    }
                }
            }
            let is_umbrella = embodied_count >= 2 && max_embodied_depth > 0
                && max_embodied_depth <= 2;
            if !is_umbrella {
                let grounded = node.edges.iter().any(|e| {
                    matches!(e.kind, EdgeKind::ContinuesIn)
                        && match &e.target {
                            EdgeTarget::Rust(p) => p.starts_with("cast_stdlib::"),
                            _ => false,
                        }
                });
                if !grounded {
                    out.push(GraphWarning {
                        concept: name.clone(),
                        kind: WarningKind::MissingStdlibGrounding,
                    });
                }
            }
        }
    }
    out
}

cast::concept! {
    name: "concept_graph_construction",
    summary: "Pure functions that fold a sequence of (Location, \
              CastAnnotation) pairs into a ConceptGraph and compute the \
              graph-level warnings (orphan, undeclared, target-not-in- \
              anchors, missing-stdlib-grounding). build_graph keys by \
              concept name across all inputs; warnings is a read-only \
              traversal of the resulting graph. Both are pure — no \
              I/O, no mutation of inputs, no time/RNG dependency. \
              Output ordering is deterministic by design (BTreeMap \
              + name-sorted iteration) so CI diffs are byte-stable.",
    anchors: [
        crate::graph::build_graph,
        crate::graph::warnings,
        crate::graph::find_colliding_names,
        crate::graph::compose_key,
    ],
    tags: ["cast_spec_graph"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "concept_graph_construction",
    why: "Output is a function of the input annotation iterator alone. \
          BTreeMap-backed sorting makes the output byte-stable across \
          calls.",
}

cast::concept! {
    name: "concept_graph_value",
    summary: "In-memory concept graph built by build_graph. Holds \
              concepts keyed by name, each with declarations and \
              edges.",
    anchors: [
        crate::graph::ConceptGraph,
        crate::graph::ConceptNode,
        crate::graph::AnchorRef,
        crate::graph::DeclarationRef,
        crate::graph::ConceptEdge,
        crate::graph::GraphWarning,
    ],
    tags: ["cast_spec_graph"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::product_type,
    concept: "concept_graph_value",
    why: "Each is a struct with all fields simultaneously inhabited.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::value_type,
    concept: "concept_graph_value",
    why: "Cloneable, structurally compared, no resource handles.",
}

cast::concept! {
    name: "concept_graph_categories",
    summary: "Sum-typed discriminators for the concept graph: edge \
              kind, edge target shape, warning kind.",
    anchors: [
        crate::graph::EdgeKind,
        crate::graph::EdgeTarget,
        crate::graph::WarningKind,
    ],
    tags: ["cast_spec_graph"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::sum_type,
    concept: "concept_graph_categories",
    why: "Each is an enum with a closed set of variants.",
}

cast::continues_in! {
    target: cast_stdlib::architecture::typed_handle,
    concept: "concept_graph_value",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::architecture::classifier_dispatch,
    concept: "concept_graph_categories",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::workflow::stateful_workflow,
    concept: "concept_graph_construction",
    why: lazy,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{
        concept::Concept, continues_in::ContinuesIn, io_continues_in::IoContinuesIn, CommonFields,
    };
    use std::path::PathBuf;
    use syn::parse_str;

    fn loc(file: &str, line: usize) -> Location {
        Location::new(PathBuf::from(file), line, 1)
    }

    fn mk_concept(name: &str, anchors: &[&str]) -> CastAnnotation {
        use crate::parser::concept::{AnchorEntry, AnchorRole};
        CastAnnotation::Concept(Concept {
            name: name.to_string(),
            summary: format!("summary for {name}"),
            anchors: anchors
                .iter()
                .map(|s| AnchorEntry {
                    path: parse_str(s).unwrap(),
                    role: AnchorRole::Embodied,
                })
                .collect(),
            parent: None,
            common: CommonFields::default(),
        })
    }

    fn mk_continues_in(concept: &str, target: &str) -> CastAnnotation {
        CastAnnotation::ContinuesIn(ContinuesIn {
            target: parse_str(target).unwrap(),
            concept: concept.to_string(),
            why: Some(WhyValue::Prose { text: "reason".to_string() }),
            common: CommonFields::default(),
        })
    }

    fn mk_io_continues_in(concept: &str, target: &str) -> CastAnnotation {
        CastAnnotation::IoContinuesIn(IoContinuesIn {
            target: target.to_string(),
            lang: Lang::Kotlin,
            concept: concept.to_string(),
            why: Some(WhyValue::Prose { text: "reason".to_string() }),
            anchor: None,
            common: CommonFields::default(),
        })
    }

    #[test]
    fn empty_input_yields_empty_graph() {
        let graph = build_graph(std::iter::empty());
        assert!(graph.concepts.is_empty());
        assert!(warnings(&graph).is_empty());
    }

    #[test]
    fn single_continues_in_is_orphan() {
        let l = loc("a.rs", 1);
        let ann = mk_continues_in("solo", "sample::a::b");
        let graph = build_graph([(&l, &ann)]);
        let w = warnings(&graph);
        assert_eq!(w.len(), 1);
        assert!(matches!(w[0].kind, WarningKind::Orphan));
    }

    #[test]
    fn two_edges_no_decl_is_undeclared_not_orphan() {
        let l1 = loc("a.rs", 1);
        let l2 = loc("b.rs", 1);
        let a = mk_continues_in("shared", "sample::a");
        let b = mk_continues_in("shared", "sample::b");
        let graph = build_graph([(&l1, &a), (&l2, &b)]);
        let w = warnings(&graph);
        assert_eq!(w.len(), 1);
        assert!(matches!(w[0].kind, WarningKind::Undeclared));
    }

    #[test]
    fn declared_with_matching_target_yields_no_warnings() {
        let l1 = loc("decl.rs", 1);
        let l2 = loc("a.rs", 1);
        let decl = mk_concept("c", &["sample::reconciler::dispatch"]);
        let edge = mk_continues_in("c", "sample::reconciler::dispatch");
        let graph = build_graph([(&l1, &decl), (&l2, &edge)]);
        assert!(warnings(&graph).is_empty());
    }

    #[test]
    fn target_not_in_anchors_warns() {
        let l1 = loc("decl.rs", 1);
        let l2 = loc("a.rs", 1);
        let decl = mk_concept("c", &["sample::reconciler::dispatch"]);
        let edge = mk_continues_in("c", "sample::reconciler::renamed");
        let graph = build_graph([(&l1, &decl), (&l2, &edge)]);
        let w = warnings(&graph);
        assert_eq!(w.len(), 1);
        assert!(matches!(w[0].kind, WarningKind::TargetNotInAnchors { .. }));
    }

    #[test]
    fn zero_anchor_concept_does_not_trigger_target_check() {
        // A concept with no anchors is by-design open to merge-by-name
        // extension via continues_in!; there is no closed declared
        // anchor set to compare against, so TargetNotInAnchors must
        // be suppressed. Without this, every legitimate cross-cutting
        // concept use would warn.
        let l1 = loc("decl.rs", 1);
        let l2 = loc("a.rs", 1);
        let decl = mk_concept("host", &[]); // no-anchor declaration
        let edge = mk_continues_in("host", "voluntas_core::node");
        let graph = build_graph([(&l1, &decl), (&l2, &edge)]);
        assert!(warnings(&graph).is_empty());
    }

    #[test]
    fn io_edges_never_trigger_target_check() {
        // External targets aren't Rust paths; they can't be declared as
        // anchors. Asymmetry is OK per GRAMMAR.
        let l1 = loc("decl.rs", 1);
        let l2 = loc("a.rs", 1);
        let decl = mk_concept("c", &["sample::api::sign"]);
        let edge = mk_io_continues_in("c", "mobile/android/.../X.kt");
        let graph = build_graph([(&l1, &decl), (&l2, &edge)]);
        assert!(warnings(&graph).is_empty());
    }

    #[test]
    fn declaration_alone_is_orphan() {
        let l = loc("decl.rs", 1);
        let decl = mk_concept("solo", &["sample::a"]);
        let graph = build_graph([(&l, &decl)]);
        let w = warnings(&graph);
        assert_eq!(w.len(), 1);
        assert!(matches!(w[0].kind, WarningKind::Orphan));
    }

    #[test]
    fn multiple_declarations_for_same_concept_merge_anchors() {
        // Two `concept!` blocks with the same name accumulate anchors;
        // an edge matching either one passes.
        let l1 = loc("a.rs", 1);
        let l2 = loc("b.rs", 1);
        let l3 = loc("c.rs", 1);
        let d1 = mk_concept("c", &["sample::a"]);
        let d2 = mk_concept("c", &["sample::b"]);
        let edge = mk_continues_in("c", "sample::b");
        let graph = build_graph([(&l1, &d1), (&l2, &d2), (&l3, &edge)]);
        assert!(warnings(&graph).is_empty());
    }
}
