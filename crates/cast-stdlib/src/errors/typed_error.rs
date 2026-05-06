//! `typed_error` — error has structured category.

/// Sentinel for `typed_error`.
pub struct TypedError;

cast::concept! {
    name: "typed_error",
    summary: "Error has a structured category. Callers branch on the \
              variant, not on a parsed string; renames in the prose \
              don't silently break recovery logic.",
    anchors: [cast_stdlib::errors::typed_error::TypedError],
    tags: ["cast_stdlib", "errors"],
}
