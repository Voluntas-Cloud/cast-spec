//! `control_loop_os_management` — OS behavior driven by control loops.

/// Sentinel for `control_loop_os_management`.
pub struct ControlLoopOsManagement;

cast::concept! {
    name: "control_loop_os_management",
    summary: "OS behavior driven by control loops.",
    anchors: [cast_os_stdlib::self_adaptive_os::control_loop_os_management::ControlLoopOsManagement],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
