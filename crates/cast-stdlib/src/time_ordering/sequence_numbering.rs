//! `sequence_numbering` — explicit order assignment.

/// Sentinel for `sequence_numbering`.
pub struct SequenceNumbering;

cast::concept! {
    name: "sequence_numbering",
    summary: "Assign an explicit order number to every event. A \
              monotonic counter from a single writer beats clocks \
              for ordering and lets readers detect gaps; the cost is \
              having a single writer.",
    anchors: [cast_stdlib::time_ordering::sequence_numbering::SequenceNumbering],
    tags: ["cast_stdlib", "time_ordering"],
}
