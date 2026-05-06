//! `partition_tolerance` — continue under network split.

/// Sentinel for `partition_tolerance`.
pub struct PartitionTolerance;

cast::concept! {
    name: "partition_tolerance",
    summary: "Continue under network split. The system keeps making \
              progress on at least one side of the partition; the \
              other side either degrades or stops.",
    anchors: [cast_stdlib::distributed::partition_tolerance::PartitionTolerance],
    tags: ["cast_stdlib", "distributed"],
}
