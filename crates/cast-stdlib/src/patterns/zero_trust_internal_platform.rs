//! `zero_trust_internal_platform` — every internal request is authenticated, authorized, and auditable.

/// Sentinel for `zero_trust_internal_platform`.
pub struct ZeroTrustInternalPlatform;

cast::concept! {
    name: "zero_trust_internal_platform",
    summary: "Every internal request is authenticated, authorized, \
              and auditable. Composes mutual_tls_identity, \
              principal_authentication, authorization_policy, \
              network_policy, least_privilege, audit_log, \
              service_identity, and secret_scoping. Used for \
              cluster-internal service security, microservice \
              platforms, the Voluntas internal API mesh, multi-node \
              personal clouds, and regulated infrastructure.",
    anchors: [cast_stdlib::patterns::zero_trust_internal_platform::ZeroTrustInternalPlatform],
    tags: ["cast_stdlib", "patterns"],
}
