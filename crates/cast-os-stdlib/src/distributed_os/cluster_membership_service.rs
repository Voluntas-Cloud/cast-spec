//! `cluster_membership_service` — track participating nodes.

/// Sentinel for `cluster_membership_service`.
pub struct ClusterMembershipService;

cast::concept! {
    name: "cluster_membership_service",
    summary: "track participating nodes.",
    anchors: [cast_os_stdlib::distributed_os::cluster_membership_service::ClusterMembershipService],
    tags: ["cast_os_stdlib", "distributed_os"],
}
