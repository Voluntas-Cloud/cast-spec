//! `kqueue_event_model` — BSD event notification model.

/// Sentinel for `kqueue_event_model`.
pub struct KqueueEventModel;

cast::concept! {
    name: "kqueue_event_model",
    summary: "BSD event notification model.",
    anchors: [cast_os_stdlib::io_architecture::kqueue_event_model::KqueueEventModel],
    tags: ["cast_os_stdlib", "io_architecture"],
}
