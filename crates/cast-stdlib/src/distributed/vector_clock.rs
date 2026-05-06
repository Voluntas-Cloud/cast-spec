//! `vector_clock` — track causal relationships.

/// Sentinel for `vector_clock`.
pub struct VectorClock;

cast::concept! {
    name: "vector_clock",
    summary: "Track causal relationships. Each node maintains a \
              counter per peer; comparing vectors reveals which events \
              happened-before, after, or concurrently.",
    anchors: [cast_stdlib::distributed::vector_clock::VectorClock],
    tags: ["cast_stdlib", "distributed"],
}
