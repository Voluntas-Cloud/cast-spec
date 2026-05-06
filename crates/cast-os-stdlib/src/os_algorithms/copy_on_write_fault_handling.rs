//! `copy_on_write_fault_handling` — duplicate page on write.

/// Sentinel for `copy_on_write_fault_handling`.
pub struct CopyOnWriteFaultHandling;

cast::concept! {
    name: "copy_on_write_fault_handling",
    summary: "duplicate page on write.",
    anchors: [cast_os_stdlib::os_algorithms::copy_on_write_fault_handling::CopyOnWriteFaultHandling],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
