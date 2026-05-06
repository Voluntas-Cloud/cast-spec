//! `SidecarSource` — the [`super::SpecSource`] notation that reads
//! `<name>.rs.cast` files placed next to the `<name>.rs` file they
//! describe.
//!
//! The sidecar shape gives `.cast` files relative-anchor ergonomics:
//! the calling module is inferred from the sibling `.rs`, so
//! `self::*`, `super::*`, and unqualified path heads resolve the
//! same way they would in an inline `cast::*!` block. Without this
//! variant, every `.cast` file requires absolute paths
//! (`<crate>::<module>::<item>`) — fine for workspace-level umbrellas
//! but verbose at per-file granularity.
//!
//! Discovery walks the project root for `*.rs.cast` files where the
//! sibling `*.rs` exists on disk AND is part of one of the loaded
//! [`crate::project::ProjectHandle`]s. Sidecars whose sibling can't
//! be matched to a handle are skipped — the calling module can't be
//! resolved without HIR, and emitting them with `calling_module:
//! None` would silently downgrade them to umbrella shape (where the
//! repo intended sidecar semantics). Skipping is the loud signal.
//!
//! Non-sidecar `.cast` files (`Cast.cast`, `spec.cast`, anything
//! whose name doesn't end in `.rs.cast`) continue to be handled by
//! [`super::cast_file::CastFileSource`] under the absolute-path
//! rule.

cast::concept! {
    name: "rs_cast_sidecar",
    summary: "A `<name>.rs.cast` file placed next to `<name>.rs`. The \
              analyzer treats the sibling `.rs` as the calling \
              module, so relative anchors (`self::*`, `super::*`, \
              unqualified path heads) resolve the same way an inline \
              `cast::*!` block in the `.rs` would. Gives `.cast` \
              files inline-grade anchor ergonomics at per-file \
              granularity, without polluting the `.rs` source itself. \
              The shape `cast::policy! { layout: sidecar_only, ... }` \
              opts a repo into using this notation exclusively.",
    anchors: [
        crate::spec_source::sidecar::SidecarSource,
    ],
    tags: ["spec_source"],
}

cast::compare! {
    inline   @ crate::spec_source::rust_macros::RustMacroSource:
        "Annotation lives inside the .rs file — single source of \
         truth, but the .rs file carries cast vocabulary that some \
         readers find noisy.",
    sidecar  @ crate::spec_source::sidecar::SidecarSource:
        "Annotation lives in `<name>.rs.cast` next to `<name>.rs`. \
         Calling-module is the sibling .rs (looked up via VFS / HIR), \
         so relative anchors work the same as inline. .rs file stays \
         pristine; sidecar can be added or deleted without modifying \
         source.",
    detached @ crate::spec_source::cast_file::CastFileSource:
        "Annotation lives in a non-sidecar `.cast` file (Cast.cast, \
         spec.cast, etc.). No calling module — anchors must be \
         absolute. Right shape for workspace-level umbrellas.",
    tags: ["spec_source"],
    note: "Three notations on a single anchor-resolution axis. The \
           sidecar variant exists so a repo can opt out of inline \
           macros in source files without losing relative-path \
           ergonomics.",
}

use crate::finder::CastInvocation;
use crate::model::Location;
use crate::project::{handle_for_file, MultiProject};
use crate::spec_source::SpecSource;
use proc_macro2::TokenStream;
use ra_ap_hir::Semantics;
use ra_ap_vfs::VfsPath;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{spanned::Spanned, Item};

/// [`SpecSource`] that loads `<name>.rs.cast` sidecars under a
/// project root. Resolves the calling module of each sidecar to the
/// sibling `<name>.rs`'s module via the loaded
/// [`MultiProject`]'s rust-analyzer state.
pub struct SidecarSource<'a> {
    pub root: PathBuf,
    pub multi: &'a MultiProject,
}

impl<'a> SidecarSource<'a> {
    pub fn new(root: impl Into<PathBuf>, multi: &'a MultiProject) -> Self {
        Self {
            root: root.into(),
            multi,
        }
    }

    /// Walk `root` recursively. For every `*.rs.cast` file whose
    /// sibling `*.rs` exists on disk, return both paths. Same ignore
    /// rules as the cast-file walker.
    fn discover(&self) -> Vec<(PathBuf, PathBuf)> {
        let mut out = Vec::new();
        walk(&self.root, &mut out);
        out
    }
}

impl<'a> SpecSource for SidecarSource<'a> {
    fn invocations(&self) -> Vec<CastInvocation> {
        let mut out = Vec::new();
        for (sidecar_path, rs_path) in self.discover() {
            let calling_module = match resolve_sibling_module(self.multi, &rs_path) {
                Some(m) => m,
                None => continue,
            };
            let Ok(src) = fs::read_to_string(&sidecar_path) else {
                continue;
            };
            let Ok(file) = syn::parse_file(&src) else {
                continue;
            };
            for item in file.items {
                if let Some(inv) =
                    invocation_from_item(&sidecar_path, &item, Some(calling_module))
                {
                    out.push(inv);
                }
            }
        }
        out
    }
}

/// Return the rust-analyzer `Module` for the `.rs` file at
/// `rs_path`, looked up through whichever loaded handle's VFS
/// contains it. `None` when no handle owns the file or when its
/// module-def can't be resolved (e.g. the file is in the VFS but
/// not part of a crate's source tree).
fn resolve_sibling_module(
    multi: &MultiProject,
    rs_path: &Path,
) -> Option<ra_ap_hir::Module> {
    let canonical = rs_path.canonicalize().ok()?;
    let handle_idx = handle_for_file(multi, &canonical)?;
    let handle = &multi.handles[handle_idx];
    let vfs_path = VfsPath::new_real_path(canonical.display().to_string());
    let (file_id, _excluded) = handle.vfs.file_id(&vfs_path)?;
    let sema = Semantics::new(&handle.db);
    sema.file_to_module_def(file_id)
}

fn invocation_from_item(
    file_path: &Path,
    item: &Item,
    calling_module: Option<ra_ap_hir::Module>,
) -> Option<CastInvocation> {
    let Item::Macro(item_mac) = item else {
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
        calling_module,
    })
}

fn render_macro_path(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

/// `is_sidecar_filename("mod.rs.cast")` → true.
/// `is_sidecar_filename("spec.cast")` → false.
/// `is_sidecar_filename("Cast.cast")` → false.
///
/// Public so [`super::cast_file::CastFileSource`] can reuse the
/// classification when filtering the umbrella set.
pub fn is_sidecar_filename(name: &str) -> bool {
    name.ends_with(".rs.cast") && name.len() > ".rs.cast".len()
}

fn walk(dir: &Path, out: &mut Vec<(PathBuf, PathBuf)>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let Ok(meta) = entry.file_type() else {
            continue;
        };
        let path = entry.path();
        if meta.is_symlink() {
            continue;
        }
        if meta.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if matches!(name, "target" | ".git" | "node_modules") {
                continue;
            }
            walk(&path, out);
        } else if meta.is_file() {
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if !is_sidecar_filename(name) {
                continue;
            }
            // Strip the trailing `.cast` to get the sibling `.rs` path.
            // `mod.rs.cast` → `mod.rs`.
            let rs_path = path.with_extension(""); // drops `.cast`
            if rs_path.is_file() {
                out.push((path, rs_path));
            }
            // Sidecar with no sibling .rs is silently dropped — it's
            // an authoring mistake (renamed the .rs but kept the
            // sidecar, or vice versa). A future commit can surface
            // this as an `orphaned_sidecar` warning.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_filenames() {
        assert!(is_sidecar_filename("mod.rs.cast"));
        assert!(is_sidecar_filename("lib.rs.cast"));
        assert!(is_sidecar_filename("a.rs.cast"));

        assert!(!is_sidecar_filename("Cast.cast"));
        assert!(!is_sidecar_filename("spec.cast"));
        assert!(!is_sidecar_filename(".rs.cast")); // empty stem
        assert!(!is_sidecar_filename("mod.rs"));
        assert!(!is_sidecar_filename("readme.md"));
    }

    #[test]
    fn discover_skips_sidecars_with_no_sibling_rs() {
        // Build a tmpdir with one orphan sidecar (no sibling .rs) and
        // one valid sidecar (with sibling). The orphan should be
        // silently dropped; the valid one is yielded.
        let tmp = tempdir();
        std::fs::write(tmp.join("orphan.rs.cast"), "").unwrap();
        std::fs::write(tmp.join("real.rs"), "").unwrap();
        std::fs::write(tmp.join("real.rs.cast"), "").unwrap();

        let mut out = Vec::new();
        walk(&tmp, &mut out);

        assert_eq!(out.len(), 1);
        assert!(out[0].0.ends_with("real.rs.cast"));
        assert!(out[0].1.ends_with("real.rs"));

        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn discover_skips_non_sidecar_cast_files() {
        let tmp = tempdir();
        std::fs::write(tmp.join("Cast.cast"), "").unwrap();
        std::fs::write(tmp.join("spec.cast"), "").unwrap();

        let mut out = Vec::new();
        walk(&tmp, &mut out);
        assert!(out.is_empty());

        std::fs::remove_dir_all(&tmp).ok();
    }

    fn tempdir() -> PathBuf {
        // Tiny, dependency-free tempdir helper. Tests clean up
        // themselves at the end.
        let p = std::env::temp_dir().join(format!(
            "cast-sidecar-test-{}-{}",
            std::process::id(),
            uniq()
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn uniq() -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static N: AtomicU64 = AtomicU64::new(0);
        N.fetch_add(1, Ordering::Relaxed)
    }
}
