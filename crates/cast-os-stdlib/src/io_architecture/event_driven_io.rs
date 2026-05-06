//! `event_driven_io` — readiness/completion drives execution.

/// Sentinel for `event_driven_io`.
pub struct EventDrivenIo;

cast::concept! {
    name: "event_driven_io",
    summary: "readiness/completion drives execution.",
    anchors: [cast_os_stdlib::io_architecture::event_driven_io::EventDrivenIo],
    tags: ["cast_os_stdlib", "io_architecture"],
}
