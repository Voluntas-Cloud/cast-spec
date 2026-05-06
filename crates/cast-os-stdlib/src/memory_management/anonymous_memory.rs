//! `anonymous_memory` — memory not backed by named file.

/// Sentinel for `anonymous_memory`.
pub struct AnonymousMemory;

cast::concept! {
    name: "anonymous_memory",
    summary: "memory not backed by named file.",
    anchors: [cast_os_stdlib::memory_management::anonymous_memory::AnonymousMemory],
    tags: ["cast_os_stdlib", "memory_management"],
}
