//! `memory_ballooning` — adjust guest VM memory under hypervisor control.

/// Sentinel for `memory_ballooning`.
pub struct MemoryBallooning;

cast::concept! {
    name: "memory_ballooning",
    summary: "adjust guest VM memory under hypervisor control.",
    anchors: [cast_os_stdlib::memory_management::memory_ballooning::MemoryBallooning],
    tags: ["cast_os_stdlib", "memory_management"],
}
