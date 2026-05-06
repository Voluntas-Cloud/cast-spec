//! `service_unit` — managed service definition.

/// Sentinel for `service_unit`.
pub struct ServiceUnit;

cast::concept! {
    name: "service_unit",
    summary: "managed service definition.",
    anchors: [cast_os_stdlib::service_management::service_unit::ServiceUnit],
    tags: ["cast_os_stdlib", "service_management"],
}
