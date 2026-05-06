//! `linux_perf_events` — performance counter/tracing system.

/// Sentinel for `linux_perf_events`.
pub struct LinuxPerfEvents;

cast::concept! {
    name: "linux_perf_events",
    summary: "performance counter/tracing system.",
    anchors: [cast_os_stdlib::observability::linux_perf_events::LinuxPerfEvents],
    tags: ["cast_os_stdlib", "observability"],
}
