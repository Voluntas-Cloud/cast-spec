//! `service_account_identity` — OS identity used by service.

/// Sentinel for `service_account_identity`.
pub struct ServiceAccountIdentity;

cast::concept! {
    name: "service_account_identity",
    summary: "OS identity used by service.",
    anchors: [cast_os_stdlib::service_management::service_account_identity::ServiceAccountIdentity],
    tags: ["cast_os_stdlib", "service_management"],
}
