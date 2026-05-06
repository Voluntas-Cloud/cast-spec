//! `atomic_operation` — hardware-supported indivisible operation.

/// Sentinel for `atomic_operation`.
pub struct AtomicOperation;

cast::concept! {
    name: "atomic_operation",
    summary: "hardware-supported indivisible operation.",
    anchors: [cast_os_stdlib::multicore_numa::atomic_operation::AtomicOperation],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
