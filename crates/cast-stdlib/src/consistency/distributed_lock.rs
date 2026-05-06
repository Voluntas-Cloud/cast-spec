//! `distributed_lock` — lock across multiple processes/nodes.

/// Sentinel for `distributed_lock`.
pub struct DistributedLock;

cast::concept! {
    name: "distributed_lock",
    summary: "Lock across multiple processes/nodes. Requires consensus \
              or a leader to issue grants; loss of the leader is the \
              failure mode to plan for.",
    anchors: [cast_stdlib::consistency::distributed_lock::DistributedLock],
    tags: ["cast_stdlib", "consistency"],
}
