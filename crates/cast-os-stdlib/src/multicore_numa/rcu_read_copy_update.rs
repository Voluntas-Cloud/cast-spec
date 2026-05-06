//! `rcu_read_copy_update` — low-overhead concurrent read pattern.

/// Sentinel for `rcu_read_copy_update`.
pub struct RcuReadCopyUpdate;

cast::concept! {
    name: "rcu_read_copy_update",
    summary: "low-overhead concurrent read pattern.",
    anchors: [cast_os_stdlib::multicore_numa::rcu_read_copy_update::RcuReadCopyUpdate],
    tags: ["cast_os_stdlib", "multicore_numa"],
}
