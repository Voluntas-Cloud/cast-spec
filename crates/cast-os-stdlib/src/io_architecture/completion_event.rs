//! `completion_event` — notification that async operation completed.

/// Sentinel for `completion_event`.
pub struct CompletionEvent;

cast::concept! {
    name: "completion_event",
    summary: "notification that async operation completed.",
    anchors: [cast_os_stdlib::io_architecture::completion_event::CompletionEvent],
    tags: ["cast_os_stdlib", "io_architecture"],
}
