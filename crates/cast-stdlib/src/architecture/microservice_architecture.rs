//! `microservice_architecture` — independently deployable services.

/// Sentinel for `microservice_architecture`.
pub struct MicroserviceArchitecture;

cast::concept! {
    name: "microservice_architecture",
    summary: "Independently deployable services. Maximum team \
              independence; cost is operational complexity multiplied \
              by service count.",
    anchors: [cast_stdlib::architecture::microservice_architecture::MicroserviceArchitecture],
    tags: ["cast_stdlib", "architecture"],
}
