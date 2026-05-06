//! `lock_contention_trace` — inspect lock bottlenecks.

/// Sentinel for `lock_contention_trace`.
pub struct LockContentionTrace;

cast::concept! {
    name: "lock_contention_trace",
    summary: "inspect lock bottlenecks.",
    anchors: [cast_os_stdlib::observability::lock_contention_trace::LockContentionTrace],
    tags: ["cast_os_stdlib", "observability"],
}
