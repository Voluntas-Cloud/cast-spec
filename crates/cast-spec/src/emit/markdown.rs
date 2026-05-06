//! Markdown rendering of a `Report`.
//!
//! Output shape:
//!
//! ```markdown
//! # cast-extract report
//!
//! ## Summary
//! - Found N invocations
//! - ...
//!
//! ## Tag: <name>
//!
//! ### cast::<macro> at file.rs:42  (kind, status)
//! - tag: ..., since: ..., note: ...
//! - paths:
//!   - resolved: sample::types::IntentId  (governs[0])
//!   - UNRESOLVED: sample::renamed::Thing  (instead_at) — <error>
//! - io: ok  target=...
//! - pipeline: <empty if none>
//!
//! ## Concept graph
//! - <concept-name>: D declarations, E edges
//!   - decl  file:line  N anchors
//!   - →     file:line  →  target
//! ```
//!
//! Designed to render as a self-contained PR comment — sections use
//! `##`/`###` headings so a wrapping document at `#` still nests cleanly.

use super::model::{
    AnnotationKind, ConceptGraphReport, ConceptWarningKindRepr, EdgeKindRepr, IoOutcomeRepr,
    IoReport, InvocationReport, InvocationStatus, PathOutcomeRepr, PathReport,
    PipelineReportEntry, Report, Summary, UNTAGGED,
};
use std::fmt::Write;

pub fn render(report: &Report) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "# cast-extract report\n");
    render_summary(&mut s, &report.summary);
    render_groups(&mut s, report);
    render_concept_graph(&mut s, &report.concept_graph);
    s
}

fn render_summary(s: &mut String, sum: &Summary) {
    let _ = writeln!(s, "## Summary\n");
    let _ = writeln!(s, "- Found {} invocation(s)", sum.invocations);
    let _ = writeln!(
        s,
        "- {} parsed, {} parse error(s), {} unimplemented",
        sum.parsed, sum.parse_errors, sum.unimplemented
    );
    let _ = writeln!(
        s,
        "- Paths: {} resolved, {} unresolved",
        sum.paths_resolved, sum.paths_unresolved
    );
    let _ = writeln!(
        s,
        "- IO targets: {} ok, {} unresolved",
        sum.io_ok, sum.io_unresolved
    );
    let _ = writeln!(s, "- Pipeline structural error(s): {}", sum.pipeline_errors);
    let _ = writeln!(s, "- Concept graph warnings: {}\n", sum.graph_warnings);
}

fn render_groups(s: &mut String, report: &Report) {
    if report.groups.is_empty() {
        return;
    }
    for (tag, invs) in &report.groups {
        let _ = writeln!(s, "## Tag: {tag}\n");
        for inv in invs {
            render_invocation(s, inv);
        }
    }
}

fn render_invocation(s: &mut String, inv: &InvocationReport) {
    let kind = annotation_kind_label(inv.kind);
    let status = match &inv.status {
        InvocationStatus::Ok => "ok".to_string(),
        InvocationStatus::ParseError { message } => format!("parse error: {message}"),
        InvocationStatus::Unimplemented { macro_path } => {
            format!("unimplemented: {macro_path}")
        }
    };
    let _ = writeln!(
        s,
        "### `{}` at {}:{}  ({kind}, {status})\n",
        inv.macro_path, inv.file, inv.line
    );

    let mut common_parts: Vec<String> = Vec::new();
    if !inv.tags.is_empty() {
        let joined = inv
            .tags
            .iter()
            .map(|t| format!("`{t}`"))
            .collect::<Vec<_>>()
            .join(", ");
        let label = if inv.tags.len() == 1 { "tag" } else { "tags" };
        common_parts.push(format!("{label}: {joined}"));
    }
    if let Some(v) = &inv.since {
        common_parts.push(format!("since: `{v}`"));
    }
    if let Some(n) = &inv.note {
        common_parts.push(format!("note: {n}"));
    }
    if !common_parts.is_empty() {
        let _ = writeln!(s, "- {}", common_parts.join(", "));
    }

    if !inv.paths.is_empty() {
        let _ = writeln!(s, "- paths:");
        for p in &inv.paths {
            render_path(s, p);
        }
    }

    if let Some(io) = &inv.io {
        render_io(s, io);
    }

    if !inv.pipeline.is_empty() {
        let _ = writeln!(s, "- pipeline structural errors:");
        for entry in &inv.pipeline {
            render_pipeline(s, entry);
        }
    }

    let _ = writeln!(s);
}

fn render_path(s: &mut String, p: &PathReport) {
    match p.outcome {
        PathOutcomeRepr::Resolved => {
            let _ = writeln!(s, "  - OK         `{}`  ({})", p.text, p.field);
        }
        PathOutcomeRepr::Unresolved => {
            let err = p.error.as_deref().unwrap_or("(no detail)");
            let _ = writeln!(
                s,
                "  - UNRESOLVED `{}`  ({}) — {err}",
                p.text, p.field
            );
        }
    }
}

fn render_io(s: &mut String, io: &IoReport) {
    let label = match io.outcome {
        IoOutcomeRepr::Ok => "OK",
        IoOutcomeRepr::FileMissing => "FILE MISSING",
        IoOutcomeRepr::RfcSyntaxInvalid => "RFC SYNTAX INVALID",
        IoOutcomeRepr::AnchorMissing => "ANCHOR MISSING",
    };
    let anchor = io
        .anchor
        .as_deref()
        .map(|a| format!(" anchor=`{a}`"))
        .unwrap_or_default();
    let _ = writeln!(
        s,
        "- io: {label}  target=`{}`{anchor} — {}",
        io.target, io.message
    );
}

fn render_pipeline(s: &mut String, entry: &PipelineReportEntry) {
    match entry {
        PipelineReportEntry::UnpermittedCycle { cycle } => {
            let _ = writeln!(s, "  - cycle without `cyclic: true`: {}", cycle.join(" -> "));
        }
        PipelineReportEntry::UnconnectedStage { stage } => {
            let _ = writeln!(s, "  - unconnected stage: `{stage}`");
        }
        PipelineReportEntry::DisconnectedComponents { components } => {
            let parts: Vec<String> = components
                .iter()
                .map(|c| format!("[{}]", c.join(", ")))
                .collect();
            let _ = writeln!(s, "  - disconnected components: {}", parts.join(" / "));
        }
    }
}

fn render_concept_graph(s: &mut String, g: &ConceptGraphReport) {
    if g.concepts.is_empty() && g.warnings.is_empty() {
        return;
    }
    let _ = writeln!(s, "## Concept graph\n");
    for (name, node) in &g.concepts {
        let _ = writeln!(
            s,
            "- **{name}**: {} declaration(s), {} edge(s)",
            node.declarations.len(),
            node.edges.len()
        );
        for d in &node.declarations {
            let _ = writeln!(
                s,
                "  - decl  {}:{}  ({} anchor{})",
                d.file,
                d.line,
                d.anchors.len(),
                if d.anchors.len() == 1 { "" } else { "s" }
            );
        }
        for e in &node.edges {
            let arrow = match e.kind {
                EdgeKindRepr::ContinuesIn => "->",
                EdgeKindRepr::IoContinuesIn => "~>",
            };
            let lang = e
                .lang
                .as_deref()
                .map(|l| format!(" [{l}]"))
                .unwrap_or_default();
            let _ = writeln!(
                s,
                "  - {arrow}  {}:{}  =>  `{}`{lang}",
                e.file, e.line, e.target
            );
        }
    }
    if !g.warnings.is_empty() {
        let _ = writeln!(s, "\n### Warnings\n");
        for w in &g.warnings {
            let kind = match w.kind {
                ConceptWarningKindRepr::Orphan => "orphan",
                ConceptWarningKindRepr::Undeclared => "undeclared",
                ConceptWarningKindRepr::TargetNotInAnchors => "target-not-in-anchors",
                ConceptWarningKindRepr::UnresolvedParent => "unresolved-parent",
                ConceptWarningKindRepr::MissingStdlibGrounding => "missing-stdlib-grounding",
            };
            let _ = writeln!(s, "- **{}** ({kind}): {}", w.concept, w.message);
        }
    }
    let _ = writeln!(s);
}

fn annotation_kind_label(k: AnnotationKind) -> &'static str {
    match k {
        AnnotationKind::Compare => "compare",
        AnnotationKind::Pipeline => "pipeline",
        AnnotationKind::Tier => "tier",
        AnnotationKind::Matrix => "matrix",
        AnnotationKind::Rule => "rule",
        AnnotationKind::AntiPattern => "anti_pattern",
        AnnotationKind::GutCheck => "gut_check",
        AnnotationKind::ContinuesIn => "continues_in",
        AnnotationKind::IoContinuesIn => "io::continues_in",
        AnnotationKind::Concept => "concept",
        AnnotationKind::Policy => "policy",
        AnnotationKind::Unknown => "unknown",
    }
}

#[allow(dead_code)]
pub(crate) fn untagged_label() -> &'static str {
    UNTAGGED
}

