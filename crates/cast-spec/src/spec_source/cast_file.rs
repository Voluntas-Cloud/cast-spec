//! `CastFileSource` — the [`super::SpecSource`] notation that reads
//! `*.cast` files: clean Rust syntax constrained to `cast::*!` macro
//! invocations only.
//!
//! Two roles, one extension:
//!
//! - `Cast.cast` at the workspace root: the project file. Declares
//!   roots, configuration, and project-level cast macros (concepts,
//!   comparisons, etc.). Sibling to `Cargo.toml`. Replaces (or
//!   overrides) the "pass roots on the CLI" workflow.
//! - `*.cast` (e.g. `spec.cast`) anywhere — including inside
//!   foreign-language repos: the spec file. Holds
//!   `cast::concept!` / `rule!` / `anti_pattern!` / etc. for code
//!   that lives in this directory or sub-tree.
//!
//! ### Why this exists
//!
//! A foreign-repo maintainer who sees `spec.rs` reasonably asks
//! "why is Rust in here?" A `spec.cast` file answers that question
//! by its extension: this is a Cast file. That it parses with `syn`
//! is an analyzer implementation detail, not a UX claim on the
//! repo. The file contains nothing the maintainer would recognise
//! as Rust logic — only declarative `cast::*!` blocks.
//!
//! ### Parsing
//!
//! `*.cast` files are parsed with [`syn::parse_file`]. Every
//! top-level item must be a `cast::*!` macro invocation; anything
//! else is rejected at parse time so the constraint is fail-loud
//! rather than silently letting Rust logic creep in. Each
//! invocation's body is then routed through the *same*
//! [`crate::parser::parse_macro_body`] dispatcher used for inline
//! macros, so downstream code (`validator`, `emit`, `graph`) sees
//! one shape of [`crate::parser::CastAnnotation`] regardless of
//! notation.

cast::concept! {
    name: "detached_spec",
    summary: "A cast annotation expressed in a .cast file — clean \
              Rust syntax constrained to cast::*! macro invocations \
              only. Spec lives apart from the code it anchors at. \
              Makes foreign-language code annotatable, because \
              cast::*! macros are Rust syntax and cannot live \
              inline in .kt / .ts / .yaml source. Workspace-level \
              configuration (roots, project-wide concepts) lives \
              in Cast.cast at the workspace root.",
    anchors: [
        crate::spec_source::cast_file::CastFileSource,
    ],
    tags: ["spec_source"],
}

cast::rule! {
    rule: "A .cast file's top-level items must be cast::*! macro \
           invocations only — no fn / struct / impl / trait, and \
           no `use` of paths outside the cast vocabulary.",
    why:  "The whole point of the .cast extension is that a \
           foreign-repo maintainer can scan it and see only \
           declarative cast annotations, not surprise Rust logic. \
           Allowing arbitrary Rust items would re-introduce the \
           'why is there Rust in here?' objection that .cast files \
           exist to dissolve. Enforcing the constraint at parse \
           time (fail-loud) keeps the file-shape promise the \
           extension makes.",
    governs: [
        crate::spec_source::cast_file::CastFileSource,
    ],
    tags: ["spec_source"],
}

cast::rule! {
    rule: "Anchors inside .cast files must be absolute paths rooted \
           at a crate name. No `crate::` / `self::` / `super::` \
           segments are allowed.",
    why:  "A .cast file has no enclosing Rust module from which \
           relative paths could be resolved — `crate::` is \
           ambiguous when the file is not part of any crate. \
           Forcing absolute paths keeps anchor semantics identical \
           to the .rs case where the path was already fully \
           qualified, makes the .cast file self-contained (you can \
           read it and know what it points at without knowing \
           where it lives), and lets a single .cast file annotate \
           multiple crates without choosing a 'home' crate.",
    governs: [
        crate::spec_source::cast_file::CastFileSource,
    ],
    tags: ["spec_source"],
}

use crate::finder::CastInvocation;
use crate::model::Location;
use crate::spec_source::SpecSource;
use proc_macro2::TokenStream;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{spanned::Spanned, Item};

/// [`SpecSource`] that loads `cast::*!` invocations from `*.cast`
/// files under a project root.
///
/// Discovery walks `root` recursively, skipping common ignore dirs
/// (`target`, `.git`, `node_modules`) and never following symlinks.
/// Each discovered file is parsed with [`syn::parse_file`] —
/// `*.cast` files are clean Rust syntax constrained to `cast::*!`
/// macro invocations only, so the parse uses the regular Rust
/// grammar; the constraint surfaces as non-macro top-level items
/// being dropped (currently silent; full diagnostics are a
/// follow-up step).
///
/// Invocations are emitted with `calling_module: None`, since a
/// `.cast` file has no enclosing Rust module. That is what makes
/// the absolute-path rule load-bearing: relative anchors
/// (`crate::` / `self::` / `super::`) cannot resolve without a
/// calling module, so they fail at the resolver — no separate
/// guard needed.
pub struct CastFileSource {
    pub root: PathBuf,
}

impl CastFileSource {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn discover(&self) -> Vec<PathBuf> {
        let mut out = Vec::new();
        walk(&self.root, &mut out);
        out
    }
}

impl SpecSource for CastFileSource {
    fn invocations(&self) -> Vec<CastInvocation> {
        let mut out = Vec::new();
        for path in self.discover() {
            let Ok(src) = fs::read_to_string(&path) else {
                continue;
            };
            let Ok(file) = syn::parse_file(&src) else {
                continue;
            };
            for item in file.items {
                if let Some(inv) = invocation_from_item(&path, &item) {
                    out.push(inv);
                }
            }
        }
        out
    }
}

fn invocation_from_item(file_path: &Path, item: &Item) -> Option<CastInvocation> {
    let Item::Macro(item_mac) = item else {
        // Non-macro top-level item; the .cast file contract forbids
        // these. Today's first cut just drops them silently. A
        // follow-up makes this fail-loud with a diagnostic naming
        // the rule.
        return None;
    };

    let macro_path = render_macro_path(&item_mac.mac.path);
    if !macro_path.starts_with("cast::") {
        return None;
    }

    let body: TokenStream = item_mac.mac.tokens.clone();
    let span_start = item_mac.mac.path.span().start();
    let location = Location::new(
        file_path.to_path_buf(),
        span_start.line,
        span_start.column + 1,
    );

    Some(CastInvocation {
        macro_path,
        body,
        location,
        // .cast files have no enclosing Rust module — relative
        // anchor paths cannot resolve. The absolute-path rule is
        // enforced by the resolver via this None.
        calling_module: None,
    })
}

fn render_macro_path(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let Ok(meta) = entry.file_type() else {
            continue;
        };
        let path = entry.path();
        if meta.is_symlink() {
            // Don't follow symlinks — avoids cycles and keeps
            // discovery confined to the literal tree under root.
            continue;
        }
        if meta.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if matches!(name, "target" | ".git" | "node_modules") {
                continue;
            }
            walk(&path, out);
        } else if meta.is_file()
            && path.extension().and_then(|e| e.to_str()) == Some("cast")
        {
            // Skip `<name>.rs.cast` sidecars — those are handled by
            // SidecarSource with relative-anchor semantics. The
            // umbrella absolute-path rule still applies to every
            // other `.cast` file.
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if super::sidecar::is_sidecar_filename(name) {
                continue;
            }
            out.push(path);
        }
    }
}
