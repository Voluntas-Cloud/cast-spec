//! `capability_based_access_system` — authorization by scoped capabilities, not broad identity.

/// Sentinel for `capability_based_access_system`.
pub struct CapabilityBasedAccessSystem;

cast::concept! {
    name: "capability_based_access_system",
    summary: "Authorization based on possession of scoped \
              capabilities rather than broad identity alone. \
              Composes capability_token, scope_limited_token, \
              audience_bound_token, time_bound_credential, \
              revocable_credential, least_privilege, and \
              delegated_authority. Used for temporary access links, \
              agent tool permissions, service-to-service delegation, \
              limited admin sessions, and user-approved automation.",
    anchors: [cast_stdlib::patterns::capability_based_access_system::CapabilityBasedAccessSystem],
    tags: ["cast_stdlib", "patterns"],
}
