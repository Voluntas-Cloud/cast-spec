//! `lock_contention_collapse` — performance collapses due to locks.

/// Sentinel for `lock_contention_collapse`.
pub struct LockContentionCollapse;

cast::concept! {
    name: "lock_contention_collapse",
    summary: "performance collapses due to locks.",
    anchors: [cast_os_stdlib::failure_modes::lock_contention_collapse::LockContentionCollapse],
    tags: ["cast_os_stdlib", "failure_modes"],
}
