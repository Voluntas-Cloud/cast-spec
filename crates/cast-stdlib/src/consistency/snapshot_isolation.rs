//! `snapshot_isolation` — readers see a coherent point-in-time view.

/// Sentinel for `snapshot_isolation`.
pub struct SnapshotIsolation;

cast::concept! {
    name: "snapshot_isolation",
    summary: "Each reader sees a consistent point-in-time view of the \
              data: every write that committed before the read started \
              is visible, none that committed after it are. Concurrent \
              writers don't block readers and vice versa — typically \
              implemented over MVCC. Sits between `read_committed` \
              (no anomalies inside a single statement) and \
              `serializable_transaction` (no anomalies across the \
              whole transaction sequence).",
    anchors: [cast_stdlib::consistency::snapshot_isolation::SnapshotIsolation],
    tags: ["cast_stdlib", "consistency"],
}
