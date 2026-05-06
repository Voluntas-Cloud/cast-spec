//! `distributed_checkpoint` — consistent snapshot across nodes.

/// Sentinel for `distributed_checkpoint`.
pub struct DistributedCheckpoint;

cast::concept! {
    name: "distributed_checkpoint",
    summary: "Consistent snapshot across nodes. A coherent slice of \
              the cluster's state at a logical moment; recoverable \
              starting point after failure.",
    anchors: [cast_stdlib::distributed::distributed_checkpoint::DistributedCheckpoint],
    tags: ["cast_stdlib", "distributed"],
}
