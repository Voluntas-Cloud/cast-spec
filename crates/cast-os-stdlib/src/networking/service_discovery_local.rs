//! `service_discovery_local` — OS-local discovery of services.

/// Sentinel for `service_discovery_local`.
pub struct ServiceDiscoveryLocal;

cast::concept! {
    name: "service_discovery_local",
    summary: "OS-local discovery of services.",
    anchors: [cast_os_stdlib::networking::service_discovery_local::ServiceDiscoveryLocal],
    tags: ["cast_os_stdlib", "networking"],
}
