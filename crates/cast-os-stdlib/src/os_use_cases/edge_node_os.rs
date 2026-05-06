//! `edge_node_os` — remote/edge compute node OS.

/// Sentinel for `edge_node_os`.
pub struct EdgeNodeOs;

cast::concept! {
    name: "edge_node_os",
    summary: "remote/edge compute node OS.",
    anchors: [cast_os_stdlib::os_use_cases::edge_node_os::EdgeNodeOs],
    tags: ["cast_os_stdlib", "os_use_cases"],
}
