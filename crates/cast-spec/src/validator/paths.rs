//! Per-annotation path collection.
//!
//! Each annotation type carries a different shape of path fields —
//! `Rule` has a single `governs:` list, `Pipeline` has optional anchors
//! per stage, etc. This module flattens them into `(label, &syn::Path)`
//! pairs the validator can iterate uniformly.
//!
//! Optional paths are emitted only when set; the warn-on-missing
//! policy is the validator-level concern, not collection's.

use crate::parser::concept::AnchorRole;
use crate::parser::CastAnnotation;
use quote::ToTokens;

/// One row from `collect_paths`: where the path appears, the path itself,
/// and (for `cast::concept!` `anchors[i]` only) the role declared on
/// the anchor. `role` is `None` for every other field.
pub struct CollectedPath<'a> {
    pub field: String,
    pub path: &'a syn::Path,
    pub role: Option<AnchorRole>,
}

/// Collect every concrete path on this annotation, with a human-readable
/// field label suitable for diagnostics.
pub fn collect_paths(annotation: &CastAnnotation) -> Vec<CollectedPath<'_>> {
    let mut out: Vec<CollectedPath<'_>> = Vec::new();
    match annotation {
        CastAnnotation::Compare(c) => {
            for (i, entry) in c.entries.iter().enumerate() {
                if let Some(p) = &entry.anchor.path {
                    out.push(CollectedPath {
                        field: format!("entries[{i}].{}", entry.anchor.name),
                        path: p,
                        role: None,
                    });
                }
            }
        }
        CastAnnotation::Pipeline(p) => {
            for (i, stage) in p.stages.iter().enumerate() {
                if let Some(path) = &stage.path {
                    out.push(CollectedPath {
                        field: format!("stages[{i}].{}", stage.name),
                        path,
                        role: None,
                    });
                }
            }
        }
        CastAnnotation::Tier(t) => {
            if let Some(p) = &t.of {
                out.push(CollectedPath { field: "of".to_string(), path: p, role: None });
            }
            for (i, stage) in t.stages.iter().enumerate() {
                if let Some(p) = &stage.anchor.path {
                    out.push(CollectedPath {
                        field: format!("stages[{i}].{}", stage.anchor.name),
                        path: p,
                        role: None,
                    });
                }
            }
        }
        CastAnnotation::Matrix(m) => {
            for (i, row) in m.rows.iter().enumerate() {
                if let Some(p) = &row.anchor.path {
                    out.push(CollectedPath {
                        field: format!("rows[{i}].{}", row.anchor.name),
                        path: p,
                        role: None,
                    });
                }
            }
        }
        CastAnnotation::Rule(r) => {
            for (i, p) in r.governs.iter().enumerate() {
                out.push(CollectedPath { field: format!("governs[{i}]"), path: p, role: None });
            }
        }
        CastAnnotation::AntiPattern(a) => {
            if let Some(p) = &a.instead_at {
                out.push(CollectedPath {
                    field: "instead_at".to_string(),
                    path: p,
                    role: None,
                });
            }
            for (i, p) in a.governs.iter().enumerate() {
                out.push(CollectedPath { field: format!("governs[{i}]"), path: p, role: None });
            }
        }
        CastAnnotation::GutCheck(g) => {
            if let Some(p) = &g.yes_at {
                out.push(CollectedPath { field: "yes_at".to_string(), path: p, role: None });
            }
            if let Some(p) = &g.no_at {
                out.push(CollectedPath { field: "no_at".to_string(), path: p, role: None });
            }
        }
        CastAnnotation::ContinuesIn(c) => {
            out.push(CollectedPath {
                field: "target".to_string(),
                path: &c.target,
                role: None,
            });
        }
        CastAnnotation::IoContinuesIn(_) => {
            // target is a string (out-of-Rust path); validation here is
            // file-existence + lang-specific grep, handled separately
            // in the cross-language validator (TBD).
        }
        CastAnnotation::Concept(c) => {
            for (i, entry) in c.anchors.iter().enumerate() {
                out.push(CollectedPath {
                    field: format!("anchors[{i}]"),
                    path: &entry.path,
                    role: Some(entry.role),
                });
            }
        }
        CastAnnotation::Policy(_) => {
            // No anchors: cast::policy! is repo-level metadata, not a
            // concept that points at code.
        }
    }
    out
}

/// Render a `syn::Path` back to source text for diagnostics.
/// `quote!` produces tokens with single spaces; we strip those so the
/// output reads like the original source.
pub fn syn_path_to_string(path: &syn::Path) -> String {
    path.to_token_stream().to_string().replace(' ', "")
}
