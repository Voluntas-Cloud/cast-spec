//! `target_unit` — grouping/synchronization point.

/// Sentinel for `target_unit`.
pub struct TargetUnit;

cast::concept! {
    name: "target_unit",
    summary: "grouping/synchronization point.",
    anchors: [cast_os_stdlib::service_management::target_unit::TargetUnit],
    tags: ["cast_os_stdlib", "service_management"],
}
