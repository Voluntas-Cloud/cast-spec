//! `scheduling_latency` — delay before task runs.

/// Sentinel for `scheduling_latency`.
pub struct SchedulingLatency;

cast::concept! {
    name: "scheduling_latency",
    summary: "delay before task runs.",
    anchors: [cast_os_stdlib::realtime::scheduling_latency::SchedulingLatency],
    tags: ["cast_os_stdlib", "realtime"],
}
