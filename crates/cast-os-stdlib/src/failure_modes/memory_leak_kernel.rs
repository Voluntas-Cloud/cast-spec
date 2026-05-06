//! `memory_leak_kernel` — unreclaimed kernel memory.

/// Sentinel for `memory_leak_kernel`.
pub struct MemoryLeakKernel;

cast::concept! {
    name: "memory_leak_kernel",
    summary: "unreclaimed kernel memory.",
    anchors: [cast_os_stdlib::failure_modes::memory_leak_kernel::MemoryLeakKernel],
    tags: ["cast_os_stdlib", "failure_modes"],
}
