//! `shared_memory_segment` — memory shared between processes.

/// Sentinel for `shared_memory_segment`.
pub struct SharedMemorySegment;

cast::concept! {
    name: "shared_memory_segment",
    summary: "memory shared between processes.",
    anchors: [cast_os_stdlib::memory_management::shared_memory_segment::SharedMemorySegment],
    tags: ["cast_os_stdlib", "memory_management"],
}
