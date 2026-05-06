//! `monotonic_time` — non-decreasing process-local time.

/// Sentinel for `monotonic_time`.
pub struct MonotonicTime;

cast::concept! {
    name: "monotonic_time",
    summary: "A process-local clock that never moves backwards. The \
              right tool for measuring how long something took; \
              meaningless for talking about \"when\" across process \
              boundaries.",
    anchors: [cast_stdlib::time_ordering::monotonic_time::MonotonicTime],
    tags: ["cast_stdlib", "time_ordering"],
}
