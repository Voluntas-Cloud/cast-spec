//! `physical_memory` — actual RAM.

/// Sentinel for `physical_memory`.
pub struct PhysicalMemory;

cast::concept! {
    name: "physical_memory",
    summary: "actual RAM.",
    anchors: [cast_os_stdlib::memory_management::physical_memory::PhysicalMemory],
    tags: ["cast_os_stdlib", "memory_management"],
}
