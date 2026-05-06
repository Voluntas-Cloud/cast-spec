//! `read_replica` — copy optimized for reads.

/// Sentinel for `read_replica`.
pub struct ReadReplica;

cast::concept! {
    name: "read_replica",
    summary: "Copy optimized for reads. Lags the primary by some \
              delay; offloads read traffic at the cost of stale-read \
              tolerance from consumers.",
    anchors: [cast_stdlib::distributed::read_replica::ReadReplica],
    tags: ["cast_stdlib", "distributed"],
}
