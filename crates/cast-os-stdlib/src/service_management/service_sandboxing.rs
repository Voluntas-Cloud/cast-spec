//! `service_sandboxing` — per-service isolation policy.

/// Sentinel for `service_sandboxing`.
pub struct ServiceSandboxing;

cast::concept! {
    name: "service_sandboxing",
    summary: "per-service isolation policy.",
    anchors: [cast_os_stdlib::service_management::service_sandboxing::ServiceSandboxing],
    tags: ["cast_os_stdlib", "service_management"],
}
