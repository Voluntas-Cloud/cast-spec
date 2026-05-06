//! Serializable report of a cast-extract run.
//!
//! Built once during the CLI's main loop, then handed to a renderer
//! (text / json / markdown). Keeping this distinct from the validator
//! types means JSON consumers see a stable shape that doesn't change
//! when a `PathError` variant is added.
//!
//! Grouping by `tag` is the headline organising principle — `tag`
//! identifies a workstream (e.g. `"trust_foundation"`, `"reconciler"`),
//! and the report's primary structure is a flat list of tag groups so
//! a CI consumer can filter directly without traversing.

use crate::graph::{ConceptGraph, EdgeKind, EdgeTarget, GraphWarning, WarningKind};
use crate::model::Location;
use crate::parser::common::WhyValue;
use crate::parser::policy::{InlineInRustPolicy, Policy};
use crate::parser::CastAnnotation;
use crate::validator::{
    IoDiagnostic, IoOutcome, PathDiagnostic, PathOutcome, PipelineDiagKind, PipelineDiagnostic,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Sentinel tag bucket name for invocations that didn't set `tag:`.
/// Chosen so it sorts last under BTreeMap's lexicographic ordering and
/// reads unambiguously in both JSON and Markdown.
pub const UNTAGGED: &str = "(untagged)";

#[derive(Debug, Serialize)]
pub struct Report {
    pub summary: Summary,
    /// One bucket per `tag:` value. Use a `BTreeMap` so JSON output is
    /// deterministic — same input → byte-identical report.
    pub groups: BTreeMap<String, Vec<InvocationReport>>,
    pub concept_graph: ConceptGraphReport,
    /// Repo-level `cast::policy!` declarations. Empty when no policy
    /// block exists. Multiple entries are allowed today (one per
    /// declaration); enforcement uses the first non-None value per
    /// field. A future commit may add a `duplicate_policy` warning.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policies: Vec<PolicyDecl>,
    /// Soft warnings emitted by the policy-enforcement pass after the
    /// graph has been built. Independent of `concept_graph.warnings`
    /// — those are graph-shape diagnostics, these are convention
    /// violations the repo opted into via `cast::policy!`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_warnings: Vec<PolicyWarning>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct Summary {
    /// Total cast::*! invocations discovered in the workspace.
    pub invocations: usize,
    pub parsed: usize,
    pub parse_errors: usize,
    pub unimplemented: usize,
    pub paths_resolved: usize,
    pub paths_unresolved: usize,
    pub io_ok: usize,
    pub io_unresolved: usize,
    pub pipeline_errors: usize,
    pub graph_warnings: usize,
    /// Number of `cast::policy!` violations surfaced by the
    /// enforcement pass. Independent count from `graph_warnings`.
    #[serde(default)]
    pub policy_warnings: usize,
}

/// Wire-shape mirror of `parser::policy::Policy`. The parser enums
/// (`LayoutPolicy`, `InlineInRustPolicy`) lower to their canonical
/// source-form strings here so the report stays JSON-stable and
/// downstream consumers don't need to know the parser type names.
#[derive(Debug, Serialize, Clone)]
pub struct PolicyDecl {
    /// `layout: ...` — `sidecar_only`, `sidecar_preferred`,
    /// `inline_only`, `mixed`. `None` when the field was absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    /// `inline_in_rust: ...` — `allow`, `warn`, `forbid`. `None`
    /// when absent (resolved from `layout` at enforcement time).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_in_rust: Option<String>,
    /// `umbrella_files: [...]`. Empty when absent.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub umbrella_files: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Where the `cast::policy!` block was declared.
    pub file: String,
    pub line: usize,
}

#[derive(Debug, Serialize, Clone)]
pub struct PolicyWarning {
    pub kind: PolicyWarningKindRepr,
    /// File where the violation occurred — NOT where the policy was
    /// declared. For `inline_macro_forbidden`, this is the `.rs` file
    /// the offending `cast::*!` block lives in.
    pub file: String,
    pub line: usize,
    /// Optional macro path for inline-violation warnings (`cast::concept`,
    /// `cast::rule`, ...). `None` for warnings that don't pertain to a
    /// specific invocation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macro_path: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyWarningKindRepr {
    /// A `cast::*!` block was found in a `.rs` file while the active
    /// `cast::policy!` declared `inline_in_rust: forbid` (or `warn`,
    /// or derived `forbid` from `layout: sidecar_only`).
    InlineMacroForbidden,
}

#[derive(Debug, Serialize, Clone)]
pub struct InvocationReport {
    pub macro_path: String,
    pub kind: AnnotationKind,
    pub file: String,
    pub line: usize,
    /// Always present in JSON: empty Vec when the invocation carried no
    /// `tag:`/`tags:` (it lands in the `(untagged)` bucket); otherwise
    /// the full list of declared tags.
    pub tags: Vec<String>,
    pub since: Option<String>,
    pub note: Option<String>,
    /// Result of parsing the macro body. `Ok` invocations also carry
    /// validation results; failed ones carry just the error message.
    pub status: InvocationStatus,
    /// One entry per Rust path field on the annotation (governs[i],
    /// instead_at, anchors[j], ...). Empty for parse failures.
    pub paths: Vec<PathReport>,
    /// Set only for `cast::io::continues_in!`.
    pub io: Option<IoReport>,
    /// Set only for `cast::pipeline!`. May be empty if validation passed.
    pub pipeline: Vec<PipelineReportEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationKind {
    Compare,
    Pipeline,
    Tier,
    Matrix,
    Rule,
    AntiPattern,
    GutCheck,
    ContinuesIn,
    IoContinuesIn,
    Concept,
    Policy,
    /// The macro was a `cast::*!` invocation but parser doesn't know it
    /// (or body parsing failed). The `status` carries the detail.
    Unknown,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum InvocationStatus {
    Ok,
    ParseError { message: String },
    Unimplemented { macro_path: String },
}

#[derive(Debug, Serialize, Clone)]
pub struct PathReport {
    pub field: String,
    pub text: String,
    pub outcome: PathOutcomeRepr,
    /// Human-readable explanation for unresolved paths; `None` on success.
    pub error: Option<String>,
    /// Anchor role for `cast::concept!` `anchors[i]` paths — `embodied`
    /// (default) or `primitive` (when source carried `CAST::AS_PRIMITIVE::`).
    /// `None` for non-anchor fields (governs, target, instead_at, etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<AnchorRoleRepr>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PathOutcomeRepr {
    Resolved,
    Unresolved,
}

#[derive(Debug, Serialize, Clone)]
pub struct IoReport {
    pub target: String,
    pub anchor: Option<String>,
    pub outcome: IoOutcomeRepr,
    /// Human-readable explanation. Always populated.
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum IoOutcomeRepr {
    Ok,
    FileMissing,
    RfcSyntaxInvalid,
    AnchorMissing,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PipelineReportEntry {
    UnpermittedCycle { cycle: Vec<String> },
    UnconnectedStage { stage: String },
    DisconnectedComponents { components: Vec<Vec<String>> },
}

#[derive(Debug, Serialize)]
pub struct ConceptGraphReport {
    pub concepts: BTreeMap<String, ConceptReport>,
    pub warnings: Vec<ConceptWarningReport>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ConceptReport {
    pub declarations: Vec<DeclarationReport>,
    pub edges: Vec<EdgeReport>,
}

#[derive(Debug, Serialize, Clone)]
pub struct DeclarationReport {
    pub file: String,
    pub line: usize,
    pub summary: String,
    pub anchors: Vec<AnchorReport>,
    /// `tags:` (or legacy single `tag:`) field on the originating
    /// `cast::concept!` block. Surfaces in concept-stream query results
    /// so clients can filter on tags without re-joining invocations.
    /// Empty Vec serialises as an empty JSON array.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Optional explicit parent concept name from `parent:` on the
    /// `cast::concept!` block. Read by tree::place_concepts as a
    /// hard parent override. `None` (serialised as omitted) means
    /// the strict-prefix-on-anchors rule applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct AnchorReport {
    pub path: String,
    /// `embodied` (the default — the concept embodies / instantiates
    /// this code) or `primitive` (the concept rests on this atom but
    /// does not model its insides). Surfaced so clients can ask
    /// "what does concept X rest on?" without re-deriving the role.
    pub role: AnchorRoleRepr,
}

#[derive(Debug, Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AnchorRoleRepr {
    Embodied,
    Primitive,
}

impl From<crate::parser::concept::AnchorRole> for AnchorRoleRepr {
    fn from(r: crate::parser::concept::AnchorRole) -> Self {
        match r {
            crate::parser::concept::AnchorRole::Embodied => Self::Embodied,
            crate::parser::concept::AnchorRole::Primitive => Self::Primitive,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct EdgeReport {
    pub kind: EdgeKindRepr,
    pub file: String,
    pub line: usize,
    pub target: String,
    /// `lang` is set only for `io_continues_in` edges.
    pub lang: Option<String>,
    /// `why:` value as written. `Some(Lazy)` is the deferred marker;
    /// `Some(Prose { text })` carries the writer's prose; `None` means
    /// the field was omitted entirely. Wire shape:
    /// `{"kind":"lazy"}` or `{"kind":"prose","text":"..."}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub why: Option<WhyValue>,
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKindRepr {
    ContinuesIn,
    IoContinuesIn,
}

#[derive(Debug, Serialize)]
pub struct ConceptWarningReport {
    pub concept: String,
    pub kind: ConceptWarningKindRepr,
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ConceptWarningKindRepr {
    Orphan,
    Undeclared,
    TargetNotInAnchors,
    UnresolvedParent,
    MissingStdlibGrounding,
}

// ─── Conversions from internal types ─────────────────────────────────────

impl From<&PathDiagnostic> for PathReport {
    fn from(d: &PathDiagnostic) -> Self {
        let (outcome, error) = match &d.outcome {
            PathOutcome::Resolved => (PathOutcomeRepr::Resolved, None),
            PathOutcome::Unresolved(e) => (PathOutcomeRepr::Unresolved, Some(e.to_string())),
        };
        PathReport {
            field: d.field.clone(),
            text: d.path_text.clone(),
            outcome,
            error,
            role: d.role.map(AnchorRoleRepr::from),
        }
    }
}

impl From<&IoDiagnostic> for IoReport {
    fn from(d: &IoDiagnostic) -> Self {
        let outcome = match &d.outcome {
            IoOutcome::Ok => IoOutcomeRepr::Ok,
            IoOutcome::FileMissing { .. } => IoOutcomeRepr::FileMissing,
            IoOutcome::RfcSyntaxInvalid => IoOutcomeRepr::RfcSyntaxInvalid,
            IoOutcome::AnchorMissing { .. } => IoOutcomeRepr::AnchorMissing,
        };
        IoReport {
            target: d.target.clone(),
            anchor: d.anchor.clone(),
            outcome,
            message: d.outcome.to_string(),
        }
    }
}

impl From<&PipelineDiagnostic> for PipelineReportEntry {
    fn from(d: &PipelineDiagnostic) -> Self {
        match &d.kind {
            PipelineDiagKind::UnpermittedCycle { cycle } => PipelineReportEntry::UnpermittedCycle {
                cycle: cycle.clone(),
            },
            PipelineDiagKind::UnconnectedStage { stage } => {
                PipelineReportEntry::UnconnectedStage {
                    stage: stage.clone(),
                }
            }
            PipelineDiagKind::DisconnectedComponents { components } => {
                PipelineReportEntry::DisconnectedComponents {
                    components: components.clone(),
                }
            }
        }
    }
}

/// Pull the canonical tag list for a parsed annotation. Every
/// `CastAnnotation` variant carries `pub common: CommonFields`, so the
/// match is mechanical — but it's centralised here so the renderers
/// don't replicate the variant list.
pub fn tags_of(annotation: &CastAnnotation) -> &[String] {
    let common = match annotation {
        CastAnnotation::Compare(c) => &c.common,
        CastAnnotation::Pipeline(c) => &c.common,
        CastAnnotation::Tier(c) => &c.common,
        CastAnnotation::Matrix(c) => &c.common,
        CastAnnotation::Rule(c) => &c.common,
        CastAnnotation::AntiPattern(c) => &c.common,
        CastAnnotation::GutCheck(c) => &c.common,
        CastAnnotation::ContinuesIn(c) => &c.common,
        CastAnnotation::IoContinuesIn(c) => &c.common,
        CastAnnotation::Concept(c) => &c.common,
        CastAnnotation::Policy(c) => &c.common,
    };
    &common.tags
}

pub fn since_of(annotation: &CastAnnotation) -> Option<&str> {
    let common = match annotation {
        CastAnnotation::Compare(c) => &c.common,
        CastAnnotation::Pipeline(c) => &c.common,
        CastAnnotation::Tier(c) => &c.common,
        CastAnnotation::Matrix(c) => &c.common,
        CastAnnotation::Rule(c) => &c.common,
        CastAnnotation::AntiPattern(c) => &c.common,
        CastAnnotation::GutCheck(c) => &c.common,
        CastAnnotation::ContinuesIn(c) => &c.common,
        CastAnnotation::IoContinuesIn(c) => &c.common,
        CastAnnotation::Concept(c) => &c.common,
        CastAnnotation::Policy(c) => &c.common,
    };
    common.since.as_deref()
}

pub fn note_of(annotation: &CastAnnotation) -> Option<&str> {
    let common = match annotation {
        CastAnnotation::Compare(c) => &c.common,
        CastAnnotation::Pipeline(c) => &c.common,
        CastAnnotation::Tier(c) => &c.common,
        CastAnnotation::Matrix(c) => &c.common,
        CastAnnotation::Rule(c) => &c.common,
        CastAnnotation::AntiPattern(c) => &c.common,
        CastAnnotation::GutCheck(c) => &c.common,
        CastAnnotation::ContinuesIn(c) => &c.common,
        CastAnnotation::IoContinuesIn(c) => &c.common,
        CastAnnotation::Concept(c) => &c.common,
        CastAnnotation::Policy(c) => &c.common,
    };
    common.note.as_deref()
}

pub fn kind_of(annotation: &CastAnnotation) -> AnnotationKind {
    match annotation {
        CastAnnotation::Compare(_) => AnnotationKind::Compare,
        CastAnnotation::Pipeline(_) => AnnotationKind::Pipeline,
        CastAnnotation::Tier(_) => AnnotationKind::Tier,
        CastAnnotation::Matrix(_) => AnnotationKind::Matrix,
        CastAnnotation::Rule(_) => AnnotationKind::Rule,
        CastAnnotation::AntiPattern(_) => AnnotationKind::AntiPattern,
        CastAnnotation::GutCheck(_) => AnnotationKind::GutCheck,
        CastAnnotation::ContinuesIn(_) => AnnotationKind::ContinuesIn,
        CastAnnotation::IoContinuesIn(_) => AnnotationKind::IoContinuesIn,
        CastAnnotation::Concept(_) => AnnotationKind::Concept,
        CastAnnotation::Policy(_) => AnnotationKind::Policy,
    }
}

impl From<&ConceptGraph> for ConceptGraphReport {
    fn from(g: &ConceptGraph) -> Self {
        let warnings = crate::graph::warnings(g)
            .iter()
            .map(ConceptWarningReport::from)
            .collect();
        let concepts = g
            .concepts
            .iter()
            .map(|(name, node)| {
                let declarations = node
                    .declarations
                    .iter()
                    .map(|d| DeclarationReport {
                        file: d.location.file.display().to_string(),
                        line: d.location.line,
                        summary: d.summary.clone(),
                        anchors: d
                            .anchors
                            .iter()
                            .map(|a| AnchorReport {
                                path: a.path.clone(),
                                role: a.role.into(),
                            })
                            .collect(),
                        tags: d.tags.clone(),
                        parent: d.parent.clone(),
                    })
                    .collect();
                let edges = node
                    .edges
                    .iter()
                    .map(|e| {
                        let (target, lang) = match &e.target {
                            EdgeTarget::Rust(t) => (t.clone(), None),
                            EdgeTarget::External { target, lang } => {
                                (target.clone(), Some(format!("{lang:?}")))
                            }
                        };
                        EdgeReport {
                            kind: match e.kind {
                                EdgeKind::ContinuesIn => EdgeKindRepr::ContinuesIn,
                                EdgeKind::IoContinuesIn => EdgeKindRepr::IoContinuesIn,
                            },
                            file: e.location.file.display().to_string(),
                            line: e.location.line,
                            target,
                            lang,
                            why: e.why.clone(),
                        }
                    })
                    .collect();
                (
                    name.clone(),
                    ConceptReport {
                        declarations,
                        edges,
                    },
                )
            })
            .collect();
        ConceptGraphReport {
            concepts,
            warnings,
        }
    }
}

impl From<&GraphWarning> for ConceptWarningReport {
    fn from(w: &GraphWarning) -> Self {
        let kind = match &w.kind {
            WarningKind::Orphan => ConceptWarningKindRepr::Orphan,
            WarningKind::Undeclared => ConceptWarningKindRepr::Undeclared,
            WarningKind::TargetNotInAnchors { .. } => ConceptWarningKindRepr::TargetNotInAnchors,
            WarningKind::UnresolvedParent { .. } => ConceptWarningKindRepr::UnresolvedParent,
            WarningKind::MissingStdlibGrounding => ConceptWarningKindRepr::MissingStdlibGrounding,
        };
        ConceptWarningReport {
            concept: w.concept.clone(),
            kind,
            message: w.kind.to_string(),
        }
    }
}

impl ConceptGraphReport {
    /// Recompute warnings against the *merged* concept graph.
    ///
    /// Mirrors the rules in [`crate::graph::warnings`] but operates on
    /// the report-shaped `ConceptGraphReport` so `merge_reports` can
    /// re-derive warnings after declarations and edges from multiple
    /// per-pass reports have been folded together. Without this, the
    /// merged `summary.graph_warnings` is a sum of per-pass counts and
    /// double-counts any concept whose declaration lives in one pass
    /// (e.g. a Rust `cast::concept!` block) and whose edges live in
    /// another (e.g. `cast::io::continues_in!` blocks discovered in
    /// `.cast` files) — even though the merged graph is sound.
    pub fn compute_warnings(&self) -> Vec<ConceptWarningReport> {
        let mut out = Vec::new();
        let known_concept_names: std::collections::BTreeSet<&str> =
            self.concepts.keys().map(|s| s.as_str()).collect();
        for (name, node) in &self.concepts {
            let total_refs = node.declarations.len() + node.edges.len();
            if total_refs <= 1 {
                out.push(ConceptWarningReport {
                    concept: name.clone(),
                    kind: ConceptWarningKindRepr::Orphan,
                    message: "orphan (only one reference)".to_string(),
                });
                continue;
            }
            if node.declarations.is_empty() {
                out.push(ConceptWarningReport {
                    concept: name.clone(),
                    kind: ConceptWarningKindRepr::Undeclared,
                    message: "no concept! declaration".to_string(),
                });
                continue;
            }
            let mut declared: Vec<&str> = Vec::new();
            for decl in &node.declarations {
                for a in &decl.anchors {
                    declared.push(a.path.as_str());
                }
            }
            // Primitive concepts (0 declared anchors) are open to
            // merge-by-name extension via continues_in!; there is no
            // closed anchor set to check against. Mirrors the same
            // suppression in graph::warnings.
            if !declared.is_empty() {
                for edge in &node.edges {
                    if !matches!(edge.kind, EdgeKindRepr::ContinuesIn) {
                        continue;
                    }
                    if !declared.contains(&edge.target.as_str()) {
                        out.push(ConceptWarningReport {
                            concept: name.clone(),
                            kind: ConceptWarningKindRepr::TargetNotInAnchors,
                            message: format!(
                                "target `{target}` at {file}:{line} not in declared anchors",
                                target = edge.target,
                                file = edge.file,
                                line = edge.line,
                            ),
                        });
                    }
                }
            }

            // UnresolvedParent: parent: field names a concept that
            // doesn't exist in the merged graph.
            for decl in &node.declarations {
                let Some(parent_name) = decl.parent.as_deref() else { continue };
                if known_concept_names.contains(parent_name) {
                    continue;
                }
                out.push(ConceptWarningReport {
                    concept: name.clone(),
                    kind: ConceptWarningKindRepr::UnresolvedParent,
                    message: format!(
                        "parent `{parent_name}` at {file}:{line} doesn't name an existing concept",
                        file = decl.file,
                        line = decl.line,
                    ),
                });
            }

            // MissingStdlibGrounding: non-umbrella concept with no
            // continues_in! to a cast_stdlib::* target. Umbrella =
            // ≥2 embodied anchors at module depth ≤ 2 (matches the
            // tree.rs heuristic).
            let mut max_embodied_depth: usize = 0;
            let mut embodied_count: usize = 0;
            for decl in &node.declarations {
                for a in &decl.anchors {
                    if matches!(a.role, AnchorRoleRepr::Embodied) {
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
                    matches!(e.kind, EdgeKindRepr::ContinuesIn)
                        && e.target.starts_with("cast_stdlib::")
                });
                if !grounded {
                    out.push(ConceptWarningReport {
                        concept: name.clone(),
                        kind: ConceptWarningKindRepr::MissingStdlibGrounding,
                        message: "no continues_in! to a cast_stdlib::* primitive".to_string(),
                    });
                }
            }
        }
        out
    }

}

impl PolicyDecl {
    /// Lower a parsed `Policy` plus the source location of its
    /// `cast::policy!` block to the wire-shape value carried in
    /// `Report.policies`.
    pub fn from_parsed(policy: &Policy, location: &Location) -> Self {
        PolicyDecl {
            layout: policy.layout.map(|l| l.as_str().to_string()),
            inline_in_rust: policy.inline_in_rust.map(|i| i.as_str().to_string()),
            umbrella_files: policy.umbrella_files.clone(),
            note: policy.common.note.clone(),
            since: policy.common.since.clone(),
            tags: policy.common.tags.clone(),
            file: location.file.display().to_string(),
            line: location.line,
        }
    }
}

/// Resolve the effective `inline_in_rust` setting for the merged
/// report. When a `cast::policy!` block sets `inline_in_rust:`
/// explicitly, that wins. Otherwise the value is derived from
/// `layout:`:
///
/// - `sidecar_only`     → `forbid`
/// - `sidecar_preferred`→ `warn`
/// - any other / absent → `allow`
///
/// Multiple policies: the first decl that pins `inline_in_rust`
/// explicitly wins; if none do, the first decl with a `layout` wins.
/// Empty policy list falls through to `allow`.
fn effective_inline_in_rust(policies: &[PolicyDecl]) -> InlineInRustPolicy {
    for p in policies {
        if let Some(ref s) = p.inline_in_rust {
            return match s.as_str() {
                "forbid" => InlineInRustPolicy::Forbid,
                "warn" => InlineInRustPolicy::Warn,
                _ => InlineInRustPolicy::Allow,
            };
        }
    }
    for p in policies {
        if let Some(ref s) = p.layout {
            return match s.as_str() {
                "sidecar_only" => InlineInRustPolicy::Forbid,
                "sidecar_preferred" => InlineInRustPolicy::Warn,
                _ => InlineInRustPolicy::Allow,
            };
        }
    }
    InlineInRustPolicy::Allow
}

/// Run the `cast::policy!` enforcement pass over a merged Report.
/// Appends to `report.policy_warnings` and updates
/// `report.summary.policy_warnings`. Idempotent — clears prior
/// policy_warnings before recomputing, so callers can re-run after
/// merging additional reports.
///
/// Today's checks (item 3 of the policy rollout):
///
/// - `inline_in_rust: forbid|warn` — every parsed `cast::*!` block
///   that lives in a `.rs` file emits an `InlineMacroForbidden`
///   warning. The `cast::policy!` declaration itself is ALWAYS
///   exempt: the policy is allowed to live inline (it's the one
///   block that carries the rule that forbids the others).
///
/// `umbrella_files` enforcement and `layout: sidecar_only`'s
/// "every annotated .rs needs a sidecar" check land in item 2
/// alongside the Sidecar SpecSource variant.
pub fn apply_policy_warnings(report: &mut Report) {
    report.policy_warnings.clear();

    let effective = effective_inline_in_rust(&report.policies);
    if matches!(
        effective,
        InlineInRustPolicy::Forbid | InlineInRustPolicy::Warn
    ) {
        let level = effective.as_str();
        // Walk every InvocationReport across all tag buckets. The
        // same logical invocation appears in N buckets for an N-tag
        // block, so dedupe by (file, line, macro_path) on the way in.
        let mut seen: std::collections::BTreeSet<(String, usize, String)> =
            std::collections::BTreeSet::new();
        let mut surface: Vec<(String, usize, String)> = Vec::new();
        for invs in report.groups.values() {
            for inv in invs {
                if !inv.file.ends_with(".rs") {
                    continue;
                }
                if matches!(inv.kind, AnnotationKind::Policy) {
                    // The policy declaration itself is exempt — see
                    // doc comment above.
                    continue;
                }
                let key = (inv.file.clone(), inv.line, inv.macro_path.clone());
                if seen.insert(key.clone()) {
                    surface.push(key);
                }
            }
        }
        // Deterministic order: file then line then macro path.
        surface.sort();
        for (file, line, macro_path) in surface {
            let message = format!(
                "inline `{macro_path}!` in {file}:{line} violates `inline_in_rust: {level}` \
                 declared in cast::policy!"
            );
            report.policy_warnings.push(PolicyWarning {
                kind: PolicyWarningKindRepr::InlineMacroForbidden,
                file,
                line,
                macro_path: Some(macro_path),
                message,
            });
        }
    }

    report.summary.policy_warnings = report.policy_warnings.len();
}

cast::concept! {
    name: "report_wire_format",
    summary: "Serialisable report shape produced by analysis_runner \
              and consumed by report_renderers. Stable across \
              releases; downstream JSON consumers depend on field \
              names and ordering.",
    anchors: [
        crate::emit::model::Report,
        crate::emit::model::Summary,
        crate::emit::model::PolicyDecl,
        crate::emit::model::PolicyWarning,
        crate::emit::model::InvocationReport,
        crate::emit::model::PathReport,
        crate::emit::model::IoReport,
        crate::emit::model::ConceptGraphReport,
        crate::emit::model::ConceptReport,
        crate::emit::model::DeclarationReport,
        crate::emit::model::AnchorReport,
        crate::emit::model::EdgeReport,
    ],
    tags: ["cast_spec_emit", "wire_format"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::product_type,
    concept: "report_wire_format",
    why: "Each is a struct with all fields simultaneously inhabited.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::value_type,
    concept: "report_wire_format",
    why: "Cloneable, structurally compared, no resource handles or \
          mutable interior. The wire-format IS the value.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::serializable,
    concept: "report_wire_format",
    why: "Every field derives Serialize/Deserialize via serde; the \
          JSON shape is the public contract.",
}

cast::concept! {
    name: "report_wire_categories",
    summary: "Sum-typed category tags inside the report wire format. \
              Each enum is a closed set of variants for kind / outcome \
              / status discrimination.",
    anchors: [
        crate::emit::model::PolicyWarningKindRepr,
        crate::emit::model::AnnotationKind,
        crate::emit::model::InvocationStatus,
        crate::emit::model::PathOutcomeRepr,
        crate::emit::model::IoOutcomeRepr,
        crate::emit::model::PipelineReportEntry,
        crate::emit::model::AnchorRoleRepr,
        crate::emit::model::EdgeKindRepr,
    ],
    tags: ["cast_spec_emit", "wire_format"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::sum_type,
    concept: "report_wire_categories",
    why: "Each is an enum; exactly one variant inhabited per value.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::serializable,
    concept: "report_wire_categories",
    why: "All derive Serialize for inclusion in the JSON wire format.",
}

cast::continues_in! {
    target: cast_stdlib::architecture::typed_handle,
    concept: "report_wire_format",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::architecture::classifier_dispatch,
    concept: "report_wire_categories",
    why: lazy,
}

#[cfg(test)]
mod policy_tests {
    use super::*;

    fn mk_policy_decl(layout: Option<&str>, inline: Option<&str>) -> PolicyDecl {
        PolicyDecl {
            layout: layout.map(str::to_string),
            inline_in_rust: inline.map(str::to_string),
            umbrella_files: Vec::new(),
            note: None,
            since: None,
            tags: Vec::new(),
            file: "Cast.cast".to_string(),
            line: 1,
        }
    }

    fn mk_inv(file: &str, line: usize, kind: AnnotationKind, macro_path: &str) -> InvocationReport {
        InvocationReport {
            macro_path: macro_path.to_string(),
            kind,
            file: file.to_string(),
            line,
            tags: Vec::new(),
            since: None,
            note: None,
            status: InvocationStatus::Ok,
            paths: Vec::new(),
            io: None,
            pipeline: Vec::new(),
        }
    }

    fn mk_report(policies: Vec<PolicyDecl>, invs: Vec<InvocationReport>) -> Report {
        let mut groups: BTreeMap<String, Vec<InvocationReport>> = BTreeMap::new();
        groups.insert(UNTAGGED.to_string(), invs);
        Report {
            summary: Summary::default(),
            groups,
            concept_graph: ConceptGraphReport {
                concepts: BTreeMap::new(),
                warnings: Vec::new(),
            },
            policies,
            policy_warnings: Vec::new(),
        }
    }

    #[test]
    fn no_policy_means_no_warnings() {
        let mut r = mk_report(
            vec![],
            vec![mk_inv("apps/foo/src/lib.rs", 10, AnnotationKind::Concept, "cast::concept")],
        );
        apply_policy_warnings(&mut r);
        assert!(r.policy_warnings.is_empty());
        assert_eq!(r.summary.policy_warnings, 0);
    }

    #[test]
    fn forbid_emits_one_per_inline_block() {
        let mut r = mk_report(
            vec![mk_policy_decl(None, Some("forbid"))],
            vec![
                mk_inv("apps/foo/src/lib.rs", 10, AnnotationKind::Concept, "cast::concept"),
                mk_inv("apps/foo/src/other.rs", 5, AnnotationKind::Rule, "cast::rule"),
            ],
        );
        apply_policy_warnings(&mut r);
        assert_eq!(r.policy_warnings.len(), 2);
        assert_eq!(r.summary.policy_warnings, 2);
        assert!(r.policy_warnings.iter().all(|w| matches!(
            w.kind,
            PolicyWarningKindRepr::InlineMacroForbidden
        )));
    }

    #[test]
    fn allow_emits_no_warnings() {
        let mut r = mk_report(
            vec![mk_policy_decl(None, Some("allow"))],
            vec![mk_inv(
                "apps/foo/src/lib.rs",
                10,
                AnnotationKind::Concept,
                "cast::concept",
            )],
        );
        apply_policy_warnings(&mut r);
        assert!(r.policy_warnings.is_empty());
    }

    #[test]
    fn cast_files_are_exempt() {
        let mut r = mk_report(
            vec![mk_policy_decl(None, Some("forbid"))],
            vec![
                mk_inv("Cast.cast", 5, AnnotationKind::Concept, "cast::concept"),
                mk_inv("apps/foo/spec.cast", 1, AnnotationKind::Rule, "cast::rule"),
            ],
        );
        apply_policy_warnings(&mut r);
        // Both invocations live in *.cast files — `inline_in_rust` is
        // about .rs files only.
        assert!(r.policy_warnings.is_empty());
    }

    #[test]
    fn policy_decl_is_exempt_from_its_own_rule() {
        let mut r = mk_report(
            vec![mk_policy_decl(None, Some("forbid"))],
            vec![mk_inv(
                "apps/foo/src/lib.rs",
                3,
                AnnotationKind::Policy,
                "cast::policy",
            )],
        );
        apply_policy_warnings(&mut r);
        assert!(r.policy_warnings.is_empty());
    }

    #[test]
    fn layout_sidecar_only_derives_forbid() {
        let mut r = mk_report(
            vec![mk_policy_decl(Some("sidecar_only"), None)],
            vec![mk_inv(
                "apps/foo/src/lib.rs",
                10,
                AnnotationKind::Concept,
                "cast::concept",
            )],
        );
        apply_policy_warnings(&mut r);
        assert_eq!(r.policy_warnings.len(), 1);
    }

    #[test]
    fn explicit_inline_overrides_layout_derivation() {
        let mut r = mk_report(
            // layout says forbid, but inline_in_rust says allow — explicit wins.
            vec![mk_policy_decl(Some("sidecar_only"), Some("allow"))],
            vec![mk_inv(
                "apps/foo/src/lib.rs",
                10,
                AnnotationKind::Concept,
                "cast::concept",
            )],
        );
        apply_policy_warnings(&mut r);
        assert!(r.policy_warnings.is_empty());
    }

    #[test]
    fn rerunning_does_not_double_count() {
        let mut r = mk_report(
            vec![mk_policy_decl(None, Some("forbid"))],
            vec![mk_inv(
                "apps/foo/src/lib.rs",
                10,
                AnnotationKind::Concept,
                "cast::concept",
            )],
        );
        apply_policy_warnings(&mut r);
        apply_policy_warnings(&mut r);
        assert_eq!(r.policy_warnings.len(), 1);
        assert_eq!(r.summary.policy_warnings, 1);
    }

    #[test]
    fn multi_tag_invocation_dedupes_to_one_warning() {
        // Same invocation lands in multiple tag buckets when it
        // carries multiple tags. The dedup keyed on (file, line,
        // macro_path) keeps it to one warning.
        let inv = mk_inv("apps/foo/src/lib.rs", 10, AnnotationKind::Concept, "cast::concept");
        let mut groups: BTreeMap<String, Vec<InvocationReport>> = BTreeMap::new();
        groups.insert("tag-a".to_string(), vec![inv.clone()]);
        groups.insert("tag-b".to_string(), vec![inv]);
        let mut r = Report {
            summary: Summary::default(),
            groups,
            concept_graph: ConceptGraphReport {
                concepts: BTreeMap::new(),
                warnings: Vec::new(),
            },
            policies: vec![mk_policy_decl(None, Some("forbid"))],
            policy_warnings: Vec::new(),
        };
        apply_policy_warnings(&mut r);
        assert_eq!(r.policy_warnings.len(), 1);
    }
}
