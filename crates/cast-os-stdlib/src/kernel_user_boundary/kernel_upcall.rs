//! `kernel_upcall` — kernel notifies user-space service/runtime.

/// Sentinel for `kernel_upcall`.
pub struct KernelUpcall;

cast::concept! {
    name: "kernel_upcall",
    summary: "kernel notifies user-space service/runtime.",
    anchors: [cast_os_stdlib::kernel_user_boundary::kernel_upcall::KernelUpcall],
    tags: ["cast_os_stdlib", "kernel_user_boundary"],
}
