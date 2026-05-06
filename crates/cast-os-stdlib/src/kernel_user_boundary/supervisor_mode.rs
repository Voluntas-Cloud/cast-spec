//! `supervisor_mode` — privileged CPU execution mode.

/// Sentinel for `supervisor_mode`.
pub struct SupervisorMode;

cast::concept! {
    name: "supervisor_mode",
    summary: "privileged CPU execution mode.",
    anchors: [cast_os_stdlib::kernel_user_boundary::supervisor_mode::SupervisorMode],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
