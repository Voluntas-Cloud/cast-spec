//! `cast` — concept-language for Rust codebases.
//!
//! Two surfaces ship from this crate:
//!
//! 1. **The macro vocabulary** (always available). Function-shaped
//!    `macro_rules!` macros — `concept!`, `rule!`, `anti_pattern!`,
//!    `pipeline!`, `tier!`, `compare!`, `matrix!`, `gut_check!`,
//!    `continues_in!`, `continues_in_io!` — that expand to nothing.
//!    Their job is to let source code *name* the concepts in a
//!    codebase at real Rust paths.
//!
//! 2. **The static analyzer** (opt-in via the `analysis` feature).
//!    Walks Rust source for `cast::*!` invocations, validates every
//!    anchor with rust-analyzer, and emits a concept graph as a typed
//!    `Report`. The `cast-extract` CLI and `cast-watch` daemon are
//!    thin wrappers over this surface.
//!
//! Compile-time annotators only need the default (vocabulary-only)
//! build; tools that *check* the annotations turn on `analysis`.

// ─── Macro vocabulary — always compiled ──────────────────────────────

// Make `cast::*!` resolve from within the cast crate itself, so the
// dogfood `cast::concept!` / `cast::rule!` invocations sprinkled
// through `finder.rs`, `graph.rs`, `validator/io_target.rs` etc. parse
// the same way they would from a downstream user. Without this they'd
// have to be written `crate::rule!`, which loses the parallel.
extern crate self as cast;

#[macro_export] macro_rules! compare         { ($($tt:tt)*) => {}; }
#[macro_export] macro_rules! pipeline        { ($($tt:tt)*) => {}; }
#[macro_export] macro_rules! tier            { ($($tt:tt)*) => {}; }
#[macro_export] macro_rules! matrix          { ($($tt:tt)*) => {}; }
#[macro_export] macro_rules! rule            { ($($tt:tt)*) => {}; }
#[macro_export] macro_rules! anti_pattern    { ($($tt:tt)*) => {}; }
#[macro_export] macro_rules! gut_check       { ($($tt:tt)*) => {}; }
#[macro_export] macro_rules! continues_in    { ($($tt:tt)*) => {}; }
#[macro_export] macro_rules! continues_in_io { ($($tt:tt)*) => {}; }
#[macro_export] macro_rules! concept          { ($($tt:tt)*) => {}; }
#[macro_export] macro_rules! policy           { ($($tt:tt)*) => {}; }

pub mod io {
    pub use crate::continues_in_io as continues_in;
}

// Two tiers ship from this crate. The cfg gate at the divider below
// is the boundary; this annotation reifies it as a queryable concept
// rather than relying on prose at the top of the file.
cast::tier! {
    axis: analyzer_cost,
    direction: increasing,
    stages: {
        vocabulary @ crate::io:
            "Macro vocabulary — `macro_rules!` definitions above that \
             expand to nothing. Zero deps. Compile time in milliseconds. \
             Always available; downstream consumers that just want to \
             annotate never pay any further cost.",
        analysis @ crate::run_multi_root_analysis:
            "Static analyzer, gated on `feature = \"analysis\"`. Pulls \
             in rust-analyzer, syn, serde, etc.; walks the workspace, \
             resolves anchors, builds the concept graph. Compile time \
             in seconds; per-workspace runtime in seconds.",
    },
    tags: ["analyzer_cost"],
    note: "The cfg gate at the `Static analyzer` divider below is the \
           tier boundary. cast-extract / cast-watch / cast-lsp each \
           depend on cast-spec with `features = [\"analysis\"]`; \
           every other downstream pays only the vocabulary tier.",
}

// ─── Static analyzer — gated on `analysis` feature ───────────────────

// The `cast_spec` per-crate umbrella concept is declared in
// `crates/cast-spec/spec.cast`, not here, so it doesn't go through
// cfg-gating: the .cast walker only runs against the cast workspace,
// where the `analysis` feature is on, and downstream standalone
// workspaces (cast-web, cast-lsp) never see the block at all.

#[cfg(feature = "analysis")]
pub mod emit;
#[cfg(feature = "analysis")]
pub mod examples;
#[cfg(feature = "analysis")]
pub mod finder;
#[cfg(feature = "analysis")]
pub mod graph;
#[cfg(feature = "analysis")]
pub mod language_backend;
#[cfg(feature = "analysis")]
pub mod model;
#[cfg(feature = "analysis")]
pub mod parser;
#[cfg(feature = "analysis")]
pub mod project;
#[cfg(feature = "analysis")]
pub mod spec_source;
#[cfg(feature = "analysis")]
pub mod tree;
#[cfg(feature = "analysis")]
pub mod validator;

#[cfg(feature = "analysis")]
pub use emit::{Format, Report};
#[cfg(feature = "analysis")]
pub use finder::{find_invocations, CastInvocation};
#[cfg(feature = "analysis")]
pub use graph::{build_graph, warnings as graph_warnings, ConceptGraph, EdgeKind, EdgeTarget};
#[cfg(feature = "analysis")]
pub use model::Location;
#[cfg(feature = "analysis")]
pub use parser::{parse_macro_body, CastAnnotation};
#[cfg(feature = "analysis")]
pub use project::{
    assert_standalone_root, find_repo_root, handle_for_file, load_project, load_projects,
    MultiProject, ProjectHandle,
};
#[cfg(feature = "analysis")]
pub use validator::{
    resolve_cross_workspace_anchor, validate_annotation, validate_annotation_multi,
    validate_io_annotation, validate_pipeline, IoDiagnostic, IoOutcome, PathDiagnostic,
    PathOutcome, PipelineDiagKind, PipelineDiagnostic,
};

#[cfg(feature = "analysis")]
mod analysis_runner {
    use crate::emit::model::{
        kind_of, note_of, since_of, tags_of, AnnotationKind, ConceptGraphReport, ConceptReport,
        InvocationReport, InvocationStatus, IoReport, PathReport, PipelineReportEntry, Summary,
        UNTAGGED,
    };
    use crate::emit::Report;
    use crate::finder::find_invocations;
    use crate::graph::{build_graph, warnings as graph_warnings};
    use crate::model::Location;
    use crate::parser::{parse_macro_body, CastAnnotation};
    use crate::project;
    use crate::validator::{
        self, validate_io_annotation, validate_pipeline, IoOutcome, PathOutcome,
    };
    use ra_ap_hir::attach_db;
    use std::collections::BTreeMap;
    use std::path::Path;

    // Per-invocation pipeline inside `build_report`. Each stage is a
    // function called once per `CastInvocation`; their outputs accrete
    // into the `Report` returned at the end. Anchors point at the
    // function called per stage, not where it's defined locally.
    cast::pipeline! {
        stages: {
            parse                @ crate::parser::parse_macro_body,
            validate_paths       @ crate::validator::validate_annotation_multi,
            validate_io          @ crate::validator::validate_io_annotation,
            validate_pipelines   @ crate::validator::validate_pipeline,
            build_graph          @ crate::graph::build_graph,
            compute_warnings     @ crate::graph::warnings,
            assemble             @ crate::analysis_runner::build_report,
        },
        flow: [
            parse              -> validate_paths,
            validate_paths     -> validate_io,
            validate_io        -> validate_pipelines,
            validate_pipelines -> build_graph,
            build_graph        -> compute_warnings,
            compute_warnings   -> assemble,
        ],
        cyclic: false,
        entry: parse,
        tags: ["analyzer_pipeline"],
        note: "validate_io and validate_pipelines run only when the \
               annotation kind matches; they're always traversed in \
               this order, but become no-ops for other kinds.",
    }

    // Two `SpecSource` notations contribute invocations; both feed the
    // same `build_report` and the per-pass `Report`s are merged. This
    // pipeline is the outer scaffolding around the per-invocation
    // pipeline above.
    cast::pipeline! {
        stages: {
            per_handle @ crate::analysis_runner::run_per_handle_analysis,
            cast_file  @ crate::analysis_runner::run_cast_file_analysis,
            sidecar    @ crate::analysis_runner::run_sidecar_analysis,
            merge      @ crate::analysis_runner::merge_reports,
        },
        flow: [
            per_handle -> merge,
            cast_file  -> merge,
            sidecar    -> merge,
        ],
        cyclic: false,
        tags: ["multi_root_analysis"],
        note: "per_handle runs once per ProjectHandle in MultiProject; \
               cast_file runs once total under handle 0's attach_db so \
               cross-workspace anchor fallback can reach every handle.",
    }

    /// Run the analysis pipeline against one ProjectHandle inside a
    /// MultiProject context and assemble a Report.
    pub fn run_per_handle_analysis(
        multi: &project::MultiProject,
        handle_idx: usize,
        repo_root: &Path,
    ) -> Report {
        let handle = &multi.handles[handle_idx];
        let invocations = find_invocations(&handle.db, &handle.vfs);
        build_report(multi, handle_idx, &invocations, repo_root)
    }

    /// Run the analysis pipeline over `*.cast` files discovered under
    /// `repo_root`. Output is the same shape as `run_per_handle_analysis`
    /// — `.cast` files are a second `SpecSource` notation, not a second
    /// pipeline. Anchor resolution uses handle 0 as the originating
    /// handle so the validator's cross-workspace fallback can span all
    /// handles; `.cast` invocations carry `calling_module: None`, so
    /// relative paths fail at the resolver and the absolute-path rule
    /// is enforced by physics, not a separate guard.
    pub fn run_cast_file_analysis(
        multi: &project::MultiProject,
        repo_root: &Path,
    ) -> Report {
        let source = crate::spec_source::cast_file::CastFileSource::new(repo_root);
        let invocations = <_ as crate::spec_source::SpecSource>::invocations(&source);
        build_report(multi, 0, &invocations, repo_root)
    }

    /// Run the analysis pipeline over `<name>.rs.cast` sidecar files
    /// discovered under `repo_root`. Same downstream shape as the
    /// other two passes; the difference is that each invocation's
    /// `calling_module` is resolved to the sibling `.rs` file's
    /// module (via VFS lookup across every loaded handle), so
    /// relative anchors (`self::*`, `super::*`, unqualified path
    /// heads) work the same as inline.
    pub fn run_sidecar_analysis(
        multi: &project::MultiProject,
        repo_root: &Path,
    ) -> Report {
        let source =
            crate::spec_source::sidecar::SidecarSource::new(repo_root, multi);
        let invocations = <_ as crate::spec_source::SpecSource>::invocations(&source);
        build_report(multi, 0, &invocations, repo_root)
    }

    /// Internal helper. Takes a stream of `CastInvocation`s — wherever
    /// they came from — and runs the parse → validate → group →
    /// build-graph pipeline over them. Both the per-handle (Rust
    /// macros) and per-repo (`.cast` files) entry points feed this.
    fn build_report(
        multi: &project::MultiProject,
        handle_idx: usize,
        invocations: &[crate::finder::CastInvocation],
        repo_root: &Path,
    ) -> Report {
        let mut parsed_ok = 0usize;
        let mut parse_errs = 0usize;
        let mut not_implemented = 0usize;
        let mut paths_resolved = 0usize;
        let mut paths_unresolved = 0usize;
        let mut io_ok = 0usize;
        let mut io_unresolved = 0usize;
        let mut pipeline_errors = 0usize;

        let mut parsed: Vec<(Location, CastAnnotation)> = Vec::new();
        let mut invocation_reports: Vec<InvocationReport> = Vec::new();

        for inv in invocations {
            let file = inv.location.file.display().to_string();
            let line = inv.location.line;

            match parse_macro_body(&inv.macro_path, inv.body.clone()) {
                Ok(annotation) => {
                    parsed_ok += 1;
                    let path_diags = validator::validate_annotation_multi(
                        multi,
                        handle_idx,
                        inv.calling_module,
                        &annotation,
                    );
                    let n_resolved = path_diags
                        .iter()
                        .filter(|d| matches!(d.outcome, PathOutcome::Resolved))
                        .count();
                    let n_unresolved = path_diags.len() - n_resolved;
                    paths_resolved += n_resolved;
                    paths_unresolved += n_unresolved;

                    let io = if let CastAnnotation::IoContinuesIn(io_ann) = &annotation {
                        let d = validate_io_annotation(repo_root, io_ann);
                        if matches!(d.outcome, IoOutcome::Ok) {
                            io_ok += 1;
                        } else {
                            io_unresolved += 1;
                        }
                        Some(IoReport::from(&d))
                    } else {
                        None
                    };

                    let pipeline_entries: Vec<PipelineReportEntry> =
                        if let CastAnnotation::Pipeline(p) = &annotation {
                            let diags = validate_pipeline(p);
                            pipeline_errors += diags.len();
                            diags.iter().map(PipelineReportEntry::from).collect()
                        } else {
                            Vec::new()
                        };

                    invocation_reports.push(InvocationReport {
                        macro_path: inv.macro_path.clone(),
                        kind: kind_of(&annotation),
                        file,
                        line,
                        tags: tags_of(&annotation).to_vec(),
                        since: since_of(&annotation).map(str::to_string),
                        note: note_of(&annotation).map(str::to_string),
                        status: InvocationStatus::Ok,
                        paths: path_diags.iter().map(PathReport::from).collect(),
                        io,
                        pipeline: pipeline_entries,
                    });
                    parsed.push((inv.location.clone(), annotation));
                }
                Err(e) if e.to_string().contains("unknown cast macro path") => {
                    not_implemented += 1;
                    invocation_reports.push(InvocationReport {
                        macro_path: inv.macro_path.clone(),
                        kind: AnnotationKind::Unknown,
                        file,
                        line,
                        tags: Vec::new(),
                        since: None,
                        note: None,
                        status: InvocationStatus::Unimplemented {
                            macro_path: inv.macro_path.clone(),
                        },
                        paths: Vec::new(),
                        io: None,
                        pipeline: Vec::new(),
                    });
                }
                Err(e) => {
                    parse_errs += 1;
                    invocation_reports.push(InvocationReport {
                        macro_path: inv.macro_path.clone(),
                        kind: AnnotationKind::Unknown,
                        file,
                        line,
                        tags: Vec::new(),
                        since: None,
                        note: None,
                        status: InvocationStatus::ParseError {
                            message: e.to_string(),
                        },
                        paths: Vec::new(),
                        io: None,
                        pipeline: Vec::new(),
                    });
                }
            }
        }

        let graph = build_graph(parsed.iter().map(|(loc, ann)| (loc, ann)));
        let g_warnings = graph_warnings(&graph);

        // Surface every cast::policy! block as a wire-shape PolicyDecl.
        // Enforcement runs post-merge in merge_reports; per-pass we
        // just collect.
        let policies: Vec<crate::emit::model::PolicyDecl> = parsed
            .iter()
            .filter_map(|(loc, ann)| {
                if let CastAnnotation::Policy(p) = ann {
                    Some(crate::emit::model::PolicyDecl::from_parsed(p, loc))
                } else {
                    None
                }
            })
            .collect();

        // One bucket per tag. An invocation with multiple tags lands in
        // every bucket it carries (clones the report into each); an
        // invocation with no tags lands in the `(untagged)` bucket
        // alone. This is the path that gives `groups` the
        // tag-organised shape downstream tools expect.
        let mut groups: BTreeMap<String, Vec<InvocationReport>> = BTreeMap::new();
        for inv in invocation_reports {
            if inv.tags.is_empty() {
                groups
                    .entry(UNTAGGED.to_string())
                    .or_default()
                    .push(inv);
            } else {
                for t in inv.tags.clone() {
                    groups.entry(t).or_default().push(inv.clone());
                }
            }
        }

        Report {
            summary: Summary {
                invocations: invocations.len(),
                parsed: parsed_ok,
                parse_errors: parse_errs,
                unimplemented: not_implemented,
                paths_resolved,
                paths_unresolved,
                io_ok,
                io_unresolved,
                pipeline_errors,
                graph_warnings: g_warnings.len(),
                policy_warnings: 0,
            },
            groups,
            concept_graph: ConceptGraphReport::from(&graph),
            policies,
            policy_warnings: Vec::new(),
        }
    }

    /// Drive the per-handle analysis pipeline across every handle in
    /// `multi`, plus a single pass over `*.cast` files under
    /// `repo_root`, and merge all Reports into one.
    ///
    /// Two `SpecSource` notations contribute: Rust macros embedded in
    /// `.rs` files (one pass per handle) and `.cast` files discovered
    /// under `repo_root` (one pass total). Both feed `merge_reports`,
    /// so downstream consumers see a single Report regardless of
    /// notation. The `.cast` pass runs inside handle 0's `attach_db`
    /// so the validator's cross-workspace fallback can reach every
    /// handle.
    pub fn run_multi_root_analysis(multi: &project::MultiProject, repo_root: &Path) -> Report {
        let mut reports: Vec<Report> = (0..multi.handles.len())
            .map(|idx| {
                attach_db(&multi.handles[idx].db, || {
                    run_per_handle_analysis(multi, idx, repo_root)
                })
            })
            .collect();

        if !multi.handles.is_empty() {
            reports.push(attach_db(&multi.handles[0].db, || {
                run_cast_file_analysis(multi, repo_root)
            }));
            // Sidecar pass needs HIR access to resolve sibling-module
            // calling contexts. attach_db on handle 0 lets the
            // SidecarSource walk all handles' VFS through MultiProject
            // while satisfying salsa's thread-local requirement.
            reports.push(attach_db(&multi.handles[0].db, || {
                run_sidecar_analysis(multi, repo_root)
            }));
        }

        merge_reports(reports)
    }

    /// Merge per-handle Reports into one.
    pub fn merge_reports(reports: Vec<Report>) -> Report {
        let mut merged = Report {
            summary: Summary::default(),
            groups: BTreeMap::new(),
            concept_graph: ConceptGraphReport {
                concepts: BTreeMap::new(),
                warnings: Vec::new(),
            },
            policies: Vec::new(),
            policy_warnings: Vec::new(),
        };
        for r in reports {
            merged.summary.invocations += r.summary.invocations;
            merged.summary.parsed += r.summary.parsed;
            merged.summary.parse_errors += r.summary.parse_errors;
            merged.summary.unimplemented += r.summary.unimplemented;
            merged.summary.paths_resolved += r.summary.paths_resolved;
            merged.summary.paths_unresolved += r.summary.paths_unresolved;
            merged.summary.io_ok += r.summary.io_ok;
            merged.summary.io_unresolved += r.summary.io_unresolved;
            merged.summary.pipeline_errors += r.summary.pipeline_errors;
            // graph_warnings and policy_warnings are intentionally NOT
            // summed here — graph_warnings is recomputed against the
            // merged graph below, and policy_warnings is recomputed
            // by apply_policy_warnings after the policies list is
            // merged. Per-pass sums would double-count.

            for (tag, invs) in r.groups {
                merged.groups.entry(tag).or_default().extend(invs);
            }
            for (name, concept) in r.concept_graph.concepts {
                let entry = merged
                    .concept_graph
                    .concepts
                    .entry(name)
                    .or_insert(ConceptReport {
                        declarations: Vec::new(),
                        edges: Vec::new(),
                    });
                entry.declarations.extend(concept.declarations);
                entry.edges.extend(concept.edges);
            }
            merged.policies.extend(r.policies);
            // Per-pass warnings are dropped — recomputed against the
            // merged graph below.
        }
        // Cross-pass collision split — same logic as `build_graph`'s
        // intra-pass collision detection, but applied to the merged
        // graph. Per-handle build_graph calls only see their own
        // pass's annotations and can't notice that another handle
        // declared a same-named concept; merge_reports's blind name-
        // keyed merge above is what causes the cross-handle silent
        // merge. The split here re-keys colliders to `name@<tag>`
        // using the smallest differentiating tag index, mirroring
        // build_graph::find_colliding_names.
        split_cross_pass_collisions(&mut merged.concept_graph);

        // Recompute warnings on the merged graph. A concept whose
        // declaration lives in one pass (Rust .rs) and whose edges
        // live in another (.cast files) is sound after merging, even
        // though each per-pass view sees only half of it. The merged
        // recomputation reflects that.
        merged.concept_graph.warnings = merged.concept_graph.compute_warnings();
        merged.summary.graph_warnings = merged.concept_graph.warnings.len();

        // Policy enforcement runs once over the merged Report so it
        // sees both the full policy list and every parsed invocation.
        // A Policy declared in Cast.cast (cast-file pass) and inline
        // violations in the Rust passes can only be reconciled here.
        crate::emit::model::apply_policy_warnings(&mut merged);

        merged
    }

    fn split_cross_pass_collisions(graph: &mut ConceptGraphReport) {
        use std::collections::BTreeSet;
        let names: Vec<String> = graph.concepts.keys().cloned().collect();
        for name in names {
            let entry = match graph.concepts.get(&name) {
                Some(e) => e,
                None => continue,
            };
            let tuples: BTreeSet<Vec<String>> = entry
                .declarations
                .iter()
                .map(|d| d.tags.clone())
                .collect();
            if tuples.len() < 2 {
                continue;
            }
            let max_len = tuples.iter().map(|t| t.len()).max().unwrap_or(0);
            let mut k: Option<usize> = None;
            for kk in 1..=max_len {
                let prefixes: BTreeSet<Vec<&str>> = tuples
                    .iter()
                    .map(|t| t.iter().take(kk).map(|s| s.as_str()).collect())
                    .collect();
                if prefixes.len() == tuples.len() {
                    k = Some(kk);
                    break;
                }
            }
            let Some(k) = k else { continue };
            let original = match graph.concepts.remove(&name) {
                Some(e) => e,
                None => continue,
            };
            for d in original.declarations.clone() {
                let suffix = d
                    .tags
                    .iter()
                    .take(k)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("-");
                let key = if suffix.is_empty() {
                    name.clone()
                } else {
                    format!("{name}@{suffix}")
                };
                let bucket = graph
                    .concepts
                    .entry(key)
                    .or_insert(ConceptReport {
                        declarations: Vec::new(),
                        edges: Vec::new(),
                    });
                bucket.declarations.push(d);
            }
            // Edges have no tags carried into ConceptReport, so we
            // cannot route them per-namespace. Fan them out to every
            // namespaced bucket — downstream tooling will see the
            // edge in both variants. (For a single-namespace target
            // there's no fan-out; for true ambiguity this is the
            // safest under-approximation.)
            let edge_keys: Vec<String> = graph
                .concepts
                .keys()
                .filter(|k| k == &&name || k.starts_with(&format!("{name}@")))
                .cloned()
                .collect();
            for ek in &edge_keys {
                if let Some(bucket) = graph.concepts.get_mut(ek) {
                    bucket.edges.extend(original.edges.clone());
                }
            }
        }
    }
}

#[cfg(feature = "analysis")]
pub use analysis_runner::{
    merge_reports, run_cast_file_analysis, run_multi_root_analysis, run_per_handle_analysis,
    run_sidecar_analysis,
};

#[cfg(feature = "analysis")]
cast::concept! {
    name: "analysis_orchestration",
    summary: "Top-level entry points that drive the analyzer pipeline \
              over one or more workspace roots. Each *_analysis \
              function discovers invocations from one SpecSource \
              notation, runs them through parse → validate → \
              build_graph, and assembles a Report. \
              run_multi_root_analysis is the multi-root umbrella that \
              fan-outs across every ProjectHandle. Non-deterministic: \
              filesystem state and the salsa-tracked RootDatabase \
              both participate, and edits between calls flip outcomes \
              without any change to inputs visible to the caller.",
    anchors: [
        crate::analysis_runner::run_per_handle_analysis,
        crate::analysis_runner::run_cast_file_analysis,
        crate::analysis_runner::run_sidecar_analysis,
        crate::analysis_runner::run_multi_root_analysis,
        crate::analysis_runner::build_report,
    ],
    tags: ["cast_spec_orchestration"],
}

#[cfg(feature = "analysis")]
cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "analysis_orchestration",
    why: "Filesystem and HIR are observable external state; same call \
          can produce different Reports across two snapshots if the \
          source tree or the salsa db changed.",
}

#[cfg(feature = "analysis")]
cast::concept! {
    name: "report_merging",
    summary: "Pure functions that combine Reports from per-handle and \
              per-spec-source passes into a single merged Report. \
              merge_reports unions by concept name; \
              split_cross_pass_collisions re-namespaces concepts that \
              were merged across passes when the per-pass tag tuples \
              prove the merge was unintended. Both are pure — no I/O, \
              no time/RNG, no shared state.",
    anchors: [
        crate::analysis_runner::merge_reports,
        crate::analysis_runner::split_cross_pass_collisions,
    ],
    tags: ["cast_spec_orchestration"],
}

#[cfg(feature = "analysis")]
cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "report_merging",
    why: "Output is a function of the input Reports alone; merging \
          and splitting are read-only on the inputs and produce a \
          new Report.",
}

#[cfg(feature = "analysis")]
cast::continues_in! {
    target: cast_stdlib::workflow::stateful_workflow,
    concept: "analysis_orchestration",
    why: lazy,
}

#[cfg(feature = "analysis")]
cast::continues_in! {
    target: cast_stdlib::integration::format_converter,
    concept: "report_merging",
    why: lazy,
}
