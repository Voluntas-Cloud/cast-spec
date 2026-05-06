//! `monotonic_sequence_id` — authority-issued strictly-increasing ID.

/// Sentinel for `monotonic_sequence_id`.
pub struct MonotonicSequenceId;

cast::concept! {
    name: "monotonic_sequence_id",
    summary: "Assigned ID that also orders. Issued by an authority in \
              strictly increasing order; comparable, dense, small.",
    anchors: [cast_stdlib::identity::monotonic_sequence_id::MonotonicSequenceId],
    tags: ["cast_stdlib", "identity"],
}
