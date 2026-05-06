//! Observability, debugging, and tracing.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod audit_event_stream;
pub mod core_dump;
pub mod diagnostic_bundle;
pub mod dtrace_dynamic_tracing;
pub mod ebpf_observability_program;
pub mod event_tracing_framework;
pub mod flame_graph_profile;
pub mod health_probe;
pub mod io_latency_trace;
pub mod kernel_crash_dump;
pub mod kernel_log_buffer;
pub mod kprobe_dynamic_probe;
pub mod linux_ftrace;
pub mod linux_perf_events;
pub mod lock_contention_trace;
pub mod scheduler_trace;
pub mod stack_trace_capture;
pub mod system_journal;
pub mod tracepoint_static_probe;
pub mod uprobe_user_probe;
pub mod windows_etw;

cast::concept! {
    name: "observability",
    summary: "Umbrella for the observability stdlib category. \
              Observability, debugging, and tracing.",
    anchors: [
        crate::observability::audit_event_stream,
        crate::observability::core_dump,
        crate::observability::diagnostic_bundle,
        crate::observability::dtrace_dynamic_tracing,
        crate::observability::ebpf_observability_program,
        crate::observability::event_tracing_framework,
        crate::observability::flame_graph_profile,
        crate::observability::health_probe,
        crate::observability::io_latency_trace,
        crate::observability::kernel_crash_dump,
        crate::observability::kernel_log_buffer,
        crate::observability::kprobe_dynamic_probe,
        crate::observability::linux_ftrace,
        crate::observability::linux_perf_events,
        crate::observability::lock_contention_trace,
        crate::observability::scheduler_trace,
        crate::observability::stack_trace_capture,
        crate::observability::system_journal,
        crate::observability::tracepoint_static_probe,
        crate::observability::uprobe_user_probe,
        crate::observability::windows_etw,
    ],
    tags: ["cast_os_stdlib", "observability"],
}

/// Sentinel for the observability stdlib group.
pub struct ObservabilityGroup;
