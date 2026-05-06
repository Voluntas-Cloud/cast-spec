//! `fault_containment` — failures stay local.

/// Sentinel for `fault_containment`.
pub struct FaultContainment;

cast::concept! {
    name: "fault_containment",
    summary: "failures stay local.",
    anchors: [cast_os_stdlib::design_qualities::fault_containment::FaultContainment],
    tags: ["cast_os_stdlib", "design_qualities"],
}
