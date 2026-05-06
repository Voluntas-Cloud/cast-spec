//! `stack_trace_capture` — capture call stack.

/// Sentinel for `stack_trace_capture`.
pub struct StackTraceCapture;

cast::concept! {
    name: "stack_trace_capture",
    summary: "capture call stack.",
    anchors: [cast_os_stdlib::observability::stack_trace_capture::StackTraceCapture],
    tags: ["cast_os_stdlib", "observability"],
}
