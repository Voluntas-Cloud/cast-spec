//! Plain-text rendering of a `Report`. Replaces the ad-hoc `println!`
//! calls that lived in `main.rs` before the emit refactor.
//!
//! Format mirrors what the CLI used to print — one line per invocation,
//! one indented line per validation diagnostic, then a summary, then
//! the concept graph. Stable output so existing scripts that grep on
//! `"ok"` / `"ERR"` keep working.

use super::model::{
    AnnotationKind, ConceptGraphReport, EdgeKindRepr, InvocationReport, InvocationStatus,
    IoOutcomeRepr, PathOutcomeRepr, PipelineReportEntry, Report,
};
use std::fmt::Write;

pub fn render(report: &Report) -> String {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "found {} cast::*! invocations",
        report.summary.invocations
    );

    for invs in report.groups.values() {
        for inv in invs {
            render_invocation(&mut s, inv);
        }
    }

    let sum = &report.summary;
    let _ = writeln!(
        s,
        "\nsummary: {} parsed, {} parse errors, {} unimplemented",
        sum.parsed, sum.parse_errors, sum.unimplemented
    );
    let _ = writeln!(
        s,
        "paths:   {} resolved, {} unresolved",
        sum.paths_resolved, sum.paths_unresolved
    );
    let _ = writeln!(
        s,
        "io:      {} ok, {} unresolved",
        sum.io_ok, sum.io_unresolved
    );
    let _ = writeln!(
        s,
        "pipeline:{:>3} structural error(s)",
        sum.pipeline_errors
    );

    render_concept_graph(&mut s, &report.concept_graph);

    s
}

fn render_invocation(s: &mut String, inv: &InvocationReport) {
    match &inv.status {
        InvocationStatus::Ok => {
            let resolved = inv
                .paths
                .iter()
                .filter(|p| matches!(p.outcome, PathOutcomeRepr::Resolved))
                .count();
            let total = inv.paths.len();
            let pipeline_failed = !inv.pipeline.is_empty();
            let io_failed = inv
                .io
                .as_ref()
                .map(|io| !matches!(io.outcome, IoOutcomeRepr::Ok))
                .unwrap_or(false);
            let status = if resolved == total && !io_failed && !pipeline_failed {
                "ok"
            } else {
                "ERR"
            };
            let _ = writeln!(
                s,
                "  {status:<4}  {} at {}:{}  ({resolved}/{total} paths resolved)",
                inv.macro_path, inv.file, inv.line
            );
            for p in &inv.paths {
                if let PathOutcomeRepr::Unresolved = p.outcome {
                    let err = p.error.as_deref().unwrap_or("(no detail)");
                    let _ = writeln!(s, "        {}: {} — {err}", p.field, p.text);
                }
            }
            if let Some(io) = &inv.io {
                let label = match io.outcome {
                    IoOutcomeRepr::Ok => "io ok ",
                    _ => "io ERR",
                };
                let anchor = io
                    .anchor
                    .as_deref()
                    .map(|a| format!(" anchor={a}"))
                    .unwrap_or_default();
                let _ = writeln!(
                    s,
                    "        {label}  target={}{anchor} — {}",
                    io.target, io.message
                );
            }
            for entry in &inv.pipeline {
                let msg = match entry {
                    PipelineReportEntry::UnpermittedCycle { cycle } => {
                        format!("cycle without `cyclic: true`: {}", cycle.join(" -> "))
                    }
                    PipelineReportEntry::UnconnectedStage { stage } => {
                        format!("unconnected stage: {stage}")
                    }
                    PipelineReportEntry::DisconnectedComponents { components } => {
                        let parts: Vec<String> = components
                            .iter()
                            .map(|c| format!("[{}]", c.join(", ")))
                            .collect();
                        format!("disconnected components: {}", parts.join(" / "))
                    }
                };
                let _ = writeln!(s, "        pipeline ERR  {msg}");
            }
        }
        InvocationStatus::ParseError { message } => {
            let _ = writeln!(
                s,
                "  ERR   {} at {}:{}: {message}",
                inv.macro_path, inv.file, inv.line
            );
        }
        InvocationStatus::Unimplemented { .. } => {
            let _ = writeln!(
                s,
                "  todo  {} at {}:{} (parser not yet implemented)",
                inv.macro_path, inv.file, inv.line
            );
        }
    }

    // `kind` is unused in text output today but the AnnotationKind enum
    // ensures the variant list is exhaustive — silence dead-code warns.
    let _: AnnotationKind = inv.kind;
}

fn render_concept_graph(s: &mut String, g: &ConceptGraphReport) {
    let edge_count: usize = g.concepts.values().map(|n| n.edges.len()).sum();
    let decl_count: usize = g.concepts.values().map(|n| n.declarations.len()).sum();
    let _ = writeln!(
        s,
        "\nconcept graph: {} concepts, {decl_count} declarations, {edge_count} edges",
        g.concepts.len()
    );

    if g.concepts.is_empty() {
        return;
    }

    for (name, node) in &g.concepts {
        let _ = writeln!(s, "  {name}");
        for d in &node.declarations {
            let _ = writeln!(
                s,
                "    decl  {}:{}  ({} anchor{})",
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
                "    {arrow}  {}:{}  →  {}{lang}",
                e.file, e.line, e.target
            );
        }
    }

    if !g.warnings.is_empty() {
        let _ = writeln!(s, "\nconcept graph warnings: {}", g.warnings.len());
        for w in &g.warnings {
            let _ = writeln!(s, "  {}: {}", w.concept, w.message);
        }
    }
}
