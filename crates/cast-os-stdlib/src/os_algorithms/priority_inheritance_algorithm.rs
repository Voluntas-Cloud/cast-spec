//! `priority_inheritance_algorithm` — propagate priority through lock ownership.

/// Sentinel for `priority_inheritance_algorithm`.
pub struct PriorityInheritanceAlgorithm;

cast::concept! {
    name: "priority_inheritance_algorithm",
    summary: "propagate priority through lock ownership.",
    anchors: [cast_os_stdlib::os_algorithms::priority_inheritance_algorithm::PriorityInheritanceAlgorithm],
    tags: ["cast_os_stdlib", "os_algorithms"],
}
