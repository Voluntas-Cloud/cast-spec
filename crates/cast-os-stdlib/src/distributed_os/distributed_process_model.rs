//! `distributed_process_model` — processes span or migrate across nodes.

/// Sentinel for `distributed_process_model`.
pub struct DistributedProcessModel;

cast::concept! {
    name: "distributed_process_model",
    summary: "processes span or migrate across nodes.",
    anchors: [cast_os_stdlib::distributed_os::distributed_process_model::DistributedProcessModel],
    tags: ["cast_os_stdlib", "distributed_os"],
}
