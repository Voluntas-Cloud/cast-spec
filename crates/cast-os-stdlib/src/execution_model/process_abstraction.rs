//! `process_abstraction` — isolated executing program with its own resources.

/// Sentinel for `process_abstraction`.
pub struct ProcessAbstraction;

cast::concept! {
    name: "process_abstraction",
    summary: "isolated executing program with its own resources.",
    anchors: [cast_os_stdlib::execution_model::process_abstraction::ProcessAbstraction],
    tags: ["cast_os_stdlib", "execution_model"],
}
