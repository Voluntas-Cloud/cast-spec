//! `not_found_error` — resource does not exist or is hidden.

/// Sentinel for `not_found_error`.
pub struct NotFoundError;

cast::concept! {
    name: "not_found_error",
    summary: "Resource does not exist or is hidden. The response is the \
              same whether the resource never existed or the caller is \
              just not allowed to see it — that ambiguity is often \
              deliberate, sometimes accidental.",
    anchors: [cast_stdlib::errors::not_found_error::NotFoundError],
    tags: ["cast_stdlib", "errors"],
}
