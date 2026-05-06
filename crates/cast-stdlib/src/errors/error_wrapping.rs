//! `error_wrapping` — preserve causal context.

/// Sentinel for `error_wrapping`.
pub struct ErrorWrapping;

cast::concept! {
    name: "error_wrapping",
    summary: "Preserve causal context. Each layer adds what it was \
              trying to do without throwing away the inner cause; the \
              final report reads as a chain rather than a single \
              decontextualized line.",
    anchors: [cast_stdlib::errors::error_wrapping::ErrorWrapping],
    tags: ["cast_stdlib", "errors"],
}
