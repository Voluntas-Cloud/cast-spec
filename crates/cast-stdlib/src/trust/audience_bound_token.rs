//! `audience_bound_token` — token valid only for a specific service.

/// Sentinel for `audience_bound_token`.
pub struct AudienceBoundToken;

cast::concept! {
    name: "audience_bound_token",
    summary: "Token valid only for a specific service. Even a leaked \
              token cannot be replayed against a different service.",
    anchors: [cast_stdlib::trust::audience_bound_token::AudienceBoundToken],
    tags: ["cast_stdlib", "trust"],
}
