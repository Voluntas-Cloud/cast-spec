//! `copy_on_write_memory` — shared memory copied only on mutation.

/// Sentinel for `copy_on_write_memory`.
pub struct CopyOnWriteMemory;

cast::concept! {
    name: "copy_on_write_memory",
    summary: "shared memory copied only on mutation.",
    anchors: [cast_os_stdlib::memory_management::copy_on_write_memory::CopyOnWriteMemory],
    tags: ["cast_os_stdlib", "memory_management"],
}
