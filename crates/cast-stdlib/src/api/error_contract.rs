//! `error_contract` — structured, predictable error responses.

/// Sentinel for `error_contract`.
pub struct ErrorContract;

cast::concept! {
    name: "error_contract",
    summary: "Structured, predictable error responses. Errors are typed; \
              callers can branch on category without parsing prose.",
    anchors: [cast_stdlib::api::error_contract::ErrorContract],
    tags: ["cast_stdlib", "api"],
}
