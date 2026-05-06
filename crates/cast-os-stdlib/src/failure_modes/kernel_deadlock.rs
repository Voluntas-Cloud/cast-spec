//! `kernel_deadlock` — kernel components wait forever.

/// Sentinel for `kernel_deadlock`.
pub struct KernelDeadlock;

cast::concept! {
    name: "kernel_deadlock",
    summary: "kernel components wait forever.",
    anchors: [cast_os_stdlib::failure_modes::kernel_deadlock::KernelDeadlock],
    tags: ["cast_os_stdlib", "failure_modes"],
}
