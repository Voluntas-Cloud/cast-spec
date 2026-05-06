//! Path validation — every Rust path field on a parsed annotation must
//! resolve to a real workspace item.
//!
//! This is the half of the extractor that makes Cast load-bearing rather
//! than decorative. A `governs:` path that doesn't resolve, or an
//! `instead_at:` that points at a renamed module, is exactly the kind
//! of stale annotation Cast exists to catch.
//!
//! Resolution lives in `resolver.rs`; per-annotation path collection
//! lives in `paths.rs`; this module ties them together with the
//! `validate_annotation` entry point and the `PathDiagnostic` value
//! the CLI prints.

pub mod io_target;
pub mod paths;
pub mod pipeline_check;
pub mod resolver;

cast::concept! {
    name: "annotation_path_validators",
    summary: "Validators for the Rust-path fields on every parsed \
              cast::*! annotation. validate_annotation walks an \
              annotation, collects every syn::Path it carries (anchors, \
              governs, target, etc.), and returns one PathDiagnostic per \
              path with the resolver's verdict. Deterministic on the \
              HIR snapshot — same annotation + same MultiProject state \
              always yields the same diagnostics — but not pure: the \
              resolver reads through ra_ap_hir's interned database, \
              which is observable shared state.",
    anchors: [
        crate::validator::validate_annotation,
        crate::validator::validate_annotation_multi,
        crate::validator::resolve_cross_workspace_anchor,
        crate::validator::paths::collect_paths,
        crate::validator::paths::syn_path_to_string,
        crate::validator::pipeline_check::validate_pipeline,
    ],
    tags: ["cast_spec_validator", "anchor_resolution"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::deterministic,
    concept: "annotation_path_validators",
    why: "Same annotation + same loaded HIR always produces the same \
          PathDiagnostic list. Variation is purely a function of \
          inputs.",
}

cast::concept! {
    name: "io_target_validators",
    summary: "Validators for cast::io::continues_in! foreign-language \
              targets. Each `target:` is a relative file path; \
              validate_io_annotation dispatches on the `lang:` tag to \
              one of the per-lang checkers (rfc / external / source). \
              File-existence check + (for languages that support it) \
              tree-sitter or foreign-grep anchor resolution against \
              the file contents. Non-deterministic: filesystem state \
              participates — a file added between two calls flips an \
              Unresolved to Ok with no annotation change.",
    anchors: [
        crate::validator::io_target::validate_io_annotation,
        crate::validator::io_target::validate_rfc_target,
        crate::validator::io_target::validate_external_target,
        crate::validator::io_target::validate_source_target,
        crate::validator::io_target::anchor_found,
    ],
    tags: ["cast_spec_validator", "io_validation"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "io_target_validators",
    why: "Filesystem state at call time decides existence and contents; \
          a file appearing or changing flips the verdict without any \
          change to the annotation under validation.",
}

cast::concept! {
    name: "io_target_pure_helpers",
    summary: "Pure string-only helpers used by the io target validators: \
              match_with_prefixes performs prefix-based string matching \
              against a byte stream, no I/O. Pure — output is a \
              function of (contents, anchor, prefixes) alone.",
    anchors: [
        crate::validator::io_target::match_with_prefixes,
    ],
    tags: ["cast_spec_validator", "io_validation"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "io_target_pure_helpers",
    why: "Operates only on its input strings; no I/O, no mutation, no \
          time/RNG dependency.",
}

cast::concept! {
    name: "validator_diagnostic_value",
    summary: "Diagnostic shapes the validators produce: PathDiagnostic \
              for Rust-path resolution, IoDiagnostic for foreign-target \
              resolution, PipelineDiagnostic for flow validation.",
    anchors: [
        crate::validator::PathDiagnostic,
        crate::validator::io_target::IoDiagnostic,
        crate::validator::pipeline_check::PipelineDiagnostic,
    ],
    tags: ["cast_spec_validator"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::product_type,
    concept: "validator_diagnostic_value",
    why: "Each is a struct with all fields simultaneously inhabited.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::value_type,
    concept: "validator_diagnostic_value",
    why: "Cloneable, structurally compared, no resource handles.",
}

cast::concept! {
    name: "validator_outcome_categories",
    summary: "Sum-typed verdicts the validators emit: PathOutcome \
              (resolved / unresolved variants), IoOutcome (Ok / \
              missing-file / anchor-not-found variants), \
              PipelineDiagKind (which flow rule failed).",
    anchors: [
        crate::validator::PathOutcome,
        crate::validator::io_target::IoOutcome,
        crate::validator::pipeline_check::PipelineDiagKind,
    ],
    tags: ["cast_spec_validator"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::sum_type,
    concept: "validator_outcome_categories",
    why: "Each is an enum with a closed set of verdict variants.",
}

cast::continues_in! {
    target: cast_stdlib::architecture::typed_handle,
    concept: "validator_diagnostic_value",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::architecture::classifier_dispatch,
    concept: "validator_outcome_categories",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::integration::canonical_mapping,
    concept: "annotation_path_validators",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::integration::adapter_integration,
    concept: "io_target_validators",
    why: lazy,
}

// Anchor-resolution flow used by `validate_annotation_multi`. Each
// path on an annotation is collected, tried in the originating
// handle, and on failure tried across the rest of the MultiProject
// before being recorded as a `PathDiagnostic`. The fallback is what
// makes cross-workspace anchors (e.g. cast-web pointing at a concept
// declared in cast-watch) resolve at all.
cast::pipeline! {
    stages: {
        collect       @ crate::validator::paths::collect_paths,
        resolve_local @ crate::validator::resolver::resolve_syn_path,
        resolve_cross @ crate::validator::resolve_cross_workspace_anchor,
        diagnose      @ crate::validator::validate_annotation_multi,
    },
    flow: [
        collect       -> resolve_local,
        resolve_local -> diagnose,
        resolve_local -> resolve_cross,
        resolve_cross -> diagnose,
    ],
    cyclic: false,
    entry: collect,
    tags: ["anchor_resolution"],
    note: "resolve_cross is reached only when resolve_local returns \
           an error AND the path is not `crate::`/`self::`/`super::` \
           (those are locally scoped — cross-workspace dispatch \
           rejects them up-front). Both resolve_local-ok and \
           resolve_cross-ok flow into diagnose as `Resolved`; \
           resolve_local-err with no cross hit becomes `Unresolved`.",
}

pub use io_target::{validate_io_annotation, IoDiagnostic, IoOutcome};
pub use pipeline_check::{validate_pipeline, PipelineDiagKind, PipelineDiagnostic};

use crate::parser::CastAnnotation;
use ra_ap_hir::Module;
use ra_ap_ide_db::RootDatabase;
use resolver::{resolve_syn_path, PathError};

/// One path's validation result.
#[derive(Debug, Clone)]
pub struct PathDiagnostic {
    /// Where in the annotation the path appeared (e.g. `governs[0]`,
    /// `instead_at`, `entry[2].anchor`).
    pub field: String,
    /// The literal path text as it appeared in source.
    pub path_text: String,
    pub outcome: PathOutcome,
    /// Anchor role for `cast::concept!` `anchors[i]` fields — `Embodied`
    /// (default) or `Primitive` (when the source carried the
    /// `CAST::AS_PRIMITIVE::` marker). `None` for any other field
    /// (governs, target, instead_at, etc.) — those don't have a role.
    pub role: Option<crate::parser::concept::AnchorRole>,
}

#[derive(Debug, Clone)]
pub enum PathOutcome {
    Resolved,
    Unresolved(PathError),
}

impl PathDiagnostic {
    pub fn is_error(&self) -> bool {
        matches!(self.outcome, PathOutcome::Unresolved(_))
    }
}

/// Validate every path on an annotation against the loaded workspace.
///
/// `calling_module` anchors relative paths (`crate::*`, `self::*`,
/// `super::*`). When `None`, any annotation that uses a relative keyword
/// will surface as `PathError::NoCallingModule`.
pub fn validate_annotation(
    db: &RootDatabase,
    calling_module: Option<Module>,
    annotation: &CastAnnotation,
) -> Vec<PathDiagnostic> {
    paths::collect_paths(annotation)
        .into_iter()
        .map(|cp| {
            let path_text = paths::syn_path_to_string(cp.path);
            let outcome = match resolve_syn_path(db, calling_module, cp.path) {
                Ok(_) => PathOutcome::Resolved,
                Err(e) => PathOutcome::Unresolved(e),
            };
            PathDiagnostic {
                field: cp.field,
                path_text,
                outcome,
                role: cp.role,
            }
        })
        .collect()
}

/// MultiProject-aware variant of `validate_annotation`. Resolves
/// every anchor path in the originating handle first; on failure,
/// retries via `resolve_cross_workspace_anchor`. If both fail the
/// originating handle's `PathError` is preserved — that's the
/// most-specific reason and the one closest to where the path was
/// written.
pub fn validate_annotation_multi(
    multi: &crate::project::MultiProject,
    handle_idx: usize,
    calling_module: Option<Module>,
    annotation: &CastAnnotation,
) -> Vec<PathDiagnostic> {
    let db = &multi.handles[handle_idx].db;
    paths::collect_paths(annotation)
        .into_iter()
        .map(|cp| {
            let path_text = paths::syn_path_to_string(cp.path);
            let outcome = match resolve_syn_path(db, calling_module, cp.path) {
                Ok(_) => PathOutcome::Resolved,
                Err(e) => {
                    if resolve_cross_workspace_anchor(multi, handle_idx, cp.path) {
                        PathOutcome::Resolved
                    } else {
                        PathOutcome::Unresolved(e)
                    }
                }
            };
            PathDiagnostic {
                field: cp.field,
                path_text,
                outcome,
                role: cp.role,
            }
        })
        .collect()
}

/// Cross-workspace fallback for an anchor that failed to resolve in
/// the originating handle. Tries every other handle in `multi`,
/// rooted at each crate's root module (so `crate::` is meaningless
/// here — only crate-qualified paths can succeed).
///
/// Returns `true` if any other handle resolves the path. Used as a
/// last-chance pass before reporting `Unresolved`.
///
/// `crate::` paths are rejected up-front: per the multi-root rule,
/// `crate::` is locally scoped and a `crate::`-prefixed path that
/// fails in the originating handle is a real bug, not a candidate
/// for cross-workspace dispatch.
pub fn resolve_cross_workspace_anchor(
    multi: &crate::project::MultiProject,
    originating_handle: usize,
    path: &syn::Path,
) -> bool {
    if path
        .segments
        .first()
        .map(|s| s.ident == "crate" || s.ident == "self" || s.ident == "super")
        .unwrap_or(false)
    {
        return false;
    }

    for (idx, handle) in multi.handles.iter().enumerate() {
        if idx == originating_handle {
            continue;
        }
        if resolve_syn_path(&handle.db, None, path).is_ok() {
            return true;
        }
    }
    false
}
