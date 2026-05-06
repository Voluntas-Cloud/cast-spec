//! `select_poll_model` — older readiness multiplexing model.

/// Sentinel for `select_poll_model`.
pub struct SelectPollModel;

cast::concept! {
    name: "select_poll_model",
    summary: "older readiness multiplexing model.",
    anchors: [cast_os_stdlib::io_architecture::select_poll_model::SelectPollModel],
    tags: ["cast_os_stdlib", "io_architecture"],
}
