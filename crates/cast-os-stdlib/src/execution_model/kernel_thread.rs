//! `kernel_thread` — thread managed directly by the kernel.

/// Sentinel for `kernel_thread`.
pub struct KernelThread;

cast::concept! {
    name: "kernel_thread",
    summary: "thread managed directly by the kernel.",
    anchors: [cast_os_stdlib::execution_model::kernel_thread::KernelThread],
    tags: ["cast_os_stdlib", "execution_model"],
}
