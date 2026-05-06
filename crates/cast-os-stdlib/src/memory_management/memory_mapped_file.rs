//! `memory_mapped_file` — file exposed as memory.

/// Sentinel for `memory_mapped_file`.
pub struct MemoryMappedFile;

cast::concept! {
    name: "memory_mapped_file",
    summary: "file exposed as memory.",
    anchors: [cast_os_stdlib::memory_management::memory_mapped_file::MemoryMappedFile],
    tags: ["cast_os_stdlib", "memory_management"],
}
