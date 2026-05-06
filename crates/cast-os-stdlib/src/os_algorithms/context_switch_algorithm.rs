//! `context_switch_algorithm` — switch CPU execution between tasks.

/// Sentinel for `context_switch_algorithm`.
pub struct ContextSwitchAlgorithm;

cast::concept! {
    name: "context_switch_algorithm",
    summary: "switch CPU execution between tasks.",
    anchors: [cast_os_stdlib::os_algorithms::context_switch_algorithm::ContextSwitchAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
