//! `memory_mapped_ipc` — mapped region used for communication.

/// Sentinel for `memory_mapped_ipc`.
pub struct MemoryMappedIpc;

cast::concept! {
    name: "memory_mapped_ipc",
    summary: "mapped region used for communication.",
    anchors: [cast_os_stdlib::ipc::memory_mapped_ipc::MemoryMappedIpc],
    tags: ["cast_os_stdlib", "ipc"],
}
