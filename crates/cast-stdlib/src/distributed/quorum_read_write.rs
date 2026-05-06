//! `quorum_read_write` — require enough replicas for an operation.

/// Sentinel for `quorum_read_write`.
pub struct QuorumReadWrite;

cast::concept! {
    name: "quorum_read_write",
    summary: "Require enough replicas for operation. Read and write \
              quorums are sized so that they overlap; readers always \
              see the most recent committed write.",
    anchors: [cast_stdlib::distributed::quorum_read_write::QuorumReadWrite],
    tags: ["cast_stdlib", "distributed"],
}
