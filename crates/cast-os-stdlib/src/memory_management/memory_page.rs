//! `memory_page` — virtual memory unit.

/// Sentinel for `memory_page`.
pub struct MemoryPage;

cast::concept! {
    name: "memory_page",
    summary: "virtual memory unit.",
    anchors: [cast_os_stdlib::memory_management::memory_page::MemoryPage],
    tags: ["cast_os_stdlib", "memory_management"],
}
