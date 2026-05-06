//! `ForeignGrepBackend` — the [`super::LanguageBackend`] impl for
//! non-Rust anchors. Wraps the existing per-language grep
//! validation in [`crate::validator::io_target`] so that today's
//! `cast::io::continues_in!` behavior is reachable through the
//! trait without changing.
//!
//! This is intentionally the same conservative declaration-prefix
//! grep as today (Kotlin `fun X`, Swift `func X`, TS
//! `function X`, …): the trait gives us a place to grow into AST
//! resolution without the validator's call sites having to know.
//! When `KotlinAstBackend` or `TypeScriptAstBackend` lands, it is
//! a peer impl, not an edit to the existing match arms.

use super::LanguageBackend;

pub struct ForeignGrepBackend;

impl LanguageBackend for ForeignGrepBackend {
    fn name(&self) -> &'static str {
        "foreign-grep"
    }
}

cast::concept! {
    name: "c_language_handler",
    summary: "C language support in the foreign-anchor validator. \
              Adds Lang::C plus a per-language anchor matcher that \
              recognises common C declaration shapes — `struct X`, \
              `enum X`, `union X`, `#define X`, and function \
              definitions (anchor followed by `(` at the start of a \
              line, after optional return-type tokens). C is the \
              first foreign language added under the encapsulation \
              seam — every variant after this is a smaller change \
              because the path is already worn.",
    anchors: [
        crate::parser::io_continues_in::Lang::C,
        crate::language_backend::foreign_grep::ForeignGrepBackend,
    ],
    tags: ["language_backend"],
}
