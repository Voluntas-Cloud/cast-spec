//! `dtrace_dynamic_tracing` — dynamic tracing system.

/// Sentinel for `dtrace_dynamic_tracing`.
pub struct DtraceDynamicTracing;

cast::concept! {
    name: "dtrace_dynamic_tracing",
    summary: "dynamic tracing system.",
    anchors: [cast_os_stdlib::observability::dtrace_dynamic_tracing::DtraceDynamicTracing],
    tags: ["cast_os_stdlib", "observability"],
}
