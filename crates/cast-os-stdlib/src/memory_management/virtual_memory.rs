//! `virtual_memory` — process-visible memory abstraction.

/// Sentinel for `virtual_memory`.
pub struct VirtualMemory;

cast::concept! {
    name: "virtual_memory",
    summary: "process-visible memory abstraction.",
    anchors: [cast_os_stdlib::memory_management::virtual_memory::VirtualMemory],
    tags: ["cast_os_stdlib", "memory_management"],
}
