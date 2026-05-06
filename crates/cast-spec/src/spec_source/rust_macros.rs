//! `RustMacroSource` — the [`super::SpecSource`] notation that reads
//! `cast::*!` invocations embedded directly in `.rs` source files.
//!
//! This is the *original* mode of cast: the spec and the code it
//! anchors at live in the same file. Discovery uses rust-analyzer's
//! HIR (`crate::finder::find_invocations`) so that the calling
//! module is known and unqualified / `self::` / `super::` paths
//! resolve the way they would in the surrounding Rust code.
//!
//! This module is the trait wrapper around the existing finder. The
//! finder itself does not change; what changes is that downstream
//! code talks to it through [`super::SpecSource`] instead of calling
//! `find_invocations` directly.

cast::concept! {
    name: "inline_spec",
    summary: "A cast annotation expressed as a Rust macro invocation \
              embedded directly in a .rs source file. Spec and \
              anchor share the same file; the analyzer picks them \
              up via rust-analyzer's HIR; the calling module is \
              known so unqualified anchor paths resolve. The \
              original mode of cast and the only mode supported \
              before .cast files landed.",
    anchors: [
        crate::spec_source::rust_macros::RustMacroSource,
        crate::finder::find_invocations,
        crate::finder::CastInvocation,
    ],
    tags: ["spec_source"],
}

use crate::finder::{find_invocations, CastInvocation};
use crate::spec_source::SpecSource;
use ra_ap_ide_db::RootDatabase;
use ra_ap_vfs::Vfs;

/// [`SpecSource`] for `cast::*!` invocations embedded in `.rs`
/// source files.
///
/// Borrows the HIR handles for one project root. The trait method
/// just calls [`find_invocations`] — the wrapper exists so call
/// sites can speak to the trait, not the free function.
pub struct RustMacroSource<'a> {
    pub db: &'a RootDatabase,
    pub vfs: &'a Vfs,
}

impl<'a> RustMacroSource<'a> {
    pub fn new(db: &'a RootDatabase, vfs: &'a Vfs) -> Self {
        Self { db, vfs }
    }
}

impl<'a> SpecSource for RustMacroSource<'a> {
    fn invocations(&self) -> Vec<CastInvocation> {
        find_invocations(self.db, self.vfs)
    }
}
