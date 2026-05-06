//! `impersonation_guard` — distinguish 'acting as' from 'is'.

/// Sentinel for `impersonation_guard`.
pub struct ImpersonationGuard;

cast::concept! {
    name: "impersonation_guard",
    summary: "Prevents confusing 'acting as' with 'is'. Audit logs \
              record both the actual principal and the principal \
              they are acting on behalf of.",
    anchors: [cast_stdlib::trust::impersonation_guard::ImpersonationGuard],
    tags: ["cast_stdlib", "trust"],
}
