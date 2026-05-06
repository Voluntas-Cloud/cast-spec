//! `memory_pressure_detection` — detect low-memory state.

/// Sentinel for `memory_pressure_detection`.
pub struct MemoryPressureDetection;

cast::concept! {
    name: "memory_pressure_detection",
    summary: "detect low-memory state.",
    anchors: [cast_os_stdlib::memory_management::memory_pressure_detection::MemoryPressureDetection],
    tags: ["cast_os_stdlib", "memory_management"],
}
