//! Mapping from `cast::Report` to LSP diagnostics.
//!
//! The analyzer only resolves to (file, 1-based line); column
//! information isn't surfaced. Each diagnostic anchors at the start of
//! the reported line — editors render that as a whole-line squiggle,
//! which is the right granularity for an annotation-block error.

use std::collections::HashMap;
use std::path::PathBuf;

use cast::emit::model::{
    AnnotationKind, InvocationReport, InvocationStatus, IoOutcomeRepr, PathOutcomeRepr,
    PipelineReportEntry, Report,
};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

cast::concept! {
    name: "report_to_diagnostics",
    summary: "Walks every InvocationReport in a cast::Report, expands \
              each unresolved anchor / parse error / pipeline error / \
              io error into one Diagnostic, and groups the result by \
              file path. The grouping key is a PathBuf so the LSP layer \
              can issue one publishDiagnostics per file even when the \
              workspace has hundreds of cast-bearing source files.",
    anchors: [
        crate::diagnostics::report_to_diagnostics,
    ],
    tags: ["cast_lsp"],
}

cast::rule! {
    rule: "Severity mapping: unresolved anchor → ERROR; pipeline error \
           → ERROR; io unresolved → ERROR; parse error → ERROR; \
           unimplemented macro path → WARNING. Severity is fixed in \
           code, not configurable.",
    why:  "An unresolved anchor or pipeline error means the codebase is \
           failing its own spec — a hard failure the user must fix \
           before merge. An unimplemented macro path is a cast-side \
           gap, not a user bug; warning surfaces the gap without \
           flagging the user's code as broken. Making severity \
           configurable would let teams hide real spec failures, \
           defeating the point.",
    governs: [
        crate::diagnostics::report_to_diagnostics,
    ],
    tags: ["cast_lsp"],
}

// Severity is fixed per InvocationStatus / per failure mode. The rule
// above explains why this isn't configurable; this matrix makes the
// mapping itself queryable so consumers can reason about which
// failures block a merge (ERROR) and which surface a cast-side gap
// (WARNING).
cast::matrix! {
    columns: [severity, why],
    rows: {
        unresolved_anchor @ crate::diagnostics::report_to_diagnostics: [
            "ERROR",
            "Annotation points at a path that doesn't exist in the workspace — the codebase is failing its own spec. User must fix the path or remove the annotation before merge.",
        ],
        pipeline_error    @ crate::diagnostics::report_to_diagnostics: [
            "ERROR",
            "Structural failure inside a `cast::pipeline!` — cycle, unconnected stage, or disconnected components. The pipeline doesn't describe a single coherent flow.",
        ],
        io_unresolved     @ crate::diagnostics::report_to_diagnostics: [
            "ERROR",
            "`cast::io::continues_in!` target file is missing — the cross-language reference is broken at the filesystem level.",
        ],
        parse_error       @ crate::diagnostics::report_to_diagnostics: [
            "ERROR",
            "Macro body didn't parse against the grammar in `crates/cast-spec/GRAMMAR.md`. User wrote a malformed annotation; nothing downstream can act on it.",
        ],
        unimplemented     @ crate::diagnostics::report_to_diagnostics: [
            "WARNING",
            "Macro path is unknown to cast (e.g. someone wrote `cast::brand_new!`). Analyzer-side gap, not a user bug — surfacing as WARNING means the user sees the gap without their code being flagged broken.",
        ],
    },
    tags: ["cast_lsp"],
    note: "All five rows anchor at `report_to_diagnostics` because \
           that's the single function that emits each Diagnostic; \
           the dispatch lives in the private `invocation_diagnostics` \
           helper but the matrix tracks the user-visible mapping.",
}

/// A diagnostic plus the file it belongs in. Convenient intermediate
/// before grouping into the per-file map LSP wants.
#[derive(Debug, Clone)]
pub struct FileDiagnostic {
    pub file: PathBuf,
    pub diagnostic: Diagnostic,
}

/// Map a `cast::Report` to a per-file diagnostic list.
///
/// The returned map's keys are the absolute paths the analyzer emitted;
/// the LSP layer translates these into `Url`s before publishing.
pub fn report_to_diagnostics(report: &Report) -> HashMap<PathBuf, Vec<Diagnostic>> {
    let mut by_file: HashMap<PathBuf, Vec<Diagnostic>> = HashMap::new();
    for invs in report.groups.values() {
        for inv in invs {
            for fd in invocation_diagnostics(inv) {
                by_file.entry(fd.file).or_default().push(fd.diagnostic);
            }
        }
    }
    by_file
}

fn invocation_diagnostics(inv: &InvocationReport) -> Vec<FileDiagnostic> {
    let mut out = Vec::new();
    let file = PathBuf::from(&inv.file);
    let range = whole_line_range(inv.line);

    match &inv.status {
        InvocationStatus::Ok => {
            for path in &inv.paths {
                if matches!(path.outcome, PathOutcomeRepr::Unresolved) {
                    let message = format!(
                        "cast: anchor `{}` at field `{}` does not resolve{}",
                        path.text,
                        path.field,
                        path.error
                            .as_deref()
                            .map(|e| format!(": {e}"))
                            .unwrap_or_default(),
                    );
                    out.push(FileDiagnostic {
                        file: file.clone(),
                        diagnostic: error(range, message),
                    });
                }
            }
            if let Some(io) = &inv.io {
                if !matches!(io.outcome, IoOutcomeRepr::Ok) {
                    let message = format!(
                        "cast: io target `{}` ({})",
                        io.target,
                        io.message,
                    );
                    out.push(FileDiagnostic {
                        file: file.clone(),
                        diagnostic: error(range, message),
                    });
                }
            }
            for entry in &inv.pipeline {
                let message = match entry {
                    PipelineReportEntry::UnpermittedCycle { cycle } => {
                        format!("cast: pipeline cycle not permitted: {}", cycle.join(" -> "))
                    }
                    PipelineReportEntry::UnconnectedStage { stage } => {
                        format!("cast: pipeline stage `{stage}` is unconnected")
                    }
                    PipelineReportEntry::DisconnectedComponents { components } => {
                        let parts: Vec<String> = components
                            .iter()
                            .map(|c| format!("[{}]", c.join(", ")))
                            .collect();
                        format!(
                            "cast: pipeline has disconnected components: {}",
                            parts.join(" / ")
                        )
                    }
                };
                out.push(FileDiagnostic {
                    file: file.clone(),
                    diagnostic: error(range, message),
                });
            }
        }
        InvocationStatus::ParseError { message } => {
            out.push(FileDiagnostic {
                file,
                diagnostic: error(
                    range,
                    format!(
                        "cast: parse error in {}: {}",
                        macro_label(inv),
                        message
                    ),
                ),
            });
        }
        InvocationStatus::Unimplemented { macro_path } => {
            out.push(FileDiagnostic {
                file,
                diagnostic: warning(
                    range,
                    format!("cast: unimplemented macro `{macro_path}`"),
                ),
            });
        }
    }

    out
}

fn macro_label(inv: &InvocationReport) -> String {
    match inv.kind {
        AnnotationKind::Unknown => inv.macro_path.clone(),
        _ => inv.macro_path.clone(),
    }
}

/// LSP positions are zero-based; `Location::line` from the analyzer is
/// 1-based. Subtract one and span the whole line (column 0 .. u32::MAX
/// is a common convention editors render correctly).
fn whole_line_range(one_based_line: usize) -> Range {
    let line: u32 = one_based_line.saturating_sub(1).try_into().unwrap_or(u32::MAX);
    Range {
        start: Position { line, character: 0 },
        end: Position {
            line,
            character: u32::MAX,
        },
    }
}

fn error(range: Range, message: String) -> Diagnostic {
    Diagnostic {
        range,
        severity: Some(DiagnosticSeverity::ERROR),
        source: Some("cast".to_string()),
        message,
        ..Default::default()
    }
}

fn warning(range: Range, message: String) -> Diagnostic {
    Diagnostic {
        range,
        severity: Some(DiagnosticSeverity::WARNING),
        source: Some("cast".to_string()),
        message,
        ..Default::default()
    }
}

cast::concept! {
    name: "diagnostic_translation",
    summary: "Pure folders that turn a cast Report into per-file LSP \
              Diagnostic lists. Maps anchor outcomes, IO outcomes, and \
              parse errors to LSP shapes; ranges, severities, and \
              messages are decided from the Report alone.",
    anchors: [
        crate::diagnostics::report_to_diagnostics,
        crate::diagnostics::invocation_diagnostics,
        crate::diagnostics::macro_label,
        crate::diagnostics::whole_line_range,
        crate::diagnostics::error,
        crate::diagnostics::warning,
    ],
    tags: ["cast_lsp_diagnostics"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "diagnostic_translation",
    why: "Output is a function of the input Report alone; no I/O, no \
          mutation, no time/RNG.",
}

cast::concept! {
    name: "file_diagnostic_value",
    summary: "Per-file Diagnostic carrier. Bundles range + severity + \
              message + source.",
    anchors: [
        crate::diagnostics::FileDiagnostic,
    ],
    tags: ["cast_lsp_diagnostics"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::product_type,
    concept: "file_diagnostic_value",
    why: "Struct with all fields simultaneously inhabited.",
}

cast::continues_in! {
    target: cast_stdlib::integration::format_converter,
    concept: "diagnostic_translation",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::architecture::typed_handle,
    concept: "file_diagnostic_value",
    why: lazy,
}

#[cfg(test)]
mod tests {
    use super::*;
    use cast::emit::model::{PathReport, Summary};
    use std::collections::BTreeMap;

    fn empty_report() -> Report {
        Report {
            summary: Summary {
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
            groups: BTreeMap::new(),
            concept_graph: cast::emit::model::ConceptGraphReport {
                concepts: BTreeMap::new(),
                warnings: vec![],
            },
        }
    }

    #[test]
    fn empty_report_yields_no_diagnostics() {
        let r = empty_report();
        let by_file = report_to_diagnostics(&r);
        assert!(by_file.is_empty());
    }

    #[test]
    fn unresolved_anchor_becomes_error_diagnostic() {
        let mut r = empty_report();
        let inv = InvocationReport {
            macro_path: "cast::rule".into(),
            kind: AnnotationKind::Rule,
            file: "/tmp/foo.rs".into(),
            line: 42,
            tags: Vec::new(),
            since: None,
            note: None,
            status: InvocationStatus::Ok,
            paths: vec![PathReport {
                field: "governs[0]".into(),
                text: "sample::missing".into(),
                outcome: PathOutcomeRepr::Unresolved,
                error: Some("not found".into()),
                role: None,
            }],
            io: None,
            pipeline: vec![],
        };
        r.groups.insert("cast_lsp".into(), vec![inv]);
        let by_file = report_to_diagnostics(&r);
        let diags = by_file.get(&PathBuf::from("/tmp/foo.rs")).unwrap();
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Some(DiagnosticSeverity::ERROR));
        assert_eq!(diags[0].range.start.line, 41);
        assert!(diags[0].message.contains("sample::missing"));
        assert_eq!(diags[0].source.as_deref(), Some("cast"));
    }
}
