//! `authorization_error` — caller lacks permission.

/// Sentinel for `authorization_error`.
pub struct AuthorizationError;

cast::concept! {
    name: "authorization_error",
    summary: "Caller lacks permission. The request is well-formed and \
              the principal is authenticated — they're just not allowed. \
              Distinct from authentication failure so logs and audits \
              can tell them apart.",
    anchors: [cast_stdlib::errors::authorization_error::AuthorizationError],
    tags: ["cast_stdlib", "errors"],
}
