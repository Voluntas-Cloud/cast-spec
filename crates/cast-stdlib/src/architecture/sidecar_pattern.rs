//! `sidecar_pattern` — helper alongside main service for cross-cutting concerns.

/// Sentinel for `sidecar_pattern`.
pub struct SidecarPattern;

cast::concept! {
    name: "sidecar_pattern",
    summary: "Helper process/container alongside main service. \
              Cross-cutting concerns (TLS, observability, policy) live \
              in the sidecar.",
    anchors: [cast_stdlib::architecture::sidecar_pattern::SidecarPattern],
    tags: ["cast_stdlib", "architecture"],
}
