//! `kernel_crash_dump` — kernel memory dump after panic.

/// Sentinel for `kernel_crash_dump`.
pub struct KernelCrashDump;

cast::concept! {
    name: "kernel_crash_dump",
    summary: "kernel memory dump after panic.",
    anchors: [cast_os_stdlib::observability::kernel_crash_dump::KernelCrashDump],
    tags: ["cast_os_stdlib", "observability"],
}
