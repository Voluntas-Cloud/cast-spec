//! `adaptive_control_loop_os` — OS adjusts itself through feedback loops.

/// Sentinel for `adaptive_control_loop_os`.
pub struct AdaptiveControlLoopOs;

cast::concept! {
    name: "adaptive_control_loop_os",
    summary: "OS adjusts itself through feedback loops.",
    anchors: [cast_os_stdlib::architectural_patterns::adaptive_control_loop_os::AdaptiveControlLoopOs],
    tags: ["cast_os_stdlib", "architectural_patterns"],
}
