//! `system_call_interface` — controlled entry from user space into kernel.

/// Sentinel for `system_call_interface`.
pub struct SystemCallInterface;

cast::concept! {
    name: "system_call_interface",
    summary: "controlled entry from user space into kernel.",
    anchors: [cast_os_stdlib::kernel_user_boundary::system_call_interface::SystemCallInterface],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
