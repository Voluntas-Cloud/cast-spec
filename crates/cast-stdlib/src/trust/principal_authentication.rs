//! `principal_authentication` — proves who is making a request.

/// Sentinel for `principal_authentication`.
pub struct PrincipalAuthentication;

cast::concept! {
    name: "principal_authentication",
    summary: "Proves who is making a request. Distinct from \
              authorization (may they do this) and capability \
              presentation (do they hold the right token).",
    anchors: [cast_stdlib::trust::principal_authentication::PrincipalAuthentication],
    tags: ["cast_stdlib", "trust"],
}
