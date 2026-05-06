//! `kernel_panic` — unrecoverable kernel failure.

/// Sentinel for `kernel_panic`.
pub struct KernelPanic;

cast::concept! {
    name: "kernel_panic",
    summary: "unrecoverable kernel failure.",
    anchors: [cast_os_stdlib::fault_recovery::kernel_panic::KernelPanic],
    tags: ["cast_os_stdlib", "fault_recovery"],
}
