//! `event_tracing_framework` — OS event tracing.

/// Sentinel for `event_tracing_framework`.
pub struct EventTracingFramework;

cast::concept! {
    name: "event_tracing_framework",
    summary: "OS event tracing.",
    anchors: [cast_os_stdlib::observability::event_tracing_framework::EventTracingFramework],
    tags: ["cast_os_stdlib", "observability"],
}
