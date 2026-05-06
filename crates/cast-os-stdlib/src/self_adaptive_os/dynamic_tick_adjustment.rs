//! `dynamic_tick_adjustment` — alter timer behavior for workload/power.

/// Sentinel for `dynamic_tick_adjustment`.
pub struct DynamicTickAdjustment;

cast::concept! {
    name: "dynamic_tick_adjustment",
    summary: "alter timer behavior for workload/power.",
    anchors: [cast_os_stdlib::self_adaptive_os::dynamic_tick_adjustment::DynamicTickAdjustment],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
