//! Finds `cast::*!` macro invocations across a loaded project using
//! ra_ap_hir's semantic resolver.
//!
//! Why hir and not raw syntax: hir resolves the macro path through any
//! `use cast::compare;` / `use cast::compare as cmp;` / `use cast::*;`
//! aliasing, so we don't miss invocations that reach our macros via
//! re-exports. The macro definition's owning crate name is the ground
//! truth for "is this ours" — not the literal text of the call.
//!
//! We use `Semantics::resolve_path` rather than `resolve_macro_call`.
//! The latter needs the call site to have been collected as a
//! `MacroCallId` during def-map construction, which only happens after
//! macro expansion runs. `resolve_path` goes through ordinary name
//! resolution and works on un-expanded call paths — the right primitive
//! for static analysis.

use crate::model::Location;
use proc_macro2::TokenStream;
use ra_ap_hir::{Module, ModuleDef, PathResolution, Semantics};
use ra_ap_ide_db::{line_index, RootDatabase};
use ra_ap_syntax::{ast, AstNode};
use ra_ap_vfs::Vfs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct CastInvocation {
    /// Canonical macro path, e.g. `cast::compare` or `cast::io::continues_in`.
    /// Always uses the macro's *defining* path, not the path written at
    /// the call site.
    pub macro_path: String,
    pub body: TokenStream,
    pub location: Location,
    /// The HIR module containing this invocation. Used by the resolver
    /// to anchor relative paths (`crate::`, `self::`, `super::`). May be
    /// `None` if ra_ap_hir can't map the file to a module — should
    /// always succeed for files inside loaded crates, but we don't
    /// crash if it doesn't.
    pub calling_module: Option<Module>,
}

pub fn find_invocations(db: &RootDatabase, vfs: &Vfs) -> Vec<CastInvocation> {
    let sema = Semantics::new(db);
    let mut found = Vec::new();

    for (file_id, vfs_path) in vfs.iter() {
        let path_str = vfs_path.as_path().map(|p| p.to_string());
        let path = match path_str {
            Some(p) => PathBuf::from(p),
            None => continue,
        };

        // VFS includes every file under each crate root (.md, .toml,
        // etc.). Skip non-Rust — they tokenise as garbage.
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }

        let source_file = sema.parse(ra_ap_hir::EditionedFileId::current_edition(db, file_id));

        for macro_call in source_file
            .syntax()
            .descendants()
            .filter_map(ast::MacroCall::cast)
        {
            let Some(invocation) = extract_cast_invocation(&sema, db, &macro_call, &path, file_id)
            else {
                continue;
            };
            found.push(invocation);
        }
    }

    found
}

fn extract_cast_invocation(
    sema: &Semantics<'_, RootDatabase>,
    db: &RootDatabase,
    macro_call: &ast::MacroCall,
    file_path: &std::path::Path,
    file_id: ra_ap_vfs::FileId,
) -> Option<CastInvocation> {
    // resolve_path on the macro's path produces the Macro item the call
    // would dispatch to. This handles `use cast::compare;` aliasing,
    // `use cast::*;` glob imports, and re-exports transparently.
    let path = macro_call.path()?;
    let macro_def = match sema.resolve_path(&path)? {
        PathResolution::Def(ModuleDef::Macro(m)) => m,
        _ => return None,
    };

    let krate = macro_def.module(db).krate(db);
    let krate_name = krate.display_name(db)?.canonical_name().to_string();

    // Only invocations of macros defined in the `cast` crate are ours.
    if krate_name != "cast" {
        return None;
    }

    // The macro's defining name (so callers can't disguise their target
    // through aliasing). cast::io::continues_in is `continues_in_io` at
    // the definition site; the canonical-path normalisation is stage 3.
    let macro_name = macro_def.name(db).as_str().to_string();
    let macro_path = format!("cast::{macro_name}");

    let body_tt = macro_call.token_tree()?;
    let body_text = body_tt.syntax().text().to_string();
    let inner = strip_outer_delims(&body_text);
    let body = TokenStream::from_str(inner).ok()?;

    let line_index = line_index(db, file_id);
    let offset = macro_call.syntax().text_range().start();
    let line_col = line_index.line_col(offset);

    let calling_module = sema.file_to_module_def(file_id);

    Some(CastInvocation {
        macro_path,
        body,
        location: Location::new(
            file_path.to_path_buf(),
            line_col.line as usize + 1,
            line_col.col as usize + 1,
        ),
        calling_module,
    })
}

fn strip_outer_delims(s: &str) -> &str {
    let trimmed = s.trim();
    let bytes = trimmed.as_bytes();
    if bytes.len() >= 2 {
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        if matches!((first, last), (b'{', b'}') | (b'[', b']') | (b'(', b')')) {
            return trimmed[1..trimmed.len() - 1].trim();
        }
    }
    trimmed
}

cast::concept! {
    name: "macro_invocation_finder",
    summary: "Walks the loaded rust-analyzer HIR to discover every \
              cast::*! macro invocation in the analyzed roots. Returns \
              one CastInvocation per call site with its location, \
              macro path, and unparsed body. Deterministic on the HIR \
              snapshot — the same loaded RootDatabase always yields \
              the same invocation list in the same order — but not \
              pure: it reads through the salsa-tracked database, \
              which is observable shared state.",
    anchors: [
        crate::finder::find_invocations,
        crate::finder::extract_cast_invocation,
    ],
    tags: ["cast_spec_finder"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::deterministic,
    concept: "macro_invocation_finder",
    why: "Same loaded RootDatabase + same Vfs snapshot always yields \
          the same invocation list.",
}

cast::concept! {
    name: "delimiter_stripping_helper",
    summary: "Pure helper that strips a single layer of outer brace, \
              bracket, or parenthesis pair from a string slice and \
              returns the inner span. Used to normalise the body bytes \
              of a cast::*! invocation before parsing.",
    anchors: [
        crate::finder::strip_outer_delims,
    ],
    tags: ["cast_spec_finder"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "delimiter_stripping_helper",
    why: "Output is a function of the input slice alone.",
}

cast::concept! {
    name: "discovered_invocation_value",
    summary: "Discovered cast::*! invocation — its location, macro \
              path, body bytes, and calling-module context. Output of \
              the finder; input to the parser.",
    anchors: [
        crate::finder::CastInvocation,
    ],
    tags: ["cast_spec_finder"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::product_type,
    concept: "discovered_invocation_value",
    why: "Struct with all fields simultaneously inhabited.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::value_type,
    concept: "discovered_invocation_value",
    why: "Cloneable, structurally compared, no resource handles.",
}

cast::continues_in! {
    target: cast_stdlib::architecture::typed_handle,
    concept: "discovered_invocation_value",
    why: lazy,
}

