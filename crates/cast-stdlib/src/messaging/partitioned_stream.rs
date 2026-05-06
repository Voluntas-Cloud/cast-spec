//! `partitioned_stream` — split by key for scaling, ordering preserved per partition.

/// Sentinel for `partitioned_stream`.
pub struct PartitionedStream;

cast::concept! {
    name: "partitioned_stream",
    summary: "Split stream by key for scaling. Messages with the same \
              key go to the same partition; ordering is preserved \
              within a partition only.",
    anchors: [cast_stdlib::messaging::partitioned_stream::PartitionedStream],
    tags: ["cast_stdlib", "messaging"],
}
