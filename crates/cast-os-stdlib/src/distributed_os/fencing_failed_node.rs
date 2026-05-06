//! `fencing_failed_node` — prevent failed/partitioned node from corrupting state.

/// Sentinel for `fencing_failed_node`.
pub struct FencingFailedNode;

cast::concept! {
    name: "fencing_failed_node",
    summary: "prevent failed/partitioned node from corrupting state.",
    anchors: [cast_os_stdlib::distributed_os::fencing_failed_node::FencingFailedNode],
    tags: ["cast_os_stdlib", "distributed_os"],
}
