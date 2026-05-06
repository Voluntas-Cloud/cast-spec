//! `driver_isolation` — contain driver faults.

/// Sentinel for `driver_isolation`.
pub struct DriverIsolation;

cast::concept! {
    name: "driver_isolation",
    summary: "contain driver faults.",
    anchors: [cast_os_stdlib::driver_model::driver_isolation::DriverIsolation],
    tags: ["cast_os_stdlib", "driver_model"],
}
