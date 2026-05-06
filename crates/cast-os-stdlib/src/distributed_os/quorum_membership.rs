//! `quorum_membership` — cluster decisions require majority.

/// Sentinel for `quorum_membership`.
pub struct QuorumMembership;

cast::concept! {
    name: "quorum_membership",
    summary: "cluster decisions require majority.",
    anchors: [cast_os_stdlib::distributed_os::quorum_membership::QuorumMembership],
    tags: ["cast_os_stdlib", "distributed_os"],
}
