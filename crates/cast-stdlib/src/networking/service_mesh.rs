//! `service_mesh` — network layer for service-to-service policy/telemetry.

/// Sentinel for `service_mesh`.
pub struct ServiceMesh;

cast::concept! {
    name: "service_mesh",
    summary: "Network layer for service-to-service policy and telemetry. \
              Sidecars or eBPF intercept inter-service traffic so retries, \
              mTLS, and metrics happen outside the application.",
    anchors: [cast_stdlib::networking::service_mesh::ServiceMesh],
    tags: ["cast_stdlib", "networking"],
}
