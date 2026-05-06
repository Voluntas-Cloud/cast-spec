//! `atomic_os_update` — switch between immutable system versions.

/// Sentinel for `atomic_os_update`.
pub struct AtomicOsUpdate;

cast::concept! {
    name: "atomic_os_update",
    summary: "switch between immutable system versions.",
    anchors: [cast_os_stdlib::update_evolution::atomic_os_update::AtomicOsUpdate],
    tags: ["cast_os_stdlib", "update_evolution"],
}
