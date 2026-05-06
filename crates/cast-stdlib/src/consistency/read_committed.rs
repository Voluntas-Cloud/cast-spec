//! `read_committed` — reads only committed data, no dirty reads.

/// Sentinel for `read_committed`.
pub struct ReadCommitted;

cast::concept! {
    name: "read_committed",
    summary: "Reads only committed data. Avoids dirty reads; allows \
              non-repeatable reads and phantoms.",
    anchors: [cast_stdlib::consistency::read_committed::ReadCommitted],
    tags: ["cast_stdlib", "consistency"],
}
