//! `authorization_policy` — what an authenticated principal may do.

/// Sentinel for `authorization_policy`.
pub struct AuthorizationPolicy;

cast::concept! {
    name: "authorization_policy",
    summary: "Decides what an authenticated principal may do. \
              Separable from authentication — same identity, \
              different permissions in different contexts.",
    anchors: [cast_stdlib::trust::authorization_policy::AuthorizationPolicy],
    tags: ["cast_stdlib", "trust"],
}
