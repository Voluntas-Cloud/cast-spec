//! `snapshot_test` — compare output to stored expected result.

/// Sentinel for `snapshot_test`.
pub struct SnapshotTest;

cast::concept! {
    name: "snapshot_test",
    summary: "Compare output to stored expected result. Cheap to write, \
              dangerous to maintain — easy to update the snapshot \
              instead of investigating the diff.",
    anchors: [cast_stdlib::testing::snapshot_test::SnapshotTest],
    tags: ["cast_stdlib", "testing"],
}
