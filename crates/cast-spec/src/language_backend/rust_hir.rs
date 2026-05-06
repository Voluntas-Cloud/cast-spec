//! `RustHirBackend` — the [`super::LanguageBackend`] impl for Rust
//! anchors. Wraps
//! [`crate::validator::resolver::resolve_syn_path`] so the existing
//! HIR-based path walker is reachable through the trait without
//! changing its behavior.
//!
//! All Rust anchors flow through here, regardless of whether the
//! spec arrived as an inline `cast::*!` macro or from a `.cast`
//! file. The only difference between the two cases is the
//! *calling-module* context: `.rs` macros inherit it from HIR;
//! `.cast` files do not have one and must use absolute paths
//! (enforced upstream by `cast_file`'s parse rule).

use super::LanguageBackend;

pub struct RustHirBackend;

impl LanguageBackend for RustHirBackend {
    fn name(&self) -> &'static str {
        "rust-hir"
    }
}
