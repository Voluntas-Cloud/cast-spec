//! `kernel_log_buffer` — kernel message ring buffer.

/// Sentinel for `kernel_log_buffer`.
pub struct KernelLogBuffer;

cast::concept! {
    name: "kernel_log_buffer",
    summary: "kernel message ring buffer.",
    anchors: [cast_os_stdlib::observability::kernel_log_buffer::KernelLogBuffer],
    tags: ["cast_os_stdlib", "observability"],
}
