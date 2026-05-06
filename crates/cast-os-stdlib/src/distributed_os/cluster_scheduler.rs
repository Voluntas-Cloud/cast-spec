//! `cluster_scheduler` — schedule workloads across nodes.

/// Sentinel for `cluster_scheduler`.
pub struct ClusterScheduler;

cast::concept! {
    name: "cluster_scheduler",
    summary: "schedule workloads across nodes.",
    anchors: [cast_os_stdlib::distributed_os::cluster_scheduler::ClusterScheduler],
    tags: ["cast_os_stdlib", "distributed_os"],
}
