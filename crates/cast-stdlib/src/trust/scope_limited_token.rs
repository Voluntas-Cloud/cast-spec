//! `scope_limited_token` — token constrained to specific actions/resources.

/// Sentinel for `scope_limited_token`.
pub struct ScopeLimitedToken;

cast::concept! {
    name: "scope_limited_token",
    summary: "Token constrained to specific actions/resources. A \
              leak still bounds the damage to the token's scope.",
    anchors: [cast_stdlib::trust::scope_limited_token::ScopeLimitedToken],
    tags: ["cast_stdlib", "trust"],
}
