//! `registry_service_discovery` — find services via registry/control plane.

/// Sentinel for `registry_service_discovery`.
pub struct RegistryServiceDiscovery;

cast::concept! {
    name: "registry_service_discovery",
    summary: "Find services via registry or control plane. A central \
              system knows the live endpoints; clients query or subscribe \
              instead of trusting DNS caches.",
    anchors: [cast_stdlib::networking::registry_service_discovery::RegistryServiceDiscovery],
    tags: ["cast_stdlib", "networking"],
}
