//! `kernel_space` — privileged execution environment.

/// Sentinel for `kernel_space`.
pub struct KernelSpace;

cast::concept! {
    name: "kernel_space",
    summary: "privileged execution environment.",
    anchors: [cast_os_stdlib::kernel_user_boundary::kernel_space::KernelSpace],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
