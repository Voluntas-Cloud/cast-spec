//! Canonical concept tree — one tree per workspace, derived from the
//! `Report` so the renderer gets a stable structured view.
//!
//! Three placement rules:
//!
//! 1. **Per-anchor parent**: for an anchor path A on concept C, the
//!    candidate parent concept P is the one whose embodied anchor B is
//!    the longest *strict* prefix of A (B + "::" prefixes A). Equality
//!    does not count — two concepts that share an embodied anchor are
//!    peers, not parent/child.
//! 2. **Multi-anchor hoist**: if C's anchors yield multiple distinct
//!    candidate parents, C's parent is the LCA of those candidates in
//!    the partial tree. Computed by an iterative fixpoint pass.
//! 3. **Zero-anchor placement**: a concept with no embodied anchors is
//!    placed under the closest "crate-umbrella" concept — the most-
//!    anchored concept whose embodied anchors all live under the lib
//!    name corresponding to the zero-anchor concept's declaration file.
//!    If no crate-umbrella matches, the concept goes under the workspace
//!    root.

use crate::emit::model::{
    AnchorRoleRepr, AnnotationKind, ConceptWarningKindRepr, EdgeKindRepr, InvocationReport,
    Report,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

/// Name of the umbrella `cast::concept!` declared in `Cast.cast` at the
/// workspace root. If it's missing, `build()` synthesises a virtual root
/// labelled `(workspace)`.
pub const ROOT_NAME: &str = "cast";

#[derive(Debug, Serialize)]
pub struct CanonicalTree {
    pub root: ConceptNode,
    pub edges: Vec<TreeEdge>,
}

#[derive(Debug, Serialize)]
pub struct ConceptNode {
    pub name: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub declared_at: Vec<DeclarationLoc>,
    pub anchors: Vec<AnchorLeaf>,
    pub children: Vec<ConceptNode>,
    pub counts: MacroCounts,
    /// Graph warnings attributed to this concept (orphan / undeclared /
    /// target_not_in_anchors). Empty Vec when the concept is clean.
    pub warnings: Vec<TreeWarning>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TreeWarning {
    pub kind: TreeWarningKind,
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TreeWarningKind {
    Orphan,
    Undeclared,
    TargetNotInAnchors,
    UnresolvedParent,
    MissingStdlibGrounding,
}

#[derive(Debug, Serialize, Clone)]
pub struct DeclarationLoc {
    pub file: String,
    pub line: usize,
}

#[derive(Debug, Serialize)]
pub struct AnchorLeaf {
    /// Normalised path (`crate::` prefix replaced with the declaring
    /// crate's lib name). The renderer can show this as-is.
    pub path: String,
    pub role: AnchorRoleRepr,
    /// `true` if no child concept lives below this anchor — the
    /// renderer should draw a leaf. `false` means a child concept exists
    /// and is reachable through `children`; the renderer can de-dupe.
    pub leaf: bool,
    /// Set when this is a Primitive anchor whose path is the embodied
    /// anchor of another concept. The renderer can label the leaf with a
    /// `→ embodied_by` cross-reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embodied_by: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct MacroCounts {
    pub rules: u32,
    pub anti_patterns: u32,
    pub comparisons: u32,
    pub pipelines: u32,
    pub tiers: u32,
    pub matrices: u32,
    pub gut_checks: u32,
    pub io_targets: u32,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TreeEdge {
    ContinuesIn {
        concept: String,
        from_at: DeclarationLoc,
        to_at: DeclarationLoc,
    },
    IoContinuesIn {
        concept: String,
        target: String,
        lang: String,
        internal: bool,
        from_at: DeclarationLoc,
    },
}

pub fn build(report: &Report) -> CanonicalTree {
    let model = build_model(report);
    let placement = place_concepts(&model);
    let counts = count_macros(report, &model, &placement);
    let edges = collect_edges(report);
    let warnings = collect_warnings(report);
    let embodied_index = build_embodied_index(&model);
    let mut visited: BTreeSet<&str> = BTreeSet::new();
    let root = build_node(
        ROOT_NAME,
        &model,
        &placement,
        &counts,
        &warnings,
        &embodied_index,
        &mut visited,
    );
    CanonicalTree {
        root: root.unwrap_or_else(|| {
            synthetic_root(
                &model,
                &placement,
                &counts,
                &warnings,
                &embodied_index,
                &mut visited,
            )
        }),
        edges,
    }
}

/// Map embodied-anchor path → name of the concept that embodies it.
/// Used by `build_node` to populate the `embodied_by` cross-reference
/// on Primitive anchors. Hoisted here so the recursion doesn't rebuild
/// the same BTreeMap on every node — that was an O(N) per-call drag
/// even when the tree was acyclic.
fn build_embodied_index(model: &BTreeMap<String, Concept>) -> BTreeMap<&str, &str> {
    let mut idx: BTreeMap<&str, &str> = BTreeMap::new();
    for other in model.values() {
        for a in &other.embodied {
            idx.entry(a.as_str()).or_insert(other.name.as_str());
        }
    }
    idx
}

/// Group concept-graph warnings by concept name. Warnings about a
/// concept that isn't placed in the tree (e.g. an `Undeclared` concept
/// that exists only as a `continues_in!` target) are still surfaced —
/// they attach to the closest tree node we can find. v1: anything
/// unattributed becomes a warning on the root node so it's visible.
fn collect_warnings(report: &Report) -> BTreeMap<String, Vec<TreeWarning>> {
    let mut out: BTreeMap<String, Vec<TreeWarning>> = BTreeMap::new();
    for w in &report.concept_graph.warnings {
        let kind = match w.kind {
            ConceptWarningKindRepr::Orphan => TreeWarningKind::Orphan,
            ConceptWarningKindRepr::Undeclared => TreeWarningKind::Undeclared,
            ConceptWarningKindRepr::TargetNotInAnchors => TreeWarningKind::TargetNotInAnchors,
            ConceptWarningKindRepr::UnresolvedParent => TreeWarningKind::UnresolvedParent,
            ConceptWarningKindRepr::MissingStdlibGrounding => TreeWarningKind::MissingStdlibGrounding,
        };
        out.entry(w.concept.clone()).or_default().push(TreeWarning {
            kind,
            message: w.message.clone(),
        });
    }
    out
}

/// Expand one concept — return the non-concept macros attributed to it
/// by the same per-anchor longest-prefix-match rule the tree builder
/// applies to placement. Excludes `cast::concept!` (those are nodes in
/// the tree itself), `continues_in!` and `io::continues_in!` (those are
/// surfaced as edges).
pub fn expand(report: &Report, concept: &str) -> Vec<InvocationReport> {
    let model = build_model(report);
    let all_embodied: Vec<(&str, &str)> = model
        .values()
        .flat_map(|c| c.embodied.iter().map(move |a| (c.name.as_str(), a.as_str())))
        .collect();

    let mut out = Vec::new();
    for invocations in report.groups.values() {
        for inv in invocations {
            if matches!(
                inv.kind,
                AnnotationKind::Concept
                    | AnnotationKind::ContinuesIn
                    | AnnotationKind::IoContinuesIn
                    | AnnotationKind::Unknown
            ) {
                continue;
            }
            let crate_prefix = lib_name_from_file(&inv.file);
            let paths: Vec<String> = inv
                .paths
                .iter()
                .map(|p| normalise_path(&p.text, crate_prefix.as_deref()))
                .collect();
            if attribute_to_concept(&paths, &all_embodied) == concept {
                out.push(inv.clone());
            }
        }
    }
    out.sort_by(|a, b| {
        (a.file.as_str(), a.line).cmp(&(b.file.as_str(), b.line))
    });
    out
}

// ──────────────────────────────────────────────────────────────────────
// Internal model — normalised view of the Report's concept graph.
// ──────────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct Concept {
    name: String,
    summary: String,
    tags: Vec<String>,
    declared_at: Vec<DeclarationLoc>,
    embodied: Vec<String>,
    primitive: Vec<String>,
    /// Lib-name corresponding to the (first) declaration's file path,
    /// used for zero-anchor placement.
    decl_crate_lib: Option<String>,
    /// Explicit parent override from the `parent:` field on
    /// `cast::concept!`. When `Some`, place_concepts uses it as a
    /// hard parent instead of the strict-prefix-on-anchors rule.
    /// When more than one declaration sets it, the first wins —
    /// later declarations should agree, but the duplicate-mismatch
    /// case isn't currently warned (TODO if it bites).
    parent_override: Option<String>,
}

fn build_model(report: &Report) -> BTreeMap<String, Concept> {
    let mut out = BTreeMap::new();
    for (name, c) in &report.concept_graph.concepts {
        let mut summary = String::new();
        let mut tags: Vec<String> = Vec::new();
        let mut declared_at = Vec::new();
        let mut embodied = Vec::new();
        let mut primitive = Vec::new();
        let mut decl_crate_lib = None;
        let mut parent_override: Option<String> = None;
        for (i, decl) in c.declarations.iter().enumerate() {
            if i == 0 {
                summary = decl.summary.clone();
                tags = decl.tags.clone();
                decl_crate_lib = lib_name_from_file(&decl.file);
                parent_override = decl.parent.clone();
            }
            declared_at.push(DeclarationLoc {
                file: decl.file.clone(),
                line: decl.line,
            });
            let crate_prefix = lib_name_from_file(&decl.file);
            for a in &decl.anchors {
                let norm = normalise_path(&a.path, crate_prefix.as_deref());
                match a.role {
                    AnchorRoleRepr::Embodied => embodied.push(norm),
                    AnchorRoleRepr::Primitive => primitive.push(norm),
                }
            }
        }
        // Stable de-duplication, keeping order.
        embodied = dedup_keep_first(embodied);
        primitive = dedup_keep_first(primitive);
        out.insert(
            name.clone(),
            Concept {
                name: name.clone(),
                summary,
                tags,
                declared_at,
                embodied,
                primitive,
                decl_crate_lib,
                parent_override,
            },
        );
    }
    out
}

fn dedup_keep_first(v: Vec<String>) -> Vec<String> {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    v.into_iter()
        .filter(|s| seen.insert(s.clone()))
        .collect()
}

/// Map a declaration file path to the lib name of the containing crate.
/// Returns `None` for paths outside `crates/<x>/`.
fn lib_name_from_file(file: &str) -> Option<String> {
    let segs: Vec<&str> = file.split('/').collect();
    let i = segs.iter().position(|s| *s == "crates")?;
    let crate_dir = segs.get(i + 1)?;
    Some(lib_name_from_crate_dir(crate_dir))
}

fn lib_name_from_crate_dir(dir: &str) -> String {
    // cast-spec is special — its [lib] name is `cast`.
    if dir == "cast-spec" {
        return "cast".to_string();
    }
    dir.replace('-', "_")
}

fn normalise_path(path: &str, crate_prefix: Option<&str>) -> String {
    if let Some(rest) = path.strip_prefix("crate::") {
        if let Some(cp) = crate_prefix {
            return format!("{cp}::{rest}");
        }
    }
    if path == "crate" {
        if let Some(cp) = crate_prefix {
            return cp.to_string();
        }
    }
    path.to_string()
}

// ──────────────────────────────────────────────────────────────────────
// Placement — assign every concept a parent.
// ──────────────────────────────────────────────────────────────────────

cast::rule! {
    rule: "Tree placement must never produce a cycle in `placement.parent`. \
           Both the zero-candidate fallback (step 2) and the orphan \
           re-parenting pass call `place_zero_anchor`, which can \
           symmetrically pick two non-umbrella concepts in the same crate \
           as each other's umbrella when their heuristics tie. Before \
           inserting `name -> fallback`, walk fallback's current ancestor \
           chain and refuse the assignment if `name` already appears — \
           leaving `name` parented at ROOT_NAME is the safe fallback.",
    why:  "build_node recurses through `placement.children` and rebuilds \
           an O(N) embodied_index on every call. A cycle in the children \
           index turns the tree build into infinite recursion that \
           allocates fast enough to OOM the cast-watch daemon and take \
           the launching shell down with it. The visited-set guard inside \
           build_node is the second line of defense; preventing the cycle \
           at its source is the first.",
    governs: [
        crate::tree::place_concepts,
        crate::tree::build_node,
    ],
    tags: ["cast_grammar"],
}

#[derive(Debug, Default)]
struct Placement {
    /// concept name → parent concept name (ROOT_NAME for top-level)
    parent: BTreeMap<String, String>,
    /// concept name → ordered list of children
    children: BTreeMap<String, Vec<String>>,
}

fn place_concepts(model: &BTreeMap<String, Concept>) -> Placement {
    let mut placement = Placement::default();

    // Step 0 — explicit parent overrides (`parent:` field on
    // `cast::concept!`). These bypass the strict-prefix rule
    // entirely for the named concept. The named parent must exist
    // in the model; if it doesn't, fall through to the strict-
    // prefix path so the concept still gets placed somewhere
    // (graph::warnings emits the UnresolvedParent diagnostic
    // separately).
    let mut explicit_parented: BTreeSet<&str> = BTreeSet::new();
    for (name, c) in model {
        let Some(parent_name) = c.parent_override.as_deref() else { continue };
        if !model.contains_key(parent_name) {
            continue;
        }
        if would_create_cycle(parent_name, name, &placement.parent) {
            continue;
        }
        placement.parent.insert(name.clone(), parent_name.to_string());
        explicit_parented.insert(name.as_str());
    }

    // Step 1 — per-anchor candidate parents.
    let candidates_per_concept = compute_anchor_candidates(model);

    // Step 2 — single-candidate placements first; multi-candidate handled
    // in a fixpoint pass that uses the partial tree for LCA. Concepts
    // already placed by step 0 (explicit parent) skip both passes.
    let mut unresolved: Vec<&String> = Vec::new();
    for (name, candidates) in &candidates_per_concept {
        if name.as_str() == ROOT_NAME {
            // The umbrella concept is the root by convention; no parent.
            continue;
        }
        if explicit_parented.contains(name.as_str()) {
            continue;
        }
        match candidates.len() {
            0 => {
                let fallback = place_zero_anchor(name, model, &candidates_per_concept);
                let parent = if fallback != ROOT_NAME
                    && fallback.as_str() != name.as_str()
                    && !would_create_cycle(&fallback, name, &placement.parent)
                {
                    fallback
                } else {
                    ROOT_NAME.to_string()
                };
                placement.parent.insert((*name).clone(), parent);
            }
            1 => {
                let p = candidates.iter().next().unwrap();
                placement.parent.insert((*name).clone(), p.clone());
            }
            _ => unresolved.push(name),
        }
    }

    // Step 3 — fixpoint for multi-candidate concepts. LCA in the partial
    // tree using already-placed concepts. If a candidate is itself
    // unplaced, defer to the next pass.
    loop {
        let mut progress = false;
        let mut still_unresolved = Vec::new();
        for name in &unresolved {
            let candidates = &candidates_per_concept[*name];
            if candidates
                .iter()
                .all(|c| c == ROOT_NAME || placement.parent.contains_key(c))
            {
                let lca = compute_lca(candidates, &placement.parent);
                placement.parent.insert((*name).clone(), lca);
                progress = true;
            } else {
                still_unresolved.push(*name);
            }
        }
        unresolved = still_unresolved;
        if !progress {
            break;
        }
    }
    // Anything still unresolved (cycles or missing parents) → root.
    for name in unresolved {
        placement.parent.insert(name.clone(), ROOT_NAME.to_string());
    }

    // "Orphans belong to their first parent." A concept whose standard
    // placement bubbled all the way up to the workspace umbrella —
    // either because its anchors didn't strict-prefix-match anything
    // closer or because the LCA hoist pulled it past every per-crate
    // umbrella — falls back to the per-crate umbrella for whichever
    // crate it's declared in. The exception is umbrella-shaped concepts
    // themselves: a concept whose embodied anchors are all at module
    // depth ≤ 2 IS a per-crate umbrella and should stay at the
    // workspace root, not be re-parented under one of its own
    // descendants.
    let names_to_recheck: Vec<String> = placement.parent.keys().cloned().collect();
    for name in names_to_recheck {
        if placement.parent.get(&name).map(|p| p.as_str()) != Some(ROOT_NAME) {
            continue;
        }
        let Some(c) = model.get(&name) else { continue };
        let max_depth = c
            .embodied
            .iter()
            .map(|a| a.split("::").count())
            .max()
            .unwrap_or(0);
        // Umbrella-shaped → stays at root. An umbrella spans a crate's
        // submodules: at least 2 embodied anchors, all at depth ≤ 2.
        // A single-anchor concept whose lone anchor sits at depth 2 is
        // NOT an umbrella (e.g., `bash_examples` anchors only at
        // `cast::examples`); it should fall through to the
        // crate-umbrella fallback.
        if max_depth > 0 && max_depth <= 2 && c.embodied.len() >= 2 {
            continue;
        }
        let fallback = place_zero_anchor(&name, model, &candidates_per_concept);
        if fallback != ROOT_NAME
            && fallback != name
            && !would_create_cycle(&fallback, &name, &placement.parent)
        {
            placement.parent.insert(name, fallback);
        }
    }

    // Build children index — sorted for determinism.
    for (child, parent) in &placement.parent {
        placement
            .children
            .entry(parent.clone())
            .or_default()
            .push(child.clone());
    }
    for v in placement.children.values_mut() {
        v.sort();
    }

    placement
}

/// For each concept, collect the set of candidate parent concepts based
/// on the longest-strict-prefix rule applied to each embodied anchor.
fn compute_anchor_candidates(
    model: &BTreeMap<String, Concept>,
) -> BTreeMap<&String, BTreeSet<String>> {
    let mut out: BTreeMap<&String, BTreeSet<String>> = BTreeMap::new();
    // Pre-build (concept_name, embodied_anchor) tuples for searching.
    let all_anchors: Vec<(&str, &str)> = model
        .values()
        .flat_map(|c| c.embodied.iter().map(move |a| (c.name.as_str(), a.as_str())))
        .collect();

    for c in model.values() {
        let mut set = BTreeSet::new();
        // Both embodied and primitive anchors carry "where does this
        // concept live?" information. Embodied = the concept instantiates
        // this code; primitive = the concept rests on it as an atom.
        // Either form locates the concept in the architecture map, so
        // both should contribute parent candidates. Without this, a
        // concept whose only anchors are primitive (e.g. a UI invariant
        // anchored CAST::AS_PRIMITIVE at the module it constrains)
        // bubbles to the workspace root and lands as a sibling of the
        // crate umbrella instead of nesting underneath it. The OTHER
        // side of the strict-prefix-match still uses embodied anchors
        // only — primitive anchors mark blackboxes a concept depends on,
        // not territory it owns, so they don't make the concept a
        // candidate parent for anyone else.
        for anchor in c.embodied.iter().chain(c.primitive.iter()) {
            let mut best: Option<(&str, usize)> = None;
            for (other_name, other_anchor) in &all_anchors {
                if *other_name == c.name {
                    continue;
                }
                if is_strict_prefix(other_anchor, anchor) {
                    let len = other_anchor.split("::").count();
                    if best.map(|(_, l)| len > l).unwrap_or(true) {
                        best = Some((*other_name, len));
                    }
                }
            }
            if let Some((p, _)) = best {
                set.insert(p.to_string());
            }
        }
        out.insert(&c.name, set);
    }
    out
}

fn is_strict_prefix(prefix: &str, full: &str) -> bool {
    if prefix == full {
        return false;
    }
    full.starts_with(prefix)
        && full[prefix.len()..].starts_with("::")
}

/// Would assigning `target -> start` close a cycle in the existing
/// parent map? Walks `start`'s ancestor chain looking for `target`;
/// stops at ROOT_NAME or at any pre-existing cycle (which shouldn't
/// happen but we guard anyway so this helper itself can't loop).
fn would_create_cycle(
    start: &str,
    target: &str,
    parent_of: &BTreeMap<String, String>,
) -> bool {
    let mut current = start;
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    seen.insert(current);
    while let Some(p) = parent_of.get(current) {
        if p == target {
            return true;
        }
        if !seen.insert(p.as_str()) {
            return false;
        }
        if p == ROOT_NAME {
            break;
        }
        current = p.as_str();
    }
    false
}

fn place_zero_anchor(
    name: &str,
    model: &BTreeMap<String, Concept>,
    _candidates: &BTreeMap<&String, BTreeSet<String>>,
) -> String {
    let Some(c) = model.get(name) else {
        return ROOT_NAME.to_string();
    };
    let Some(lib) = c.decl_crate_lib.as_deref() else {
        return ROOT_NAME.to_string();
    };

    // First principle: a fallback parent must represent at-least-as-broad
    // a scope as `name`. Anchor depth measures scope (depth 1 = crate
    // root, depth 2 = top-level module, ...): a candidate whose deepest
    // anchor is *shallower* than `name`'s shallowest anchor is broader
    // than `name` and a valid ancestor; a candidate whose anchors are
    // *deeper* lives inside `name`'s subtree and cannot be its parent.
    //
    // Without this filter, a per-crate umbrella whose anchors are at
    // depth 2 (`cast_stdlib::ai`, `cast_stdlib::api`, ...) and which
    // therefore has no candidate parent would fall back here and pick
    // its own deepest descendant (e.g. `patterns` at depth 3) as a
    // parent, closing the cycle `cast_stdlib -> patterns -> cast_stdlib`.
    // The filter rules that out by construction.
    let name_min_depth = c
        .embodied
        .iter()
        .chain(c.primitive.iter())
        .map(|a| a.split("::").count())
        .min()
        .unwrap_or(0);
    if name_min_depth == 0 {
        return ROOT_NAME.to_string();
    }

    // Best per-crate umbrella = a concept (other than `name` itself)
    // whose embodied anchors all live under `<lib>::*` (or equal `<lib>`)
    // AND whose deepest anchor is no deeper than `name`'s shallowest
    // anchor. Ranking among the surviving candidates: smallest max
    // anchor depth wins (broadest scope); tie-break by anchor count
    // (more anchors = broader coverage).
    let mut best: Option<(&str, usize, usize)> = None;
    for (other_name, other) in model {
        if other_name == name {
            continue;
        }
        if other.embodied.is_empty() {
            continue;
        }
        let all_in = other
            .embodied
            .iter()
            .all(|a| a == lib || a.starts_with(&format!("{lib}::")));
        if !all_in {
            continue;
        }
        let max_depth = other
            .embodied
            .iter()
            .map(|a| a.split("::").count())
            .max()
            .unwrap_or(0);
        if max_depth > name_min_depth {
            continue;
        }
        let count = other.embodied.len();
        let better = match best {
            None => true,
            Some((_, bd, bc)) => max_depth < bd || (max_depth == bd && count > bc),
        };
        if better {
            best = Some((other_name.as_str(), max_depth, count));
        }
    }
    best.map(|(n, _, _)| n.to_string()).unwrap_or_else(|| ROOT_NAME.to_string())
}

/// LCA of the given concepts in the parent tree built from
/// `parent_of`. Falls back to ROOT_NAME if the concepts are in
/// disconnected subtrees (shouldn't happen if the partial tree is
/// rooted, but defensive).
fn compute_lca(candidates: &BTreeSet<String>, parent_of: &BTreeMap<String, String>) -> String {
    if candidates.is_empty() {
        return ROOT_NAME.to_string();
    }
    if candidates.len() == 1 {
        return candidates.iter().next().unwrap().clone();
    }
    // Build ancestor chains (root-to-self).
    let chains: Vec<Vec<String>> = candidates
        .iter()
        .map(|c| ancestor_chain(c, parent_of))
        .collect();
    // Intersect chains by walking from the root.
    let mut lca = ROOT_NAME.to_string();
    let min_len = chains.iter().map(|c| c.len()).min().unwrap_or(0);
    for i in 0..min_len {
        let candidate = &chains[0][i];
        if chains.iter().all(|c| &c[i] == candidate) {
            lca = candidate.clone();
        } else {
            break;
        }
    }
    lca
}

fn ancestor_chain(concept: &str, parent_of: &BTreeMap<String, String>) -> Vec<String> {
    // Collect ancestors root-to-self. Guarded against a cycle in
    // parent_of: place_concepts now refuses to insert a parent edge
    // that closes a cycle, but this is cheap insurance — a future
    // caller of ancestor_chain shouldn't be a load-bearing assumption
    // about the partial tree's acyclicity.
    let mut chain = vec![concept.to_string()];
    let mut seen: BTreeSet<String> = BTreeSet::new();
    seen.insert(concept.to_string());
    let mut current = concept;
    while let Some(p) = parent_of.get(current) {
        if !seen.insert(p.clone()) {
            break;
        }
        chain.push(p.clone());
        if p == ROOT_NAME {
            break;
        }
        current = p;
    }
    chain.reverse();
    chain
}

// ──────────────────────────────────────────────────────────────────────
// Macro counts — non-concept invocations attached per concept.
// ──────────────────────────────────────────────────────────────────────

fn count_macros(
    report: &Report,
    model: &BTreeMap<String, Concept>,
    _placement: &Placement,
) -> BTreeMap<String, MacroCounts> {
    // For v1: count by attribution to a concept via the same per-anchor
    // longest-strict-prefix rule applied to the macro's resolved Rust
    // paths. io_continues_in counts via its source concept (already
    // captured on the ConceptReport.edges side).
    let mut counts: BTreeMap<String, MacroCounts> = BTreeMap::new();
    for c in model.keys() {
        counts.insert(c.clone(), MacroCounts::default());
    }

    // Precompute per-concept embodied anchors for prefix attribution.
    let all_embodied: Vec<(&str, &str)> = model
        .values()
        .flat_map(|c| c.embodied.iter().map(move |a| (c.name.as_str(), a.as_str())))
        .collect();

    for invocations in report.groups.values() {
        for inv in invocations {
            let kind = inv.kind;
            if matches!(kind, AnnotationKind::Concept | AnnotationKind::Unknown) {
                continue;
            }
            // Resolve attribution. Skip continues_in (it's an edge, not
            // an attached macro) and io_continues_in (handled separately
            // via ConceptReport.edges → io_targets count).
            if matches!(
                kind,
                AnnotationKind::ContinuesIn | AnnotationKind::IoContinuesIn
            ) {
                continue;
            }
            let crate_prefix = lib_name_from_file(&inv.file);
            let paths: Vec<String> = inv
                .paths
                .iter()
                .map(|p| normalise_path(&p.text, crate_prefix.as_deref()))
                .collect();
            let attrib = attribute_to_concept(&paths, &all_embodied);
            let bucket = counts.entry(attrib).or_default();
            match kind {
                AnnotationKind::Rule => bucket.rules += 1,
                AnnotationKind::AntiPattern => bucket.anti_patterns += 1,
                AnnotationKind::Compare => bucket.comparisons += 1,
                AnnotationKind::Pipeline => bucket.pipelines += 1,
                AnnotationKind::Tier => bucket.tiers += 1,
                AnnotationKind::Matrix => bucket.matrices += 1,
                AnnotationKind::GutCheck => bucket.gut_checks += 1,
                _ => {}
            }
        }
    }

    // io_targets count — from ConceptReport.edges directly.
    for (name, c) in &report.concept_graph.concepts {
        let n = c
            .edges
            .iter()
            .filter(|e| matches!(e.kind, EdgeKindRepr::IoContinuesIn))
            .count() as u32;
        if n > 0 {
            counts.entry(name.clone()).or_default().io_targets = n;
        }
    }

    counts
}

/// Attribute a non-concept macro to a concept by finding the
/// longest-prefix-match of one of the macro's resolved Rust paths
/// against the concept embodied-anchor set. Falls back to root.
fn attribute_to_concept(paths: &[String], all_embodied: &[(&str, &str)]) -> String {
    let mut best: Option<(&str, usize)> = None;
    for path in paths {
        for (concept, anchor) in all_embodied {
            if anchor == path || is_strict_prefix(anchor, path) {
                let len = anchor.split("::").count();
                if best.map(|(_, l)| len > l).unwrap_or(true) {
                    best = Some((*concept, len));
                }
            }
        }
    }
    best.map(|(n, _)| n.to_string()).unwrap_or_else(|| ROOT_NAME.to_string())
}

// ──────────────────────────────────────────────────────────────────────
// Edges — one entry per continues_in / io_continues_in.
// ──────────────────────────────────────────────────────────────────────

fn collect_edges(report: &Report) -> Vec<TreeEdge> {
    let mut out = Vec::new();
    for (name, c) in &report.concept_graph.concepts {
        for e in &c.edges {
            match e.kind {
                EdgeKindRepr::ContinuesIn => {
                    if let Some(to_at) = find_declaration_at_target(report, &e.target, name) {
                        out.push(TreeEdge::ContinuesIn {
                            concept: name.clone(),
                            from_at: DeclarationLoc {
                                file: e.file.clone(),
                                line: e.line,
                            },
                            to_at,
                        });
                    }
                }
                EdgeKindRepr::IoContinuesIn => {
                    out.push(TreeEdge::IoContinuesIn {
                        concept: name.clone(),
                        target: e.target.clone(),
                        lang: e.lang.clone().unwrap_or_default(),
                        // Heuristic: target paths starting with `crates/`
                        // or otherwise relative to the workspace are
                        // internal. Absolute paths or URLs are external.
                        // The cast-watch resolver also tags `lang:
                        // external` separately — but the renderer can
                        // just trust this best-effort hint.
                        internal: !e.target.starts_with('/')
                            && !e.target.starts_with("http"),
                        from_at: DeclarationLoc {
                            file: e.file.clone(),
                            line: e.line,
                        },
                    });
                }
            }
        }
    }
    out
}

fn find_declaration_at_target(
    report: &Report,
    target: &str,
    concept_name: &str,
) -> Option<DeclarationLoc> {
    // continues_in's `target` is a Rust path. The destination
    // declaration is the one whose anchors include this path.
    let c = report.concept_graph.concepts.get(concept_name)?;
    for decl in &c.declarations {
        for a in &decl.anchors {
            if a.path == target {
                return Some(DeclarationLoc {
                    file: decl.file.clone(),
                    line: decl.line,
                });
            }
        }
    }
    None
}

// ──────────────────────────────────────────────────────────────────────
// Tree assembly — recursively build ConceptNode from placement.
// ──────────────────────────────────────────────────────────────────────

fn build_node<'a>(
    name: &'a str,
    model: &'a BTreeMap<String, Concept>,
    placement: &'a Placement,
    counts: &BTreeMap<String, MacroCounts>,
    warnings: &BTreeMap<String, Vec<TreeWarning>>,
    embodied_index: &BTreeMap<&'a str, &'a str>,
    visited: &mut BTreeSet<&'a str>,
) -> Option<ConceptNode> {
    // Cycle guard. place_concepts now refuses to insert parent edges
    // that would close a cycle, but a tree builder that recursively
    // clones N-sized state on every step is too dangerous to rely on
    // a single invariant: any future placement bug becomes an OOM that
    // takes the launching shell with it. Bail cleanly on re-entry.
    if !visited.insert(name) {
        return None;
    }
    let c = model.get(name)?;
    let children: Vec<ConceptNode> = placement
        .children
        .get(name)
        .map(|kids| {
            kids.iter()
                .filter_map(|k| {
                    build_node(
                        k.as_str(),
                        model,
                        placement,
                        counts,
                        warnings,
                        embodied_index,
                        visited,
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    // Collect every embodied anchor of every direct child concept. An
    // anchor of *this* node is "covered" (and so should be hidden as a
    // leaf in the tree) when a child concept's embodied anchor is equal
    // to it OR is a strict descendant of it — in either case the child
    // concept node is the better, more specific view of that code.
    let child_anchors: Vec<&str> = placement
        .children
        .get(name)
        .map(|kids| {
            kids.iter()
                .flat_map(|k| {
                    model
                        .get(k)
                        .map(|c| c.embodied.iter().map(|s| s.as_str()).collect::<Vec<_>>())
                        .unwrap_or_default()
                })
                .collect()
        })
        .unwrap_or_default();
    let anchor_covered = |path: &str| -> bool {
        child_anchors
            .iter()
            .any(|child| *child == path || is_strict_prefix(path, child))
    };

    let mut anchors = Vec::new();
    for a in &c.embodied {
        anchors.push(AnchorLeaf {
            path: a.clone(),
            role: AnchorRoleRepr::Embodied,
            leaf: !anchor_covered(a.as_str()),
            embodied_by: None,
        });
    }
    for a in &c.primitive {
        let embodied_by = embodied_index
            .get(a.as_str())
            .filter(|owner| **owner != c.name.as_str())
            .map(|s| (*s).to_string());
        anchors.push(AnchorLeaf {
            path: a.clone(),
            role: AnchorRoleRepr::Primitive,
            leaf: true,
            embodied_by,
        });
    }

    let count = counts.get(name).cloned().unwrap_or_default();
    let node_warnings = warnings.get(name).cloned().unwrap_or_default();

    Some(ConceptNode {
        name: c.name.clone(),
        summary: c.summary.clone(),
        tags: c.tags.clone(),
        declared_at: c.declared_at.clone(),
        anchors,
        children,
        counts: count,
        warnings: node_warnings,
    })
}

fn synthetic_root<'a>(
    model: &'a BTreeMap<String, Concept>,
    placement: &'a Placement,
    counts: &BTreeMap<String, MacroCounts>,
    warnings: &BTreeMap<String, Vec<TreeWarning>>,
    embodied_index: &BTreeMap<&'a str, &'a str>,
    visited: &mut BTreeSet<&'a str>,
) -> ConceptNode {
    // Build a virtual root containing every concept whose parent is
    // ROOT_NAME — used when Cast.cast hasn't been declared.
    let children: Vec<ConceptNode> = placement
        .children
        .get(ROOT_NAME)
        .map(|kids| {
            kids.iter()
                .filter_map(|k| {
                    build_node(
                        k.as_str(),
                        model,
                        placement,
                        counts,
                        warnings,
                        embodied_index,
                        visited,
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    ConceptNode {
        name: "(workspace)".to_string(),
        summary: "Synthetic root — no `cast::concept!` block named \
                  `cast` was found at the workspace root."
            .to_string(),
        tags: vec![],
        declared_at: vec![],
        anchors: vec![],
        children,
        counts: MacroCounts::default(),
        warnings: vec![],
    }
}

impl Clone for MacroCounts {
    fn clone(&self) -> Self {
        Self {
            rules: self.rules,
            anti_patterns: self.anti_patterns,
            comparisons: self.comparisons,
            pipelines: self.pipelines,
            tiers: self.tiers,
            matrices: self.matrices,
            gut_checks: self.gut_checks,
            io_targets: self.io_targets,
        }
    }
}

// Suppress unused-import warning when Clone is auto-derived elsewhere.
#[allow(dead_code)]
fn _assert_serialize<T: Serialize>() {}

cast::concept! {
    name: "canonical_tree_placement",
    summary: "Pure functions that fold a Report into a CanonicalTree — \
              the deterministic per-concept hierarchical view used by \
              the markdown emit and the cast-web UI. Placement is \
              decided by the strict-prefix-on-anchors rule, with \
              explicit `parent:` overrides where ambiguity exists. \
              build, expand, and the supporting helpers (path \
              normalisation, LCA, ancestor chain, embodied index) are \
              all pure: output is a function of the input Report alone. \
              BTreeMap iteration ordering makes the produced tree \
              byte-stable across calls.",
    anchors: [
        crate::tree::build,
        crate::tree::expand,
        crate::tree::build_model,
        crate::tree::build_embodied_index,
        crate::tree::collect_warnings,
        crate::tree::collect_edges,
        crate::tree::place_concepts,
        crate::tree::compute_lca,
        crate::tree::ancestor_chain,
        crate::tree::attribute_to_concept,
        crate::tree::normalise_path,
        crate::tree::lib_name_from_file,
        crate::tree::lib_name_from_crate_dir,
        crate::tree::dedup_keep_first,
        crate::tree::count_macros,
        crate::tree::find_declaration_at_target,
        crate::tree::build_node,
        crate::tree::synthetic_root,
    ],
    tags: ["cast_spec_tree"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "canonical_tree_placement",
    why: "Every function in this module reads from the input Report or \
          intermediate Concept model and returns new structures; no \
          I/O, no mutation of inputs, no time/RNG dependency. Output \
          ordering is byte-stable via BTreeMap-backed traversal.",
}

cast::concept! {
    name: "canonical_tree_value",
    summary: "Hierarchical concept tree shape consumed by markdown \
              emit and the cast-web UI. Product types for the nodes \
              and supporting locations.",
    anchors: [
        crate::tree::CanonicalTree,
        crate::tree::ConceptNode,
        crate::tree::TreeWarning,
        crate::tree::DeclarationLoc,
        crate::tree::AnchorLeaf,
        crate::tree::MacroCounts,
    ],
    tags: ["cast_spec_tree"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::product_type,
    concept: "canonical_tree_value",
    why: "Each is a struct with all fields simultaneously inhabited.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::value_type,
    concept: "canonical_tree_value",
    why: "Cloneable, serialisable, no resource handles.",
}

cast::concept! {
    name: "canonical_tree_categories",
    summary: "Sum-typed discriminators on the tree: warning kind, \
              edge variant.",
    anchors: [
        crate::tree::TreeWarningKind,
        crate::tree::TreeEdge,
    ],
    tags: ["cast_spec_tree"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::sum_type,
    concept: "canonical_tree_categories",
    why: "Each is an enum with a closed set of variants.",
}

cast::continues_in! {
    target: cast_stdlib::architecture::typed_handle,
    concept: "canonical_tree_value",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::architecture::classifier_dispatch,
    concept: "canonical_tree_categories",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::integration::format_converter,
    concept: "canonical_tree_placement",
    why: lazy,
}

// Re-export internal types used in tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_prefix_basic() {
        assert!(is_strict_prefix("a::b", "a::b::c"));
        assert!(!is_strict_prefix("a::b", "a::b"));
        assert!(!is_strict_prefix("a::b", "a::bc"));
        assert!(!is_strict_prefix("a::b::c", "a::b"));
    }

    #[test]
    fn lib_name_special_cases_cast_spec() {
        assert_eq!(lib_name_from_crate_dir("cast-spec"), "cast");
        assert_eq!(lib_name_from_crate_dir("cast-web"), "cast_web");
        assert_eq!(lib_name_from_crate_dir("cast-watch"), "cast_watch");
    }

    #[test]
    fn lib_name_from_file_walks_to_crates() {
        assert_eq!(
            lib_name_from_file("/abs/git/cast/crates/cast-web/src/lib.rs"),
            Some("cast_web".to_string())
        );
        assert_eq!(
            lib_name_from_file("/abs/git/cast/crates/cast-spec/src/parser/concept.rs"),
            Some("cast".to_string())
        );
        assert_eq!(lib_name_from_file("/abs/git/cast/Cast.cast"), None);
    }

    #[test]
    fn normalise_replaces_crate_prefix() {
        assert_eq!(normalise_path("crate::foo::Bar", Some("cast_web")), "cast_web::foo::Bar");
        assert_eq!(normalise_path("cast::foo", Some("cast_web")), "cast::foo");
        assert_eq!(normalise_path("crate", Some("cast_web")), "cast_web");
    }

    fn fake_concept(name: &str, lib: &str, embodied: &[&str]) -> Concept {
        Concept {
            name: name.to_string(),
            summary: String::new(),
            tags: vec![],
            declared_at: vec![],
            embodied: embodied.iter().map(|s| (*s).to_string()).collect(),
            primitive: vec![],
            decl_crate_lib: Some(lib.to_string()),
            parent_override: None,
        }
    }

    fn assert_no_cycle(placement: &Placement) {
        for start in placement.parent.keys() {
            let mut current: &str = start.as_str();
            let mut seen: BTreeSet<String> = BTreeSet::new();
            seen.insert(current.to_string());
            while let Some(p) = placement.parent.get(current) {
                if p == ROOT_NAME {
                    break;
                }
                if !seen.insert(p.clone()) {
                    panic!(
                        "cycle in placement.parent starting from {start}: visited {seen:?}"
                    );
                }
                current = p.as_str();
            }
        }
    }

    #[test]
    fn place_concepts_breaks_symmetric_orphan_pick() {
        // Two non-umbrella concepts in the same lib, equal max_depth and
        // anchor count. Without the cycle guard, `place_zero_anchor`
        // would symmetrically pick each as the other's umbrella → cycle.
        let mut model: BTreeMap<String, Concept> = BTreeMap::new();
        model.insert(
            "alpha".to_string(),
            fake_concept("alpha", "foo", &["foo::a::x", "foo::a::y"]),
        );
        model.insert(
            "beta".to_string(),
            fake_concept("beta", "foo", &["foo::b::x", "foo::b::y"]),
        );

        let placement = place_concepts(&model);
        assert_no_cycle(&placement);

        // At least one of them must end up at ROOT_NAME (the cycle would
        // have been A->B->A; the guard breaks it by leaving the second
        // one rooted).
        let alpha_p = placement.parent.get("alpha").map(String::as_str);
        let beta_p = placement.parent.get("beta").map(String::as_str);
        assert!(
            alpha_p == Some(ROOT_NAME) || beta_p == Some(ROOT_NAME),
            "expected one of (alpha, beta) at root; got alpha={alpha_p:?} beta={beta_p:?}"
        );
    }

    #[test]
    fn build_node_terminates_on_cyclic_placement() {
        // Defense-in-depth: even if a future placement bug ever
        // produces a cyclic children index, build_node must not
        // recurse infinitely.
        let mut model: BTreeMap<String, Concept> = BTreeMap::new();
        model.insert("a".to_string(), fake_concept("a", "x", &[]));
        model.insert("b".to_string(), fake_concept("b", "x", &[]));

        let mut placement = Placement::default();
        placement.parent.insert("a".to_string(), "b".to_string());
        placement.parent.insert("b".to_string(), "a".to_string());
        placement
            .children
            .insert("a".to_string(), vec!["b".to_string()]);
        placement
            .children
            .insert("b".to_string(), vec!["a".to_string()]);

        let counts: BTreeMap<String, MacroCounts> = BTreeMap::new();
        let warnings: BTreeMap<String, Vec<TreeWarning>> = BTreeMap::new();
        let embodied_index: BTreeMap<&str, &str> = BTreeMap::new();
        let mut visited: BTreeSet<&str> = BTreeSet::new();

        let node = build_node(
            "a",
            &model,
            &placement,
            &counts,
            &warnings,
            &embodied_index,
            &mut visited,
        )
        .expect("build_node should return Some on first entry");

        assert_eq!(node.name, "a");
        assert_eq!(node.children.len(), 1, "a's only child is b");
        assert_eq!(node.children[0].name, "b");
        assert!(
            node.children[0].children.is_empty(),
            "cycle stopped: b's child (back to a) must not be expanded"
        );
    }

    #[test]
    fn would_create_cycle_detects_back_edge() {
        let mut parent: BTreeMap<String, String> = BTreeMap::new();
        parent.insert("a".to_string(), "b".to_string());
        parent.insert("b".to_string(), ROOT_NAME.to_string());
        // Attempting to set b -> a would close a cycle (a's chain
        // contains b).
        assert!(would_create_cycle("a", "b", &parent));
        // Setting a -> b is fine (already the case).
        assert!(!would_create_cycle("b", "a", &parent));
    }

    #[test]
    fn ancestor_chain_terminates_on_pre_existing_cycle() {
        // Defensive guard: even if parent_of somehow gets a cycle,
        // ancestor_chain must not loop forever pushing to its Vec.
        let mut parent: BTreeMap<String, String> = BTreeMap::new();
        parent.insert("a".to_string(), "b".to_string());
        parent.insert("b".to_string(), "a".to_string());
        let chain = ancestor_chain("a", &parent);
        // We don't assert the exact contents — just that the call
        // returns. Without the guard this would never reach the
        // assertion.
        assert!(!chain.is_empty());
    }
}
