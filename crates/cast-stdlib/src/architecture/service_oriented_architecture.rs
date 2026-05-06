//! `service_oriented_architecture` — capabilities exposed as services.

/// Sentinel for `service_oriented_architecture`.
pub struct ServiceOrientedArchitecture;

cast::concept! {
    name: "service_oriented_architecture",
    summary: "Capabilities exposed as services. Coarser-grained than \
              microservices; services align with business capabilities.",
    anchors: [cast_stdlib::architecture::service_oriented_architecture::ServiceOrientedArchitecture],
    tags: ["cast_stdlib", "architecture"],
}
