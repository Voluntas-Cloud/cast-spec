//! `service_readiness_protocol` — service declares readiness.

/// Sentinel for `service_readiness_protocol`.
pub struct ServiceReadinessProtocol;

cast::concept! {
    name: "service_readiness_protocol",
    summary: "service declares readiness.",
    anchors: [cast_os_stdlib::service_management::service_readiness_protocol::ServiceReadinessProtocol],
    tags: ["cast_os_stdlib", "service_management"],
}
