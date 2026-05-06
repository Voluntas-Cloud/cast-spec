//! `memory_ballooning` — adjust guest memory dynamically.

/// Sentinel for `memory_ballooning`.
pub struct MemoryBallooning;

cast::concept! {
    name: "memory_ballooning",
    summary: "adjust guest memory dynamically.",
    anchors: [cast_os_stdlib::virtualization::memory_ballooning::MemoryBallooning],
    tags: ["cast_os_stdlib", "virtualization"],
}
