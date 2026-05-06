//! `process_tree` — parent/child process relationship graph.

/// Sentinel for `process_tree`.
pub struct ProcessTree;

cast::concept! {
    name: "process_tree",
    summary: "parent/child process relationship graph.",
    anchors: [cast_os_stdlib::execution_model::process_tree::ProcessTree],
    tags: ["cast_os_stdlib", "execution_model"],
}
