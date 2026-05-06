//! `thread_abstraction` — schedulable execution flow within a process.

/// Sentinel for `thread_abstraction`.
pub struct ThreadAbstraction;

cast::concept! {
    name: "thread_abstraction",
    summary: "schedulable execution flow within a process.",
    anchors: [cast_os_stdlib::execution_model::thread_abstraction::ThreadAbstraction],
    tags: ["cast_os_stdlib", "execution_model"],
}
