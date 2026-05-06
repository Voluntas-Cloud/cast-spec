//! How an anchor *resolves*. A `LanguageBackend` is an anchor-
//! resolution strategy: given an anchor (a `syn::Path` for Rust, a
//! file/anchor pair for foreign code), it returns whether the
//! anchor names a real item in the corpus.
//!
//! Two backends ship today:
//!
//! - [`rust_hir::RustHirBackend`] — wraps
//!   [`crate::validator::resolver::resolve_syn_path`]. Used for
//!   anchors that point at Rust items, regardless of whether the
//!   spec arrived as an inline macro or from a `.cast` file.
//! - [`foreign_grep::ForeignGrepBackend`] — wraps the existing
//!   per-language grep validation in
//!   [`crate::validator::io_target`]. Same behavior as today, just
//!   behind the trait. The future foreign-AST backend slots in
//!   here: a `KotlinAstBackend`, a `TypeScriptAstBackend`, etc.
//!   would be peer impls without touching the validator's call
//!   sites.

cast::concept! {
    name: "language_backend",
    summary: "An anchor-resolution strategy. RustHirBackend wraps \
              today's rust-analyzer-backed resolve_syn_path; \
              ForeignGrepBackend wraps today's per-language grep \
              validation. New backends — foreign-AST resolvers in \
              particular — slot in as peer impls without touching \
              the validator's call sites. The trait is the seam \
              that makes language a pluggable axis instead of a \
              hard-coded match arm.",
    anchors: [
        crate::language_backend::LanguageBackend,
        crate::language_backend::rust_hir::RustHirBackend,
        crate::language_backend::foreign_grep::ForeignGrepBackend,
    ],
    tags: ["language_backend"],
}

pub mod foreign_grep;
pub mod rust_hir;
pub mod tree_sitter;

cast::continues_in! {
    target: cast_stdlib::function_properties::deterministic,
    concept: "language_backend",
    why: "Resolution is a function of (anchor, corpus state) — both \
          loaded into memory before the call. Same input + same loaded \
          HIR/file-bytes always yields the same verdict.",
}

cast::continues_in! {
    target: cast_stdlib::function_properties::deterministic,
    concept: "tree_sitter_backend",
    why: "tree-sitter parse + traversal is deterministic on the input \
          byte-slice; same bytes always produce the same parse tree \
          and the same anchor-found verdict.",
}

cast::continues_in! {
    target: cast_stdlib::function_properties::deterministic,
    concept: "c_language_handler",
    why: "Same input bytes + same anchor string always produce the \
          same anchor-found verdict.",
}

cast::continues_in! {
    target: cast_stdlib::function_properties::deterministic,
    concept: "markdown_language_backend",
    why: "Same markdown content + same anchor string always produce \
          the same heading-path match verdict.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::sentinel_type,
    concept: "language_backend",
    why: "Each backend impl (RustHirBackend, ForeignGrepBackend) is a \
          zero-sized marker. Trait-method dispatch on a unit struct.",
}

/// An anchor-resolution strategy.
///
/// This trait is intentionally minimal in this first cut — it names
/// the backend (`name()` is used for diagnostics) and that is all.
/// Today's typed entry points
/// ([`crate::validator::resolver::resolve_syn_path`] for Rust,
/// [`crate::validator::io_target::validate_io_annotation`] for
/// foreign code) keep their existing signatures because their input
/// shapes are too different to unify cleanly in one step:
///
/// - Rust takes a `syn::Path` plus a calling module.
/// - Foreign takes a file path plus a language tag plus an
///   optional anchor string.
///
/// Unifying them behind a single `resolve()` method would require
/// an `Anchor` enum and a synthetic verdict type that both arms
/// map to. That is design work for the *next* step. For now the
/// trait is the named seam — adding a foreign-AST backend later
/// means adding a new impl, not editing the validator's call sites.
pub trait LanguageBackend {
    /// Short identifier used in diagnostics and emit output.
    fn name(&self) -> &'static str;
}
